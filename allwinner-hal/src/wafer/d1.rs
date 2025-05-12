//! SoC configuration on D1-like chips.

use crate::{smhc, spi, uart};
use core::num::NonZeroU32;

// UART PINS
impl_pins_trait! {
    ('B', 0, 6): uart::Transmit<0>;
    ('B', 0, 7): uart::Transmit<2>;
    ('B', 1, 6): uart::Receive<0>;
    ('B', 1, 7): uart::Receive<2>;
    ('B', 2, 7): uart::Transmit<4>;
    ('B', 3, 7): uart::Receive<4>;
    ('B', 4, 7): uart::Transmit<5>;
    ('B', 5, 7): uart::Receive<5>;
    ('B', 6, 7): uart::Transmit<3>;
    ('B', 7, 7): uart::Receive<3>;
    ('B', 8, 6): uart::Transmit<0>;
    ('B', 8, 7): uart::Transmit<1>;
    ('B', 9, 6): uart::Receive<0>;
    ('B', 9, 7): uart::Receive<1>;
    ('C', 0, 2): uart::Transmit<2>;
    ('C', 1, 2): uart::Receive<2>;
    ('C', 6, 4): uart::Transmit<3>;
    ('C', 7, 4): uart::Receive<3>;
    ('D', 1, 5): uart::Transmit<2>;
    ('D', 2, 5): uart::Receive<2>;
    ('D', 5, 5): uart::Transmit<5>;
    ('D', 6, 5): uart::Receive<5>;
    ('D', 7, 5): uart::Transmit<4>;
    ('D', 8, 5): uart::Receive<4>;
    ('D', 10, 5): uart::Transmit<3>;
    ('D', 11, 5): uart::Receive<3>;
    ('D', 21, 4): uart::Transmit<1>;
    ('D', 22, 4): uart::Receive<1>;
    ('E', 2, 3): uart::Transmit<2>;
    ('E', 2, 6): uart::Transmit<0>;
    ('E', 3, 3): uart::Receive<2>;
    ('E', 3, 6): uart::Receive<0>;
    ('E', 4, 3): uart::Transmit<4>;
    ('E', 5, 3): uart::Receive<4>;
    ('E', 6, 3): uart::Transmit<5>;
    ('E', 7, 3): uart::Receive<5>;
    ('E', 8, 5): uart::Transmit<3>;
    ('E', 9, 5): uart::Receive<3>;
    ('E', 10, 3): uart::Transmit<1>;
    ('E', 11, 3): uart::Receive<1>;
    ('G', 0, 3): uart::Transmit<3>;
    ('G', 1, 3): uart::Receive<3>;
    ('G', 2, 5): uart::Transmit<4>;
    ('G', 3, 5): uart::Receive<4>;
    ('G', 4, 3): uart::Transmit<5>;
    ('G', 5, 3): uart::Receive<5>;
    ('G', 6, 2): uart::Transmit<1>;
    ('G', 7, 2): uart::Receive<1>;
    ('G', 8, 5): uart::Transmit<3>;
    ('G', 9, 5): uart::Receive<3>;
    ('G', 17, 2): uart::Transmit<2>;
    ('G', 18, 2): uart::Receive<2>;
}

// SPI PINS
impl_pins_trait! {
    ('B', 9, 5): spi::Miso<1>;
    ('B', 10, 5): spi::Mosi<1>;
    ('B', 11, 5): spi::Clk<1>;
    ('C', 2, 2): spi::Clk<0>;
    ('C', 4, 2): spi::Mosi<0>;
    ('C', 5, 2): spi::Miso<0>;
    ('D', 11, 4): spi::Clk<1>;
    ('D', 12, 4): spi::Mosi<1>;
    ('D', 13, 4): spi::Miso<1>;
}

// SMHC pins
impl_pins_trait! {
    ('F', 0, 2): smhc::Data<1>;
    ('F', 1, 2): smhc::Data<0>;
    ('F', 2, 2): smhc::Clk;
    ('F', 3, 2): smhc::Cmd;
    ('F', 4, 2): smhc::Data<3>;
    ('F', 5, 2): smhc::Data<2>;
    ('G', 0, 2): smhc::Clk;
    ('G', 1, 2): smhc::Cmd;
    ('G', 2, 2): smhc::Data<0>;
    ('G', 3, 2): smhc::Data<1>;
    ('G', 4, 2): smhc::Data<2>;
    ('G', 5, 2): smhc::Data<3>;
    ('C', 2, 3): smhc::Clk;
    ('C', 3, 3): smhc::Cmd;
    ('C', 4, 3): smhc::Data<2>;
    ('C', 5, 3): smhc::Data<1>;
    ('C', 6, 3): smhc::Data<0>;
    ('C', 7, 3): smhc::Data<3>;
}

/// Allwinner D1 interrupts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Interrupt {
    /// Universal Asynchronous Receiver-Transmitter 0.
    UART0 = 18,
    /// Universal Asynchronous Receiver-Transmitter 1.
    UART1 = 19,
    /// Universal Asynchronous Receiver-Transmitter 2.
    UART2 = 20,
    /// Universal Asynchronous Receiver-Transmitter 3.
    UART3 = 21,
    /// Universal Asynchronous Receiver-Transmitter 4.
    UART4 = 22,
    /// Universal Asynchronous Receiver-Transmitter 5.
    UART5 = 23,
    /// Serial Peripheral Interface 0.
    SPI0 = 31,
    /// Serial Peripheral Interface 1.
    SPI1 = 32,
}

impl plic::InterruptSource for Interrupt {
    fn id(self) -> NonZeroU32 {
        // note(unwarp): self as u32 representation has no zero value.
        NonZeroU32::new(self as u32).unwrap()
    }
}

/// Machine mode hart context for T-Head C906 core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Machine;

impl plic::HartContext for Machine {
    fn index(self) -> usize {
        0
    }
}

/// Supervisor mode hart context for T-Head C906 core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Supevisor;

impl plic::HartContext for Supevisor {
    fn index(self) -> usize {
        1
    }
}
