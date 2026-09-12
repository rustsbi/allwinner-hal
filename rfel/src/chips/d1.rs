use log::debug;

use crate::{Fel, write_all};

use super::util::{read32_via_stub, write32_via_stub};
use super::{Chip, ChipError, ChipSpi, DdrProfile, SpiContext, ddr, payload};

pub struct D1;

const D1_SRAM_BASE: u32 = 0x0002_0000;
const SPI_PAYLOAD_BASE: u32 = 0x0002_0000;
const SPI_COMMAND_BASE: u32 = 0x0002_1000;
const SPI_SWAP_BASE: u32 = 0x0002_2000;

impl Chip for D1 {
    fn name(&self) -> String {
        "D1/F133".to_string()
    }

    /// Note: This function hasn't gone through comprehensive upper-level testing yet.
    fn reset(&self, fel: &Fel<'_>) -> Result<(), ChipError> {
        // Write watchdog reset register via write32 stub
        const RESET_REG: u32 = 0x0205_00A8; // 0x020500a0 + 0x08
        const RESET_VAL: u32 = (0x16aa << 16) | 1;
        write32_via_stub(fel, RESET_REG, RESET_VAL)
    }

    /// Note: This function hasn't gone through comprehensive upper-level testing yet.
    fn sid(&self, fel: &Fel<'_>) -> Result<Vec<u8>, ChipError> {
        // Read 4 words via read32 stub from SID base
        const SID_BASE: u32 = 0x0300_6200;
        let w0 = read32_via_stub(fel, SID_BASE)?;
        let w1 = read32_via_stub(fel, SID_BASE + 0x4)?;
        let w2 = read32_via_stub(fel, SID_BASE + 0x8)?;
        let w3 = read32_via_stub(fel, SID_BASE + 0xC)?;
        let mut out = Vec::with_capacity(16);
        out.extend_from_slice(&w0.to_le_bytes());
        out.extend_from_slice(&w1.to_le_bytes());
        out.extend_from_slice(&w2.to_le_bytes());
        out.extend_from_slice(&w3.to_le_bytes());
        Ok(out)
    }

    /// Note: This function hasn't gone through comprehensive upper-level testing yet.
    fn jtag(&self, fel: &Fel<'_>, enable: bool) -> Result<(), ChipError> {
        if !enable {
            return Err(ChipError::Unsupported("disable jtag not implemented"));
        }
        if payload::JTAG_RV64.is_empty() {
            return Err(ChipError::NotImplemented(
                "jtag payload missing: put assets/payloads/jtag_rv64.bin",
            ));
        }
        debug!(
            "executing JTAG enable blob at 0x{D1_SRAM_BASE:08x} ({} bytes)",
            payload::JTAG_RV64.len()
        );
        // Write in chunks and execute
        write_all(fel, D1_SRAM_BASE, payload::JTAG_RV64)?;
        fel.exec(D1_SRAM_BASE)?;
        Ok(())
    }

    /// Note: This function hasn't gone through comprehensive upper-level testing yet.
    fn ddr(&self, fel: &Fel<'_>, profile: Option<DdrProfile>) -> Result<(), ChipError> {
        let parameters = match profile {
            Some(DdrProfile::D1) => &ddr::D1,
            Some(DdrProfile::F133) => &ddr::F133,
            _ => {
                return Err(ChipError::Unsupported(
                    "D1/F133 requires a d1 or f133 DDR profile",
                ));
            }
        };
        ddr::run(fel, D1_SRAM_BASE, payload::DDR_INIT_D1, parameters, || {
            Ok(())
        })
    }

    fn as_spi(&self) -> Option<&dyn ChipSpi> {
        Some(self)
    }
}

impl ChipSpi for D1 {
    fn spi_init(&self, fel: &Fel<'_>) -> Result<SpiContext, ChipError> {
        if payload::SPI_INIT_D1.is_empty() {
            return Err(ChipError::NotImplemented(
                "missing assets/payloads/spi_d1.bin",
            ));
        }
        debug!(
            "loading SPI helper payload at 0x{SPI_PAYLOAD_BASE:08x} ({} bytes)",
            payload::SPI_INIT_D1.len()
        );
        write_all(fel, SPI_PAYLOAD_BASE, payload::SPI_INIT_D1)?;
        Ok(SpiContext {
            payload_base: SPI_PAYLOAD_BASE,
            command_base: SPI_COMMAND_BASE,
            command_len: 4096,
            swap_base: SPI_SWAP_BASE,
            swap_len: 65_536,
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
        debug!(
            "executing SPI helper (cmd {} bytes @0x{:#010x})",
            commands.len(),
            context.command_base
        );
        write_all(fel, context.command_base, commands)?;
        fel.exec(context.payload_base)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d1_name_and_consts() {
        let d1 = D1;
        assert_eq!(d1.name(), "D1/F133");
        // Basic constant relationships
        assert_eq!(D1_SRAM_BASE, 0x0002_0000);
        assert_eq!(SPI_COMMAND_BASE, SPI_PAYLOAD_BASE + 0x1000);
        assert!(SPI_PAYLOAD_BASE + payload::SPI_INIT_D1.len() as u32 <= SPI_COMMAND_BASE);
    }
}
