//! Universal Asynchronous Receiver-Transmitter.

use core::cell::UnsafeCell;

use crate::ccu::{self, ClockGate, Clocks};
use embedded_time::rate::Baud;
use uart16550::{CharLen, PARITY, Register, Uart16550};

/// Universal Asynchronous Receiver-Transmitter registers.
#[repr(C)]
pub struct RegisterBlock {
    uart16550: Uart16550<u32>,
    _reserved0: [u32; 24],
    usr: USR<u32>, // offset = 31(0x7c)
}

/// Serial configuration structure.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    /// Serial baudrate in `Bps`.
    pub baudrate: Baud,
    /// Word length, can be 5, 6, 7 or 8.
    pub wordlength: WordLength,
    /// Parity checks, can be `None`, `Odd` or `Even`.
    pub parity: Parity,
    /// Number of stop bits, can be `One` or `Two`.
    pub stopbits: StopBits,
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        use embedded_time::rate::Extensions;
        Self {
            baudrate: 115200.Bd(),
            wordlength: WordLength::Eight,
            parity: Parity::None,
            stopbits: StopBits::One,
        }
    }
}

/// Serial word length settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WordLength {
    /// 5 bits per word.
    Five,
    /// 6 bits per word.
    Six,
    /// 7 bits per word.
    Seven,
    /// 8 bits per word.
    Eight,
}

/// Serial parity bit settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Parity {
    /// No parity checks.
    None,
    /// Odd parity.
    Odd,
    /// Even parity.
    Even,
}

/// Stop bit settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StopBits {
    /// 1 stop bit
    One,
    /// 2 stop bits, or 1.5 bits when WordLength is Five
    Two,
}

impl core::ops::Deref for RegisterBlock {
    type Target = Uart16550<u32>;

    fn deref(&self) -> &Self::Target {
        &self.uart16550
    }
}

/// Extend constructor to owned UART register blocks.
pub trait UartExt<'a, const I: usize> {
    /// Creates a polling serial instance, without interrupt or DMA configurations.
    fn serial<PADS>(
        self,
        pads: PADS,
        config: impl Into<Config>,
        clocks: &Clocks,
        ccu: &ccu::RegisterBlock,
    ) -> Serial<'a, PADS>
    where
        PADS: Pads<I>;
}

/// Peripheral instance of UART.
pub trait Instance<'a> {
    /// Retrieve register block for this instance.
    fn register_block(self) -> &'a RegisterBlock;
}

/// Managed serial structure with peripheral and pads.
pub struct Serial<'a, PADS> {
    uart: &'a RegisterBlock,
    pads: PADS,
}

impl<'a, PADS> Serial<'a, PADS> {
    /// Create a serial instance.
    #[inline]
    pub fn new<const I: usize>(
        uart: impl Instance<'a>,
        pads: PADS,
        config: impl Into<Config>,
        clocks: &Clocks,
        ccu: &ccu::RegisterBlock,
    ) -> Self
    where
        PADS: Pads<I>,
    {
        // 1. unwrap parameters
        let Config {
            baudrate,
            wordlength,
            parity,
            stopbits,
        } = config.into();
        let bps = baudrate.0;
        // 2. init peripheral clocks
        // note(unsafe): async read and write using ccu registers
        unsafe { PADS::Clock::reset(ccu) };
        // 3. set interrupt configuration
        // on BT0 stage we disable all uart interrupts
        let uart = uart.register_block();
        let interrupt_types = uart.ier().read();
        uart.ier().write(
            interrupt_types
                .disable_ms()
                .disable_rda()
                .disable_rls()
                .disable_thre(),
        );
        // 4. calculate and set baudrate
        let uart_clk = (clocks.apb1.0 + 8 * bps) / (16 * bps);
        uart.write_divisor(uart_clk as u16);
        // 5. additional configurations
        let char_len = match wordlength {
            WordLength::Five => CharLen::FIVE,
            WordLength::Six => CharLen::SIX,
            WordLength::Seven => CharLen::SEVEN,
            WordLength::Eight => CharLen::EIGHT,
        };
        let one_stop_bit = matches!(stopbits, StopBits::One);
        let parity = match parity {
            Parity::None => PARITY::NONE,
            Parity::Odd => PARITY::ODD,
            Parity::Even => PARITY::EVEN,
        };
        let lcr = uart.lcr().read();
        uart.lcr().write(
            lcr.set_char_len(char_len)
                .set_one_stop_bit(one_stop_bit)
                .set_parity(parity),
        );
        // 6. return the instance
        Serial { uart, pads }
    }
    /// Get a temporary borrow on the underlying GPIO pads.
    #[inline]
    pub fn pads<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut PADS) -> T,
    {
        f(&mut self.pads)
    }
    /// Close uart and release peripheral.
    #[inline]
    pub fn free<const I: usize>(self, ccu: &ccu::RegisterBlock) -> PADS
    where
        PADS: Pads<I>,
    {
        // clock is closed for self.clock_gate is dropped
        unsafe { PADS::Clock::free(ccu) };
        self.pads
    }
}

impl<'a, TX, RX> Serial<'a, (TX, RX)> {
    /// Split serial instance into transmit and receive halves.
    #[inline]
    pub fn split<const I: usize>(self) -> (TransmitHalf<'a, TX>, ReceiveHalf<'a, RX>)
    where
        TX: Transmit<I>,
        RX: Receive<I>,
    {
        (
            TransmitHalf {
                uart: self.uart,
                _pads: self.pads.0,
            },
            ReceiveHalf {
                uart: self.uart,
                _pads: self.pads.1,
            },
        )
    }
}

