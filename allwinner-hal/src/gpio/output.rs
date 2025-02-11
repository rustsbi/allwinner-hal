use super::{
    disabled::Disabled,
    eint::EintPad,
    function::Function,
    input::Input,
    mode::{HasMode, borrow_with_mode, set_mode},
    port_index,
    register::RegisterBlock,
};

/// Output mode pad.
pub struct Output<'a, const P: char, const N: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8> Output<'a, P, N> {
    /// Configures the pad to operate as an input pad.
    #[inline]
    pub fn into_input(self) -> Input<'a, P, N> {
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
    /// Configures the pad to operate as a disabled pad.
    #[inline]
    pub fn into_disabled(self) -> Disabled<'a, P, N> {
        set_mode(self)
    }
    /// Borrows the pad to temporarily use it as an input pad.
    #[inline]
    pub fn with_input<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Input<'a, P, N>) -> T,
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
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::ErrorType for Output<'a, P, N> {
    type Error = core::convert::Infallible;
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::OutputPin for Output<'a, P, N> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let idx = const { port_index(P) };
        unsafe { self.gpio.port[idx].dat.modify(|value| value & !(1 << N)) };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let idx = const { port_index(P) };
        unsafe { self.gpio.port[idx].dat.modify(|value| value | (1 << N)) };
        Ok(())
    }
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::StatefulOutputPin for Output<'a, P, N> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port[const { port_index(P) }].dat.read() & (1 << N) != 0)
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port[const { port_index(P) }].dat.read() & (1 << N) == 0)
    }
}

impl<'a, const P: char, const N: u8> HasMode<'a> for Output<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
    const VALUE: u8 = 1;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}
