//! V861/V881 FEL E907 support, following xfel's chips/v881.c.
use super::util::{read32_via_payload, write32_via_payload};
use super::{Chip, ChipError, ChipSpi, DdrProfile, SpiContext, payload};
use crate::{Fel, read_all, write_all};

pub struct V861;

const SPI_PAYLOAD_BASE: u32 = 0x0010_0000;
const SPI_COMMAND_BASE: u32 = 0x0010_1000;
const SPI_STATUS: u32 = 0x0010_1ffc;
const SPI_SWAP_BASE: u32 = 0x0010_2000;

impl V861 {
    fn read32(&self, fel: &Fel<'_>, address: u32) -> Result<u32, ChipError> {
        if address & 3 != 0 {
            return Err(ChipError::Unsupported("unaligned register address"));
        }
        read32_via_payload(fel, payload::READ32_RV32, address)
    }

    pub(crate) fn write32(&self, fel: &Fel<'_>, address: u32, value: u32) -> Result<(), ChipError> {
        if address & 3 != 0 {
            return Err(ChipError::Unsupported("unaligned register address"));
        }
        write32_via_payload(fel, payload::WRITE32_RV32, address, value)
    }
}

impl Chip for V861 {
    fn name(&self) -> String {
        "V861/V881".into()
    }

    fn read_memory(&self, fel: &Fel<'_>, address: u32, out: &mut [u8]) -> Result<(), ChipError> {
        // Match xfel: register-sized reads execute on the E907; bulk memory
        // transfers continue to use the ROM's FEL read command.
        if out.len() == 4 && address & 3 == 0 {
            out.copy_from_slice(&self.read32(fel, address)?.to_le_bytes());
        } else {
            read_all(fel, address, out)?;
        }
        Ok(())
    }

    fn reset(&self, fel: &Fel<'_>) -> Result<(), ChipError> {
        let value = self.read32(fel, 0x0709_02a0)?;
        self.write32(fel, 0x0709_02a0, value | (1 << 1) | (0x429b << 16))?;
        self.write32(fel, 0x0800_9018, 0x16aa << 16)?;
        self.write32(fel, 0x0800_9018, (0x16aa << 16) | 0x21)
    }

    fn sid(&self, fel: &Fel<'_>) -> Result<Vec<u8>, ChipError> {
        let mut sid = Vec::with_capacity(16);
        for offset in [0, 4, 8, 12] {
            sid.extend_from_slice(&self.read32(fel, 0x0709_1200 + offset)?.to_be_bytes());
        }
        Ok(sid)
    }

    fn jtag(&self, _: &Fel<'_>, _: bool) -> Result<(), ChipError> {
        Err(ChipError::Unsupported(
            "V861/V881 JTAG setup is not supported by the reference implementation",
        ))
    }

    fn ddr(&self, _: &Fel<'_>, _: Option<DdrProfile>) -> Result<(), ChipError> {
        Err(ChipError::Unsupported(
            "V861/V881 DDR initialization is not supported; SPI NOR uses SRAM only",
        ))
    }

    fn as_spi(&self) -> Option<&dyn ChipSpi> {
        Some(self)
    }
}

impl ChipSpi for V861 {
    fn spi_init(&self, fel: &Fel<'_>) -> Result<SpiContext, ChipError> {
        write_all(fel, SPI_PAYLOAD_BASE, payload::SPI_INIT_V861)?;
        Ok(SpiContext {
            payload_base: SPI_PAYLOAD_BASE,
            command_base: SPI_COMMAND_BASE,
            command_len: 256,
            swap_base: SPI_SWAP_BASE,
            swap_len: 65536,
        })
    }

    fn spi_run(
        &self,
        fel: &Fel<'_>,
        context: &SpiContext,
        commands: &[u8],
    ) -> Result<(), ChipError> {
        if commands.is_empty() || commands.len() > 256 || commands.last() != Some(&0) {
            return Err(ChipError::Unsupported("invalid V861 SPI command buffer"));
        }
        // Sentinel distinguishes a helper that failed to publish its result.
        write_all(fel, SPI_STATUS, &i32::MIN.to_le_bytes())?;
        write_all(fel, context.command_base, commands)?;
        fel.exec(context.payload_base)?;
        let mut status = [0; 4];
        read_all(fel, SPI_STATUS, &mut status)?;
        check_status(i32::from_le_bytes(status))
    }
}

fn check_status(status: i32) -> Result<(), ChipError> {
    match status {
        0 => Ok(()),
        -1 => Err(ChipError::Unsupported(
            "V861 SPIF requires single-lane SPI NOR with three-byte addresses and transfers up to 64 KiB",
        )),
        -2 | -3 => Err(ChipError::Other("V861 SPIF reset timed out")),
        -4 | -6 => Err(ChipError::Other("V861 SPIF transfer timed out")),
        -5 => Err(ChipError::Other("V861 SPIF DMA error")),
        _ => Err(ChipError::Other(
            "V861 SPIF payload did not report a valid result",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_status() {
        assert!(SPI_PAYLOAD_BASE + payload::SPI_INIT_V861.len() as u32 <= SPI_COMMAND_BASE);
        assert_eq!(SPI_STATUS + 4, SPI_SWAP_BASE);
        const { assert!(SPI_SWAP_BASE + 65536 <= 0x0011_fc00) };
        assert!(check_status(0).is_ok());
        for status in [-1, -2, -3, -4, -5, -6, i32::MIN] {
            assert!(check_status(status).is_err());
        }
    }
    #[test]
    fn detects_hardware_version() {
        let mut version = [0; 32];
        version[..8].copy_from_slice(b"AWUSBFEX");
        version[8..12].copy_from_slice(&0x0019_1800u32.to_le_bytes());
        version[20..24].copy_from_slice(&0x0011_fc00u32.to_le_bytes());
        let version = crate::Version::from(version);
        assert!(matches!(version.chip(), Some(crate::Chip::V861)));
        assert_eq!(version.scratchpad(), 0x0011_fc00);
    }
}
