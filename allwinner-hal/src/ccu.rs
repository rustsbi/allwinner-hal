//! Clock Control Unit peripheral.

mod factor;
mod pll;
mod source;

pub(crate) use factor::calculate_best_peripheral_factors_nm;
pub use factor::{AxiFactorN, FactorP, PeriFactorN};
pub use pll::{PllCpuControl, PllDdrControl, PllPeri0Control};
pub use source::{CpuClockSource, DramClockSource, SmhcClockSource, SpiClockSource};

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
    /// 0x10 - DDR PLL Control register.
    pub pll_ddr_control: RW<PllDdrControl>,
    _reserved1: [u32; 3],
    /// 0x20 - Peripheral PLL 0 Control register.
    pub pll_peri0_control: RW<PllPeri0Control>,
    _reserved2: [u32; 311],
    /// 0x500 - CPU AXI Configuration register.
    pub cpu_axi_config: RW<CpuAxiConfig>,
    _reserved3: [u32; 15],
    /// 0x540 - MBUS Clock register.
    pub mbus_clock: RW<MbusClock>,
    _reserved4: [u32; 175],
    /// 0x800 - DRAM Clock register.
    pub dram_clock: RW<DramClock>,
    _reserved5: [u32; 2],
    /// 0x80c - DRAM Bus Gating Reset register.
    pub dram_bgr: RW<DramBusGating>,
    _reserved6: [u32; 8],
    /// 0x830..=0x838 - SMHC0 Clock register, SMHC1 Clock register and SMHC2 Clock register.
    pub smhc_clk: [RW<SmhcClock>; 3],
    _reserved7: [u32; 4],
    /// 0x84c - SMHC Bus Gating Reset register.
    pub smhc_bgr: RW<SmhcBusGating>,
    _reserved8: [u32; 47],
    /// 0x90c - UART Bus Gating Reset register.
    pub uart_bgr: RW<UartBusGating>,
    _reserved9: [u32; 12],
    /// 0x940..=0x944 - SPI0 Clock register and SPI1 Clock register.
    pub spi_clk: [RW<SpiClock>; 2],
    _reserved10: [u32; 9],
    /// 0x96c - SPI Bus Gating Reset register.
    pub spi_bgr: RW<SpiBusGating>,
}

/// CPU AXI Configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CpuAxiConfig(u32);

impl CpuAxiConfig {
    const CPU_CLK_SEL: u32 = 0x7 << 24;
    const FACTOR_P: u32 = 0x3 << 16;
    const FACTOR_N: u32 = 0x3 << 8;
    const FACTOR_M: u32 = 0x3 << 0;

    /// Get AXI CPU clock source.
    #[inline]
    pub const fn clock_source(self) -> CpuClockSource {
        match (self.0 & Self::CPU_CLK_SEL) >> 24 {
            0 => CpuClockSource::Hosc,
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
            CpuClockSource::Hosc => 0,
            CpuClockSource::Clk32K => 1,
            CpuClockSource::Clk16MRC => 2,
            CpuClockSource::PllCpu => 3,
            CpuClockSource::PllPeri1x => 4,
            CpuClockSource::PllPeri2x => 5,
            CpuClockSource::PllPeri800M => 6,
        };
        Self((self.0 & !Self::CPU_CLK_SEL) | (val << 24))
    }
    /// Get AXI CPU clock divide factor P.
    #[inline]
    pub const fn factor_p(self) -> FactorP {
        match (self.0 & Self::FACTOR_P) >> 16 {
            0 => FactorP::P1,
            1 => FactorP::P2,
            2 => FactorP::P4,
            _ => unreachable!(),
        }
    }
    /// Set AXI CPU clock divide factor P.
    #[inline]
    pub const fn set_factor_p(self, val: FactorP) -> Self {
        let val = match val {
            FactorP::P1 => 0,
            FactorP::P2 => 1,
            FactorP::P4 => 2,
        };
        Self((self.0 & !Self::FACTOR_P) | (val << 16))
    }
    /// Get AXI CPU clock divide factor N.
    #[inline]
    pub const fn factor_n(self) -> AxiFactorN {
        match (self.0 & Self::FACTOR_N) >> 8 {
            0x1 => AxiFactorN::N2,
            0x2 => AxiFactorN::N3,
            0x3 => AxiFactorN::N4,
            _ => unreachable!(),
        }
    }
    /// Set AXI CPU clock divide factor N.
    #[inline]
    pub const fn set_factor_n(self, val: AxiFactorN) -> Self {
        Self((self.0 & !Self::FACTOR_N) | ((val as u32) << 8))
    }
    /// Get AXI CPU clock divide factor M.
    #[inline]
    pub const fn factor_m(self) -> u8 {
        (self.0 & Self::FACTOR_M) as u8
    }
    /// Set AXI CPU clock divide factor M.
    #[inline]
    pub const fn set_factor_m(self, val: u8) -> Self {
        Self((self.0 & !Self::FACTOR_M) | val as u32)
    }
}

