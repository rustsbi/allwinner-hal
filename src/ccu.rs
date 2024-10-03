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
    /// 0x10 - DDR PLL Control register.
    pub pll_ddr_control: RW<PllDdrControl>,
    _reserved1: [u32; 3],
    /// 0x20 - Peripheral PLL Control register 0.
    pub pll_peri0_control: RW<PllPeri0Control>,
    _reserved2: [u32; 311],
    /// 0x500 - CPU AXI Configuration register.
    pub cpu_axi_config: RW<CpuAxiConfig>,
    _reserved3: [u32; 15],
    /// 0x540 - MBUS Clock register.
    pub mbus_clock: RW<MbusClock>,
    _reserved4: [u32; 175],
    /// 0x800 - DRAM Clock Register.
    pub dram_clock: RW<DramClock>,
    _reserved5: [u32; 2],
    /// 0x80c - DRAM Bus Gating Reset register.
    pub dram_bgr: RW<DramBusGating>,
    _reserved6: [u32; 63],
    /// 0x90c - UART Bus Gating Reset register.
    pub uart_bgr: RW<UartBusGating>,
    _reserved7: [u32; 12],
    /// 0x940..=0x944 - SPI0 Clock Register and SPI1 Clock Register.
    pub spi_clk: [RW<SpiClock>; 2],
    _reserved8: [u32; 9],
    /// 0x96c - SPI Bus Gating Reset register.
    pub spi_bgr: RW<SpiBusGating>,
}

/// CPU PLL Control register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PllCpuControl(u32);

impl PllCpuControl {
    const PLL_ENABLE: u32 = 1 << 31;
    const PLL_LDO_ENABLE: u32 = 1 << 30;
    const LOCK_ENABLE: u32 = 1 << 29;
    const LOCK: u32 = 1 << 28;
    const PLL_OUTPUT_GATE: u32 = 1 << 27;
    const PLL_N: u32 = 0xff << 8;
    const PLL_M: u32 = 0x3 << 0;

    /// Get if PLL is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        self.0 & Self::PLL_ENABLE != 0
    }
    /// Enable PLL.
    #[inline]
    pub const fn enable_pll(self) -> Self {
        Self(self.0 | Self::PLL_ENABLE)
    }
    /// Disable PLL.
    #[inline]
    pub const fn disable_pll(self) -> Self {
        Self(self.0 & !Self::PLL_ENABLE)
    }
    /// Get if PLL LDO is enabled.
    #[inline]
    pub const fn is_pll_ldo_enabled(self) -> bool {
        self.0 & Self::PLL_LDO_ENABLE != 0
    }
    /// Enable PLL LDO.
    #[inline]
    pub const fn enable_pll_ldo(self) -> Self {
        Self(self.0 | Self::PLL_LDO_ENABLE)
    }
    /// Disable PLL LDO.
    #[inline]
    pub const fn disable_pll_ldo(self) -> Self {
        Self(self.0 & !Self::PLL_LDO_ENABLE)
    }
    /// Get if PLL lock is enabled.
    #[inline]
    pub const fn is_lock_enabled(self) -> bool {
        self.0 & Self::LOCK_ENABLE != 0
    }
    /// Enable PLL lock.
    #[inline]
    pub const fn enable_lock(self) -> Self {
        Self(self.0 | Self::LOCK_ENABLE)
    }
    /// Disable PLL lock.
    #[inline]
    pub const fn disable_lock(self) -> Self {
        Self(self.0 & !Self::LOCK_ENABLE)
    }
    /// Get if the PLL locked state is set by hardware.
    #[inline]
    pub const fn is_locked(self) -> bool {
        self.0 & Self::LOCK != 0
    }
    /// Unmask (enable) PLL output.
    pub const fn unmask_pll_output(self) -> Self {
        Self(self.0 | Self::PLL_OUTPUT_GATE)
    }
    /// Mask (disable) PLL output.
    #[inline]
    pub const fn mask_pll_output(self) -> Self {
        Self(self.0 & !Self::PLL_OUTPUT_GATE)
    }
    /// Get if PLL output is unmasked.
    #[inline]
    pub const fn is_pll_output_unmasked(self) -> bool {
        self.0 & Self::PLL_OUTPUT_GATE != 0
    }
    /// Get PLL N factor.
    #[inline]
    pub const fn pll_n(self) -> u8 {
        ((self.0 & Self::PLL_N) >> 8) as u8
    }
    /// Set PLL N factor.
    #[inline]
    pub const fn set_pll_n(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_N) | ((val as u32) << 8))
    }
    /// Get PLL M factor.
    #[inline]
    pub const fn pll_m(self) -> u8 {
        (self.0 & Self::PLL_M) as u8
    }
    /// Set PLL M factor.
    #[inline]
    pub const fn set_pll_m(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_M) | val as u32)
    }
}

