use core::cell::UnsafeCell;
use uart16550::{Register, Uart16550};

/// Universal Asynchronous Receiver-Transmitter registers.
#[repr(C)]
pub struct RegisterBlock {
    uart16550: Uart16550<u32>,
    _reserved0: [u32; 24],
    pub usr: USR<u32>, // offset = 31(0x7c)
}

/// UART Status Register.
#[derive(Debug)]
#[repr(transparent)]
pub struct USR<R: Register>(UnsafeCell<R>);

/// Status settings for current peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UartStatus(u8);

impl<R: uart16550::Register> USR<R> {
    /// Write UART status settings.
    #[inline]
    pub fn write(&self, val: UartStatus) {
        unsafe { self.0.get().write_volatile(R::from(val.0)) }
    }

    /// Read UART status settings.
    #[inline]
    pub fn read(&self) -> UartStatus {
        UartStatus(unsafe { self.0.get().read_volatile() }.val())
    }
}

impl UartStatus {
    const RFF: u8 = 1 << 4;
    const RFNE: u8 = 1 << 3;
    const TFE: u8 = 1 << 2;
    const TFNF: u8 = 1 << 1;
    const BUSY: u8 = 1 << 0;

    /// Returns if the receive FIFO is full.
    #[inline]
    pub const fn receive_fifo_full(self) -> bool {
        self.0 & Self::RFF != 0
    }

    /// Returns if the receive FIFO is non-empty.
    #[inline]
    pub const fn receive_fifo_not_empty(self) -> bool {
        self.0 & Self::RFNE != 0
    }

    /// Returns if the transmit FIFO is empty.
    #[inline]
    pub const fn transmit_fifo_empty(self) -> bool {
        self.0 & Self::TFE != 0
    }

    /// Returns if the transmit FIFO is not full.
    #[inline]
    pub const fn transmit_fifo_not_full(self) -> bool {
        self.0 & Self::TFNF != 0
    }

    /// Returns if the peripheral is busy.
    #[inline]
    pub const fn busy(self) -> bool {
        self.0 & Self::BUSY != 0
    }
}

impl core::ops::Deref for RegisterBlock {
    type Target = Uart16550<u32>;

    fn deref(&self) -> &Self::Target {
        &self.uart16550
    }
}

#[cfg(test)]
mod tests {
    use super::{RegisterBlock, UartStatus};
    use core::mem::offset_of;
    #[test]
    fn offset_uart() {
        assert_eq!(offset_of!(RegisterBlock, usr), 0x7c);
    }

    #[test]
    fn test_uart_status() {
        // Scenario 1: Test when all status bits are set to 1
        let status_all_set = UartStatus(0b11111); // 0x1F

        assert!(status_all_set.receive_fifo_full());
        assert!(status_all_set.receive_fifo_not_empty());
        assert!(status_all_set.transmit_fifo_empty());
        assert!(status_all_set.transmit_fifo_not_full());
        assert!(status_all_set.busy());

        // Scenario 2: Test when all status bits are set to 0
        let status_all_clear = UartStatus(0b00000); // 0x0

        assert!(!status_all_clear.receive_fifo_full());
        assert!(!status_all_clear.receive_fifo_not_empty());
        assert!(!status_all_clear.transmit_fifo_empty());
        assert!(!status_all_clear.transmit_fifo_not_full());
        assert!(!status_all_clear.busy());
    }
}
