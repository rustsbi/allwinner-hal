use super::{
    input::Input,
    mode::{FromRegisters, PortAndNumber, borrow_with_mode, set_mode},
    output::Output,
    register::RegisterBlock,
};

/// Alternate function pad.
///
/// F should be in 2..=8.
pub struct Function<'a, const P: char, const N: u8, const F: u8> {
    gpio: &'a RegisterBlock,
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
    pub unsafe fn __new(gpio: &'a RegisterBlock) -> Self {
        set_mode(Self { gpio })
    }
}

impl<'a, const P: char, const N: u8, const F: u8> PortAndNumber<'a> for Function<'a, P, N, F> {
    #[inline]
    fn port_number(&self) -> (char, u8) {
        (P, N)
    }
    #[inline]
    fn register_block(&self) -> &'a RegisterBlock {
        self.gpio
    }
}

impl<'a, const P: char, const N: u8, const F: u8> FromRegisters<'a> for Function<'a, P, N, F> {
    const VALUE: u8 = F;
    #[inline]
    unsafe fn from_gpio(_: char, _: u8, gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}
