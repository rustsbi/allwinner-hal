//! D1-H, D1s, F133, F133A/B chip platforms.

use allwinner_hal::{gpio::PadExt, uart::UartExt};
use core::num::NonZeroU32;
use embedded_time::rate::{Extensions, Hertz};

/// ROM runtime peripheral ownership and configurations.
pub struct Peripherals {
    /// General Purpose Input/Output peripheral.
    pub gpio: Pads,
    /// Clock control unit peripheral.
    pub ccu: CCU,
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub uart0: UART0,
    /// Common control peripheral of DDR SDRAM.
    pub com: COM,
    /// Memory controller physical layer (PHY) of DDR SDRAM.
    pub phy: PHY,
    /// SD/MMC Host Controller peripheral 0.
    pub smhc0: SMHC0,
    /// SD/MMC Host Controller peripheral 1.
    pub smhc1: SMHC1,
    /// SD/MMC Host Controller peripheral 2.
    pub smhc2: SMHC2,
    /// Serial Peripheral Interface peripheral 0.
    pub spi0: SPI0,
    /// Platform-local Interrupt Controller.
    pub plic: PLIC,
}

soc! {
    /// General Purpose Input/Output peripheral.
    pub struct GPIO => 0x02000000, allwinner_hal::gpio::RegisterBlock;
    /// Clock control unit peripheral.
    pub struct CCU => 0x02001000, allwinner_hal::ccu::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub struct UART0 => 0x02500000, allwinner_hal::uart::RegisterBlock;
    /// Common control peripheral of DDR SDRAM.
    pub struct COM => 0x03102000, allwinner_hal::com::RegisterBlock;
    /// Memory controller physical layer (PHY) of DDR SDRAM.
    pub struct PHY => 0x03103000, allwinner_hal::phy::RegisterBlock;
    /// SD/MMC Host Controller peripheral 0.
    pub struct SMHC0 => 0x04020000, allwinner_hal::smhc::RegisterBlock;
    /// SD/MMC Host Controller peripheral 1.
    pub struct SMHC1 => 0x04021000, allwinner_hal::smhc::RegisterBlock;
    /// SD/MMC Host Controller peripheral 2.
    pub struct SMHC2 => 0x04022000, allwinner_hal::smhc::RegisterBlock;
    /// Serial Peripheral Interface peripheral 0.
    pub struct SPI0 => 0x04025000, allwinner_hal::spi::RegisterBlock;
    /// Platform-local Interrupt Controller.
    pub struct PLIC => 0x10000000, plic::Plic;
}

impl_uart! {
    0 => UART0,
}

/// Ownership of a D1 GPIO pad.
pub struct Pad<const P: char, const N: u8> {
    _private: (),
}

impl<const P: char, const N: u8> Pad<P, N> {
    /// Macro internal constructor.
    #[doc(hidden)]
    #[inline]
    pub const fn __new() -> Self {
        Self { _private: () }
    }
}

impl<'a, const P: char, const N: u8> allwinner_hal::gpio::PadExt<'a, P, N> for &'a mut Pad<P, N> {
    #[inline]
    fn into_input(self) -> allwinner_hal::gpio::Input<'a> {
        unsafe { allwinner_hal::gpio::Input::__new(P, N, &GPIO { _private: () }) }
    }
    #[inline]
    fn into_output(self) -> allwinner_hal::gpio::Output<'a> {
        unsafe { allwinner_hal::gpio::Output::__new(P, N, &GPIO { _private: () }) }
    }
    #[inline]
    fn into_function<const F: u8>(self) -> allwinner_hal::gpio::Function<'a, P, N, F> {
        unsafe { allwinner_hal::gpio::Function::__new(&GPIO { _private: () }) }
    }
    #[inline]
    fn into_eint(self) -> allwinner_hal::gpio::EintPad<'a> {
        unsafe { allwinner_hal::gpio::EintPad::__new(P, N, &GPIO { _private: () }) }
    }
}

impl<const P: char, const N: u8> allwinner_hal::gpio::PadExt<'static, P, N> for Pad<P, N> {
    #[inline]
    fn into_input(self) -> allwinner_hal::gpio::Input<'static> {
        unsafe { allwinner_hal::gpio::Input::__new(P, N, &GPIO { _private: () }) }
    }
    #[inline]
    fn into_output(self) -> allwinner_hal::gpio::Output<'static> {
        unsafe { allwinner_hal::gpio::Output::__new(P, N, &GPIO { _private: () }) }
    }
    #[inline]
    fn into_function<const F: u8>(self) -> allwinner_hal::gpio::Function<'static, P, N, F> {
        unsafe { allwinner_hal::gpio::Function::__new(&GPIO { _private: () }) }
    }
    #[inline]
    fn into_eint(self) -> allwinner_hal::gpio::EintPad<'static> {
        unsafe { allwinner_hal::gpio::EintPad::__new(P, N, &GPIO { _private: () }) }
    }
}

