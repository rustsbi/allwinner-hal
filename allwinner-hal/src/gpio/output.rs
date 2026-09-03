use super::{
    input::Input,
    mode::{FromRegisters, PortAndNumber, borrow_with_mode, set_mode},
    register::{
        AnyRegisterBlock, GpioVersion, Versioned, v1::RegisterBlockV1, v2::RegisterBlockV2,
    },
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
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_v1(port: char, number: u8, gpio: &'a RegisterBlockV1) -> Self {
        set_mode(Self {
            gpio: gpio.as_any(),
            port,
            version: GpioVersion::V1,
            number,
        })
    }
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_v2(port: char, number: u8, gpio: &'a RegisterBlockV2) -> Self {
        set_mode(Self {
            gpio: gpio.as_any(),
            port,
            version: GpioVersion::V2,
            number,
        })
    }
}

impl<'a> embedded_hal::digital::ErrorType for Output<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal::digital::OutputPin for Output<'a> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        match unsafe { self.gpio.with_version(self.version) } {
            Versioned::V1(gpio) => unsafe {
                gpio.port(self.port)
                    .dat
                    .modify(|value| value & !(1 << self.number))
            },
            Versioned::V2(gpio) => unsafe {
                gpio.port(self.port)
                    .dat
                    .modify(|value| value & !(1 << self.number))
            },
        }
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        match unsafe { self.gpio.with_version(self.version) } {
            Versioned::V1(gpio) => unsafe {
                gpio.port(self.port)
                    .dat
                    .modify(|value| value | (1 << self.number))
            },
            Versioned::V2(gpio) => unsafe {
                gpio.port(self.port)
                    .dat
                    .modify(|value| value | (1 << self.number))
            },
        }
        Ok(())
    }
}

impl<'a> embedded_hal::digital::StatefulOutputPin for Output<'a> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        let value = match unsafe { self.gpio.with_version(self.version) } {
            Versioned::V1(gpio) => gpio.port(self.port).dat.read(),
            Versioned::V2(gpio) => gpio.port(self.port).dat.read(),
        };
        Ok(value & (1 << self.number) != 0)
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        let value = match unsafe { self.gpio.with_version(self.version) } {
            Versioned::V1(gpio) => gpio.port(self.port).dat.read(),
            Versioned::V2(gpio) => gpio.port(self.port).dat.read(),
        };
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