/// DDR PLL Control register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PllDdrControl(u32);

impl PllDdrControl {
    const PLL_ENABLE: u32 = 1 << 31;
    const PLL_LDO_ENABLE: u32 = 1 << 30;
    const LOCK_ENABLE: u32 = 1 << 29;
    const LOCK: u32 = 1 << 28;
    const PLL_OUTPUT_GATE: u32 = 1 << 27;
    const PLL_N: u32 = 0xff << 8;
    const PLL_M1: u32 = 0x1 << 1;
    const PLL_M0: u32 = 0x1 << 0;

    /// Get if PLL is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        self.0 & Self::PLL_ENABLE != 0
    }
    /// Enable PLL.
    #[inline]
    pub const fn enable_pll(self) -> Self {
        Self(self.0 | Self::PLL_ENABLE)
    }
    /// Disable PLL.
    #[inline]
    pub const fn disable_pll(self) -> Self {
        Self(self.0 & !Self::PLL_ENABLE)
    }
    /// Get if PLL LDO is enabled.
    #[inline]
    pub const fn is_pll_ldo_enabled(self) -> bool {
        self.0 & Self::PLL_LDO_ENABLE != 0
    }
    /// Enable PLL LDO.
    #[inline]
    pub const fn enable_pll_ldo(self) -> Self {
        Self(self.0 | Self::PLL_LDO_ENABLE)
    }
    /// Disable PLL LDO.
    #[inline]
    pub const fn disable_pll_ldo(self) -> Self {
        Self(self.0 & !Self::PLL_LDO_ENABLE)
    }
    /// Get if PLL lock is enabled.
    #[inline]
    pub const fn is_lock_enabled(self) -> bool {
        self.0 & Self::LOCK_ENABLE != 0
    }
    /// Enable PLL lock.
    #[inline]
    pub const fn enable_lock(self) -> Self {
        Self(self.0 | Self::LOCK_ENABLE)
    }
    /// Disable PLL lock.
    #[inline]
    pub const fn disable_lock(self) -> Self {
        Self(self.0 & !Self::LOCK_ENABLE)
    }
    /// Get if the PLL locked state is set by hardware.
    #[inline]
    pub const fn is_locked(self) -> bool {
        self.0 & Self::LOCK != 0
    }
    /// Unmask (enable) PLL output.
    pub const fn unmask_pll_output(self) -> Self {
        Self(self.0 | Self::PLL_OUTPUT_GATE)
    }
    /// Mask (disable) PLL output.
    #[inline]
    pub const fn mask_pll_output(self) -> Self {
        Self(self.0 & !Self::PLL_OUTPUT_GATE)
    }
    /// Get if PLL output is unmasked.
    #[inline]
    pub const fn is_pll_output_unmasked(self) -> bool {
        self.0 & Self::PLL_OUTPUT_GATE != 0
    }
    /// Get PLL N factor.
    #[inline]
    pub const fn pll_n(self) -> u8 {
        ((self.0 & Self::PLL_N) >> 8) as u8
    }
    /// Set PLL N factor.
    #[inline]
    pub const fn set_pll_n(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_N) | ((val as u32) << 8))
    }
    /// Get PLL M1 factor.
    #[inline]
    pub const fn pll_m1(self) -> u8 {
        ((self.0 & Self::PLL_M1) >> 1) as u8
    }
    /// Set PLL M1 factor.
    #[inline]
    pub const fn set_pll_m1(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_M1) | ((val as u32) << 1))
    }
    /// Get PLL M0 factor.
    #[inline]
    pub const fn pll_m0(self) -> u8 {
        (self.0 & Self::PLL_M0) as u8
    }
    /// Set PLL M0 factor.
    #[inline]
    pub const fn set_pll_m0(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_M0) | val as u32)
    }
}

