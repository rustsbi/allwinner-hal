use crate::gpio::FlexPad;

/// Clock signal pad.
pub trait IntoClk<'a> {
    fn into_smhc_clk(self) -> FlexPad<'a>;
}

/// Command signal pad.
pub trait IntoCmd<'a> {
    fn into_smhc_cmd(self) -> FlexPad<'a>;
}

/// Data input and output pad.
///
/// This is documented in the User Manual as `D[3:0]`.
pub trait IntoData<'a, const I: usize> {
    fn into_smhc_data(self) -> FlexPad<'a>;
}
