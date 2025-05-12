//! SoC configuration on D1-like chips.

use crate::{smhc, spi, uart};
use core::num::NonZeroU32;

impl_gpio_pins! {
    pb0: ('B', 0, Pad);
    pb1: ('B', 1, Pad);
    pb2: ('B', 2, Pad);
    pb3: ('B', 3, Pad);
    pb4: ('B', 4, Pad);
    pb5: ('B', 5, Pad);
    pb6: ('B', 6, Pad);
    pb7: ('B', 7, Pad);
    pb8: ('B', 8, Pad);
    pb9: ('B', 9, Pad);
    pb10: ('B', 10, Pad);
    pb11: ('B', 11, Pad);
    pb12: ('B', 12, Pad);
    pc0: ('C', 0, Pad);
    pc1: ('C', 1, Pad);
    pc2: ('C', 2, Pad);
    pc3: ('C', 3, Pad);
    pc4: ('C', 4, Pad);
    pc5: ('C', 5, Pad);
    pc6: ('C', 6, Pad);
    pc7: ('C', 7, Pad);
    pd0: ('D', 0, Pad);
    pd1: ('D', 1, Pad);
    pd2: ('D', 2, Pad);
    pd3: ('D', 3, Pad);
    pd4: ('D', 4, Pad);
    pd5: ('D', 5, Pad);
    pd6: ('D', 6, Pad);
    pd7: ('D', 7, Pad);
    pd8: ('D', 8, Pad);
    pd9: ('D', 9, Pad);
    pd10: ('D', 10, Pad);
    pd11: ('D', 11, Pad);
    pd12: ('D', 12, Pad);
    pd13: ('D', 13, Pad);
    pd14: ('D', 14, Pad);
    pd15: ('D', 15, Pad);
    pd16: ('D', 16, Pad);
    pd17: ('D', 17, Pad);
    pd18: ('D', 18, Pad);
    pd19: ('D', 19, Pad);
    pd20: ('D', 20, Pad);
    pd21: ('D', 21, Pad);
    pd22: ('D', 22, Pad);
    pe0: ('E', 0, Pad);
    pe1: ('E', 1, Pad);
    pe2: ('E', 2, Pad);
    pe3: ('E', 3, Pad);
    pe4: ('E', 4, Pad);
    pe5: ('E', 5, Pad);
    pe6: ('E', 6, Pad);
    pe7: ('E', 7, Pad);
    pe8: ('E', 8, Pad);
    pe9: ('E', 9, Pad);
    pe10: ('E', 10, Pad);
    pe11: ('E', 11, Pad);
    pe12: ('E', 12, Pad);
    pe13: ('E', 13, Pad);
    pe14: ('E', 14, Pad);
    pe15: ('E', 15, Pad);
    pe16: ('E', 16, Pad);
    pe17: ('E', 17, Pad);
    pf0: ('F', 0, Pad);
    pf1: ('F', 1, Pad);
    pf2: ('F', 2, Pad);
    pf3: ('F', 3, Pad);
    pf4: ('F', 4, Pad);
    pf5: ('F', 5, Pad);
    pf6: ('F', 6, Pad);
    pg0: ('G', 0, Pad);
    pg1: ('G', 1, Pad);
    pg2: ('G', 2, Pad);
    pg3: ('G', 3, Pad);
    pg4: ('G', 4, Pad);
    pg5: ('G', 5, Pad);
    pg6: ('G', 6, Pad);
    pg7: ('G', 7, Pad);
    pg8: ('G', 8, Pad);
    pg9: ('G', 9, Pad);
    pg10: ('G', 10, Pad);
    pg11: ('G', 11, Pad);
    pg12: ('G', 12, Pad);
    pg13: ('G', 13, Pad);
    pg14: ('G', 14, Pad);
    pg15: ('G', 15, Pad);
    pg16: ('G', 16, Pad);
    pg17: ('G', 17, Pad);
    pg18: ('G', 18, Pad);
}

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
