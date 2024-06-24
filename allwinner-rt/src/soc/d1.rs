//! D1-H, D1s, F133, F133A/B chip platforms.

use allwinner_hal::{ccu::Clocks, gpio::Disabled, wafer::d1::Pads};
use embedded_time::rate::Extensions;

/// ROM runtime peripheral ownership and configurations.
pub struct Peripherals<'a> {
    /// General Purpose Input/Output peripheral.
    pub gpio: Pads<'a>,
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub uart0: UART0,
    /// Serial Peripheral Interface peripheral 0.
    pub spi0: SPI0,
    /// Common control peripheral of DDR SDRAM.
    pub com: COM,
    /// Clock control unit peripheral.
    pub ccu: CCU,
    /// Platform-local Interrupt Controller.
    pub plic: PLIC,
}

soc! {
    /// General Purpose Input/Output peripheral.
    pub struct GPIO => 0x02000000, allwinner_hal::gpio::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub struct UART0 => 0x02500000, allwinner_hal::uart::RegisterBlock;
    /// Serial Peripheral Interface peripheral 0.
    pub struct SPI0 => 0x04025000, allwinner_hal::spi::RegisterBlock;
    /// Common control peripheral of DDR SDRAM.
    pub struct COM => 0x03102000, allwinner_hal::com::RegisterBlock;
    /// Clock control unit peripheral.
    pub struct CCU => 0x02001000, allwinner_hal::ccu::RegisterBlock;

    /// Platform-local Interrupt Controller.
    pub struct PLIC => 0x10000000, plic::Plic;
}