/// MBUS Clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MbusClock(u32);

impl MbusClock {
    const MBUS_RST: u32 = 0x1 << 30;

    /// If reset is asserted.
    #[inline]
    pub const fn is_reset_asserted(self) -> bool {
        self.0 & Self::MBUS_RST == 0
    }
    /// Assert reset.
    #[inline]
    pub const fn assert_reset(self) -> Self {
        Self(self.0 & !Self::MBUS_RST)
    }
    /// De-assert reset.
    #[inline]
    pub const fn deassert_reset(self) -> Self {
        Self(self.0 | Self::MBUS_RST)
    }
}

/// DRAM Clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramClock(u32);

impl DramClock {
    const DRAM_CLK_GATING: u32 = 0x1 << 31;
    const DRAM_CLK_SEL: u32 = 0x7 << 24;
    const DRAM_N: u32 = 0x3 << 8;
    const DRAM_M: u32 = 0x3 << 0;

    /// If clock is unmasked.
    #[inline]
    pub const fn is_clock_unmasked(self) -> bool {
        self.0 & Self::DRAM_CLK_GATING != 0
    }
    /// Unmask (enable) clock.
    #[inline]
    pub const fn unmask_clock(self) -> Self {
        Self(self.0 | Self::DRAM_CLK_GATING)
    }
    /// Mask (disable) clock.
    #[inline]
    pub const fn mask_clock(self) -> Self {
        Self(self.0 & !Self::DRAM_CLK_GATING)
    }
    /// Get clock source.
    #[inline]
    pub const fn clock_source(self) -> DramClockSource {
        match ((self.0 & Self::DRAM_CLK_SEL) >> 24) as u8 {
            0x0 => DramClockSource::PllDdr,
            0x1 => DramClockSource::PllAudio1Div2,
            0x2 => DramClockSource::PllPeri2x,
            0x3 => DramClockSource::PllPeri800M,
            _ => unreachable!(),
        }
    }
    /// Set clock source.
    #[inline]
    pub const fn set_clock_source(self, val: DramClockSource) -> Self {
        Self((self.0 & !Self::DRAM_CLK_SEL) | ((val as u32) << 24))
    }
    /// Get factor n.
    #[inline]
    pub const fn factor_n(self) -> PeriFactorN {
        match ((self.0 & Self::DRAM_N) >> 8) as u8 {
            0x0 => PeriFactorN::N1,
            0x1 => PeriFactorN::N2,
            0x2 => PeriFactorN::N4,
            0x3 => PeriFactorN::N8,
            _ => unreachable!(),
        }
    }
    /// Set factor n.
    #[inline]
    pub const fn set_factor_n(self, val: PeriFactorN) -> Self {
        Self((self.0 & !Self::DRAM_N) | ((val as u32) << 8))
    }
    /// Get factor m (from 0 to 3).
    #[inline]
    pub const fn factor_m(self) -> u8 {
        ((self.0 & Self::DRAM_M) >> 0) as u8
    }
    /// Set factor m (from 0 to 3).
    #[inline]
    pub const fn set_factor_m(self, val: u8) -> Self {
        Self((self.0 & !Self::DRAM_M) | ((val as u32) << 0))
    }
}

