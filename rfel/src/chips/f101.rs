//! F101 FEL support, following xfel v1.3.6 `chips/f101.c`.
//!
//! Register helpers run at the scratchpad reported by the BootROM. DDR and SPI
//! helpers use the host-selected SRAM layout; SPI commands follow the entry by 0x1000.

use log::debug;
use std::time::{Duration, Instant};

use crate::{Fel, write_all};

use super::util::{exec_stub, read32_via_payload, write32_via_payload};
use super::{Chip, ChipError, ChipSpi, DdrProfile, SpiContext, payload};

pub struct F101;

const DDR_PAYLOAD_BASE: u32 = 0x0002_8000;
const SPI_PAYLOAD_BASE: u32 = 0x0002_8000;
const SPI_COMMAND_BASE: u32 = 0x0002_9000;
const SPI_COMMAND_LEN: u32 = 4096;
const SPI_SWAP_BASE: u32 = 0x0002_a000;
const SPI_SWAP_LEN: u32 = 8192;

fn read32(fel: &Fel<'_>, address: u32) -> Result<u32, ChipError> {
    read32_via_payload(fel, payload::READ32_RV32, address)
}

fn write32(fel: &Fel<'_>, address: u32, value: u32) -> Result<(), ChipError> {
    write32_via_payload(fel, payload::WRITE32_RV32, address, value)
}

