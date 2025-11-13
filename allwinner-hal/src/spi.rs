//! Serial Peripheral Interface bus.

pub mod blocking;
pub mod register;
pub use blocking::Spi as BlockingSpi;
use embedded_time::rate::Hertz;
pub use register::*;

use crate::gpio::FlexPad;

/// Valid SPI pins.
pub trait Pads<'a, const I: usize> {
    fn into_spi_pads(
        self,
    ) -> (
        Option<FlexPad<'a>>,
        Option<FlexPad<'a>>,
        Option<FlexPad<'a>>,
    );
}

/// Valid clk pin for SPI peripheral.
pub trait IntoClk<'a, const I: usize> {
    fn into_spi_clk(self) -> FlexPad<'a>;
}

/// Valid mosi pin for SPI peripheral.
pub trait IntoMosi<'a, const I: usize> {
    fn into_spi_mosi(self) -> FlexPad<'a>;
}

/// Valid miso pin for SPI peripheral.
pub trait IntoMiso<'a, const I: usize> {
    fn into_spi_miso(self) -> FlexPad<'a>;
}

impl<'a, const I: usize, CLK, MOSI, MISO> Pads<'a, I> for (CLK, MOSI, MISO)
where
    CLK: IntoClk<'a, I>,
    MOSI: IntoMosi<'a, I>,
    MISO: IntoMiso<'a, I>,
{
    #[inline]
    fn into_spi_pads(
        self,
    ) -> (
        Option<FlexPad<'a>>,
        Option<FlexPad<'a>>,
        Option<FlexPad<'a>>,
    ) {
        (
            Some(self.0.into_spi_clk()),
            Some(self.1.into_spi_mosi()),
            Some(self.2.into_spi_miso()),
        )
    }
}

pub trait Clock {
    fn spi_clock(&self) -> Hertz;
}