impl_gpio_pins! {
    pb0: ('B', 0);
    pb1: ('B', 1);
    pb2: ('B', 2);
    pb3: ('B', 3);
    pb4: ('B', 4);
    pb5: ('B', 5);
    pb6: ('B', 6);
    pb7: ('B', 7);
    pb8: ('B', 8);
    pb9: ('B', 9);
    pb10: ('B', 10);
    pb11: ('B', 11);
    pb12: ('B', 12);
    pc0: ('C', 0);
    pc1: ('C', 1);
    pc2: ('C', 2);
    pc3: ('C', 3);
    pc4: ('C', 4);
    pc5: ('C', 5);
    pc6: ('C', 6);
    pc7: ('C', 7);
    pd0: ('D', 0);
    pd1: ('D', 1);
    pd2: ('D', 2);
    pd3: ('D', 3);
    pd4: ('D', 4);
    pd5: ('D', 5);
    pd6: ('D', 6);
    pd7: ('D', 7);
    pd8: ('D', 8);
    pd9: ('D', 9);
    pd10: ('D', 10);
    pd11: ('D', 11);
    pd12: ('D', 12);
    pd13: ('D', 13);
    pd14: ('D', 14);
    pd15: ('D', 15);
    pd16: ('D', 16);
    pd17: ('D', 17);
    pd18: ('D', 18);
    pd19: ('D', 19);
    pd20: ('D', 20);
    pd21: ('D', 21);
    pd22: ('D', 22);
    pe0: ('E', 0);
    pe1: ('E', 1);
    pe2: ('E', 2);
    pe3: ('E', 3);
    pe4: ('E', 4);
    pe5: ('E', 5);
    pe6: ('E', 6);
    pe7: ('E', 7);
    pe8: ('E', 8);
    pe9: ('E', 9);
    pe10: ('E', 10);
    pe11: ('E', 11);
    pe12: ('E', 12);
    pe13: ('E', 13);
    pe14: ('E', 14);
    pe15: ('E', 15);
    pe16: ('E', 16);
    pe17: ('E', 17);
    pf0: ('F', 0);
    pf1: ('F', 1);
    pf2: ('F', 2);
    pf3: ('F', 3);
    pf4: ('F', 4);
    pf5: ('F', 5);
    pf6: ('F', 6);
    pg0: ('G', 0);
    pg1: ('G', 1);
    pg2: ('G', 2);
    pg3: ('G', 3);
    pg4: ('G', 4);
    pg5: ('G', 5);
    pg6: ('G', 6);
    pg7: ('G', 7);
    pg8: ('G', 8);
    pg9: ('G', 9);
    pg10: ('G', 10);
    pg11: ('G', 11);
    pg12: ('G', 12);
    pg13: ('G', 13);
    pg14: ('G', 14);
    pg15: ('G', 15);
    pg16: ('G', 16);
    pg17: ('G', 17);
    pg18: ('G', 18);
}

