//! D1-H, D1s, F133, F133A/B chip platforms.

use allwinner_hal::{ccu::Clocks, uart::UartExt};
use embedded_time::rate::Extensions;

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
    pb0: (Pad, 'B', 0);
    pb1: (Pad, 'B', 1);
    pb2: (Pad, 'B', 2);
    pb3: (Pad, 'B', 3);
    pb4: (Pad, 'B', 4);
    pb5: (Pad, 'B', 5);
    pb6: (Pad, 'B', 6);
    pb7: (Pad, 'B', 7);
    pb8: (Pad, 'B', 8);
    pb9: (Pad, 'B', 9);
    pb10: (Pad, 'B', 10);
    pb11: (Pad, 'B', 11);
    pb12: (Pad, 'B', 12);
    pc0: (Pad, 'C', 0);
    pc1: (Pad, 'C', 1);
    pc2: (Pad, 'C', 2);
    pc3: (Pad, 'C', 3);
    pc4: (Pad, 'C', 4);
    pc5: (Pad, 'C', 5);
    pc6: (Pad, 'C', 6);
    pc7: (Pad, 'C', 7);
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
    pe0: (Pad, 'E', 0);
    pe1: (Pad, 'E', 1);
    pe2: (Pad, 'E', 2);
    pe3: (Pad, 'E', 3);
    pe4: (Pad, 'E', 4);
    pe5: (Pad, 'E', 5);
    pe6: (Pad, 'E', 6);
    pe7: (Pad, 'E', 7);
    pe8: (Pad, 'E', 8);
    pe9: (Pad, 'E', 9);
    pe10: (Pad, 'E', 10);
    pe11: (Pad, 'E', 11);
    pe12: (Pad, 'E', 12);
    pe13: (Pad, 'E', 13);
    pe14: (Pad, 'E', 14);
    pe15: (Pad, 'E', 15);
    pe16: (Pad, 'E', 16);
    pe17: (Pad, 'E', 17);
    pf0: (Pad, 'F', 0);
    pf1: (Pad, 'F', 1);
    pf2: (Pad, 'F', 2);
    pf3: (Pad, 'F', 3);
    pf4: (Pad, 'F', 4);
    pf5: (Pad, 'F', 5);
    pf6: (Pad, 'F', 6);
    pg0: (Pad, 'G', 0);
    pg1: (Pad, 'G', 1);
    pg2: (Pad, 'G', 2);
    pg3: (Pad, 'G', 3);
    pg4: (Pad, 'G', 4);
    pg5: (Pad, 'G', 5);
    pg6: (Pad, 'G', 6);
    pg7: (Pad, 'G', 7);
    pg8: (Pad, 'G', 8);
    pg9: (Pad, 'G', 9);
    pg10: (Pad, 'G', 10);
    pg11: (Pad, 'G', 11);
    pg12: (Pad, 'G', 12);
    pg13: (Pad, 'G', 13);
    pg14: (Pad, 'G', 14);
    pg15: (Pad, 'G', 15);
    pg16: (Pad, 'G', 16);
    pg17: (Pad, 'G', 17);
    pg18: (Pad, 'G', 18);
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
