//! Clock Control Unit peripheral.

use crate::ccu;
use embedded_time::rate::Hertz;
use volatile_register::RW;

/// Clock configuration on current SoC.
#[derive(Debug)]
pub struct Clocks {
    /// PSI clock frequency.
    pub psi: Hertz,
    /// Advanced Peripheral Bus 1 clock frequency.
    pub apb1: Hertz,
}

/// Clock Control Unit registers.
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0 - CPU PLL Control register.
    pub pll_cpu_control: RW<PllCpuControl>,
    _reserved0: [u32; 3],
    /// 0x20 - DDR PLL Control register.
    pub pll_ddr_control: RW<PllDdrControl>,
    _reserved1: [u32; 3],
    /// 0x20 - Peripheral PLL Control register 0.
    pub pll_peri0_control: RW<PllPeri0Control>,
    _reserved2: [u32; 311],
    /// 0x500 - CPU AXI Configuration register.
    pub cpu_axi_config: RW<CpuAxiConfig>,
    _reserved3: [u32; 258],
    /// 0x90c - UART Bus Gating Reset register.
    pub uart_bgr: RW<UartBusGating>,
    _reserved4: [u32; 12],
    /// 0x940..=0x944 - SPI0 Clock Register and SPI1 Clock Register.
    pub spi_clk: [RW<SpiClock>; 2],
    _reserved5: [u32; 9],
    /// 0x96c - SPI Bus Gating Reset register.
    pub spi_bgr: RW<SpiBusGating>,
}

/// CPU PLL Control register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PllCpuControl(u32);

impl PllCpuControl {
    // p0 = (reg32 >> 16) & 0x03;
}

/// DDR PLL Control register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PllDdrControl(u32);

/// Peripheral PLL Control register 0.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PllPeri0Control(u32);

/// AXI CPU clock source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CpuClockSource {
    /// 24-MHz external oscillator.
    Osc24M,
    /// 32-KHz clock.
    Clk32K,
    /// 16-MHz RC oscillator.
    Clk16MRC,
    /// CPU PLL.
    PllCpu,
    /// Peripheral PLL (1x).
    PllPeri1x,
    /// Peripheral PLL (2x).
    PllPeri2x,
    /// 800-MHz Peripheral PLL.
    PllPeri800M,
}

/// CPU AXI Configuration register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CpuAxiConfig(u32);

impl CpuAxiConfig {
    const CPU_CLK_SEL: u32 = 0x7 << 24;

    /// Get AXI CPU clock source.
    #[inline]
    pub const fn clock_source(self) -> CpuClockSource {
        match (self.0 & Self::CPU_CLK_SEL) >> 24 {
            0 => CpuClockSource::Osc24M,
            1 => CpuClockSource::Clk32K,
            2 => CpuClockSource::Clk16MRC,
            3 => CpuClockSource::PllCpu,
            4 => CpuClockSource::PllPeri1x,
            5 => CpuClockSource::PllPeri2x,
            6 => CpuClockSource::PllPeri800M,
            _ => panic!("impossible clock source"),
        }
    }
    /// Set AXI CPU clock source.
    #[inline]
    pub const fn set_clock_source(self, val: CpuClockSource) -> Self {
        let val = match val {
            CpuClockSource::Osc24M => 0,
            CpuClockSource::Clk32K => 1,
            CpuClockSource::Clk16MRC => 2,
            CpuClockSource::PllCpu => 3,
            CpuClockSource::PllPeri1x => 4,
            CpuClockSource::PllPeri2x => 5,
            CpuClockSource::PllPeri800M => 6,
        };
        Self((self.0 & !Self::CPU_CLK_SEL) | (val << 24))
    }
}

/// Clock divide factor N.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactorN {
    /// Don't divide.
    N1,
    /// Divide frequency by 2.
    N2,
    /// Divide frequency by 4.
    N4,
    /// Divide frequency by 8.
    N8,
}

