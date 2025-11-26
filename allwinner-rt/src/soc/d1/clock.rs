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
                .modify(|v| v.gate_pass::<0>().deassert_reset::<0>())
        };
        UartClock { apb1: self.apb1 }
    }
}

// No UART clock; should enable first.

impl allwinner_hal::spi::Clock for Clocks {
    #[inline]
    fn spi_clock(&self) -> embedded_time::rate::Hertz {
        // TODO calculate from more clock parameters
        self.psi
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
