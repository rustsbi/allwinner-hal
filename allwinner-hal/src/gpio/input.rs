use super::{
    mode::{FromRegisters, PortAndNumber, borrow_with_mode, set_mode},
    output::Output,
    register::RegisterBlock,
};

/// Input mode pad.
pub struct Input<'a> {
    port: char,
    number: u8,
    gpio: &'a RegisterBlock,
}

impl<'a> Input<'a> {
    /// Borrows the pad to temporarily use it as an output pad.
    #[inline]
    pub fn with_output<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Output<'a>) -> T,
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

impl<'a> embedded_hal::digital::ErrorType for Input<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal::digital::InputPin for Input<'a> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port(self.port).dat.read() & (1 << self.number) != 0)
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port(self.port).dat.read() & (1 << self.number) == 0)
    }
}

impl<'a> PortAndNumber<'a> for Input<'a> {
    #[inline]
    fn port_number(&self) -> (char, u8) {
        (self.port, self.number)
    }
    #[inline]
    fn register_block(&self) -> &'a RegisterBlock {
        self.gpio
    }
}

impl<'a> FromRegisters<'a> for Input<'a> {
    const VALUE: u8 = 0;
    #[inline]
    unsafe fn from_gpio(port: char, number: u8, gpio: &'a RegisterBlock) -> Self {
        Self { port, number, gpio }
    }
}