fn efuse_read(fel: &Fel<'_>, offset: u32) -> Result<u32, ChipError> {
    const SID_CONTROL: u32 = 0x0300_6000;
    const SID_ADDRESS: u32 = 0x0300_6004;
    const SID_READ_KEY: u32 = 0x0300_600c;
    const READ_START: u32 = 1 << 1;
    const READ_UNLOCK: u32 = 0xadbf << 16;

    // FEL owns the controller while this helper runs. Clear both command bits
    // and the old key before issuing a read; never enable eFuse programming.
    write32(fel, SID_ADDRESS, offset / 4)?;
    let control = read32(fel, SID_CONTROL)? & !((0xffff << 16) | 0x3);
    write32(fel, SID_CONTROL, control | READ_UNLOCK | READ_START)?;
    let start = Instant::now();
    while read32(fel, SID_CONTROL)? & READ_START != 0 {
        if start.elapsed() >= Duration::from_secs(1) {
            write32(fel, SID_CONTROL, control)?;
            return Err(ChipError::Other("F101 SID read timed out"));
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    write32(fel, SID_CONTROL, control)?;
    read32(fel, SID_READ_KEY)
}

fn ddr_payload(profile: Option<DdrProfile>) -> Result<&'static [u8], ChipError> {
    match profile {
        Some(DdrProfile::F101S2) => Ok(payload::DDR_INIT_F101_S2),
        Some(DdrProfile::F101S3) => Ok(payload::DDR_INIT_F101_S3),
        _ => Err(ChipError::Unsupported(
            "F101 requires a PSRAM profile: rfel ddr --profile f101-s2 | f101-s3",
        )),
    }
}

impl Chip for F101 {
    fn name(&self) -> String {
        "F101".to_string()
    }

    fn reset(&self, fel: &Fel<'_>) -> Result<(), ChipError> {
        const WATCHDOG_MODE: u32 = 0x0205_00a8;
        write32(fel, WATCHDOG_MODE, (0x16aa << 16) | 1)
    }

    fn sid(&self, fel: &Fel<'_>) -> Result<Vec<u8>, ChipError> {
        let mut sid = Vec::with_capacity(16);
        for offset in [0, 4, 8, 12] {
            // Preserve xfel's hexadecimal word order when the CLI prints bytes.
            sid.extend_from_slice(&efuse_read(fel, offset)?.to_be_bytes());
        }
        Ok(sid)
    }

    fn jtag(&self, fel: &Fel<'_>, enable: bool) -> Result<(), ChipError> {
        if !enable {
            return Err(ChipError::Unsupported("disable jtag not implemented"));
        }

        const GPIOF_CFG0: u32 = 0x0200_00f0;
        // PF0, PF1, PF3 and PF5 use function 4 for JTAG. Preserve other pins.
        let mut value = read32(fel, GPIOF_CFG0)?;
        for pin in [0, 1, 3, 5] {
            let shift = pin * 4;
            value = (value & !(0xf << shift)) | (4 << shift);
        }
        write32(fel, GPIOF_CFG0, value)
    }

    fn ddr(&self, fel: &Fel<'_>, profile: Option<DdrProfile>) -> Result<(), ChipError> {
        let payload = ddr_payload(profile)?;
        debug!(
            "F101 PSRAM: payload @0x{DDR_PAYLOAD_BASE:08x} ({} bytes)",
            payload.len()
        );
        write_all(fel, DDR_PAYLOAD_BASE, payload)?;
        // SPI and PSRAM share this address. The boot image has no initial
        // fence.i, so invalidate cached SPI instructions from the scratchpad
        // before entering the new image.
        exec_stub(fel, payload::FENCE_I_F101, &[], 0)?;
        fel.exec(DDR_PAYLOAD_BASE)?;
        Ok(())
    }

    fn as_spi(&self) -> Option<&dyn ChipSpi> {
        Some(self)
    }
}

impl ChipSpi for F101 {
    fn spi_init(&self, fel: &Fel<'_>) -> Result<SpiContext, ChipError> {
        debug!(
            "loading F101 SPI helper at 0x{SPI_PAYLOAD_BASE:08x} ({} bytes)",
            payload::SPI_INIT_F101.len()
        );
        write_all(fel, SPI_PAYLOAD_BASE, payload::SPI_INIT_F101)?;
        // The old PSRAM entry can still be cached here and bypass the SPI
        // helper's own fence.i. Synchronize from the scratchpad in both orders.
        exec_stub(fel, payload::FENCE_I_F101, &[], 0)?;
        Ok(SpiContext {
            payload_base: SPI_PAYLOAD_BASE,
            command_base: SPI_COMMAND_BASE,
            command_len: SPI_COMMAND_LEN,
            swap_base: SPI_SWAP_BASE,
            swap_len: SPI_SWAP_LEN,
        })
    }

    fn spi_run(
        &self,
        fel: &Fel<'_>,
        context: &SpiContext,
        commands: &[u8],
    ) -> Result<(), ChipError> {
        if commands.len() > context.command_len as usize {
            return Err(ChipError::Unsupported("spi command buffer exceeds limit"));
        }
        write_all(fel, context.command_base, commands)?;
        fel.exec(context.payload_base)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psram_profiles() {
        // These are distinct complete boot images with their parameters embedded.
        let s2 = ddr_payload(Some(DdrProfile::F101S2)).unwrap();
        let s3 = ddr_payload(Some(DdrProfile::F101S3)).unwrap();
        assert_ne!(s2, s3);
        for image in [s2, s3] {
            assert_eq!(&image[4..12], b"eGON.BT0");
            assert_eq!(
                u32::from_le_bytes(image[16..20].try_into().unwrap()) as usize,
                image.len()
            );
            // The images encode their SRAM load address in the boot header.
            assert_eq!(
                u32::from_le_bytes(image[32..36].try_into().unwrap()),
                DDR_PAYLOAD_BASE
            );
            assert_eq!(image.len(), 18_560);
        }
        for profile in [None, Some(DdrProfile::D1), Some(DdrProfile::F133)] {
            assert!(matches!(
                ddr_payload(profile),
                Err(ChipError::Unsupported(_))
            ));
        }
    }

    #[test]
    fn test_spi_payload_fits_sram_layout() {
        assert!(SPI_PAYLOAD_BASE + payload::SPI_INIT_F101.len() as u32 <= SPI_COMMAND_BASE);
        assert_eq!(SPI_COMMAND_BASE, SPI_PAYLOAD_BASE + 0x1000);
        assert_eq!(SPI_COMMAND_BASE + SPI_COMMAND_LEN, SPI_SWAP_BASE);
        // xfel's F101 helper reserves only 8 KiB for data, unlike D1/V821.
        assert_eq!(SPI_SWAP_BASE + SPI_SWAP_LEN, 0x0002_c000);
    }

    #[test]
    fn test_instruction_sync_replaces_cached_register_helper() {
        // Every scratchpad helper must execute the same prefix through fence.i
        // before fetching its replacement's instructions.
        let prefix = &payload::FENCE_I_F101[..12];
        assert_eq!(prefix, &payload::READ32_RV32[..12]);
        assert_eq!(prefix, &payload::WRITE32_RV32[..12]);
        assert_eq!(&prefix[8..12], &0x0000_100fu32.to_le_bytes());
    }
}