/// Dram Bus Gating Reset register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramBusGating(u32);

impl DramBusGating {
    const DRAM_RST: u32 = 1 << 16;
    const DRAM_GATING: u32 = 1 << 0;

    /// Assert dram reset.
    #[inline]
    pub const fn assert_reset(self) -> Self {
        Self(self.0 & !Self::DRAM_RST)
    }
    /// De-assert dram reset.
    #[inline]
    pub const fn deassert_reset(self) -> Self {
        Self(self.0 | Self::DRAM_RST)
    }
    /// Mask the dram gating.
    #[inline]
    pub const fn gate_mask(self) -> Self {
        Self(self.0 & !Self::DRAM_GATING)
    }
    /// Unmask (pass) the dram gating.
    #[inline]
    pub const fn gate_pass(self) -> Self {
        Self(self.0 | Self::DRAM_GATING)
    }
}

/// UART Bus Gating Reset register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

/// SPI Clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub const fn factor_n(self) -> PeriFactorN {
        match (self.0 & Self::FACTOR_N) >> 8 {
            0 => PeriFactorN::N1,
            1 => PeriFactorN::N2,
            2 => PeriFactorN::N4,
            3 => PeriFactorN::N8,
            _ => unreachable!(),
        }
    }
    /// Set SPI clock divide factor N.
    #[inline]
    pub const fn set_factor_n(self, val: PeriFactorN) -> Self {
        let val = match val {
            PeriFactorN::N1 => 0,
            PeriFactorN::N2 => 1,
            PeriFactorN::N4 => 2,
            PeriFactorN::N8 => 3,
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

/// SPI Bus Gating Reset register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

/// SMHC Clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SmhcClock(u32);

impl SmhcClock {
    const CLK_SRC_SEL: u32 = 0x7 << 24;
    const FACTOR_N: u32 = 0x3 << 8;
    const FACTOR_M: u32 = 0xf << 0;
    const CLK_GATING: u32 = 1 << 31;

    /// Get SMHC clock source.
    #[inline]
    pub const fn clock_source(self) -> SmhcClockSource {
        match (self.0 & Self::CLK_SRC_SEL) >> 24 {
            0x0 => SmhcClockSource::Hosc,
            0x1 => SmhcClockSource::PllPeri1x,
            0x2 => SmhcClockSource::PllPeri2x,
            0x3 => SmhcClockSource::PllPeri800M,
            0x4 => SmhcClockSource::PllAudio1Div2,
            _ => panic!("impossible clock source"),
        }
    }
    /// Set SMHC clock source.
    #[inline]
    pub const fn set_clock_source(self, val: SmhcClockSource) -> Self {
        let val = match val {
            SmhcClockSource::Hosc => 0x0,
            SmhcClockSource::PllPeri1x => 0x1,
            SmhcClockSource::PllPeri2x => 0x2,
            SmhcClockSource::PllPeri800M => 0x3,
            SmhcClockSource::PllAudio1Div2 => 0x4,
        };
        Self((self.0 & !Self::CLK_SRC_SEL) | (val << 24))
    }
    /// Get SMHC clock divide factor N.
    #[inline]
    pub const fn factor_n(self) -> PeriFactorN {
        match (self.0 & Self::FACTOR_N) >> 8 {
            0 => PeriFactorN::N1,
            1 => PeriFactorN::N2,
            2 => PeriFactorN::N4,
            3 => PeriFactorN::N8,
            _ => unreachable!(),
        }
    }
    /// Set SMHC clock divide factor N.
    #[inline]
    pub const fn set_factor_n(self, val: PeriFactorN) -> Self {
        let val = match val {
            PeriFactorN::N1 => 0,
            PeriFactorN::N2 => 1,
            PeriFactorN::N4 => 2,
            PeriFactorN::N8 => 3,
        };
        Self((self.0 & !Self::FACTOR_N) | (val << 8))
    }
    /// Get SMHC clock divide factor M.
    #[inline]
    pub const fn factor_m(self) -> u8 {
        (self.0 & Self::FACTOR_M) as u8
    }
    /// Set SMHC clock divide factor M.
    #[inline]
    pub const fn set_factor_m(self, val: u8) -> Self {
        Self((self.0 & !Self::FACTOR_M) | val as u32)
    }
    /// Enable clock gating.
    #[inline]
    pub const fn enable_clock_gating(self) -> Self {
        Self(self.0 | Self::CLK_GATING)
    }
    /// Disable clock gating.
    #[inline]
    pub const fn disable_clock_gating(self) -> Self {
        Self(self.0 & !Self::CLK_GATING)
    }
    /// Get if clock gating is enabled.
    #[inline]
    pub const fn is_clock_gating_enabled(self) -> bool {
        self.0 & Self::CLK_GATING != 0
    }
}

/// SMHC Clock Reset register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SmhcBusGating(u32);

impl SmhcBusGating {
    /// Disable clock gate for SMHC `I`.
    #[inline]
    pub const fn gate_mask<const I: usize>(self) -> Self {
        Self(self.0 & !(1 << I))
    }
    /// Enable clock gate for SMHC `I`.
    #[inline]
    pub const fn gate_pass<const I: usize>(self) -> Self {
        Self(self.0 | (1 << I))
    }
    /// Assert reset signal for SMHC `I`.
    #[inline]
    pub const fn assert_reset<const I: usize>(self) -> Self {
        Self(self.0 & !(1 << (I + 16)))
    }
    /// Deassert reset signal for SMHC `I`.
    #[inline]
    pub const fn deassert_reset<const I: usize>(self) -> Self {
        Self(self.0 | (1 << (I + 16)))
    }
}

/// Peripheral that have clock reset feature in CCU.
pub trait ClockReset {
    /// Assert reset signal.
    unsafe fn assert_reset_only(ccu: &RegisterBlock);
    /// Deassert reset signal.
    unsafe fn deassert_reset_only(ccu: &RegisterBlock);
}

/// Peripheral that can be clock gated by CCU.
pub trait ClockGate: ClockReset {
    /// Unmask clock gate.
    unsafe fn unmask_gate_only(ccu: &RegisterBlock);
    /// Mask clock gate.
    unsafe fn mask_gate_only(ccu: &RegisterBlock);
    /// Assert reset signal and mask the clock gate.
    unsafe fn disable_in(ccu: &RegisterBlock);
    /// Deassert reset signal and unmask the clock gate.
    unsafe fn enable_in(ccu: &RegisterBlock);
    /// Reset this peripheral without reconfiguring clocks (if applicable).
    #[inline]
    unsafe fn reset(ccu: &RegisterBlock) {
        unsafe {
            // assert reset and then deassert reset.
            Self::disable_in(ccu);
            Self::enable_in(ccu);
        }
    }
    /// Free this peripheral by provided `ccu`.
    #[inline]
    unsafe fn free(ccu: &RegisterBlock) {
        unsafe {
            // by default, asserting reset signal and mask clock gate.
            Self::disable_in(ccu);
        }
    }
}

/// Peripheral whose clock can be configurated by CCU.
pub trait ClockConfig {
    /// Type of clock source.
    type Source;
    /// Configure peripheral clock.
    ///
    /// Value `factor_m` should be in 0 ..= 15.
    unsafe fn configure(
        ccu: &RegisterBlock,
        source: Self::Source,
        factor_m: u8,
        factor_n: PeriFactorN,
    );
    /// Reconfigure peripheral clock by applying clock parameters while asserting reset.
    #[inline]
    unsafe fn reconfigure(
        ccu: &RegisterBlock,
        source: Self::Source,
        factor_m: u8,
        factor_n: PeriFactorN,
    ) where
        Self: ClockGate,
    {
        unsafe {
            Self::disable_in(ccu);
            Self::configure(ccu, source, factor_m, factor_n);
            Self::enable_in(ccu);
        }
    }
    /// Reconfigure this clock with dependency to a resettable clock type `T`.
    #[inline]
    unsafe fn reconfigure_with<T: ClockReset, F, G>(
        ccu: &RegisterBlock,
        dependency: T,
        before_configure: F,
        after_configure: G,
    ) where
        Self: ClockGate,
        F: FnOnce(&RegisterBlock) -> (Self::Source, u8, PeriFactorN),
        G: FnOnce(&RegisterBlock),
    {
        unsafe {
            let _ = dependency; // does not use value, the type T is used instead
            T::assert_reset_only(ccu);
            Self::disable_in(ccu);
            let (source, factor_m, factor_n) = before_configure(ccu);
            Self::configure(ccu, source, factor_m, factor_n);
            after_configure(ccu);
            Self::deassert_reset_only(ccu);
            T::deassert_reset_only(ccu);
            Self::unmask_gate_only(ccu);
        }
    }
}

// TODO: a more proper abstraction considering the PLL source behind peripheral clock

/// Dynamic Random-Access Memory (DRAM) clock type.
pub struct DRAM;

impl ClockReset for DRAM {
    #[inline]
    unsafe fn deassert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.dram_bgr.modify(|v| v.deassert_reset());
        }
    }
    #[inline]
    unsafe fn assert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.dram_bgr.modify(|v| v.assert_reset());
        }
    }
}

