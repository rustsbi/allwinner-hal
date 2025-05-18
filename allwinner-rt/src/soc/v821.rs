//! V821 chip platforms.

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
