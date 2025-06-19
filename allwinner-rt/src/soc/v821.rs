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
    /// General Purpose Input/Output peripheral for PA, PC and PD pads.
    pub struct SYS_GPIO => 0x42000000, allwinner_hal::gpio::RegisterBlock;
    /// General Purpose Input/Output peripheral for PL pads.
    pub struct RTC_GPIO => 0x42000540, allwinner_hal::gpio::RegisterBlock;
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

/// Ownership of a V821 system domain GPIO pad.
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

/// Ownership of a V821 RTC domain GPIO pad.
pub struct RtcPad<const P: char, const N: u8> {
    _private: (),
}

impl<const P: char, const N: u8> RtcPad<P, N> {
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
    pa0: (Pad, 'A', 0);
    pa1: (Pad, 'A', 1);
    pa2: (Pad, 'A', 2);
    pa3: (Pad, 'A', 3);
    pa4: (Pad, 'A', 4);
    pa5: (Pad, 'A', 5);
    pa6: (Pad, 'A', 6);
    pa7: (Pad, 'A', 7);
    pa8: (Pad, 'A', 8);
    pa9: (Pad, 'A', 9);
    pa10: (Pad, 'A', 10);
    pa11: (Pad, 'A', 11);
    pa12: (Pad, 'A', 12);
    pc0: (Pad, 'C', 0);
    pc1: (Pad, 'C', 1);
    pc2: (Pad, 'C', 2);
    pc3: (Pad, 'C', 3);
    pc4: (Pad, 'C', 4);
    pc5: (Pad, 'C', 5);
    pc6: (Pad, 'C', 6);
    pc7: (Pad, 'C', 7);
    pc8: (Pad, 'C', 8);
    pc9: (Pad, 'C', 9);
    pc10: (Pad, 'C', 10);
    pc11: (Pad, 'C', 11);
    pc12: (Pad, 'C', 12);
    pc13: (Pad, 'C', 13);
    pc14: (Pad, 'C', 14);
    pc15: (Pad, 'C', 15);
    pc16: (Pad, 'C', 16);
    pd0: (Pad, 'D', 0);
    pd1: (Pad, 'D', 1);
    pd2: (Pad, 'D', 2);
    pd3: (Pad, 'D', 3);
    pd4: (Pad, 'D', 4);
    pd5: (Pad, 'D', 5);
    pd6: (Pad, 'D', 6);
    pd7: (Pad, 'D', 7);
    pd8: (Pad, 'D', 8);
    pd9: (Pad, 'D', 9);
    pd10: (Pad, 'D', 10);
    pd11: (Pad, 'D', 11);
    pd12: (Pad, 'D', 12);
    pd13: (Pad, 'D', 13);
    pd14: (Pad, 'D', 14);
    pd15: (Pad, 'D', 15);
    pd16: (Pad, 'D', 16);
    pd17: (Pad, 'D', 17);
    pd18: (Pad, 'D', 18);
    pd19: (Pad, 'D', 19);
    pd20: (Pad, 'D', 20);
    pd21: (Pad, 'D', 21);
    pd22: (Pad, 'D', 22);
    pd23: (Pad, 'D', 23);
    pl0: (RtcPad, 'L', 0);
    pl1: (RtcPad, 'L', 1);
    pl2: (RtcPad, 'L', 2);
    pl3: (RtcPad, 'L', 3);
    pl4: (RtcPad, 'L', 4);
    pl5: (RtcPad, 'L', 5);
    pl6: (RtcPad, 'L', 6);
    pl7: (RtcPad, 'L', 7);
}