impl ClockGate for DRAM {
    #[inline]
    unsafe fn unmask_gate_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.dram_bgr.modify(|v| v.gate_pass());
        }
    }
    #[inline]
    unsafe fn mask_gate_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.dram_bgr.modify(|v| v.gate_mask());
        }
    }
    #[inline]
    unsafe fn disable_in(ccu: &RegisterBlock) {
        unsafe {
            ccu.dram_bgr.modify(|v| v.gate_mask().assert_reset());
        }
    }
    #[inline]
    unsafe fn enable_in(ccu: &RegisterBlock) {
        unsafe {
            ccu.dram_bgr.modify(|v| v.gate_pass().deassert_reset());
        }
    }
}

impl ClockConfig for DRAM {
    type Source = DramClockSource;

    #[inline]
    unsafe fn configure(
        ccu: &RegisterBlock,
        source: Self::Source,
        factor_m: u8,
        factor_n: PeriFactorN,
    ) {
        unsafe {
            let dram_clk = ccu.dram_clock.read();
            ccu.dram_clock.write(
                dram_clk
                    .set_clock_source(source)
                    .set_factor_m(factor_m)
                    .set_factor_n(factor_n),
            )
        }
    }
}

/// MCTL Bus (MBUS) clock type.
pub struct MBUS;

