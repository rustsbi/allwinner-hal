/// AXI CPU clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CpuClockSource {
    /// 24-MHz 'HOSC' external oscillator.
    Hosc = 0,
    /// 32-KHz clock.
    Clk32K = 1,
    /// 16-MHz RC oscillator.
    Clk16MRC = 2,
    /// CPU PLL.
    PllCpu = 3,
    /// Peripheral PLL (1x frequency).
    PllPeri1x = 4,
    /// Peripheral PLL (2x frequency).
    PllPeri2x = 5,
    /// Peripheral PLL (800-MHz).
    PllPeri800M = 6,
}

/// Dram clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DramClockSource {
    /// DRAM PLL.
    PllDdr = 0,
    /// Audio PLL 1 (divided by 2).
    PllAudio1Div2 = 1,
    /// Peripheral PLL (2x frequency).
    PllPeri2x = 2,
    /// Peripheral PLL (800-MHz).
    PllPeri800M = 3,
}

/// SPI clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpiClockSource {
    /// 24-MHz 'HOSC' external oscillator.
    Hosc = 0,
    /// Peripheral PLL (1x frequency).
    PllPeri1x = 1,
    /// Peripheral PLL (2x frequency).
    PllPeri2x = 2,
    /// Audio PLL (div 2).
    PllAudio1Div2 = 3,
    /// Audio PLL (div 5).
    PllAudio1Div5 = 4,
}

/// SMHC clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SmhcClockSource {
    /// 24-MHz 'HOSC' external oscillator.
    Hosc = 0,
    /// Peripheral PLL (1x frequency).
    PllPeri1x = 1,
    /// Peripheral PLL (2x frequency).
    PllPeri2x = 2,
    /// Peripheral PLL (800-MHz).
    PllPeri800M = 3,
    /// Audio PLL 1 (divided by 2).
    PllAudio1Div2 = 4,
}