/// UART Bus Gating Reset register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UartBusGating(u32);
impl UartBusGating {
    /// Disable clock gate for UART `I`.
    #[inline]
    pub const fn gate_mask<const I: usize>(self) -> Self {
        Self(self.0 & !(1 << I))
    }
    /// Enable clock gate for UART `I`.
    #[inline]
    pub const fn gate_pass<const I: usize>(self) -> Self {
        Self(self.0 | (1 << I))
    }
    /// Assert reset signal for UART `I`.
    #[inline]
    pub const fn assert_reset<const I: usize>(self) -> Self {
        Self(self.0 & !(1 << (I + 16)))
    }
    /// Deassert reset signal for UART `I`.
    #[inline]
    pub const fn deassert_reset<const I: usize>(self) -> Self {
        Self(self.0 | (1 << (I + 16)))
    }
}

/// SPI Clock Register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SpiClock(u32);

impl SpiClock {
    const CLK_SRC_SEL: u32 = 0x7 << 24;
    const FACTOR_N: u32 = 0x3 << 8;
    const FACTOR_M: u32 = 0xf << 0;
    /// Get SPI clock source.
    #[inline]
    pub const fn clock_source(self) -> SpiClockSource {
        match (self.0 & Self::CLK_SRC_SEL) >> 24 {
            0x0 => SpiClockSource::Hosc,
            0x1 => SpiClockSource::PllPeri1x,
            0x2 => SpiClockSource::PllPeri2x,
            0x3 => SpiClockSource::PllAudio1Div2,
            0x4 => SpiClockSource::PllAudio1Div5,
            _ => panic!("impossible clock source"),
        }
    }
    /// Set SPI clock source.
    #[inline]
    pub const fn set_clock_source(self, val: SpiClockSource) -> Self {
        let val = match val {
            SpiClockSource::Hosc => 0x0,
            SpiClockSource::PllPeri1x => 0x1,
            SpiClockSource::PllPeri2x => 0x2,
            SpiClockSource::PllAudio1Div2 => 0x3,
            SpiClockSource::PllAudio1Div5 => 0x4,
        };
        Self((self.0 & !Self::CLK_SRC_SEL) | (val << 24))
    }
    /// Get SPI clock divide factor N.
    #[inline]
    pub const fn factor_n(self) -> FactorN {
        match (self.0 & Self::FACTOR_N) >> 8 {
            0 => FactorN::N1,
            1 => FactorN::N2,
            2 => FactorN::N4,
            3 => FactorN::N8,
            _ => unreachable!(),
        }
    }
    /// Set SPI clock divide factor N.
    #[inline]
    pub const fn set_factor_n(self, val: FactorN) -> Self {
        let val = match val {
            FactorN::N1 => 0,
            FactorN::N2 => 1,
            FactorN::N4 => 2,
            FactorN::N8 => 3,
        };
        Self((self.0 & !Self::FACTOR_N) | (val << 8))
    }
    /// Get SPI clock divide factor M.
    #[inline]
    pub const fn factor_m(self) -> u8 {
        (self.0 & Self::FACTOR_M) as u8
    }
    /// Set SPI clock divide factor M.
    #[inline]
    pub const fn set_factor_m(self, val: u8) -> Self {
        Self((self.0 & !Self::FACTOR_M) | val as u32)
    }
}

/// SPI clock source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpiClockSource {
    /// HOSC.
    Hosc,
    /// Peripheral PLL (1x).
    PllPeri1x,
    /// Peripheral PLL (2x).
    PllPeri2x,
    /// Audio PLL (div 2).
    PllAudio1Div2,
    /// Audio PLL (div 5).
    PllAudio1Div5,
}

/// SPI Bus Gating Reset register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SpiBusGating(u32);

