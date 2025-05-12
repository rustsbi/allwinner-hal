use super::{
    eint::EintPad,
    function::Function,
    input::Input,
    mode::{HasMode, borrow_with_mode, set_mode},
    output::Output,
    pad_ext::PadExt,
    register::RegisterBlock,
};

/// Ownership of a GPIO pad.
pub struct Pad<'a, const P: char, const N: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8> Pad<'a, P, N> {
    /// Internal constructor for ROM runtime. Do not use.
    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn __new(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

impl<'a, const P: char, const N: u8> PadExt<'a, P, N> for Pad<'a, P, N> {
    /// Configures the pad to operate as an input pad.
    #[inline]
    fn into_input(self) -> Input<'a, P, N> {
        set_mode(self)
    }
    #[inline]
    fn into_output(self) -> Output<'a, P, N> {
        set_mode(self)
    }
    #[inline]
    fn into_function<const F: u8>(self) -> Function<'a, P, N, F> {
        set_mode(self)
    }
    #[inline]
    fn into_eint(self) -> EintPad<'a, P, N> {
        set_mode(self)
    }
    #[inline]
    fn with_output<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Output<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
    #[inline]
    fn with_input<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Input<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
    #[inline]
    fn with_function<const G: u8, F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Function<'a, P, N, G>) -> T,
    {
        borrow_with_mode(self, f)
    }
}

impl<'a, const P: char, const N: u8> HasMode<'a> for Pad<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
    // Value of a Pad matches 'Disabled' in the Manual; but no pad structure
    // actually convert it into a `Pad`, so this is okay here.
    const VALUE: u8 = 15;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}
