use allwinner_hal::Pins;

/// ROM runtime peripheral ownership and configurations.
pub struct Peripherals {
    /// General Purpose Input/Output peripheral.
    pub gpio: Pins<GPIO>,
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