impl_uart_pads! {
    ('B', 0, 6):  IntoTransmit, Transmit, into_uart_transmit, 0;
    ('B', 0, 7):  IntoTransmit, Transmit, into_uart_transmit, 2;
    ('B', 1, 6):  IntoReceive, Receive, into_uart_receive, 0;
    ('B', 1, 7):  IntoReceive, Receive, into_uart_receive, 2;
    ('B', 2, 7):  IntoTransmit, Transmit, into_uart_transmit, 4;
    ('B', 3, 7):  IntoReceive, Receive, into_uart_receive, 4;
    ('B', 4, 7):  IntoTransmit, Transmit, into_uart_transmit, 5;
    ('B', 5, 7):  IntoReceive, Receive, into_uart_receive, 5;
    ('B', 6, 7):  IntoTransmit, Transmit, into_uart_transmit, 3;
    ('B', 7, 7):  IntoReceive, Receive, into_uart_receive, 3;
    ('B', 8, 6):  IntoTransmit, Transmit, into_uart_transmit, 0;
    ('B', 8, 7):  IntoTransmit, Transmit, into_uart_transmit, 1;
    ('B', 9, 6):  IntoReceive, Receive, into_uart_receive, 0;
    ('B', 9, 7):  IntoReceive, Receive, into_uart_receive, 1;
    ('C', 0, 2):  IntoTransmit, Transmit, into_uart_transmit, 2;
    ('C', 1, 2):  IntoReceive, Receive, into_uart_receive, 2;
    ('C', 6, 4):  IntoTransmit, Transmit, into_uart_transmit, 3;
    ('C', 7, 4):  IntoReceive, Receive, into_uart_receive, 3;
    ('D', 1, 5):  IntoTransmit, Transmit, into_uart_transmit, 2;
    ('D', 2, 5):  IntoReceive, Receive, into_uart_receive, 2;
    ('D', 5, 5):  IntoTransmit, Transmit, into_uart_transmit, 5;
    ('D', 6, 5):  IntoReceive, Receive, into_uart_receive, 5;
    ('D', 7, 5):  IntoTransmit, Transmit, into_uart_transmit, 4;
    ('D', 8, 5):  IntoReceive, Receive, into_uart_receive, 4;
    ('D', 10, 5): IntoTransmit, Transmit, into_uart_transmit, 3;
    ('D', 11, 5): IntoReceive, Receive, into_uart_receive, 3;
    ('D', 21, 4): IntoTransmit, Transmit, into_uart_transmit, 1;
    ('D', 22, 4): IntoReceive, Receive, into_uart_receive, 1;
    ('E', 2, 3):  IntoTransmit, Transmit, into_uart_transmit, 2;
    ('E', 2, 6):  IntoTransmit, Transmit, into_uart_transmit, 0;
    ('E', 3, 3):  IntoReceive, Receive, into_uart_receive, 2;
    ('E', 3, 6):  IntoReceive, Receive, into_uart_receive, 0;
    ('E', 4, 3):  IntoTransmit, Transmit, into_uart_transmit, 4;
    ('E', 5, 3):  IntoReceive, Receive, into_uart_receive, 4;
    ('E', 6, 3):  IntoTransmit, Transmit, into_uart_transmit, 5;
    ('E', 7, 3):  IntoReceive, Receive, into_uart_receive, 5;
    ('E', 8, 5):  IntoTransmit, Transmit, into_uart_transmit, 3;
    ('E', 9, 5):  IntoReceive, Receive, into_uart_receive, 3;
    ('E', 10, 3): IntoTransmit, Transmit, into_uart_transmit, 1;
    ('E', 11, 3): IntoReceive, Receive, into_uart_receive, 1;
    ('G', 0, 3):  IntoTransmit, Transmit, into_uart_transmit, 3;
    ('G', 1, 3):  IntoReceive, Receive, into_uart_receive, 3;
    ('G', 2, 5):  IntoTransmit, Transmit, into_uart_transmit, 4;
    ('G', 3, 5):  IntoReceive, Receive, into_uart_receive, 4;
    ('G', 4, 3):  IntoTransmit, Transmit, into_uart_transmit, 5;
    ('G', 5, 3):  IntoReceive, Receive, into_uart_receive, 5;
    ('G', 6, 2):  IntoTransmit, Transmit, into_uart_transmit, 1;
    ('G', 7, 2):  IntoReceive, Receive, into_uart_receive, 1;
    ('G', 8, 5):  IntoTransmit, Transmit, into_uart_transmit, 3;
    ('G', 9, 5):  IntoReceive, Receive, into_uart_receive, 3;
    ('G', 17, 2): IntoTransmit, Transmit, into_uart_transmit, 2;
    ('G', 18, 2): IntoReceive, Receive, into_uart_receive, 2;
}

/// ROM clock configuration on current SoC.
#[derive(Debug)]
pub struct Clocks {
    /// PSI clock frequency.
    pub psi: Hertz,
    /// Advanced Peripheral Bus 1 clock frequency.
    pub apb1: Hertz,
}

impl allwinner_hal::uart::Clock for Clocks {
    #[inline]
    fn uart_clock(&self) -> embedded_time::rate::Hertz {
        self.apb1
    }
}

impl allwinner_hal::spi::Clock for Clocks {
    #[inline]
    fn spi_clock(&self) -> embedded_time::rate::Hertz {
        // TODO calculate from more clock parameters
        self.psi
    }
}

impl allwinner_hal::smhc::Clock for Clocks {
    #[inline]
    fn smhc_clock(&self) -> embedded_time::rate::Hertz {
        self.psi
    }
}

#[doc(hidden)]
#[inline]
pub fn __rom_init_params() -> (Peripherals, Clocks) {
    let peripherals = Peripherals {
        gpio: Pads::__new(),
        ccu: CCU { _private: () },
        uart0: UART0 { _private: () },
        com: COM { _private: () },
        phy: PHY { _private: () },
        smhc0: SMHC0 { _private: () },
        smhc1: SMHC1 { _private: () },
        smhc2: SMHC2 { _private: () },
        spi0: SPI0 { _private: () },
        plic: PLIC { _private: () },
    };
    let clocks = Clocks {
        psi: 600_000_000.Hz(),
        apb1: 24_000_000.Hz(),
    };
    (peripherals, clocks)
}

/// Allwinner D1 C906 hart interrupts.
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

/// Machine mode hart context for Allwinner D1 T-Head C906 core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Machine;

impl plic::HartContext for Machine {
    fn index(self) -> usize {
        0
    }
}

/// Supervisor mode hart context for Allwinner D1 T-Head C906 core.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Supevisor;

impl plic::HartContext for Supevisor {
    fn index(self) -> usize {
        1
    }
}