impl ClockReset for MBUS {
    #[inline]
    unsafe fn assert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.mbus_clock.modify(|v| v.assert_reset());
        }
    }
    #[inline]
    unsafe fn deassert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.mbus_clock.modify(|v| v.deassert_reset());
        }
    }
}

/// Universal Asynchronous Receiver-Transmitter clock type.
///
/// UART peripheral should be indexed by type parameter `IDX`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UART<const IDX: usize>;

impl<const I: usize> ClockReset for UART<I> {
    #[inline]
    unsafe fn assert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.uart_bgr.modify(|v| v.assert_reset::<I>());
        }
    }
    #[inline]
    unsafe fn deassert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.uart_bgr.modify(|v| v.deassert_reset::<I>());
        }
    }
}

impl<const I: usize> ClockGate for UART<I> {
    #[inline]
    unsafe fn unmask_gate_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.uart_bgr.modify(|v| v.gate_pass::<I>());
        }
    }
    #[inline]
    unsafe fn mask_gate_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.uart_bgr.modify(|v| v.gate_mask::<I>());
        }
    }
    #[inline]
    unsafe fn disable_in(ccu: &RegisterBlock) {
        unsafe {
            ccu.uart_bgr
                .modify(|v| v.gate_mask::<I>().assert_reset::<I>());
        }
    }
    #[inline]
    unsafe fn enable_in(ccu: &RegisterBlock) {
        unsafe {
            ccu.uart_bgr
                .modify(|v| v.gate_pass::<I>().deassert_reset::<I>());
        }
    }
}

