//! Universal Asynchronous Receiver-Transmitter.

pub mod blocking;
pub mod config;
pub mod register;

pub use blocking::{
    ReceiveHalf as BlockingReceiveHalf, Serial as BlockingSerial,
    TransmitHalf as BlockingTransmitHalf,
};
pub use config::{Config, Parity, StopBits, WordLength};
use embedded_time::rate::Hertz;
pub use register::RegisterBlock;

use crate::gpio::FlexPad;

/// Extend constructor to owned UART register blocks.
pub trait UartExt<'a, const I: usize> {
    /// Creates a polling serial instance, without interrupt or DMA configurations.
    fn serial(
        self,
        pads: impl Pads<'a, I>,
        config: impl Into<Config>,
        clock: impl Clock,
    ) -> BlockingSerial<'a>;
}

/// Peripheral instance of UART.
pub trait Instance<'a> {
    /// Retrieve register block for this instance.
    fn register_block(self) -> &'a RegisterBlock;
}

/// Valid serial pads.
pub trait Pads<'a, const I: usize> {
    fn into_uart_pads(self) -> (Option<FlexPad<'a>>, Option<FlexPad<'a>>);
}

/// Valid transmit pin for UART peripheral.
#[diagnostic::on_unimplemented(message = "selected pad does not connect to UART{I} TX signal")]
pub trait IntoTransmit<'a, const I: usize> {
    fn into_uart_transmit(self) -> FlexPad<'a>;
}

/// Valid receive pin for UART peripheral.
#[diagnostic::on_unimplemented(message = "selected pad does not connect to UART{I} RX signal")]
pub trait IntoReceive<'a, const I: usize> {
    fn into_uart_receive(self) -> FlexPad<'a>;
}

impl<'a, const I: usize, T, R> Pads<'a, I> for (T, R)
where
    T: IntoTransmit<'a, I>,
    R: IntoReceive<'a, I>,
{
    #[inline]
    fn into_uart_pads(self) -> (Option<FlexPad<'a>>, Option<FlexPad<'a>>) {
        (
            Some(self.0.into_uart_transmit()),
            Some(self.1.into_uart_receive()),
        )
    }
}

/// Valid clock input for UART peripheral.
pub trait Clock {
    /// UART clock frequency in hertz.
    fn uart_clock(&self) -> Hertz;
}
