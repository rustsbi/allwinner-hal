use super::{
    eint::EintPad,
    input::Input,
    mode::{FromRegisters, IntoRegisters, borrow_with_mode, set_mode},
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
    /// Configures the pad to operate as an input pad.
    #[inline]
    pub fn into_input(self) -> Input<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an output pad.
    #[inline]
    pub fn into_output(self) -> Output<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an alternate function pad.
    #[inline]
    pub fn into_function<const F2: u8>(self) -> Function<'a, P, N, F2> {
        set_mode(self)
    }
    /// Configures the pad to operate as an external interrupt pad.
    #[inline]
    pub fn into_eint(self) -> EintPad<'a, P, N> {
        set_mode(self)
    }
    /// Borrows the pad to temporarily use it as an input pad.
    #[inline]
    pub fn with_input<G, T>(&mut self, f: G) -> T
    where
        G: FnOnce(&mut Input<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
    /// Borrows the pad to temporarily use it as an output pad.
    #[inline]
    pub fn with_output<G, T>(&mut self, f: G) -> T
    where
        G: FnOnce(&mut Output<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

impl<'a, const P: char, const N: u8, const F: u8> IntoRegisters<'a> for Function<'a, P, N, F> {
    const P: char = P;
    const N: u8 = N;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
}

impl<'a, const P: char, const N: u8, const F: u8> FromRegisters<'a> for Function<'a, P, N, F> {
    const VALUE: u8 = F;
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}
