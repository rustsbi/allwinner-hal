//! D1-H, D1s, F133, F133A/B chip platforms.

use allwinner_hal::{ccu::Clocks, gpio::Pad, uart::UartExt, wafer::d1::Pads};
use embedded_time::rate::Extensions;

/// ROM runtime peripheral ownership and configurations.
pub struct Peripherals<'a> {
    /// General Purpose Input/Output peripheral.
    pub gpio: Pads<'a>,
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

#[doc(hidden)]
#[inline]
pub fn __rom_init_params() -> (Peripherals<'static>, Clocks) {
    static _GPIO: GPIO = GPIO { _private: () };
    let peripherals = Peripherals {
        gpio: Pads {
            pb0: unsafe { Pad::__new(&_GPIO) },
            pb1: unsafe { Pad::__new(&_GPIO) },
            pb2: unsafe { Pad::__new(&_GPIO) },
            pb3: unsafe { Pad::__new(&_GPIO) },
            pb4: unsafe { Pad::__new(&_GPIO) },
            pb5: unsafe { Pad::__new(&_GPIO) },
            pb6: unsafe { Pad::__new(&_GPIO) },
            pb7: unsafe { Pad::__new(&_GPIO) },
            pb8: unsafe { Pad::__new(&_GPIO) },
            pb9: unsafe { Pad::__new(&_GPIO) },
            pb10: unsafe { Pad::__new(&_GPIO) },
            pb11: unsafe { Pad::__new(&_GPIO) },
            pb12: unsafe { Pad::__new(&_GPIO) },
            pc0: unsafe { Pad::__new(&_GPIO) },
            pc1: unsafe { Pad::__new(&_GPIO) },
            pc2: unsafe { Pad::__new(&_GPIO) },
            pc3: unsafe { Pad::__new(&_GPIO) },
            pc4: unsafe { Pad::__new(&_GPIO) },
            pc5: unsafe { Pad::__new(&_GPIO) },
            pc6: unsafe { Pad::__new(&_GPIO) },
            pc7: unsafe { Pad::__new(&_GPIO) },
            pd0: unsafe { Pad::__new(&_GPIO) },
            pd1: unsafe { Pad::__new(&_GPIO) },
            pd2: unsafe { Pad::__new(&_GPIO) },
            pd3: unsafe { Pad::__new(&_GPIO) },
            pd4: unsafe { Pad::__new(&_GPIO) },
            pd5: unsafe { Pad::__new(&_GPIO) },
            pd6: unsafe { Pad::__new(&_GPIO) },
            pd7: unsafe { Pad::__new(&_GPIO) },
            pd8: unsafe { Pad::__new(&_GPIO) },
            pd9: unsafe { Pad::__new(&_GPIO) },
            pd10: unsafe { Pad::__new(&_GPIO) },
            pd11: unsafe { Pad::__new(&_GPIO) },
            pd12: unsafe { Pad::__new(&_GPIO) },
            pd13: unsafe { Pad::__new(&_GPIO) },
            pd14: unsafe { Pad::__new(&_GPIO) },
            pd15: unsafe { Pad::__new(&_GPIO) },
            pd16: unsafe { Pad::__new(&_GPIO) },
            pd17: unsafe { Pad::__new(&_GPIO) },
            pd18: unsafe { Pad::__new(&_GPIO) },
            pd19: unsafe { Pad::__new(&_GPIO) },
            pd20: unsafe { Pad::__new(&_GPIO) },
            pd21: unsafe { Pad::__new(&_GPIO) },
            pd22: unsafe { Pad::__new(&_GPIO) },
            pe0: unsafe { Pad::__new(&_GPIO) },
            pe1: unsafe { Pad::__new(&_GPIO) },
            pe2: unsafe { Pad::__new(&_GPIO) },
            pe3: unsafe { Pad::__new(&_GPIO) },
            pe4: unsafe { Pad::__new(&_GPIO) },
            pe5: unsafe { Pad::__new(&_GPIO) },
            pe6: unsafe { Pad::__new(&_GPIO) },
            pe7: unsafe { Pad::__new(&_GPIO) },
            pe8: unsafe { Pad::__new(&_GPIO) },
            pe9: unsafe { Pad::__new(&_GPIO) },
            pe10: unsafe { Pad::__new(&_GPIO) },
            pe11: unsafe { Pad::__new(&_GPIO) },
            pe12: unsafe { Pad::__new(&_GPIO) },
            pe13: unsafe { Pad::__new(&_GPIO) },
            pe14: unsafe { Pad::__new(&_GPIO) },
            pe15: unsafe { Pad::__new(&_GPIO) },
            pe16: unsafe { Pad::__new(&_GPIO) },
            pe17: unsafe { Pad::__new(&_GPIO) },
            pf0: unsafe { Pad::__new(&_GPIO) },
            pf1: unsafe { Pad::__new(&_GPIO) },
            pf2: unsafe { Pad::__new(&_GPIO) },
            pf3: unsafe { Pad::__new(&_GPIO) },
            pf4: unsafe { Pad::__new(&_GPIO) },
            pf5: unsafe { Pad::__new(&_GPIO) },
            pf6: unsafe { Pad::__new(&_GPIO) },
            pg0: unsafe { Pad::__new(&_GPIO) },
            pg1: unsafe { Pad::__new(&_GPIO) },
            pg2: unsafe { Pad::__new(&_GPIO) },
            pg3: unsafe { Pad::__new(&_GPIO) },
            pg4: unsafe { Pad::__new(&_GPIO) },
            pg5: unsafe { Pad::__new(&_GPIO) },
            pg6: unsafe { Pad::__new(&_GPIO) },
            pg7: unsafe { Pad::__new(&_GPIO) },
            pg8: unsafe { Pad::__new(&_GPIO) },
            pg9: unsafe { Pad::__new(&_GPIO) },
            pg10: unsafe { Pad::__new(&_GPIO) },
            pg11: unsafe { Pad::__new(&_GPIO) },
            pg12: unsafe { Pad::__new(&_GPIO) },
            pg13: unsafe { Pad::__new(&_GPIO) },
            pg14: unsafe { Pad::__new(&_GPIO) },
            pg15: unsafe { Pad::__new(&_GPIO) },
            pg16: unsafe { Pad::__new(&_GPIO) },
            pg17: unsafe { Pad::__new(&_GPIO) },
            pg18: unsafe { Pad::__new(&_GPIO) },
        },
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
