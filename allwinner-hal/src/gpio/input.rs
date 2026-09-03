use super::{
    mode::{FromRegisters, PortAndNumber, borrow_with_mode},
    output::Output,
    register::{AnyRegisterBlock, GpioVersion},
};

/// Input mode pad.
pub struct Input<'a> {
    port: char,
    number: u8,
    version: GpioVersion,
    gpio: &'a AnyRegisterBlock,
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
    impl_gpio_constructors!(pad);
}

impl<'a> embedded_hal::digital::ErrorType for Input<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal::digital::InputPin for Input<'a> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        let value = unsafe { self.gpio.with_version(self.version) }
            .port(self.port)
            .dat
            .read();
        Ok(value & (1 << self.number) != 0)
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        let value = unsafe { self.gpio.with_version(self.version) }
            .port(self.port)
            .dat
            .read();
        Ok(value & (1 << self.number) == 0)
    }
}

impl<'a> PortAndNumber<'a> for Input<'a> {
    #[inline]
    fn port_number(&self) -> (char, u8) {
        (self.port, self.number)
    }
    #[inline]
    fn gpio_version(&self) -> GpioVersion {
        self.version
    }
    #[inline]
    fn register_block(&self) -> &'a AnyRegisterBlock {
        self.gpio
    }
}

impl<'a> FromRegisters<'a> for Input<'a> {
    #[inline]
    fn mode_value(_: GpioVersion) -> u8 {
        0
    }
    #[inline]
    unsafe fn from_gpio(
        port: char,
        number: u8,
        version: GpioVersion,
        gpio: &'a AnyRegisterBlock,
    ) -> Self {
        Self {
            port,
            number,
            version,
            gpio,
        }
    }
}