impl SpiBusGating {
    /// Disable clock gate for SPI `I`.
    #[inline]
    pub const fn gate_mask<const I: usize>(self) -> Self {
        Self(self.0 & !(1 << I))
    }
    /// Enable clock gate for SPI `I`.
    #[inline]
    pub const fn gate_pass<const I: usize>(self) -> Self {
        Self(self.0 | (1 << I))
    }
    /// Assert reset signal for SPI `I`.
    #[inline]
    pub const fn assert_reset<const I: usize>(self) -> Self {
        Self(self.0 & !(1 << (I + 16)))
    }
    /// Deassert reset signal for SPI `I`.
    #[inline]
    pub const fn deassert_reset<const I: usize>(self) -> Self {
        Self(self.0 | (1 << (I + 16)))
    }
}

/// Peripheral that can be clock gated by CCU.
pub trait ClockGate {
    /// Reset this peripheral by provided `ccu`.
    unsafe fn reset(ccu: &ccu::RegisterBlock);
    /// Free this peripheral by provided `ccu`.
    unsafe fn free(ccu: &ccu::RegisterBlock);
}

/// Peripheral whose clock can be configurated by CCU.
pub trait ClockConfig {
    /// Type of clock source.
    type Source;
    /// Configure peripheral clock.
    ///
    /// Value `factor_m` should be in 0 ..= 15.
    unsafe fn config(
        source: Self::Source,
        factor_m: u8,
        factor_n: FactorN,
        ccu: &ccu::RegisterBlock,
    );
}

/// Universal Asynchronous Receiver-Transmitter clock gate.
///
/// UART peripheral should be indexed by type parameter `IDX`.
pub struct UART<const IDX: usize>;

impl<const I: usize> ClockGate for UART<I> {
    #[inline]
    unsafe fn reset(ccu: &ccu::RegisterBlock) {
        let uart_bgr = ccu.uart_bgr.read();
        ccu.uart_bgr
            .write(uart_bgr.gate_mask::<I>().assert_reset::<I>());
        let uart_bgr = ccu.uart_bgr.read();
        ccu.uart_bgr
            .write(uart_bgr.gate_pass::<I>().deassert_reset::<I>());
    }

    #[inline]
    unsafe fn free(ccu: &ccu::RegisterBlock) {
        let uart_bgr = ccu.uart_bgr.read();
        ccu.uart_bgr
            .write(uart_bgr.gate_mask::<I>().assert_reset::<I>());
    }
}

/// Serial Peripheral Interface clock gate.
pub struct SPI<const IDX: usize>;

impl<const I: usize> ClockGate for SPI<I> {
    #[inline]
    unsafe fn reset(ccu: &ccu::RegisterBlock) {
        let spi_bgr = ccu.spi_bgr.read();
        ccu.spi_bgr
            .write(spi_bgr.gate_mask::<I>().assert_reset::<I>());
        let spi_bgr = ccu.spi_bgr.read();
        ccu.spi_bgr
            .write(spi_bgr.gate_pass::<I>().deassert_reset::<I>());
    }

    #[inline]
    unsafe fn free(ccu: &ccu::RegisterBlock) {
        let spi_bgr = ccu.spi_bgr.read();
        ccu.spi_bgr
            .write(spi_bgr.gate_mask::<I>().assert_reset::<I>());
    }
}

impl<const I: usize> ClockConfig for SPI<I> {
    type Source = SpiClockSource;

    unsafe fn config(
        source: Self::Source,
        factor_m: u8,
        factor_n: FactorN,
        ccu: &ccu::RegisterBlock,
    ) {
        let spi_clk = ccu.spi_clk[I].read();
        ccu.spi_clk[I].write(
            spi_clk
                .set_clock_source(source)
                .set_factor_m(factor_m)
                .set_factor_n(factor_n),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;
    #[test]
    fn offset_ccu() {
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_control), 0x0);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_control), 0x10);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_control), 0x20);
        assert_eq!(offset_of!(RegisterBlock, cpu_axi_config), 0x500);
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, spi_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
    }
}
