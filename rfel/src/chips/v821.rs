use log::debug;

use crate::{Fel, read_all, write_all};

use super::util::{exec_stub, read32_via_payload, u32_params_le, write32_via_payload};
use super::{Chip, ChipError, ChipSpi, DdrProfile, SpiContext, payload};

pub struct V821;

const DDR_PAYLOAD_BASE: u32 = 0x0200_8000;
const DDR_PARAM_ADDR: u32 = 0x0200_8038;
const SPI_PAYLOAD_BASE: u32 = 0x0200_0000;
const SPI_COMMAND_BASE: u32 = 0x0200_1000;
const SPI_SWAP_BASE: u32 = 0x0200_2000;
/// Size of the valid V821 BootROM image in bytes.
pub const BOOTROM_SIZE: u32 = 0x0000_c000;
const BOOTROM_COPY_BASE: u32 = 0x0200_2000;
const BOOTROM_COPY_CHUNK: usize = 4096;

fn read_bootrom(fel: &Fel<'_>, mut address: u32, mut out: &mut [u8]) -> Result<(), ChipError> {
    while !out.is_empty() {
        let requested = out.len().min(BOOTROM_COPY_CHUNK);
        let aligned_address = address & !3;
        let leading = (address - aligned_address) as usize;
        let copy_len = (leading + requested + 3) & !3;
        let params = u32_params_le(&[aligned_address, BOOTROM_COPY_BASE, copy_len as u32]);

        exec_stub(fel, payload::COPY_V821, &params, 0)?;
        let mut copied = vec![0u8; copy_len];
        read_all(fel, BOOTROM_COPY_BASE, &mut copied)?;
        out[..requested].copy_from_slice(&copied[leading..leading + requested]);

        address += requested as u32;
        out = &mut out[requested..];
    }
    Ok(())
}

fn read32(fel: &Fel<'_>, addr: u32) -> Result<u32, ChipError> {
    read32_via_payload(fel, payload::READ32_V821, addr)
}

fn write32(fel: &Fel<'_>, addr: u32, value: u32) -> Result<(), ChipError> {
    write32_via_payload(fel, payload::WRITE32_V821, addr, value)
}

impl Chip for V821 {
    fn name(&self) -> String {
        "V821".to_string()
    }

    fn read_memory(&self, fel: &Fel<'_>, address: u32, out: &mut [u8]) -> Result<(), ChipError> {
        let bootrom_len = if address < BOOTROM_SIZE {
            out.len().min((BOOTROM_SIZE - address) as usize)
        } else {
            0
        };
        let (bootrom, direct) = out.split_at_mut(bootrom_len);
        if !bootrom.is_empty() {
            debug!(
                "reading V821 BootROM 0x{address:08x}..0x{:08x} through CPU copy helper",
                address + bootrom.len() as u32
            );
            read_bootrom(fel, address, bootrom)?;
        }
        if !direct.is_empty() {
            read_all(fel, address.wrapping_add(bootrom_len as u32), direct)?;
        }
        Ok(())
    }

    fn reset(&self, fel: &Fel<'_>) -> Result<(), ChipError> {
        const RTC_VDD_OFF_GATING: u32 = 0x4a00_001c;
        const WATCHDOG_CONFIG: u32 = 0x4a00_1018;
        const WATCHDOG_MODE: u32 = 0x4a00_1008;

        let value = read32(fel, RTC_VDD_OFF_GATING)? | (1 << 3);
        write32(fel, RTC_VDD_OFF_GATING, value)?;
        write32(fel, WATCHDOG_CONFIG, 0x16aa << 16)?;
        write32(fel, WATCHDOG_MODE, (0x16aa << 16) | 1)
    }

    fn sid(&self, fel: &Fel<'_>) -> Result<Vec<u8>, ChipError> {
        const SID_BASE: u32 = 0x4300_6200;

        let mut sid = Vec::with_capacity(16);
        for offset in [0, 4, 8, 12] {
            // xfel renders each SID register as an eight-digit hexadecimal word.
            sid.extend_from_slice(&read32(fel, SID_BASE + offset)?.to_be_bytes());
        }
        Ok(sid)
    }

