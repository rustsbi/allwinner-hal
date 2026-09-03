use super::{
    input::Input,
    mode::{FromRegisters, PortAndNumber, borrow_with_mode},
    register::{AnyRegisterBlock, GpioVersion},
};

/// Output mode pad.
pub struct Output<'a> {
    port: char,
    number: u8,
    version: GpioVersion,
    gpio: &'a AnyRegisterBlock,
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
    impl_gpio_constructors!(pad);
}

impl<'a> embedded_hal::digital::ErrorType for Output<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal::digital::OutputPin for Output<'a> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.gpio
                .with_version(self.version)
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
                .with_version(self.version)
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
        let value = unsafe { self.gpio.with_version(self.version) }
            .port(self.port)
            .dat
            .read();
        Ok(value & (1 << self.number) != 0)
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        let value = unsafe { self.gpio.with_version(self.version) }
            .port(self.port)
            .dat
            .read();
        Ok(value & (1 << self.number) == 0)
    }
}

impl<'a> PortAndNumber<'a> for Output<'a> {
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

impl<'a> FromRegisters<'a> for Output<'a> {
    #[inline]
    fn mode_value(_: GpioVersion) -> u8 {
        1
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
