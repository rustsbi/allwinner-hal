//! F101 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun252iw2` platform.

use super::{BusGatingReset, SingleBusGatingReset};
use crate::ccu::PeriFactorN;
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
    _reserved_0910: [u8; 0x030],
    /// 0x0940 - SPI0 module clock register.
    pub spi_clk: RW<SpiClock>,
    _reserved_0944: [u8; 0x028],
    /// 0x096c - SPI0 bus clock gating and reset register.
    pub spi_bgr: RW<SingleBusGatingReset>,
    _reserved_0970: [u8; 0x390],
    /// 0x0d00 - `RISCV_CLK_REG`.
    pub riscv_clk: RW<u32>,
}

/// F101 SPI0 module clock register, as used by the xfel F101 SPI payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SpiClock(u32);

impl SpiClock {
    /// Unmask (enable) the SPI module clock.
    #[inline]
    pub const fn unmask_clock(self) -> Self {
        Self(self.0 | (1 << 31))
    }

    /// Select the high-speed oscillator through source bits 25:24.
    #[inline]
    pub const fn select_hosc(self) -> Self {
        Self(self.0 & !(0x3 << 24))
    }

    /// Set the power-of-two clock divisor N.
    #[inline]
    pub const fn set_factor_n(self, factor: PeriFactorN) -> Self {
        Self((self.0 & !(0x3 << 8)) | ((factor as u32) << 8))
    }

    /// Set the encoded clock divisor M (the divisor is `factor + 1`).
    #[inline]
    pub const fn set_factor_m(self, factor: u8) -> Self {
        Self((self.0 & !0xf) | (factor as u32 & 0xf))
    }
}

#[cfg(test)]
mod tests {
    use super::{RegisterBlock, SpiClock};
    use crate::ccu::PeriFactorN;
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
        assert_eq!(offset_of!(RegisterBlock, spi_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
        assert_eq!(offset_of!(RegisterBlock, riscv_clk), 0xd00);
        assert_eq!(size_of::<RegisterBlock>(), 0xd04);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn spi_clock_fields() {
        assert_eq!(SpiClock(0x1234_5678).unmask_clock().0, 0x9234_5678);
        assert_eq!(SpiClock(u32::MAX).select_hosc().0, 0xfcff_ffff);
        for (factor, bits) in [
            (PeriFactorN::N1, 0),
            (PeriFactorN::N2, 1),
            (PeriFactorN::N4, 2),
            (PeriFactorN::N8, 3),
        ] {
            assert_eq!(
                SpiClock(u32::MAX).set_factor_n(factor).0,
                0xffff_fcff | (bits << 8)
            );
        }
        for factor in [0, 1, 5, 15, 255] {
            assert_eq!(
                SpiClock(u32::MAX).set_factor_m(factor).0,
                0xffff_fff0 | (factor as u32 & 0xf)
            );
        }
    }
}
