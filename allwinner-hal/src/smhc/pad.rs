/// Clock signal pad.
pub trait Clk {}

/// Command signal pad.
pub trait Cmd {}

/// Data input and output pad.
///
/// This is documented in the User Manual as `D[3:0]`.
pub trait Data<const I: usize> {}
