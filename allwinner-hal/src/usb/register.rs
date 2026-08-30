//! V821 (sun300iw1p1) USB device-controller and PHY registers.

use core::cell::UnsafeCell;

/// Volatile read-only register.
#[repr(transparent)]
pub struct ReadOnly<T: Copy>(UnsafeCell<T>);

impl<T: Copy> ReadOnly<T> {
    /// Read the register value.
    #[inline(always)]
    pub fn read(&self) -> T {
        // SAFETY: instances are fields of a source-verified MMIO register
        // block and `UnsafeCell` supplies the required interior mutability.
        unsafe { self.0.get().read_volatile() }
    }
}

/// Volatile read-write register.
#[repr(transparent)]
pub struct ReadWrite<T: Copy>(UnsafeCell<T>);

impl<T: Copy> ReadWrite<T> {
    /// Read the register value.
    #[inline(always)]
    pub fn read(&self) -> T {
        // SAFETY: see [`ReadOnly::read`].
        unsafe { self.0.get().read_volatile() }
    }

    /// Write the register value.
    #[inline(always)]
    pub fn write(&self, value: T) {
        // SAFETY: see [`ReadOnly::read`]. Callers must own the USB controller
        // and follow each register's documented command semantics.
        unsafe { self.0.get().write_volatile(value) }
    }
}

/// Volatile interrupt status register whose asserted bits are cleared by writing one.
#[repr(transparent)]
pub struct WriteOneToClear<T: Copy>(UnsafeCell<T>);

impl<T: Copy> WriteOneToClear<T> {
    /// Read pending interrupt status without clearing it.
    #[inline(always)]
    pub fn status(&self) -> T {
        // SAFETY: USB interrupt status reads are non-destructive.
        unsafe { self.0.get().read_volatile() }
    }

    /// Acknowledge exactly the interrupt bits selected by `mask`.
    #[inline(always)]
    pub fn acknowledge(&self, mask: T) {
        // SAFETY: this performs one exact volatile write and deliberately does
        // not read-modify-write a W1C register.
        unsafe { self.0.get().write_volatile(mask) }
    }
}

/// USB PHY interface status and control register (ISCR).
///
/// Change-detect status bits 6 through 4 are W1C, so ordinary control-field
/// updates must write zero to those bits instead of copying their readback.
#[repr(transparent)]
pub struct InterfaceStatusControl(UnsafeCell<u32>);

impl InterfaceStatusControl {
    const CHANGE_DETECT: u32 = 0x70;

    /// Read the complete status and control value.
    #[inline(always)]
    pub fn read(&self) -> u32 {
        // SAFETY: see [`ReadOnly::read`].
        unsafe { self.0.get().read_volatile() }
    }

    /// Modify control fields without acknowledging change-detect status.
    #[inline(always)]
    pub fn modify(&self, f: impl FnOnce(u32) -> u32) {
        let control = f(self.read() & !Self::CHANGE_DETECT) & !Self::CHANGE_DETECT;
        // SAFETY: this performs one volatile control write while forcing every
        // W1C change-detect bit to zero.
        unsafe { self.0.get().write_volatile(control) }
    }
}

/// Endpoint FIFO data port.
#[repr(transparent)]
pub struct Fifo(UnsafeCell<u32>);

impl Fifo {
    /// Read one byte from the endpoint FIFO.
    #[inline(always)]
    pub fn read_byte(&self) -> u8 {
        // SAFETY: the Allwinner USB FIFO supports byte-wide PIO accesses.
        unsafe { self.0.get().cast::<u8>().read_volatile() }
    }

    /// Write one byte to the endpoint FIFO.
    #[inline(always)]
    pub fn write_byte(&self, value: u8) {
        // SAFETY: the Allwinner USB FIFO supports byte-wide PIO accesses.
        unsafe { self.0.get().cast::<u8>().write_volatile(value) }
    }
}

