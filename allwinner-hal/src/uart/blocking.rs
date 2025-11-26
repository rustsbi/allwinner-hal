use super::{
    Instance, Pads,
    config::{Config, Parity, StopBits, WordLength},
    register::RegisterBlock,
};
use crate::{gpio::FlexPad, uart::Clock};
use uart16550::{CharLen, PARITY};

/// Managed serial structure with peripheral and pads.
pub struct Serial<'a> {
    uart: &'a RegisterBlock,
    pads: (Option<FlexPad<'a>>, Option<FlexPad<'a>>),
}

impl<'a> Serial<'a> {
    /// Create a serial instance.
    #[inline]
    pub fn new<const I: usize>(
        uart: impl Instance<'a>,
        pads: impl Pads<'a, I>,
        config: impl Into<Config>,
        clock: impl Clock<I>,
    ) -> Serial<'a> {
        // 1. unwrap parameters
        let Config {
            baudrate,
            wordlength,
            parity,
            stopbits,
        } = config.into();
        let bps = baudrate.0;
        // 2. set interrupt configuration
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
        // 3. calculate and set baudrate
        let uart_clk = (clock.uart_clock().0 + 8 * bps) / (16 * bps);
        uart.write_divisor(uart_clk as u16);
        // 4. additional configurations
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
        // 5. return the instance
        let pads = pads.into_uart_pads();
        Serial { uart, pads }
    }
}

impl<'a> Serial<'a> {
    /// Split serial instance into transmit and receive halves.
    #[inline]
    pub fn split(self) -> (TransmitHalf<'a>, ReceiveHalf<'a>) {
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
pub struct TransmitHalf<'a> {
    uart: &'a RegisterBlock,
    _pads: Option<FlexPad<'a>>,
}

/// Receive half from splitted serial structure.
pub struct ReceiveHalf<'a> {
    uart: &'a RegisterBlock,
    _pads: Option<FlexPad<'a>>,
}

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
        while !uart.lsr().read().is_data_ready() {
            core::hint::spin_loop()
        }
        *c = uart.rbr_thr().rx_data();
    }
    Ok(len)
}

impl<'a> embedded_io::ErrorType for Serial<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_io::ErrorType for TransmitHalf<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_io::ErrorType for ReceiveHalf<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_io::Write for Serial<'a> {
    #[inline]
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        uart_write_blocking(self.uart, buffer)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        uart_flush_blocking(self.uart)
    }
}

impl<'a> embedded_io::Write for TransmitHalf<'a> {
    #[inline]
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        uart_write_blocking(self.uart, buffer)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        uart_flush_blocking(self.uart)
    }
}

impl<'a> embedded_io::Read for Serial<'a> {
    #[inline]
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read_blocking(self.uart, buffer)
    }
}

impl<'a> embedded_io::Read for ReceiveHalf<'a> {
    #[inline]
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read_blocking(self.uart, buffer)
    }
}
