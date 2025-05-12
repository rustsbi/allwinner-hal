use super::{
    eint::EintPad,
    function::Function,
    mode::{FromRegisters, IntoRegisters, borrow_with_mode, set_mode},
    output::Output,
    port_index,
    register::RegisterBlock,
};

/// Input mode pad.
pub struct Input<'a, const P: char, const N: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8> Input<'a, P, N> {
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
    /// Borrows the pad to temporarily use it as an output pad.
    #[inline]
    pub fn with_output<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Output<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
    /// Borrows the pad to temporarily use it an alternate function pad.
    #[inline]
    pub fn with_function<const G: u8, F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Function<'a, P, N, G>) -> T,
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

impl<'a, const P: char, const N: u8> embedded_hal::digital::ErrorType for Input<'a, P, N> {
    type Error = core::convert::Infallible;
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::InputPin for Input<'a, P, N> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port[const { port_index(P) }].dat.read() & (1 << N) != 0)
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port[const { port_index(P) }].dat.read() & (1 << N) == 0)
    }
}

impl<'a, const P: char, const N: u8> IntoRegisters<'a> for Input<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
}

impl<'a, const P: char, const N: u8> FromRegisters<'a> for Input<'a, P, N> {
    const VALUE: u8 = 0;
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}
