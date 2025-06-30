use super::{
    input::Input,
    mode::{FromRegisters, PortAndNumber, borrow_with_mode, set_mode},
    register::RegisterBlock,
};

/// Output mode pad.
pub struct Output<'a> {
    port: char,
    number: u8,
    gpio: &'a RegisterBlock,
}

impl<'a> Output<'a> {
    /// Borrows the pad to temporarily use it as an input pad.
    #[inline]
    pub fn with_input<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Input<'a>) -> T,
    {
        borrow_with_mode(self, f)
    }
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(port: char, number: u8, gpio: &'a RegisterBlock) -> Self {
        set_mode(Self { gpio, port, number })
    }
}

impl<'a> embedded_hal::digital::ErrorType for Output<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal::digital::OutputPin for Output<'a> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.gpio
                .port(self.port)
                .dat
                .modify(|value| value & !(1 << self.number))
        };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.gpio
                .port(self.port)
                .dat
                .modify(|value| value | (1 << self.number))
        };
        Ok(())
    }
}

impl<'a> embedded_hal::digital::StatefulOutputPin for Output<'a> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port(self.port).dat.read() & (1 << self.number) != 0)
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port(self.port).dat.read() & (1 << self.number) == 0)
    }
}

impl<'a> PortAndNumber<'a> for Output<'a> {
    #[inline]
    fn port_number(&self) -> (char, u8) {
        (self.port, self.number)
    }
    #[inline]
    fn register_block(&self) -> &'a RegisterBlock {
        self.gpio
    }
}

impl<'a> FromRegisters<'a> for Output<'a> {
    const VALUE: u8 = 1;
    #[inline]
    unsafe fn from_gpio(port: char, number: u8, gpio: &'a RegisterBlock) -> Self {
        Self { port, number, gpio }
    }
}