/// Transmit half from splitted serial structure.
pub struct TransmitHalf<'a, PADS> {
    uart: &'a RegisterBlock,
    _pads: PADS,
}

/// Receive half from splitted serial structure.
pub struct ReceiveHalf<'a, PADS> {
    uart: &'a RegisterBlock,
    _pads: PADS,
}

/// Valid serial pads.
pub trait Pads<const I: usize> {
    type Clock: ccu::ClockGate + ccu::ClockReset;
}

/// Valid transmit pin for UART peripheral.
#[diagnostic::on_unimplemented(message = "selected pad does not connect to UART{I} TX signal")]
pub trait Transmit<const I: usize> {}

/// Valid receive pin for UART peripheral.
#[diagnostic::on_unimplemented(message = "selected pad does not connect to UART{I} RX signal")]
pub trait Receive<const I: usize> {}

#[inline]
fn uart_write_blocking(
    uart: &RegisterBlock,
    buffer: &[u8],
) -> Result<usize, core::convert::Infallible> {
    for c in buffer {
        // FIXME: should be transmit_fifo_not_full
        while uart.usr.read().busy() {
            core::hint::spin_loop()
        }
        uart.rbr_thr().tx_data(*c);
    }
    Ok(buffer.len())
}

#[inline]
fn uart_flush_blocking(uart: &RegisterBlock) -> Result<(), core::convert::Infallible> {
    while !uart.usr.read().transmit_fifo_empty() {
        core::hint::spin_loop()
    }
    Ok(())
}

#[inline]
fn uart_read_blocking(
    uart: &RegisterBlock,
    buffer: &mut [u8],
) -> Result<usize, core::convert::Infallible> {
    let len = buffer.len();
    for c in buffer {
        while !uart.uart16550.lsr().read().is_data_ready() {
            core::hint::spin_loop()
        }
        *c = uart.rbr_thr().rx_data();
    }
    Ok(len)
}

impl<const I: usize, T, R> Pads<I> for (T, R)
where
    T: Transmit<I>,
    R: Receive<I>,
{
    type Clock = ccu::UART<I>;
}

impl<'a, PADS> embedded_io::ErrorType for Serial<'a, PADS> {
    type Error = core::convert::Infallible;
}

impl<'a, PADS> embedded_io::ErrorType for TransmitHalf<'a, PADS> {
    type Error = core::convert::Infallible;
}

impl<'a, PADS> embedded_io::ErrorType for ReceiveHalf<'a, PADS> {
    type Error = core::convert::Infallible;
}

impl<'a, PADS> embedded_io::Write for Serial<'a, PADS> {
    #[inline]
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        uart_write_blocking(self.uart, buffer)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        uart_flush_blocking(self.uart)
    }
}

impl<'a, PADS> embedded_io::Write for TransmitHalf<'a, PADS> {
    #[inline]
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        uart_write_blocking(self.uart, buffer)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        uart_flush_blocking(self.uart)
    }
}

impl<'a, PADS> embedded_io::Read for Serial<'a, PADS> {
    #[inline]
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read_blocking(self.uart, buffer)
    }
}

impl<'a, PADS> embedded_io::Read for ReceiveHalf<'a, PADS> {
    #[inline]
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read_blocking(self.uart, buffer)
    }
}

/// UART Status Register.
#[derive(Debug)]
#[repr(transparent)]
pub struct USR<R: Register>(UnsafeCell<R>);

/// Status settings for current peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UartStatus(u8);

impl<R: uart16550::Register> USR<R> {
    /// Write UART status settings.
    #[inline]
    pub fn write(&self, val: UartStatus) {
        unsafe { self.0.get().write_volatile(R::from(val.0)) }
    }

    /// Read UART status settings.
    #[inline]
    pub fn read(&self) -> UartStatus {
        UartStatus(unsafe { self.0.get().read_volatile() }.val())
    }
}

impl UartStatus {
    const RFF: u8 = 1 << 4;
    const RFNE: u8 = 1 << 3;
    const TFE: u8 = 1 << 2;
    const TFNF: u8 = 1 << 1;
    const BUSY: u8 = 1 << 0;

    /// Returns if the receive FIFO is full.
    #[inline]
    pub const fn receive_fifo_full(self) -> bool {
        self.0 & Self::RFF != 0
    }

    /// Returns if the receive FIFO is non-empty.
    #[inline]
    pub const fn receive_fifo_not_empty(self) -> bool {
        self.0 & Self::RFNE != 0
    }

    /// Returns if the transmit FIFO is empty.
    #[inline]
    pub const fn transmit_fifo_empty(self) -> bool {
        self.0 & Self::TFE != 0
    }

    /// Returns if the transmit FIFO is not full.
    #[inline]
    pub const fn transmit_fifo_not_full(self) -> bool {
        self.0 & Self::TFNF != 0
    }

    /// Returns if the peripheral is busy.
    #[inline]
    pub const fn busy(self) -> bool {
        self.0 & Self::BUSY != 0
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::offset_of;
    #[test]
    fn offset_uart() {
        assert_eq!(offset_of!(RegisterBlock, usr), 0x7c);
    }
}
