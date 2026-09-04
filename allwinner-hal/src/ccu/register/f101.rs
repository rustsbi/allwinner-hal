//! F101 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun252iw2` platform.

use super::{BusGatingReset, SingleBusGatingReset};
use volatile_register::RW;

/// F101 main CCU register block.
#[doc(alias = "sun252iw2")]
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - `PLL_CPU_CTRL_REG`.
    pub pll_cpu_ctrl: RW<u32>,
    _reserved_0004: [u8; 0x01c],
    /// 0x0020 - `PLL_PERI_CTRL_REG`.
    pub pll_peri_ctrl: RW<u32>,
    _reserved_0024: [u8; 0x4ec],
    /// 0x0510 - `PSI_CLK_REG`.
    pub psi_clk: RW<u32>,
    _reserved_0514: [u8; 0x00c],
    /// 0x0520 - `APB0_CLK_REG`.
    pub apb0_clk: RW<u32>,
    _reserved_0524: [u8; 0x020],
    /// 0x0544 - `MBUS_CLK_REG`.
    pub mbus_clk: RW<u32>,
    _reserved_0548: [u8; 0x1c4],
    /// 0x070c - `DMA_BGR_REG`.
    pub dma_bgr: RW<SingleBusGatingReset>,
    _reserved_0710: [u8; 0x1fc],
    /// 0x090c - `UART_BGR_REG`.
    pub uart_bgr: RW<BusGatingReset<1>>,
    _reserved_0910: [u8; 0x3f0],
    /// 0x0d00 - `RISCV_CLK_REG`.
    pub riscv_clk: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_ctrl), 0x000);
        assert_eq!(offset_of!(RegisterBlock, pll_peri_ctrl), 0x020);
        assert_eq!(offset_of!(RegisterBlock, psi_clk), 0x510);
        assert_eq!(offset_of!(RegisterBlock, apb0_clk), 0x520);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk), 0x544);
        assert_eq!(offset_of!(RegisterBlock, dma_bgr), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, riscv_clk), 0xd00);
        assert_eq!(size_of::<RegisterBlock>(), 0xd04);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
