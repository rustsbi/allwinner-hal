use super::{
    eint::EintPad,
    function::Function,
    input::Input,
    mode::{HasMode, set_mode},
    output::Output,
    register::RegisterBlock,
};

/// Disabled GPIO pad.
pub struct Disabled<'a, const P: char, const N: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8> Disabled<'a, P, N> {
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
    pub fn into_function<const F: u8>(self) -> Function<'a, P, N, F> {
        set_mode(self)
    }
    /// Configures the pad to operate as an external interrupt pad.
    #[inline]
    pub fn into_eint(self) -> EintPad<'a, P, N> {
        set_mode(self)
    }

    /// Internal constructor for ROM runtime. Do not use.
    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn __new(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

impl<'a, const P: char, const N: u8> HasMode<'a> for Disabled<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
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