/// Serial Peripheral Interface clock type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SPI<const IDX: usize>;

impl<const I: usize> ClockReset for SPI<I> {
    #[inline]
    unsafe fn assert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.spi_bgr.modify(|v| v.assert_reset::<I>());
        }
    }
    #[inline]
    unsafe fn deassert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.spi_bgr.modify(|v| v.deassert_reset::<I>());
        }
    }
}

impl<const I: usize> ClockGate for SPI<I> {
    #[inline]
    unsafe fn unmask_gate_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.spi_bgr.modify(|v| v.gate_pass::<I>());
        }
    }
    #[inline]
    unsafe fn mask_gate_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.spi_bgr.modify(|v| v.gate_mask::<I>());
        }
    }
    #[inline]
    unsafe fn disable_in(ccu: &RegisterBlock) {
        unsafe {
            ccu.spi_bgr
                .modify(|v| v.gate_mask::<I>().assert_reset::<I>());
        }
    }
    #[inline]
    unsafe fn enable_in(ccu: &RegisterBlock) {
        unsafe {
            ccu.spi_bgr
                .modify(|v| v.gate_pass::<I>().deassert_reset::<I>());
        }
    }
}

impl<const I: usize> ClockConfig for SPI<I> {
    type Source = SpiClockSource;

    unsafe fn configure(
        ccu: &RegisterBlock,
        source: Self::Source,
        factor_m: u8,
        factor_n: PeriFactorN,
    ) {
        unsafe {
            let spi_clk = ccu.spi_clk[I].read();
            ccu.spi_clk[I].write(
                spi_clk
                    .set_clock_source(source)
                    .set_factor_m(factor_m)
                    .set_factor_n(factor_n),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AxiFactorN, CpuAxiConfig, CpuClockSource, DramBusGating, DramClock, DramClockSource,
        FactorP, MbusClock, PeriFactorN, RegisterBlock,
    };
    use memoffset::offset_of;
    #[test]
    fn offset_ccu() {
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_control), 0x0);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_control), 0x10);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_control), 0x20);
        assert_eq!(offset_of!(RegisterBlock, cpu_axi_config), 0x500);
        assert_eq!(offset_of!(RegisterBlock, mbus_clock), 0x540);
        assert_eq!(offset_of!(RegisterBlock, dram_clock), 0x800);
        assert_eq!(offset_of!(RegisterBlock, dram_bgr), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, smhc_clk), 0x830);
        assert_eq!(offset_of!(RegisterBlock, smhc_bgr), 0x84c);
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, spi_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
    }

    #[test]
    fn struct_cpu_axi_config_functions() {
        let mut val = CpuAxiConfig(0x0);

        for i in 0..7 as u8 {
            let tmp = match i {
                0 => CpuClockSource::Hosc,
                1 => CpuClockSource::Clk32K,
                2 => CpuClockSource::Clk16MRC,
                3 => CpuClockSource::PllCpu,
                4 => CpuClockSource::PllPeri1x,
                5 => CpuClockSource::PllPeri2x,
                6 => CpuClockSource::PllPeri800M,
                _ => unreachable!(),
            };

            val = val.set_clock_source(tmp);

            match i {
                0 => assert_eq!(val.0, 0x00000000),
                1 => assert_eq!(val.0, 0x01000000),
                2 => assert_eq!(val.0, 0x02000000),
                3 => assert_eq!(val.0, 0x03000000),
                4 => assert_eq!(val.0, 0x04000000),
                5 => assert_eq!(val.0, 0x05000000),
                6 => assert_eq!(val.0, 0x06000000),
                _ => unreachable!(),
            }

            assert_eq!(val.clock_source(), tmp);
        }

        val = val.set_clock_source(CpuClockSource::Hosc);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.clock_source(), CpuClockSource::Hosc);

        for i in 0..3 as u8 {
            let tmp = match i {
                0 => FactorP::P1,
                1 => FactorP::P2,
                2 => FactorP::P4,
                _ => unreachable!(),
            };

            val = val.set_factor_p(tmp);

            match i {
                0 => assert_eq!(val.0, 0x00000000),
                1 => assert_eq!(val.0, 0x00010000),
                2 => assert_eq!(val.0, 0x00020000),
                _ => unreachable!(),
            }

            assert_eq!(val.factor_p(), tmp);
        }

        val = val.set_factor_p(FactorP::P1);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.factor_p(), FactorP::P1);

        val = val.set_factor_n(AxiFactorN::N4);
        assert_eq!(val.0, 0x00000300);
        assert_eq!(val.factor_n(), AxiFactorN::N4);

        val = val.set_factor_n(AxiFactorN::N2);
        assert_eq!(val.0, 0x00000100);
        assert_eq!(val.factor_n(), AxiFactorN::N2);

        val = val.set_factor_m(0x03);
        assert_eq!(val.0, 0x00000103);
        assert_eq!(val.factor_m(), 0x03);

        val = val.set_factor_m(0x0);
        assert_eq!(val.0, 0x00000100);
        assert_eq!(val.factor_m(), 0x0);
    }

    #[test]
    fn struct_mbus_clock_functions() {
        let mut val = MbusClock(0x0);

        val = val.deassert_reset();
        assert!(!val.is_reset_asserted());
        assert_eq!(val.0, 0x40000000);

        val = val.assert_reset();
        assert!(val.is_reset_asserted());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_dram_clock_functions() {
        let mut val = DramClock(0x0);

        val = val.unmask_clock();
        assert!(val.is_clock_unmasked());
        assert_eq!(val.0, 0x80000000);

        val = val.mask_clock();
        assert!(!val.is_clock_unmasked());
        assert_eq!(val.0, 0x00000000);

        for i in 0..4 as u8 {
            let cs_tmp = match i {
                0x0 => DramClockSource::PllDdr,
                0x1 => DramClockSource::PllAudio1Div2,
                0x2 => DramClockSource::PllPeri2x,
                0x3 => DramClockSource::PllPeri800M,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0x0 => 0x00000000,
                0x1 => 0x01000000,
                0x2 => 0x02000000,
                0x3 => 0x03000000,
                _ => unreachable!(),
            };

            val = val.set_clock_source(cs_tmp);
            assert_eq!(val.clock_source(), cs_tmp);
            assert_eq!(val.0, val_tmp);
        }

        val = DramClock(0x0);

        for i in 0..4 as u8 {
            let fn_tmp = match i {
                0x0 => PeriFactorN::N1,
                0x1 => PeriFactorN::N2,
                0x2 => PeriFactorN::N4,
                0x3 => PeriFactorN::N8,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0x0 => 0x00000000,
                0x1 => 0x00000100,
                0x2 => 0x00000200,
                0x3 => 0x00000300,
                _ => unreachable!(),
            };

            val = val.set_factor_n(fn_tmp);
            assert_eq!(val.factor_n(), fn_tmp);
            assert_eq!(val.0, val_tmp);
        }

        val = DramClock(0x0);
        val = val.set_factor_m(0x03);
        assert_eq!(val.factor_m(), 0x03);
        assert_eq!(val.0, 0x00000003);
    }

    #[test]
    fn struct_dram_bgr_functions() {
        let mut val = DramBusGating(0x0);

        val = val.deassert_reset();
        assert_eq!(val.0, 0x00010000);

        val = val.assert_reset();
        assert_eq!(val.0, 0x00000000);

        val = val.gate_pass();
        assert_eq!(val.0, 0x00000001);

        val = val.gate_mask();
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_uart_bgr_functions() {
        let mut val = super::UartBusGating(0x0);

        val = val.gate_pass::<0>();
        assert_eq!(val.0, 0x00000001);

        val = val.gate_mask::<0>();
        assert_eq!(val.0, 0x00000000);

        val = val.deassert_reset::<0>();
        assert_eq!(val.0, 0x00010000);

        val = val.assert_reset::<0>();
        assert_eq!(val.0, 0x00000000);

        val = val.gate_pass::<1>();
        assert_eq!(val.0, 0x00000002);

        val = val.gate_mask::<1>();
        assert_eq!(val.0, 0x00000000);

        val = val.deassert_reset::<1>();
        assert_eq!(val.0, 0x00020000);

        val = val.assert_reset::<1>();
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_spi_clock_functions() {
        let mut val = super::SpiClock(0x0);

        for i in 0..5 as u8 {
            let cs_tmp = match i {
                0x0 => super::SpiClockSource::Hosc,
                0x1 => super::SpiClockSource::PllPeri1x,
                0x2 => super::SpiClockSource::PllPeri2x,
                0x3 => super::SpiClockSource::PllAudio1Div2,
                0x4 => super::SpiClockSource::PllAudio1Div5,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0x0 => 0x00000000,
                0x1 => 0x01000000,
                0x2 => 0x02000000,
                0x3 => 0x03000000,
                0x4 => 0x04000000,
                _ => unreachable!(),
            };

            val = val.set_clock_source(cs_tmp);
            assert_eq!(val.clock_source(), cs_tmp);
            assert_eq!(val.0, val_tmp);
        }

        val = super::SpiClock(0x0);

        for i in 0..4 as u8 {
            let fn_tmp = match i {
                0x0 => PeriFactorN::N1,
                0x1 => PeriFactorN::N2,
                0x2 => PeriFactorN::N4,
                0x3 => PeriFactorN::N8,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0x0 => 0x00000000,
                0x1 => 0x00000100,
                0x2 => 0x00000200,
                0x3 => 0x00000300,
                _ => unreachable!(),
            };

            val = val.set_factor_n(fn_tmp);
            assert_eq!(val.factor_n(), fn_tmp);
            assert_eq!(val.0, val_tmp);
        }

        val = super::SpiClock(0x0);
        val = val.set_factor_m(0x03);
        assert_eq!(val.factor_m(), 0x03);
        assert_eq!(val.0, 0x00000003);
    }

    #[test]
    fn struct_spi_bgr_functions() {
        let mut val = super::SpiBusGating(0x0);

        val = val.gate_pass::<0>();
        assert_eq!(val.0, 0x00000001);

        val = val.gate_mask::<0>();
        assert_eq!(val.0, 0x00000000);

        val = val.deassert_reset::<0>();
        assert_eq!(val.0, 0x00010000);

        val = val.assert_reset::<0>();
        assert_eq!(val.0, 0x00000000);

        val = val.gate_pass::<1>();
        assert_eq!(val.0, 0x00000002);

        val = val.gate_mask::<1>();
        assert_eq!(val.0, 0x00000000);

        val = val.deassert_reset::<1>();
        assert_eq!(val.0, 0x00020000);

        val = val.assert_reset::<1>();
        assert_eq!(val.0, 0x00000000);
    }
}
