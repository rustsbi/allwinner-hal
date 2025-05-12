use super::{EintPad, Function, Input, Output};

/// Extension of `Pad` or `&mut Pad`.
pub trait PadExt<'a, const P: char, const N: u8> {
    /// Configures the pad to operate as an input pad.
    fn into_input(self) -> Input<'a, P, N>;
    /// Configures the pad to operate as an output pad.
    fn into_output(self) -> Output<'a, P, N>;
    /// Configures the pad to operate as an alternate function pad.
    fn into_function<const F: u8>(self) -> Function<'a, P, N, F>;
    /// Configures the pad to operate as an external interrupt pad.
    fn into_eint(self) -> EintPad<'a, P, N>;
    /// Borrows the pad to temporarily use it as an input pad.
    fn with_input<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Input<'a, P, N>) -> T;
    /// Borrows the pad to temporarily use it as an output pad.
    fn with_output<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Output<'a, P, N>) -> T;
    /// Borrows the pad to temporarily use it an alternate function pad.
    fn with_function<const G: u8, F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Function<'a, P, N, G>) -> T;
}
