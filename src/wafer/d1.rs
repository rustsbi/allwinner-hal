//! SoC configuration on D1-like chips.

use crate::{
    smhc, spi,
    uart::{Receive, Transmit},
};
use core::num::NonZeroU32;

impl_gpio_pins! {
    pb0: ('B', 0, Disabled);
    pb1: ('B', 1, Disabled);
    pb2: ('B', 2, Disabled);
    pb3: ('B', 3, Disabled);
    pb4: ('B', 4, Disabled);
    pb5: ('B', 5, Disabled);
    pb6: ('B', 6, Disabled);
    pb7: ('B', 7, Disabled);
    pb8: ('B', 8, Disabled);
    pb9: ('B', 9, Disabled);
    pb10: ('B', 10, Disabled);
    pb11: ('B', 11, Disabled);
    pb12: ('B', 12, Disabled);
    pc0: ('C', 0, Disabled);
    pc1: ('C', 1, Disabled);
    pc2: ('C', 2, Disabled);
    pc3: ('C', 3, Disabled);
    pc4: ('C', 4, Disabled);
    pc5: ('C', 5, Disabled);
    pc6: ('C', 6, Disabled);
    pc7: ('C', 7, Disabled);
    pd0: ('D', 0, Disabled);
    pd1: ('D', 1, Disabled);
    pd2: ('D', 2, Disabled);
    pd3: ('D', 3, Disabled);
    pd4: ('D', 4, Disabled);
    pd5: ('D', 5, Disabled);
    pd6: ('D', 6, Disabled);
    pd7: ('D', 7, Disabled);
    pd8: ('D', 8, Disabled);
    pd9: ('D', 9, Disabled);
    pd10: ('D', 10, Disabled);
    pd11: ('D', 11, Disabled);
    pd12: ('D', 12, Disabled);
    pd13: ('D', 13, Disabled);
    pd14: ('D', 14, Disabled);
    pd15: ('D', 15, Disabled);
    pd16: ('D', 16, Disabled);
    pd17: ('D', 17, Disabled);
    pd18: ('D', 18, Disabled);
    pd19: ('D', 19, Disabled);
    pd20: ('D', 20, Disabled);
    pd21: ('D', 21, Disabled);
    pd22: ('D', 22, Disabled);
    pe0: ('E', 0, Disabled);
    pe1: ('E', 1, Disabled);
    pe2: ('E', 2, Disabled);
    pe3: ('E', 3, Disabled);
    pe4: ('E', 4, Disabled);
    pe5: ('E', 5, Disabled);
    pe6: ('E', 6, Disabled);
    pe7: ('E', 7, Disabled);
    pe8: ('E', 8, Disabled);
    pe9: ('E', 9, Disabled);
    pe10: ('E', 10, Disabled);
    pe11: ('E', 11, Disabled);
    pe12: ('E', 12, Disabled);
    pe13: ('E', 13, Disabled);
    pe14: ('E', 14, Disabled);
    pe15: ('E', 15, Disabled);
    pe16: ('E', 16, Disabled);
    pe17: ('E', 17, Disabled);
    pf0: ('F', 0, Disabled);
    pf1: ('F', 1, Disabled);
    pf2: ('F', 2, Disabled);
    pf3: ('F', 3, Disabled);
    pf4: ('F', 4, Disabled);
    pf5: ('F', 5, Disabled);
    pf6: ('F', 6, Disabled);
    pg0: ('G', 0, Disabled);
    pg1: ('G', 1, Disabled);
    pg2: ('G', 2, Disabled);
    pg3: ('G', 3, Disabled);
    pg4: ('G', 4, Disabled);
    pg5: ('G', 5, Disabled);
    pg6: ('G', 6, Disabled);
    pg7: ('G', 7, Disabled);
    pg8: ('G', 8, Disabled);
    pg9: ('G', 9, Disabled);
    pg10: ('G', 10, Disabled);
    pg11: ('G', 11, Disabled);
    pg12: ('G', 12, Disabled);
    pg13: ('G', 13, Disabled);
    pg14: ('G', 14, Disabled);
    pg15: ('G', 15, Disabled);
    pg16: ('G', 16, Disabled);
    pg17: ('G', 17, Disabled);
    pg18: ('G', 18, Disabled);
}

// UART PINS
impl_pins_trait! {
    ('B', 0, 6): Transmit<0>;
    ('B', 0, 7): Transmit<2>;
    ('B', 1, 6): Receive<0>;
    ('B', 1, 7): Receive<2>;
    ('B', 2, 7): Transmit<4>;
    ('B', 3, 7): Receive<4>;
    ('B', 4, 7): Transmit<5>;
    ('B', 5, 7): Receive<5>;
    ('B', 6, 7): Transmit<3>;
    ('B', 7, 7): Receive<3>;
    ('B', 8, 6): Transmit<0>;
    ('B', 8, 7): Transmit<1>;
    ('B', 9, 6): Receive<0>;
    ('B', 9, 7): Receive<1>;
    ('C', 0, 2): Transmit<2>;
    ('C', 1, 2): Receive<2>;
    ('C', 6, 4): Transmit<3>;
    ('C', 7, 4): Receive<3>;
    ('D', 1, 5): Transmit<2>;
    ('D', 2, 5): Receive<2>;
    ('D', 5, 5): Transmit<5>;
    ('D', 6, 5): Receive<5>;
    ('D', 7, 5): Transmit<4>;
    ('D', 8, 5): Receive<4>;
    ('D', 10, 5): Transmit<3>;
    ('D', 11, 5): Receive<3>;
    ('D', 21, 4): Transmit<1>;
    ('D', 22, 4): Receive<1>;
    ('E', 2, 3): Transmit<2>;
    ('E', 2, 6): Transmit<0>;
    ('E', 3, 3): Receive<2>;
    ('E', 3, 6): Receive<0>;
    ('E', 4, 3): Transmit<4>;
    ('E', 5, 3): Receive<4>;
    ('E', 6, 3): Transmit<5>;
    ('E', 7, 3): Receive<5>;
    ('E', 8, 5): Transmit<3>;
    ('E', 9, 5): Receive<3>;
    ('E', 10, 3): Transmit<1>;
    ('E', 11, 3): Receive<1>;
    ('G', 0, 3): Transmit<3>;
    ('G', 1, 3): Receive<3>;
    ('G', 2, 5): Transmit<4>;
    ('G', 3, 5): Receive<4>;
    ('G', 4, 3): Transmit<5>;
    ('G', 5, 3): Receive<5>;
    ('G', 6, 2): Transmit<1>;
    ('G', 7, 2): Receive<1>;
    ('G', 8, 5): Transmit<3>;
    ('G', 9, 5): Receive<3>;
    ('G', 17, 2): Transmit<2>;
    ('G', 18, 2): Receive<2>;
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
    // TODO other SDC{0,1,2} pins. Please refer to Section 9.7.3.2 'GPIO Multiplex Function'.
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
