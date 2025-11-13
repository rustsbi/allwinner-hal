//! Serial Peripheral Interface bus.

pub mod blocking;
pub mod register;
pub use blocking::Spi as BlockingSpi;
use embedded_time::rate::Hertz;
pub use register::*;

use crate::ccu::{self, SpiClockSource};

/// Valid SPI pins.
pub trait Pins<const I: usize> {
    type Clock: ccu::ClockGate + ccu::ClockConfig<Source = SpiClockSource>;
}

/// Valid clk pin for SPI peripheral.
pub trait IntoClk<const I: usize> {}

/// Valid mosi pin for SPI peripheral.
pub trait IntoMosi<const I: usize> {}

/// Valid miso pin for SPI peripheral.
pub trait IntoMiso<const I: usize> {}

impl<const I: usize, CLK, MOSI, MISO> Pins<I> for (CLK, MOSI, MISO)
where
    CLK: IntoClk<I>,
    MOSI: IntoMosi<I>,
    MISO: IntoMiso<I>,
{
    type Clock = ccu::SPI<I>;
}

pub trait Clock {
    fn spi_clock(&self) -> Hertz;
}
