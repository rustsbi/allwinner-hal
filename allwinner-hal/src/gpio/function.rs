use super::{
    input::Input,
    mode::{FromRegisters, PortAndNumber, borrow_with_mode},
    output::Output,
    register::{AnyRegisterBlock, GpioVersion},
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
    impl_gpio_constructors!(function);
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
