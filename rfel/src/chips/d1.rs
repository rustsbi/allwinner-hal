use log::debug;

use crate::{Fel, write_all};

use super::util::{read32_via_stub, u32_params_le, write32_via_stub};
use super::{Chip, ChipError, ChipSpi, DdrProfile, SpiContext, payload};

pub struct D1;

const D1_SRAM_BASE: u32 = 0x0002_0000;
const DDR_PARAM_ADDR: u32 = D1_SRAM_BASE + 0x18;
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
        let w0 = read32_via_stub(fel, SID_BASE + 0x0)?;
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
        if payload::JTAG_ENABLE_D1.is_empty() {
            return Err(ChipError::NotImplemented(
                "jtag payload missing: put assets/payloads/jtag_d1.bin",
            ));
        }
        debug!(
            "executing JTAG enable blob at 0x{D1_SRAM_BASE:08x} ({} bytes)",
            payload::JTAG_ENABLE_D1.len()
        );
        // Write in chunks and execute
        write_all(fel, D1_SRAM_BASE, payload::JTAG_ENABLE_D1);
        fel.exec(D1_SRAM_BASE);
        Ok(())
    }

    /// Note: This function hasn't gone through comprehensive upper-level testing yet.
    fn ddr(&self, fel: &Fel<'_>, profile: Option<DdrProfile>) -> Result<(), ChipError> {
        let Some(kind) = profile else {
            return Err(ChipError::Unsupported(
                "usage: rfel ddr --profile d1 | f133",
            ));
        };

        match kind {
            DdrProfile::D1 => {
                if payload::DDR_INIT_D1.is_empty() {
                    return Err(ChipError::NotImplemented(
                        "missing assets/payloads/ddr_d1.bin",
                    ));
                }
                // Fixed parameter table (ddr3_param_t)
                let params: [u32; 32] = [
                    792,        // dram_clk
                    3,          // dram_type
                    0x007b7bfb, // dram_zq
                    0x00000001, // dram_odt_en
                    0x000010d2, // dram_para1
                    0x00000000, // dram_para2
                    0x00001c70, // dram_mr0
                    0x00000042, // dram_mr1
                    0x00000018, // dram_mr2
                    0x00000000, // dram_mr3
                    0x004a2195, // tpr0
                    0x02423190, // tpr1
                    0x0008b061, // tpr2
                    0xb4787896, // tpr3
                    0x00000000, // tpr4
                    0x48484848, // tpr5
                    0x00000048, // tpr6
                    0x1620121e, // tpr7
                    0x00000000, // tpr8
                    0x00000000, // tpr9
                    0x00000000, // tpr10
                    0x00870000, // tpr11
                    0x00000024, // tpr12
                    0x34050100, // tpr13
                    0, 0, 0, 0, 0, 0, 0, 0, // reserve[8]
                ];
                debug!(
                    "DDR d1: payload @0x{D1_SRAM_BASE:08x} ({} bytes), params @0x{DDR_PARAM_ADDR:08x} ({} bytes)",
                    payload::DDR_INIT_D1.len(),
                    params.len() * 4
                );
                write_all(fel, D1_SRAM_BASE, payload::DDR_INIT_D1);
                write_all(fel, DDR_PARAM_ADDR, &u32_params_le(&params));
                fel.exec(D1_SRAM_BASE);
                Ok(())
            }
            DdrProfile::F133 => {
                if payload::DDR_INIT_F133.is_empty() {
                    return Err(ChipError::NotImplemented(
                        "missing assets/payloads/ddr_f133.bin",
                    ));
                }
                // Fixed parameter table (ddr2_param_t)
                let params: [u32; 32] = [
                    528,        // dram_clk
                    2,          // dram_type
                    0x007b7bf9, // dram_zq
                    0x00000000, // dram_odt_en
                    0x000000d2, // dram_para1
                    0x00000000, // dram_para2
                    0x00000e73, // dram_mr0
                    0x00000002, // dram_mr1
                    0x00000000, // dram_mr2
                    0x00000000, // dram_mr3
                    0x00471992, // tpr0
                    0x0131a10c, // tpr1
                    0x00057041, // tpr2
                    0xb4787896, // tpr3
                    0x00000000, // tpr4
                    0x48484848, // tpr5
                    0x00000048, // tpr6
                    0x1621121e, // tpr7
                    0x00000000, // tpr8
                    0x00000000, // tpr9
                    0x00000000, // tpr10
                    0x00030010, // tpr11
                    0x00000035, // tpr12
                    0x34000000, // tpr13
                    0, 0, 0, 0, 0, 0, 0, 0, // reserve[8]
                ];
                debug!(
                    "DDR f133: payload @0x{D1_SRAM_BASE:08x} ({} bytes), params @0x{DDR_PARAM_ADDR:08x} ({} bytes)",
                    payload::DDR_INIT_F133.len(),
                    params.len() * 4
                );
                write_all(fel, D1_SRAM_BASE, payload::DDR_INIT_F133);
                write_all(fel, DDR_PARAM_ADDR, &u32_params_le(&params));
                fel.exec(D1_SRAM_BASE);
                Ok(())
            }
        }
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
        write_all(fel, SPI_PAYLOAD_BASE, payload::SPI_INIT_D1);
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
        write_all(fel, context.command_base, commands);
        fel.exec(context.payload_base);
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
        assert_eq!(DDR_PARAM_ADDR, D1_SRAM_BASE + 0x18);
    }
}
