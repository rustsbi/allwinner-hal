//! Universal Asynchronous Receiver-Transmitter.

pub mod blocking;
pub mod config;
pub mod register;

pub use blocking::{
    ReceiveHalf as BlockingReceiveHalf, Serial as BlockingSerial,
    TransmitHalf as BlockingTransmitHalf,
};
pub use config::{Config, Parity, StopBits, WordLength};
pub use register::RegisterBlock;

use crate::ccu::{self, Clocks};

/// Extend constructor to owned UART register blocks.
pub trait UartExt<'a, const I: usize> {
    /// Creates a polling serial instance, without interrupt or DMA configurations.
    fn serial<PADS>(
        self,
        pads: PADS,
        config: impl Into<Config>,
        clocks: &Clocks,
        ccu: &ccu::RegisterBlock,
    ) -> BlockingSerial<'a, PADS>
    where
        PADS: Pads<I>;
}

/// Peripheral instance of UART.
pub trait Instance<'a> {
    /// Retrieve register block for this instance.
    fn register_block(self) -> &'a RegisterBlock;
}

/// Valid serial pads.
pub trait Pads<const I: usize> {
    type Clock: ccu::ClockGate + ccu::ClockReset;
    type Pads;
    fn into_uart_pads(self) -> Self::Pads;
}

/// Valid transmit pin for UART peripheral.
#[diagnostic::on_unimplemented(message = "selected pad does not connect to UART{I} TX signal")]
pub trait IntoTransmit<const I: usize> {
    type Transmit;
    fn into_uart_transmit(self) -> Self::Transmit;
}

/// Valid receive pin for UART peripheral.
#[diagnostic::on_unimplemented(message = "selected pad does not connect to UART{I} RX signal")]
pub trait IntoReceive<const I: usize> {
    type Receive;
    fn into_uart_receive(self) -> Self::Receive;
}

impl<const I: usize, T, R> Pads<I> for (T, R)
where
    T: IntoTransmit<I>,
    R: IntoReceive<I>,
{
    type Clock = ccu::UART<I>;
    type Pads = (
        <T as IntoTransmit<I>>::Transmit,
        <R as IntoReceive<I>>::Receive,
    );

    #[inline]
    fn into_uart_pads(self) -> Self::Pads {
        (self.0.into_uart_transmit(), self.1.into_uart_receive())
    }
}