    fn jtag(&self, fel: &Fel<'_>, enable: bool) -> Result<(), ChipError> {
        if !enable {
            return Err(ChipError::Unsupported("disable jtag not implemented"));
        }

        const GPIOC_CFG0: u32 = 0x4200_0060;
        const JTAG_FUNCTION: u32 = 3;

        for pin in [0, 5] {
            let shift = (pin & 7) * 4;
            let value = (read32(fel, GPIOC_CFG0)? & !(0xf << shift)) | (JTAG_FUNCTION << shift);
            write32(fel, GPIOC_CFG0, value)?;
        }
        Ok(())
    }

    fn ddr(&self, fel: &Fel<'_>, profile: Option<DdrProfile>) -> Result<(), ChipError> {
        if profile.is_some() {
            return Err(ChipError::Unsupported("V821 does not use a DDR profile"));
        }

        let params: [u32; 24] = [
            528,         // dram_clk
            2,           // dram_type
            0x007b_7bf9, // dram_zq
            0,           // dram_odt_en
            0x0000_00d2, // dram_para1
            0x0040_0000, // dram_para2
            0x0000_0e73, // dram_mr0
            0x0000_0002, // dram_mr1
            0,           // dram_mr2
            0,           // dram_mr3
            0x0047_1992, // dram_tpr0
            0x0131_a10c, // dram_tpr1
            0x0005_7041, // dram_tpr2
            0xb478_7896, // dram_tpr3
            0,           // dram_tpr4
            0x4848_4848, // dram_tpr5
            0x0000_0048, // dram_tpr6
            0x1621_121e, // dram_tpr7
            0,           // dram_tpr8
            0,           // dram_tpr9
            0,           // dram_tpr10
            0,           // dram_tpr11
            0,           // dram_tpr12
            0x3400_0100, // dram_tpr13
        ];

        debug!(
            "V821 DDR: payload @0x{DDR_PAYLOAD_BASE:08x} ({} bytes), params @0x{DDR_PARAM_ADDR:08x}",
            payload::DDR_INIT_V821.len()
        );
        write_all(fel, DDR_PAYLOAD_BASE, payload::DDR_INIT_V821)?;
        write_all(fel, DDR_PARAM_ADDR, &u32_params_le(&params))?;
        fel.exec(DDR_PAYLOAD_BASE)?;
        Ok(())
    }

    fn as_spi(&self) -> Option<&dyn ChipSpi> {
        Some(self)
    }
}

impl ChipSpi for V821 {
    fn spi_init(&self, fel: &Fel<'_>) -> Result<SpiContext, ChipError> {
        debug!(
            "loading V821 SPI helper at 0x{SPI_PAYLOAD_BASE:08x} ({} bytes)",
            payload::SPI_INIT_V821.len()
        );
        write_all(fel, SPI_PAYLOAD_BASE, payload::SPI_INIT_V821)?;
        Ok(SpiContext {
            payload_base: SPI_PAYLOAD_BASE,
            command_base: SPI_COMMAND_BASE,
            command_len: 4096,
            swap_base: SPI_SWAP_BASE,
            swap_len: 98_304,
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
    fn test_v821_layout() {
        assert_eq!(V821.name(), "V821");
        assert_eq!(DDR_PARAM_ADDR, DDR_PAYLOAD_BASE + 0x38);
        assert_eq!(SPI_COMMAND_BASE, SPI_PAYLOAD_BASE + 0x1000);
        assert_eq!(SPI_SWAP_BASE, SPI_PAYLOAD_BASE + 0x2000);
        assert_eq!(payload::READ32_V821.len(), 44);
        assert_eq!(payload::WRITE32_V821.len(), 44);
        assert_eq!(payload::COPY_V821.len(), 68);
        assert_eq!(payload::DDR_INIT_V821.len(), 14_976);
        assert_eq!(payload::SPI_INIT_V821.len(), 1_206);
    }

    #[test]
    fn test_bootrom_read_boundaries() {
        assert_eq!(BOOTROM_SIZE, 48 * 1024);
        assert_eq!(BOOTROM_COPY_CHUNK % 4, 0);
        assert_eq!(BOOTROM_COPY_BASE % 4, 0);
    }
}
