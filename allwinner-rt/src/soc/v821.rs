//! V821 chip platforms.

use allwinner_hal::ccu::Clocks;
use embedded_time::rate::Extensions;

/// ROM runtime peripheral ownership and configurations.
pub struct Peripherals {
    /// General Purpose Input/Output peripheral.
    pub gpio: Pads,
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub uart0: UART0,
    /// Universal Asynchronous Receiver/Transmitter 1.
    pub uart1: UART1,
    /// Universal Asynchronous Receiver/Transmitter 2.
    pub uart2: UART2,
    /// Universal Asynchronous Receiver/Transmitter 3.
    pub uart3: UART3,
}

soc! {
    /// General Purpose Input/Output peripheral for PA, PC and PD.
    pub struct GPIO => 0x42000000, allwinner_hal::gpio::RegisterBlock;
    /// General Purpose Input/Output peripheral for PL.
    pub struct GPIO_R => 0x42000540, allwinner_hal::gpio::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub struct UART0 => 0x42500000, allwinner_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 1.
    pub struct UART1 => 0x42500400, allwinner_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 2.
    pub struct UART2 => 0x42500800, allwinner_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 3.
    pub struct UART3 => 0x42500C00, allwinner_hal::uart::RegisterBlock;
}

// TODO GPIO_R logic in allwinner-hal

/// Ownership of a V821 GPIO pad.
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

#[doc(hidden)]
#[inline]
pub fn __rom_init_params() -> (Peripherals, Clocks) {
    let peripherals = Peripherals {
        gpio: Pads::__new(),
        uart0: UART0 { _private: () },
        uart1: UART1 { _private: () },
        uart2: UART2 { _private: () },
        uart3: UART3 { _private: () },
    };
    // TODO: correct clock configuration
    let clocks = Clocks {
        psi: 600_000_000.Hz(),
        apb1: 24_000_000.Hz(),
    };
    (peripherals, clocks)
}

impl_gpio_pins! {
    pa0: ('A', 0);
    pa1: ('A', 1);
    pa2: ('A', 2);
    pa3: ('A', 3);
    pa4: ('A', 4);
    pa5: ('A', 5);
    pa6: ('A', 6);
    pa7: ('A', 7);
    pa8: ('A', 8);
    pa9: ('A', 9);
    pa10: ('A', 10);
    pa11: ('A', 11);
    pa12: ('A', 12);
    pc0: ('C', 0);
    pc1: ('C', 1);
    pc2: ('C', 2);
    pc3: ('C', 3);
    pc4: ('C', 4);
    pc5: ('C', 5);
    pc6: ('C', 6);
    pc7: ('C', 7);
    pc8: ('C', 8);
    pc9: ('C', 9);
    pc10: ('C', 10);
    pc11: ('C', 11);
    pc12: ('C', 12);
    pc13: ('C', 13);
    pc14: ('C', 14);
    pc15: ('C', 15);
    pc16: ('C', 16);
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
    pd23: ('D', 23);
}
