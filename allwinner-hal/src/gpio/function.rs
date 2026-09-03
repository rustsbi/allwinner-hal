use super::{
    input::Input,
    mode::{FromRegisters, PortAndNumber, borrow_with_mode, set_mode},
    output::Output,
    register::{AnyRegisterBlock, GpioVersion, v1::RegisterBlockV1, v2::RegisterBlockV2},
};

/// Alternate function pad.
///
/// F should be in 2..=8.
pub struct Function<'a, const P: char, const N: u8, const F: u8> {
    version: GpioVersion,
    gpio: &'a AnyRegisterBlock,
}

impl<'a, const P: char, const N: u8, const F: u8> Function<'a, P, N, F> {
    /// Borrows the pad to temporarily use it as an input pad.
    #[inline]
    pub fn with_input<G, T>(&mut self, f: G) -> T
    where
        G: FnOnce(&mut Input<'a>) -> T,
    {
        borrow_with_mode(self, f)
    }
    /// Borrows the pad to temporarily use it as an output pad.
    #[inline]
    pub fn with_output<G, T>(&mut self, f: G) -> T
    where
        G: FnOnce(&mut Output<'a>) -> T,
    {
        borrow_with_mode(self, f)
    }
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_v1(gpio: &'a RegisterBlockV1) -> Self {
        set_mode(Self {
            version: GpioVersion::V1,
            gpio: gpio.as_any(),
        })
    }
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new_v2(gpio: &'a RegisterBlockV2) -> Self {
        set_mode(Self {
            version: GpioVersion::V2,
            gpio: gpio.as_any(),
        })
    }
}

impl<'a, const P: char, const N: u8, const F: u8> PortAndNumber<'a> for Function<'a, P, N, F> {
    #[inline]
    fn port_number(&self) -> (char, u8) {
        (P, N)
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

impl<'a, const P: char, const N: u8, const F: u8> FromRegisters<'a> for Function<'a, P, N, F> {
    #[inline]
    fn mode_value(_: GpioVersion) -> u8 {
        F
    }
    #[inline]
    unsafe fn from_gpio(_: char, _: u8, version: GpioVersion, gpio: &'a AnyRegisterBlock) -> Self {
        Self { version, gpio }
    }
}
