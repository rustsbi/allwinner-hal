//! Clock Control Unit peripheral.

mod factor;
mod pll;
mod register;
mod source;

pub(crate) use factor::calculate_best_peripheral_factors_nm;
pub use factor::{AxiFactorN, FactorP, PeriFactorN};
pub use pll::{PllCpuControl, PllDdrControl, PllPeri0Control};
pub use register::*;
pub use source::{CpuClockSource, DramClockSource, SmhcClockSource, SpiClockSource};

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

/// LED Control (LEDC) clock type.
pub struct LEDC;

impl ClockReset for LEDC {
    #[inline]
    unsafe fn deassert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.ledc_bgr.modify(|v| v.deassert_reset());
        }
    }
    #[inline]
    unsafe fn assert_reset_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.ledc_bgr.modify(|v| v.assert_reset());
        }
    }
}

impl ClockGate for LEDC {
    #[inline]
    unsafe fn unmask_gate_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.ledc_bgr.modify(|v| v.gate_pass());
        }
    }
    #[inline]
    unsafe fn mask_gate_only(ccu: &RegisterBlock) {
        unsafe {
            ccu.ledc_bgr.modify(|v| v.gate_mask());
        }
    }
    #[inline]
    unsafe fn disable_in(ccu: &RegisterBlock) {
        unsafe {
            ccu.ledc_bgr.modify(|v| v.gate_mask().assert_reset());
        }
    }
    #[inline]
    unsafe fn enable_in(ccu: &RegisterBlock) {
        unsafe {
            ccu.ledc_bgr.modify(|v| v.gate_pass().deassert_reset());
        }
    }
}
