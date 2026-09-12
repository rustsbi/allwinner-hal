use super::peripheral::CCU;
use embedded_time::rate::Hertz;

/// ROM clock configuration on current SoC.
#[derive(Debug)]
pub struct Clocks {
    /// PSI clock frequency.
    pub psi: Hertz,
    /// Advanced Peripheral Bus 1 clock frequency.
    pub apb1: Hertz,
}

impl Clocks {
    /// Enable clock of UART `I`.
    #[inline]
    pub fn enable_uart<const I: usize>(&self, ccu: &CCU) -> UartClock<I> {
        unsafe {
            ccu.uart_bgr
                .modify(|v| v.gate_pass::<I>().deassert_reset::<I>())
        };
        UartClock { apb1: self.apb1 }
    }

    /// Configure SPI `I` from the 24 MHz HOSC and enable its clocks.
    ///
    /// Returns the fastest representable frequency not exceeding `frequency`,
    /// or `None` below 187.5 kHz. Requests above 24 MHz select 24 MHz.
    /// Call before constructing the SPI driver, with the controller idle.
    #[inline]
    pub fn enable_spi<const I: usize>(
        &self,
        ccu: &mut CCU,
        frequency: Hertz,
    ) -> Option<SpiClock<I>> {
        const { assert!(I < 2, "D1 only has SPI0 and SPI1") };
        let (n, m, frequency) = spi_dividers(frequency)?;
        // SAFETY: the mutable CCU token serializes configuration. Configure the
        // module while gated, then enable its module and bus clocks and reset.
        unsafe {
            ccu.spi_clk[I].modify(|v| v.mask_clock());
            ccu.spi_clk[I].modify(|v| {
                v.set_clock_source(allwinner_hal::ccu::SpiClockSource::Hosc)
                    .set_factor_n(n)
                    .set_factor_m(m)
                    .unmask_clock()
            });
            ccu.spi_bgr
                .modify(|v| v.gate_pass::<I>().deassert_reset::<I>());
        }
        Some(SpiClock { frequency })
    }
}

impl allwinner_hal::smhc::Clock for Clocks {
    #[inline]
    fn smhc_clock(&self) -> embedded_time::rate::Hertz {
        self.psi
    }
}

/// Dynamic configurated clock configuration on current SoC.
pub struct UartClock<const I: usize> {
    /// Inherited from Advanced Peripheral Bus 1 clock frequency.
    apb1: Hertz,
}

impl<const I: usize> allwinner_hal::uart::Clock<I> for UartClock<I> {
    #[inline]
    fn uart_clock(&self) -> embedded_time::rate::Hertz {
        self.apb1
    }
}

impl<'a, const I: usize> allwinner_hal::uart::Clock<I> for &'a UartClock<I> {
    #[inline]
    fn uart_clock(&self) -> embedded_time::rate::Hertz {
        self.apb1
    }
}

/// Enabled SPI clock configured from the D1 HOSC.
pub struct SpiClock<const I: usize> {
    frequency: Hertz,
}

impl<const I: usize> allwinner_hal::spi::Clock<I> for SpiClock<I> {
    #[inline]
    fn spi_clock(&self) -> Hertz {
        self.frequency
    }
}

impl<const I: usize> allwinner_hal::spi::Clock<I> for &SpiClock<I> {
    #[inline]
    fn spi_clock(&self) -> Hertz {
        self.frequency
    }
}

fn spi_dividers(frequency: Hertz) -> Option<(allwinner_hal::ccu::PeriFactorN, u8, Hertz)> {
    use allwinner_hal::ccu::PeriFactorN;
    let mut best = None;
    for (n, divisor_n) in [
        (PeriFactorN::N1, 1),
        (PeriFactorN::N2, 2),
        (PeriFactorN::N4, 4),
        (PeriFactorN::N8, 8),
    ] {
        for m in 0..16u8 {
            let divisor = divisor_n * (u32::from(m) + 1);
            // Compare before division so fractional hertz never exceed the limit.
            if 24_000_000u64 > u64::from(frequency.0) * u64::from(divisor) {
                continue;
            }
            let actual = 24_000_000 / divisor;
            if best.is_none_or(|(_, _, Hertz(previous))| actual > previous) {
                best = Some((n, m, Hertz(actual)));
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spi_frequency_limits() {
        for (requested, expected) in [
            (0, None),
            (187_499, None),
            (187_500, Some(187_500)),
            (1_000_000, Some(1_000_000)),
            (10_000_000, Some(8_000_000)),
            (24_000_000, Some(24_000_000)),
            (u32::MAX, Some(24_000_000)),
        ] {
            assert_eq!(
                spi_dividers(Hertz(requested)).map(|(_, _, hz)| hz.0),
                expected
            );
        }
    }
}