/// USB PHY registers used by the V821 BootROM initialization sequence.
#[repr(C)]
pub struct PhyRegisterBlock {
    /// 0x000 - interface status and control register (ISCR).
    pub interface_status_control: InterfaceStatusControl,
    _reserved_004: [u8; 0x0c],
    /// 0x010 - 28 nm PHY and VC-bus serial control register.
    pub phy_control_28nm: ReadWrite<u32>,
    _reserved_014: [u8; 0x0c],
    /// 0x020 - PHY controller path-selection register (PHYSEL).
    pub phy_select: ReadWrite<u32>,
}

/// Partial V821 USB device-controller map used by the polled device driver.
///
/// Endpoint configuration registers from `tx_max_packet` through
/// `rx_fifo_address` are indexed by [`Self::index`].
#[repr(C)]
pub struct RegisterBlock {
    /// 0x000..0x00c - endpoint FIFO data ports 0 through 3.
    pub fifo: [Fifo; 4],
    _reserved_010: [u8; 0x30],
    /// 0x040 - power and connection control register.
    pub power: ReadWrite<u8>,
    /// 0x041 - device control and bus state register.
    pub device_control: ReadWrite<u8>,
    /// 0x042 - indexed endpoint selector.
    pub index: ReadWrite<u8>,
    /// 0x043 - vendor bus-mode control register.
    pub vendor_control: ReadWrite<u8>,
    /// 0x044 - transmit endpoint interrupt status.
    pub interrupt_tx: WriteOneToClear<u16>,
    /// 0x046 - receive endpoint interrupt status.
    pub interrupt_rx: WriteOneToClear<u16>,
    /// 0x048 - transmit endpoint interrupt enable.
    pub interrupt_tx_enable: ReadWrite<u16>,
    /// 0x04a - receive endpoint interrupt enable.
    pub interrupt_rx_enable: ReadWrite<u16>,
    /// 0x04c - USB bus interrupt status.
    pub interrupt_usb: WriteOneToClear<u8>,
    _reserved_04d: [u8; 3],
    /// 0x050 - USB bus interrupt enable.
    pub interrupt_usb_enable: ReadWrite<u8>,
    _reserved_051: [u8; 3],
    _reserved_054: [u8; 0x2c],
    /// 0x080 - indexed transmit maximum packet size.
    pub tx_max_packet: ReadWrite<u16>,
    /// 0x082 - endpoint-zero CSR or indexed transmit CSR.
    ///
    /// This mixes status, persistent configuration, and self-clearing command
    /// bits; callers must use command-specific writes rather than generic RMW.
    pub tx_csr: ReadWrite<u16>,
    /// 0x084 - indexed receive maximum packet size.
    pub rx_max_packet: ReadWrite<u16>,
    /// 0x086 - indexed receive CSR.
    ///
    /// This mixes status, write-zero-to-clear, and self-clearing command bits;
    /// callers must use command-specific writes rather than generic RMW.
    pub rx_csr: ReadWrite<u16>,
    /// 0x088 - endpoint-zero byte count or indexed receive byte count.
    pub rx_count: ReadOnly<u16>,
    _reserved_08a: [u8; 6],
    /// 0x090 - indexed transmit FIFO size configuration.
    pub tx_fifo_size: ReadWrite<u8>,
    _reserved_091: u8,
    /// 0x092 - indexed transmit FIFO address in eight-byte units.
    pub tx_fifo_address: ReadWrite<u16>,
    /// 0x094 - indexed receive FIFO size configuration.
    pub rx_fifo_size: ReadWrite<u8>,
    _reserved_095: u8,
    /// 0x096 - indexed receive FIFO address in eight-byte units.
    pub rx_fifo_address: ReadWrite<u16>,
    /// 0x098 - USB device function address.
    pub function_address: ReadWrite<u8>,
    _reserved_099: [u8; 0x367],
    /// 0x400 - USB PHY register group.
    pub phy: PhyRegisterBlock,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, fifo), 0x000);
        assert_eq!(offset_of!(RegisterBlock, power), 0x040);
        assert_eq!(offset_of!(RegisterBlock, device_control), 0x041);
        assert_eq!(offset_of!(RegisterBlock, index), 0x042);
        assert_eq!(offset_of!(RegisterBlock, vendor_control), 0x043);
        assert_eq!(offset_of!(RegisterBlock, interrupt_tx), 0x044);
        assert_eq!(offset_of!(RegisterBlock, interrupt_rx), 0x046);
        assert_eq!(offset_of!(RegisterBlock, interrupt_tx_enable), 0x048);
        assert_eq!(offset_of!(RegisterBlock, interrupt_rx_enable), 0x04a);
        assert_eq!(offset_of!(RegisterBlock, interrupt_usb), 0x04c);
        assert_eq!(offset_of!(RegisterBlock, interrupt_usb_enable), 0x050);
        assert_eq!(offset_of!(RegisterBlock, tx_max_packet), 0x080);
        assert_eq!(offset_of!(RegisterBlock, tx_csr), 0x082);
        assert_eq!(offset_of!(RegisterBlock, rx_max_packet), 0x084);
        assert_eq!(offset_of!(RegisterBlock, rx_csr), 0x086);
        assert_eq!(offset_of!(RegisterBlock, rx_count), 0x088);
        assert_eq!(offset_of!(RegisterBlock, tx_fifo_size), 0x090);
        assert_eq!(offset_of!(RegisterBlock, tx_fifo_address), 0x092);
        assert_eq!(offset_of!(RegisterBlock, rx_fifo_size), 0x094);
        assert_eq!(offset_of!(RegisterBlock, rx_fifo_address), 0x096);
        assert_eq!(offset_of!(RegisterBlock, function_address), 0x098);
        assert_eq!(offset_of!(RegisterBlock, phy), 0x400);
        assert_eq!(size_of::<RegisterBlock>(), 0x424);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn phy_register_layout() {
        assert_eq!(offset_of!(PhyRegisterBlock, interface_status_control), 0x00);
        assert_eq!(offset_of!(PhyRegisterBlock, phy_control_28nm), 0x10);
        assert_eq!(offset_of!(PhyRegisterBlock, phy_select), 0x20);
        assert_eq!(size_of::<PhyRegisterBlock>(), 0x24);
    }

    #[test]
    fn access_widths_match_the_bus_registers() {
        assert_eq!(size_of::<ReadOnly<u16>>(), 2);
        assert_eq!(size_of::<ReadWrite<u8>>(), 1);
        assert_eq!(size_of::<ReadWrite<u16>>(), 2);
        assert_eq!(size_of::<ReadWrite<u32>>(), 4);
        assert_eq!(size_of::<WriteOneToClear<u8>>(), 1);
        assert_eq!(size_of::<WriteOneToClear<u16>>(), 2);
        assert_eq!(size_of::<InterfaceStatusControl>(), 4);
        assert_eq!(size_of::<Fifo>(), 4);
    }

    #[test]
    fn volatile_accessors_store_exact_values() {
        let read_write = ReadWrite(UnsafeCell::new(0x12_u8));
        assert_eq!(read_write.read(), 0x12);
        read_write.write(0xa5);
        assert_eq!(read_write.read(), 0xa5);

        let w1c = WriteOneToClear(UnsafeCell::new(0x55aa_u16));
        assert_eq!(w1c.status(), 0x55aa);
        w1c.acknowledge(0x0042);
        assert_eq!(w1c.status(), 0x0042);

        let interface = InterfaceStatusControl(UnsafeCell::new(0x0000_0075));
        interface.modify(|value| value | 0x0000_c000);
        assert_eq!(interface.read(), 0x0000_c005);

        let fifo = Fifo(UnsafeCell::new(0));
        fifo.write_byte(0x5a);
        assert_eq!(fifo.read_byte(), 0x5a);
    }
}
