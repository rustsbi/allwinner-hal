use super::{EintPad, Function, Input, Output};

/// Extension of `Pad` or `&mut Pad`.
pub trait PadExt<'a, const P: char, const N: u8> {
    /// Configures the pad to operate as an input pad.
    fn into_input(self) -> Input<'a>;
    /// Configures the pad to operate as an output pad.
    fn into_output(self) -> Output<'a>;
    /// Configures the pad to operate as an alternate function pad.
    fn into_function<const F: u8>(self) -> Function<'a, P, N, F>;
    /// Configures the pad to operate as an external interrupt pad.
    fn into_eint(self) -> EintPad<'a>;
}