#[doc(hidden)]
#[inline]
pub fn __rom_init_params() -> (Peripherals<'static>, Clocks) {
    static _GPIO: GPIO = GPIO { _private: () };
    let peripherals = Peripherals {
        gpio: Pads {
            pb0: unsafe { Disabled::__new(&_GPIO) },
            pb1: unsafe { Disabled::__new(&_GPIO) },
            pb2: unsafe { Disabled::__new(&_GPIO) },
            pb3: unsafe { Disabled::__new(&_GPIO) },
            pb4: unsafe { Disabled::__new(&_GPIO) },
            pb5: unsafe { Disabled::__new(&_GPIO) },
            pb6: unsafe { Disabled::__new(&_GPIO) },
            pb7: unsafe { Disabled::__new(&_GPIO) },
            pb8: unsafe { Disabled::__new(&_GPIO) },
            pb9: unsafe { Disabled::__new(&_GPIO) },
            pb10: unsafe { Disabled::__new(&_GPIO) },
            pb11: unsafe { Disabled::__new(&_GPIO) },
            pb12: unsafe { Disabled::__new(&_GPIO) },
            pc0: unsafe { Disabled::__new(&_GPIO) },
            pc1: unsafe { Disabled::__new(&_GPIO) },
            pc2: unsafe { Disabled::__new(&_GPIO) },
            pc3: unsafe { Disabled::__new(&_GPIO) },
            pc4: unsafe { Disabled::__new(&_GPIO) },
            pc5: unsafe { Disabled::__new(&_GPIO) },
            pc6: unsafe { Disabled::__new(&_GPIO) },
            pc7: unsafe { Disabled::__new(&_GPIO) },
            pd0: unsafe { Disabled::__new(&_GPIO) },
            pd1: unsafe { Disabled::__new(&_GPIO) },
            pd2: unsafe { Disabled::__new(&_GPIO) },
            pd3: unsafe { Disabled::__new(&_GPIO) },
            pd4: unsafe { Disabled::__new(&_GPIO) },
            pd5: unsafe { Disabled::__new(&_GPIO) },
            pd6: unsafe { Disabled::__new(&_GPIO) },
            pd7: unsafe { Disabled::__new(&_GPIO) },
            pd8: unsafe { Disabled::__new(&_GPIO) },
            pd9: unsafe { Disabled::__new(&_GPIO) },
            pd10: unsafe { Disabled::__new(&_GPIO) },
            pd11: unsafe { Disabled::__new(&_GPIO) },
            pd12: unsafe { Disabled::__new(&_GPIO) },
            pd13: unsafe { Disabled::__new(&_GPIO) },
            pd14: unsafe { Disabled::__new(&_GPIO) },
            pd15: unsafe { Disabled::__new(&_GPIO) },
            pd16: unsafe { Disabled::__new(&_GPIO) },
            pd17: unsafe { Disabled::__new(&_GPIO) },
            pd18: unsafe { Disabled::__new(&_GPIO) },
            pd19: unsafe { Disabled::__new(&_GPIO) },
            pd20: unsafe { Disabled::__new(&_GPIO) },
            pd21: unsafe { Disabled::__new(&_GPIO) },
            pd22: unsafe { Disabled::__new(&_GPIO) },
            pe0: unsafe { Disabled::__new(&_GPIO) },
            pe1: unsafe { Disabled::__new(&_GPIO) },
            pe2: unsafe { Disabled::__new(&_GPIO) },
            pe3: unsafe { Disabled::__new(&_GPIO) },
            pe4: unsafe { Disabled::__new(&_GPIO) },
            pe5: unsafe { Disabled::__new(&_GPIO) },
            pe6: unsafe { Disabled::__new(&_GPIO) },
            pe7: unsafe { Disabled::__new(&_GPIO) },
            pe8: unsafe { Disabled::__new(&_GPIO) },
            pe9: unsafe { Disabled::__new(&_GPIO) },
            pe10: unsafe { Disabled::__new(&_GPIO) },
            pe11: unsafe { Disabled::__new(&_GPIO) },
            pe12: unsafe { Disabled::__new(&_GPIO) },
            pe13: unsafe { Disabled::__new(&_GPIO) },
            pe14: unsafe { Disabled::__new(&_GPIO) },
            pe15: unsafe { Disabled::__new(&_GPIO) },
            pe16: unsafe { Disabled::__new(&_GPIO) },
            pe17: unsafe { Disabled::__new(&_GPIO) },
            pf0: unsafe { Disabled::__new(&_GPIO) },
            pf1: unsafe { Disabled::__new(&_GPIO) },
            pf2: unsafe { Disabled::__new(&_GPIO) },
            pf3: unsafe { Disabled::__new(&_GPIO) },
            pf4: unsafe { Disabled::__new(&_GPIO) },
            pf5: unsafe { Disabled::__new(&_GPIO) },
            pf6: unsafe { Disabled::__new(&_GPIO) },
            pg0: unsafe { Disabled::__new(&_GPIO) },
            pg1: unsafe { Disabled::__new(&_GPIO) },
            pg2: unsafe { Disabled::__new(&_GPIO) },
            pg3: unsafe { Disabled::__new(&_GPIO) },
            pg4: unsafe { Disabled::__new(&_GPIO) },
            pg5: unsafe { Disabled::__new(&_GPIO) },
            pg6: unsafe { Disabled::__new(&_GPIO) },
            pg7: unsafe { Disabled::__new(&_GPIO) },
            pg8: unsafe { Disabled::__new(&_GPIO) },
            pg9: unsafe { Disabled::__new(&_GPIO) },
            pg10: unsafe { Disabled::__new(&_GPIO) },
            pg11: unsafe { Disabled::__new(&_GPIO) },
            pg12: unsafe { Disabled::__new(&_GPIO) },
            pg13: unsafe { Disabled::__new(&_GPIO) },
            pg14: unsafe { Disabled::__new(&_GPIO) },
            pg15: unsafe { Disabled::__new(&_GPIO) },
            pg16: unsafe { Disabled::__new(&_GPIO) },
            pg17: unsafe { Disabled::__new(&_GPIO) },
            pg18: unsafe { Disabled::__new(&_GPIO) },
        },
        uart0: UART0 { _private: () },
        spi0: SPI0 { _private: () },
        com: COM { _private: () },
        ccu: CCU { _private: () },
        plic: PLIC { _private: () },
    };
    let clocks = Clocks {
        psi: 600_000_000.Hz(),
        apb1: 24_000_000.Hz(),
    };
    (peripherals, clocks)
}