/// Peripheral PLL Control register 0.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PllPeri0Control(u32);

impl PllPeri0Control {
    const PLL_ENABLE: u32 = 1 << 31;
    const PLL_LDO_ENABLE: u32 = 1 << 30;
    const LOCK_ENABLE: u32 = 1 << 29;
    const LOCK: u32 = 1 << 28;
    const PLL_OUTPUT_GATE: u32 = 1 << 27;
    const PLL_P1: u32 = 0x07 << 20;
    const PLL_P0: u32 = 0x07 << 16;
    const PLL_N: u32 = 0xff << 8;
    const PLL_M: u32 = 0x1 << 1;

    /// Get if PLL is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        self.0 & Self::PLL_ENABLE != 0
    }
    /// Enable PLL.
    #[inline]
    pub const fn enable_pll(self) -> Self {
        Self(self.0 | Self::PLL_ENABLE)
    }
    /// Disable PLL.
    #[inline]
    pub const fn disable_pll(self) -> Self {
        Self(self.0 & !Self::PLL_ENABLE)
    }
    /// Get if PLL LDO is enabled.
    #[inline]
    pub const fn is_pll_ldo_enabled(self) -> bool {
        self.0 & Self::PLL_LDO_ENABLE != 0
    }
    /// Enable PLL LDO.
    #[inline]
    pub const fn enable_pll_ldo(self) -> Self {
        Self(self.0 | Self::PLL_LDO_ENABLE)
    }
    /// Disable PLL LDO.
    #[inline]
    pub const fn disable_pll_ldo(self) -> Self {
        Self(self.0 & !Self::PLL_LDO_ENABLE)
    }
    /// Get if PLL lock is enabled.
    #[inline]
    pub const fn is_lock_enabled(self) -> bool {
        self.0 & Self::LOCK_ENABLE != 0
    }
    /// Enable PLL lock.
    #[inline]
    pub const fn enable_lock(self) -> Self {
        Self(self.0 | Self::LOCK_ENABLE)
    }
    /// Disable PLL lock.
    #[inline]
    pub const fn disable_lock(self) -> Self {
        Self(self.0 & !Self::LOCK_ENABLE)
    }
    /// Get if the PLL locked state is set by hardware.
    #[inline]
    pub const fn is_locked(self) -> bool {
        self.0 & Self::LOCK != 0
    }
    /// Unmask (enable) PLL output.
    pub const fn unmask_pll_output(self) -> Self {
        Self(self.0 | Self::PLL_OUTPUT_GATE)
    }
    /// Mask (disable) PLL output.
    #[inline]
    pub const fn mask_pll_output(self) -> Self {
        Self(self.0 & !Self::PLL_OUTPUT_GATE)
    }
    /// Get if PLL output is unmasked.
    #[inline]
    pub const fn is_pll_output_unmasked(self) -> bool {
        self.0 & Self::PLL_OUTPUT_GATE != 0
    }
    /// Get PLL P1 factor.
    #[inline]
    pub const fn pll_p1(self) -> u8 {
        ((self.0 & Self::PLL_P1) >> 20) as u8
    }
    /// Set PLL P1 factor.
    #[inline]
    pub const fn set_pll_p1(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_P1) | ((val as u32) << 20))
    }
    /// Get PLL P0 factor.
    #[inline]
    pub const fn pll_p0(self) -> u8 {
        ((self.0 & Self::PLL_P0) >> 16) as u8
    }
    /// Set PLL P0 factor.
    #[inline]
    pub const fn set_pll_p0(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_P0) | ((val as u32) << 16))
    }
    /// Get PLL N factor.
    #[inline]
    pub const fn pll_n(self) -> u8 {
        ((self.0 & Self::PLL_N) >> 8) as u8
    }
    /// Set PLL N factor.
    #[inline]
    pub const fn set_pll_n(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_N) | ((val as u32) << 8))
    }
    /// Get PLL M factor.
    #[inline]
    pub const fn pll_m(self) -> u8 {
        ((self.0 & Self::PLL_M) >> 1) as u8
    }
    /// Set PLL M factor.
    #[inline]
    pub const fn set_pll_m(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_M) | ((val as u32) << 1))
    }
}

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
    const FACTOR_P: u32 = 0x3 << 16;
    const FACTOR_N: u32 = 0x3 << 8;
    const FACTOR_M: u32 = 0x3 << 0;

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
    pub const fn factor_n(self) -> u8 {
        ((self.0 & Self::FACTOR_N) >> 8) as u8
    }
    /// Set AXI CPU clock divide factor N.
    #[inline]
    pub const fn set_factor_n(self, val: u8) -> Self {
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
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct MbusClock(u32);

impl MbusClock {
    // TODO assert_reset, deassert_reset, is_reset_asserted
}

// TODO enum DramClockSource { PllDdr, PllAudio1, PllPeri2x, PllPeri800M }

/// DRAM Clock Register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DramClock(u32);

impl DramClock {
    // TODO bit 31 - is_clock_unmasked, mask_clock, unmask_clock
    // TODO bit 26:24 - clock_source, set_clock_source (DramClockSource)
    // TODO bit 9:8 - factor_n, set_factor_n (FactorN)
    // TODO bit 1:0 - factor_m, set_factor_m (u8)
}

/// Dram Bus Gating Reset register.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DramBusGating(u32);

impl DramBusGating {
    // TODO bit 16: pub const fn assert_reset(self) -> Self, deassert_reset
    // TODO bit 0: gate_mask, gate_pass
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

/// Clock divide factor P.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactorP {
    /// Don't divide.
    P1,
    /// Divide frequency by 2.
    P2,
    /// Divide frequency by 4.
    P4,
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
    use super::{
        CpuAxiConfig, CpuClockSource, FactorP, PllCpuControl, PllDdrControl, PllPeri0Control,
        RegisterBlock,
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
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, spi_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
    }

    #[test]
    fn struct_pll_cpu_control_fuctions() {
        let mut val = PllCpuControl(0x0);

        val = val.enable_pll();
        assert_eq!(val.0, 0x80000000);
        assert!(val.is_pll_enabled());

        val = val.disable_pll();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_enabled());

        val = val.enable_pll_ldo();
        assert_eq!(val.0, 0x40000000);
        assert!(val.is_pll_ldo_enabled());

        val = val.disable_pll_ldo();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_ldo_enabled());

        val = val.enable_lock();
        assert_eq!(val.0, 0x20000000);
        assert!(val.is_lock_enabled());

        val = val.disable_lock();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_lock_enabled());

        let val = PllCpuControl(0x10000000);
        assert!(val.is_locked());
        let val = PllCpuControl(0x0);
        assert!(!val.is_locked());

        let mut val = PllCpuControl(0x0);

        val = val.unmask_pll_output();
        assert_eq!(val.0, 0x08000000);
        assert!(val.is_pll_output_unmasked());

        val = val.mask_pll_output();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_output_unmasked());

        val = val.set_pll_n(0xFF);
        assert_eq!(val.0, 0x0000FF00);
        assert_eq!(val.pll_n(), 0xFF);

        val = val.set_pll_n(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_n(), 0x0);

        val = val.set_pll_m(0x03);
        assert_eq!(val.0, 0x00000003);
        assert_eq!(val.pll_m(), 0x03);

        val = val.set_pll_m(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_m(), 0x0);
    }

    #[test]
    fn struct_pll_ddr_control_fuctions() {
        let mut val = PllDdrControl(0x0);

        val = val.enable_pll();
        assert_eq!(val.0, 0x80000000);
        assert!(val.is_pll_enabled());

        val = val.disable_pll();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_enabled());

        val = val.enable_pll_ldo();
        assert_eq!(val.0, 0x40000000);
        assert!(val.is_pll_ldo_enabled());

        val = val.disable_pll_ldo();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_ldo_enabled());

        val = val.enable_lock();
        assert_eq!(val.0, 0x20000000);
        assert!(val.is_lock_enabled());

        val = val.disable_lock();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_lock_enabled());

        let val = PllDdrControl(0x10000000);
        assert!(val.is_locked());
        let val = PllDdrControl(0x0);
        assert!(!val.is_locked());

        let mut val = PllDdrControl(0x0);

        val = val.unmask_pll_output();
        assert_eq!(val.0, 0x08000000);
        assert!(val.is_pll_output_unmasked());

        val = val.mask_pll_output();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_output_unmasked());

        val = val.set_pll_n(0xFF);
        assert_eq!(val.0, 0x0000FF00);
        assert_eq!(val.pll_n(), 0xFF);

        val = val.set_pll_n(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_n(), 0x0);

        val = val.set_pll_m1(0x01);
        assert_eq!(val.0, 0x00000002);
        assert_eq!(val.pll_m1(), 0x01);

        val = val.set_pll_m1(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_m1(), 0x0);

        val = val.set_pll_m0(0x01);
        assert_eq!(val.0, 0x00000001);
        assert_eq!(val.pll_m0(), 0x01);

        val = val.set_pll_m0(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_m0(), 0x0);
    }

    #[test]
    fn struct_pll_peri0_control_fuctions() {
        let mut val = PllPeri0Control(0x0);

        val = val.enable_pll();
        assert_eq!(val.0, 0x80000000);
        assert!(val.is_pll_enabled());

        val = val.disable_pll();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_enabled());

        val = val.enable_pll_ldo();
        assert_eq!(val.0, 0x40000000);
        assert!(val.is_pll_ldo_enabled());

        val = val.disable_pll_ldo();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_ldo_enabled());

        val = val.enable_lock();
        assert_eq!(val.0, 0x20000000);
        assert!(val.is_lock_enabled());

        val = val.disable_lock();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_lock_enabled());

        let val = PllPeri0Control(0x10000000);
        assert!(val.is_locked());
        let val = PllPeri0Control(0x0);
        assert!(!val.is_locked());

        let mut val = PllPeri0Control(0x0);

        val = val.unmask_pll_output();
        assert_eq!(val.0, 0x08000000);
        assert!(val.is_pll_output_unmasked());

        val = val.mask_pll_output();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_output_unmasked());

        val = val.set_pll_p1(0x07);
        assert_eq!(val.0, 0x00700000);
        assert_eq!(val.pll_p1(), 0x07);

        val = val.set_pll_p1(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_p1(), 0x0);

        val = val.set_pll_p0(0x07);
        assert_eq!(val.0, 0x00070000);
        assert_eq!(val.pll_p0(), 0x07);

        val = val.set_pll_p0(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_p0(), 0x0);

        val = val.set_pll_n(0xFF);
        assert_eq!(val.0, 0x0000FF00);
        assert_eq!(val.pll_n(), 0xFF);

        val = val.set_pll_n(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_n(), 0x0);

        val = val.set_pll_m(0x01);
        assert_eq!(val.0, 0x00000002);
        assert_eq!(val.pll_m(), 0x01);

        val = val.set_pll_m(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_m(), 0x0);
    }

    #[test]
    fn struct_cpu_axi_config_fuctions() {
        let mut val = CpuAxiConfig(0x0);

        for i in 0..7 as u8 {
            let tmp = match i {
                0 => CpuClockSource::Osc24M,
                1 => CpuClockSource::Clk32K,
                2 => CpuClockSource::Clk16MRC,
                3 => CpuClockSource::PllCpu,
                4 => CpuClockSource::PllPeri1x,
                5 => CpuClockSource::PllPeri2x,
                6 => CpuClockSource::PllPeri800M,
                _ => panic!("impossible clock source"),
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
                _ => panic!("impossible clock source"),
            }

            assert_eq!(val.clock_source(), tmp);
        }

        val = val.set_clock_source(CpuClockSource::Osc24M);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.clock_source(), CpuClockSource::Osc24M);

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

        val = val.set_factor_n(0x03);
        assert_eq!(val.0, 0x00000300);
        assert_eq!(val.factor_n(), 0x03);

        val = val.set_factor_n(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.factor_n(), 0x0);

        val = val.set_factor_m(0x03);
        assert_eq!(val.0, 0x00000003);
        assert_eq!(val.factor_m(), 0x03);

        val = val.set_factor_m(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.factor_m(), 0x0);
    }

    // TODO structure read/write function unit tests.
    // Please refer to this link while implementing: https://github.com/rustsbi/bouffalo-hal/blob/6ee8ebf5fde184a68f4c3d5a1b7838dbbc7bfdd3/bouffalo-hal/src/i2c.rs#L902
    // TODO struct_mbus_clock_functions
    // TODO struct_dram_clock_functions
    // TODO struct_dram_bgr_functions
}
