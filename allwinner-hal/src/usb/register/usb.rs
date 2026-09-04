//! MUSB-compatible USB device-controller registers.

use core::cell::UnsafeCell;
use volatile_register::{RO, RW};

/// Partial MUSB-compatible USB device-controller map used by the polled device
/// driver.
///
/// Endpoint configuration registers from `tx_max_packet` through
/// `rx_fifo_address` are indexed by [`Self::index`].
#[repr(C)]
pub struct RegisterBlock {
    /// 0x000..0x00c - endpoint FIFO data ports 0 through 3.
    pub fifo: [Fifo; 4],
    _reserved_010: [u8; 0x30],
    /// 0x040 - power and connection control register.
    pub power: RW<Power>,
    /// 0x041 - device control and bus state register.
    pub device_control: RO<DeviceControl>,
    /// 0x042 - indexed endpoint selector.
    pub index: RW<EndpointIndex>,
    /// 0x043 - vendor bus-mode control register.
    pub vendor_control: RW<VendorControl>,
    /// 0x044 - transmit endpoint interrupt status.
    pub interrupt_tx: WriteOneToClear<TransmitInterruptStatus>,
    /// 0x046 - receive endpoint interrupt status.
    pub interrupt_rx: WriteOneToClear<ReceiveInterruptStatus>,
    /// 0x048 - transmit endpoint interrupt enable.
    pub interrupt_tx_enable: RW<TransmitInterruptEnable>,
    /// 0x04a - receive endpoint interrupt enable.
    pub interrupt_rx_enable: RW<ReceiveInterruptEnable>,
    /// 0x04c - USB bus interrupt status.
    pub interrupt_usb: WriteOneToClear<BusInterruptStatus>,
    _reserved_04d: [u8; 3],
    /// 0x050 - USB bus interrupt enable.
    pub interrupt_usb_enable: RW<BusInterruptEnable>,
    _reserved_051: [u8; 3],
    _reserved_054: [u8; 0x2c],
    /// 0x080 - indexed transmit maximum packet size.
    pub tx_max_packet: RW<MaximumPacketSize>,
    /// 0x082 - endpoint-zero CSR or indexed transmit CSR.
    pub tx_csr: IndexedControlStatusRegister,
    /// 0x084 - indexed receive maximum packet size.
    pub rx_max_packet: RW<MaximumPacketSize>,
    /// 0x086 - indexed receive CSR.
    pub rx_csr: ReceiveControlStatusRegister,
    /// 0x088 - endpoint-zero byte count or indexed receive byte count.
    pub rx_count: RO<ReceiveByteCount>,
    _reserved_08a: [u8; 6],
    /// 0x090 - indexed transmit FIFO size configuration.
    pub tx_fifo_size: RW<FifoSize>,
    _reserved_091: u8,
    /// 0x092 - indexed transmit FIFO address.
    pub tx_fifo_address: RW<FifoAddress>,
    /// 0x094 - indexed receive FIFO size configuration.
    pub rx_fifo_size: RW<FifoSize>,
    _reserved_095: u8,
    /// 0x096 - indexed receive FIFO address.
    pub rx_fifo_address: RW<FifoAddress>,
    /// 0x098 - USB device function address.
    pub function_address: RW<FunctionAddress>,
}

/// USB power and connection control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Power(u8);

impl Power {
    const HIGH_SPEED_ENABLE: u8 = 1 << 5;
    const SOFT_CONNECT: u8 = 1 << 6;
    const ISO_UPDATE: u8 = 1 << 7;

    /// Enable or disable high-speed negotiation.
    #[inline]
    pub const fn set_high_speed_enabled(self, enabled: bool) -> Self {
        Self(
            (self.0 & !Self::HIGH_SPEED_ENABLE) | if enabled { Self::HIGH_SPEED_ENABLE } else { 0 },
        )
    }

    /// Connect or disconnect the device pull-up under software control.
    #[inline]
    pub const fn set_soft_connected(self, connected: bool) -> Self {
        Self((self.0 & !Self::SOFT_CONNECT) | if connected { Self::SOFT_CONNECT } else { 0 })
    }

    /// Enable or disable ISO update mode.
    #[inline]
    pub const fn set_iso_update_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::ISO_UPDATE) | if enabled { Self::ISO_UPDATE } else { 0 })
    }
}

/// USB device control and bus state.
///
/// This partial model exposes only the VBUS state consumed by the current
/// device examples. Other DEVCTL fields include writable host/session controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DeviceControl(u8);

impl DeviceControl {
    const VBUS_LEVEL: u8 = 0x3 << 3;

    /// Return whether VBUS is at or above the valid level.
    #[inline]
    pub const fn is_vbus_valid(self) -> bool {
        self.0 & Self::VBUS_LEVEL == Self::VBUS_LEVEL
    }
}

/// Indexed endpoint selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct EndpointIndex(u8);

impl EndpointIndex {
    /// Select endpoint zero through fifteen.
    #[inline]
    pub const fn new(endpoint: u8) -> Self {
        assert!(endpoint < 16);
        Self(endpoint)
    }

    /// Return the selected endpoint number.
    #[inline]
    pub const fn endpoint(self) -> u8 {
        self.0
    }
}

/// Vendor-specific USB bus-mode control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VendorControl(u8);

impl VendorControl {
    const BUS_SELECT: u8 = 1;

    /// Select programmed-I/O access rather than DMA bus mode.
    #[inline]
    pub const fn select_pio_bus(self) -> Self {
        Self(self.0 & !Self::BUS_SELECT)
    }
}

const fn endpoint_interrupt_mask(endpoint: u8) -> u16 {
    assert!(endpoint < 16);
    1 << endpoint
}

/// Transmit endpoint interrupt status and W1C acknowledgement mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TransmitInterruptStatus(u16);

impl TransmitInterruptStatus {
    /// Return whether no transmit endpoint interrupt is pending.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Return whether an endpoint interrupt is pending.
    #[inline]
    pub const fn endpoint_pending(self, endpoint: u8) -> bool {
        self.0 & endpoint_interrupt_mask(endpoint) != 0
    }

    /// Create an exact acknowledgement mask for one endpoint.
    #[inline]
    pub const fn for_endpoint(endpoint: u8) -> Self {
        Self(endpoint_interrupt_mask(endpoint))
    }

    /// Remove one endpoint from an acknowledgement mask.
    #[inline]
    pub const fn without_endpoint(self, endpoint: u8) -> Self {
        Self(self.0 & !endpoint_interrupt_mask(endpoint))
    }
}

/// Receive endpoint interrupt status and W1C acknowledgement mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ReceiveInterruptStatus(u16);

impl ReceiveInterruptStatus {
    /// Return whether no receive endpoint interrupt is pending.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Return whether an endpoint interrupt is pending.
    #[inline]
    pub const fn endpoint_pending(self, endpoint: u8) -> bool {
        self.0 & endpoint_interrupt_mask(endpoint) != 0
    }

    /// Create an exact acknowledgement mask for one endpoint.
    #[inline]
    pub const fn for_endpoint(endpoint: u8) -> Self {
        Self(endpoint_interrupt_mask(endpoint))
    }
}

/// Transmit endpoint interrupt-enable mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TransmitInterruptEnable(u16);

impl TransmitInterruptEnable {
    /// Enable the interrupt for one endpoint.
    #[inline]
    pub const fn enable_endpoint(self, endpoint: u8) -> Self {
        Self(self.0 | endpoint_interrupt_mask(endpoint))
    }

    /// Return whether an endpoint interrupt is enabled.
    #[inline]
    pub const fn endpoint_enabled(self, endpoint: u8) -> bool {
        self.0 & endpoint_interrupt_mask(endpoint) != 0
    }
}

/// Receive endpoint interrupt-enable mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ReceiveInterruptEnable(u16);

impl ReceiveInterruptEnable {
    /// Enable the interrupt for one endpoint.
    #[inline]
    pub const fn enable_endpoint(self, endpoint: u8) -> Self {
        Self(self.0 | endpoint_interrupt_mask(endpoint))
    }

    /// Return whether an endpoint interrupt is enabled.
    #[inline]
    pub const fn endpoint_enabled(self, endpoint: u8) -> bool {
        self.0 & endpoint_interrupt_mask(endpoint) != 0
    }
}

/// USB bus interrupt status and W1C acknowledgement mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BusInterruptStatus(u8);

impl BusInterruptStatus {
    const RESET: u8 = 1 << 2;

    /// Return whether no USB bus interrupt is pending.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Return whether a bus reset is pending.
    #[inline]
    pub const fn reset_pending(self) -> bool {
        self.0 & Self::RESET != 0
    }
}

/// USB bus interrupt-enable mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct BusInterruptEnable(u8);

impl BusInterruptEnable {
    const SUSPEND: u8 = 1 << 0;
    const RESUME: u8 = 1 << 1;
    const RESET: u8 = 1 << 2;

    /// Enable suspend interrupts.
    #[inline]
    pub const fn enable_suspend(self) -> Self {
        Self(self.0 | Self::SUSPEND)
    }

    /// Enable resume interrupts.
    #[inline]
    pub const fn enable_resume(self) -> Self {
        Self(self.0 | Self::RESUME)
    }

    /// Enable bus-reset interrupts.
    #[inline]
    pub const fn enable_reset(self) -> Self {
        Self(self.0 | Self::RESET)
    }
}

/// Maximum packet size for an indexed endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct MaximumPacketSize(u16);

impl MaximumPacketSize {
    const MAXIMUM: u16 = 0x07ff;

    /// Encode a maximum packet size in bytes.
    #[inline]
    pub const fn new(bytes: u16) -> Self {
        assert!(bytes <= Self::MAXIMUM);
        Self(bytes)
    }

    /// Return the configured maximum packet size in bytes.
    #[inline]
    pub const fn bytes(self) -> u16 {
        self.0
    }
}

/// Receive byte count for endpoint zero or an indexed OUT endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ReceiveByteCount(u16);

impl ReceiveByteCount {
    /// Return the number of bytes waiting in the selected endpoint FIFO.
    #[inline]
    pub const fn bytes(self) -> usize {
        self.0 as usize
    }
}

/// Dynamic FIFO size and buffering mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoSize(u8);

impl FifoSize {
    const DOUBLE_BUFFERED: u8 = 1 << 4;
    const SIZE_EXPONENT: u8 = 0x0f;

    /// Configure one 512-byte FIFO bank.
    #[inline]
    pub const fn single_512() -> Self {
        Self(0x06)
    }

    /// Configure two 512-byte FIFO banks.
    #[inline]
    pub const fn double_512() -> Self {
        Self(Self::DOUBLE_BUFFERED | 0x06)
    }

    /// Return whether the FIFO uses two banks.
    #[inline]
    pub const fn is_double_buffered(self) -> bool {
        self.0 & Self::DOUBLE_BUFFERED != 0
    }

    /// Return the total reserved FIFO space in bytes.
    #[inline]
    pub const fn total_bytes(self) -> usize {
        let bank = 1_usize << ((self.0 & Self::SIZE_EXPONENT) as usize + 3);
        if self.is_double_buffered() {
            bank * 2
        } else {
            bank
        }
    }
}

/// Dynamic FIFO address in the controller's eight-byte units.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FifoAddress(u16);

impl FifoAddress {
    const MAXIMUM_UNITS: u16 = 0x1fff;

    /// Encode an eight-byte-aligned FIFO byte offset.
    #[inline]
    pub const fn from_byte_offset(offset: u16) -> Self {
        assert!(offset.is_multiple_of(8));
        let units = offset / 8;
        assert!(units <= Self::MAXIMUM_UNITS);
        Self(units)
    }

    /// Return the FIFO byte offset.
    #[inline]
    pub const fn byte_offset(self) -> usize {
        self.0 as usize * 8
    }
}

/// USB device function address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct FunctionAddress(u8);

impl FunctionAddress {
    /// Encode a seven-bit USB device address.
    #[inline]
    pub const fn new(address: u8) -> Self {
        assert!(address <= 0x7f);
        Self(address)
    }

    /// Return the encoded USB device address.
    #[inline]
    pub const fn address(self) -> u8 {
        self.0
    }
}

/// Endpoint-zero control status at offset 0x082 when INDEX is zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct EndpointZeroControlStatus(u16);

impl EndpointZeroControlStatus {
    const RECEIVED_PACKET_READY: u16 = 1 << 0;
    const TRANSMIT_PACKET_READY: u16 = 1 << 1;
    const SENT_STALL: u16 = 1 << 2;
    const DATA_END: u16 = 1 << 3;
    const SETUP_END: u16 = 1 << 4;
    const SEND_STALL: u16 = 1 << 5;
    const SERVICE_RECEIVED_PACKET: u16 = 1 << 6;
    const SERVICE_SETUP_END: u16 = 1 << 7;

    /// Return whether endpoint zero has received a packet.
    #[inline]
    pub const fn received_packet_ready(self) -> bool {
        self.0 & Self::RECEIVED_PACKET_READY != 0
    }

    /// Return whether an endpoint-zero IN packet is pending.
    #[inline]
    pub const fn transmit_packet_ready(self) -> bool {
        self.0 & Self::TRANSMIT_PACKET_READY != 0
    }

    /// Return whether endpoint zero has sent a STALL handshake.
    #[inline]
    pub const fn sent_stall(self) -> bool {
        self.0 & Self::SENT_STALL != 0
    }

    /// Return whether a control transfer ended before the current stage.
    #[inline]
    pub const fn setup_end(self) -> bool {
        self.0 & Self::SETUP_END != 0
    }

    /// Clear endpoint-zero command and status bits.
    #[inline]
    pub const fn clear() -> Self {
        Self(0)
    }

    /// Service SETUPEND.
    #[inline]
    pub const fn service_setup_end() -> Self {
        Self(Self::SERVICE_SETUP_END)
    }

    /// Service the received endpoint-zero packet.
    #[inline]
    pub const fn service_received_packet() -> Self {
        Self(Self::SERVICE_RECEIVED_PACKET)
    }

    /// Service the received packet and any outstanding SETUPEND.
    #[inline]
    pub const fn service_received_packet_and_setup_end() -> Self {
        Self(Self::SERVICE_RECEIVED_PACKET | Self::SERVICE_SETUP_END)
    }

    /// Service the received packet and complete the control transfer.
    #[inline]
    pub const fn service_received_packet_and_complete() -> Self {
        Self(Self::SERVICE_RECEIVED_PACKET | Self::DATA_END)
    }

    /// Service the received packet and request a STALL handshake.
    #[inline]
    pub const fn service_received_packet_and_stall() -> Self {
        Self(Self::SERVICE_RECEIVED_PACKET | Self::SEND_STALL)
    }

    /// Request an endpoint-zero STALL when no receive packet needs servicing.
    #[inline]
    pub const fn stall() -> Self {
        Self(Self::SEND_STALL)
    }

    /// Queue an endpoint-zero IN packet and optionally end the data stage.
    #[inline]
    pub const fn queue_transmit_packet(data_end: bool) -> Self {
        Self(Self::TRANSMIT_PACKET_READY | if data_end { Self::DATA_END } else { 0 })
    }
}

/// Transmit endpoint control status at offset 0x082 when INDEX is nonzero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct TransmitControlStatus(u16);

impl TransmitControlStatus {
    const PACKET_READY: u16 = 1 << 0;
    const FLUSH_FIFO: u16 = 1 << 3;
    const SEND_STALL: u16 = 1 << 4;
    const SENT_STALL: u16 = 1 << 5;
    const CLEAR_DATA_TOGGLE: u16 = 1 << 6;
    const MODE: u16 = 1 << 13;

    /// Return whether an IN packet is waiting for transmission.
    #[inline]
    pub const fn packet_ready(self) -> bool {
        self.0 & Self::PACKET_READY != 0
    }

    /// Return whether a STALL is requested or has been sent.
    #[inline]
    pub const fn is_stalled(self) -> bool {
        self.0 & (Self::SEND_STALL | Self::SENT_STALL) != 0
    }

    /// Return whether the endpoint can accept another packet.
    #[inline]
    pub const fn can_accept_packet(self) -> bool {
        !self.packet_ready() && !self.is_stalled()
    }

    /// Clear transmit endpoint control bits.
    #[inline]
    pub const fn clear() -> Self {
        Self(0)
    }

    /// Request a STALL from a transmit endpoint.
    #[inline]
    pub const fn stall() -> Self {
        Self(Self::MODE | Self::SEND_STALL)
    }

    /// Queue one transmit packet.
    #[inline]
    pub const fn queue_packet() -> Self {
        Self(Self::MODE | Self::PACKET_READY)
    }

    /// Flush one FIFO bank and reset the endpoint data toggle.
    #[inline]
    pub const fn flush_and_clear_data_toggle() -> Self {
        Self(Self::MODE | Self::FLUSH_FIFO | Self::CLEAR_DATA_TOGGLE)
    }
}

/// Receive endpoint control status and exact command values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ReceiveControlStatus(u16);

impl ReceiveControlStatus {
    const PACKET_READY: u16 = 1 << 0;
    const FLUSH_FIFO: u16 = 1 << 4;
    const SEND_STALL: u16 = 1 << 5;
    const SENT_STALL: u16 = 1 << 6;
    const CLEAR_DATA_TOGGLE: u16 = 1 << 7;

    /// Return whether an OUT packet is waiting in the FIFO.
    #[inline]
    pub const fn packet_ready(self) -> bool {
        self.0 & Self::PACKET_READY != 0
    }

    /// Return whether a STALL is requested or has been sent.
    #[inline]
    pub const fn is_stalled(self) -> bool {
        self.0 & (Self::SEND_STALL | Self::SENT_STALL) != 0
    }

    /// Clear receive status and release the current OUT packet.
    #[inline]
    pub const fn clear() -> Self {
        Self(0)
    }

    /// Request a STALL from a receive endpoint.
    #[inline]
    pub const fn stall() -> Self {
        Self(Self::SEND_STALL)
    }

    /// Flush one FIFO bank and reset the endpoint data toggle.
    #[inline]
    pub const fn flush_and_clear_data_toggle() -> Self {
        Self(Self::FLUSH_FIFO | Self::CLEAR_DATA_TOGGLE)
    }
}

/// Volatile W1C interrupt register.
#[repr(transparent)]
pub struct WriteOneToClear<T: Copy>(RW<T>);

impl<T: Copy> WriteOneToClear<T> {
    /// Read pending interrupt status without clearing it.
    #[inline(always)]
    pub fn status(&self) -> T {
        self.0.read()
    }

    /// Acknowledge exactly the interrupt bits selected by mask.
    #[inline(always)]
    pub fn acknowledge(&self, mask: T) {
        // SAFETY: this dedicated accessor performs one exact volatile write
        // and deliberately does not expose read-modify-write for W1C state.
        unsafe { self.0.write(mask) }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct RawIndexedControlStatus(u16);

/// Indexed endpoint-zero or transmit control/status register.
#[repr(transparent)]
pub struct IndexedControlStatusRegister(RW<RawIndexedControlStatus>);

impl IndexedControlStatusRegister {
    /// Read the register using endpoint-zero bit semantics.
    #[inline(always)]
    pub fn read_endpoint_zero(&self) -> EndpointZeroControlStatus {
        EndpointZeroControlStatus(self.0.read().0)
    }

    /// Write one exact endpoint-zero command value.
    #[inline(always)]
    pub fn write_endpoint_zero(&self, value: EndpointZeroControlStatus) {
        // SAFETY: the caller selects endpoint zero before using this typed
        // command accessor; the volatile write preserves the required width.
        unsafe { self.0.write(RawIndexedControlStatus(value.0)) }
    }

    /// Read the register using nonzero transmit-endpoint bit semantics.
    #[inline(always)]
    pub fn read_transmit(&self) -> TransmitControlStatus {
        TransmitControlStatus(self.0.read().0)
    }

    /// Write one exact nonzero transmit-endpoint command value.
    #[inline(always)]
    pub fn write_transmit(&self, value: TransmitControlStatus) {
        // SAFETY: the caller selects a nonzero IN endpoint before using this
        // accessor; the volatile write preserves the required width.
        unsafe { self.0.write(RawIndexedControlStatus(value.0)) }
    }
}

/// Receive control/status register with no generic read-modify-write access.
#[repr(transparent)]
pub struct ReceiveControlStatusRegister(RW<ReceiveControlStatus>);

impl ReceiveControlStatusRegister {
    /// Read receive endpoint status.
    #[inline(always)]
    pub fn read(&self) -> ReceiveControlStatus {
        self.0.read()
    }

    /// Write one exact receive endpoint command value.
    #[inline(always)]
    pub fn write(&self, value: ReceiveControlStatus) {
        // SAFETY: typed commands avoid copying W0C status back to RXCSR.
        unsafe { self.0.write(value) }
    }
}

#[repr(transparent)]
struct FifoWord(u32);

/// Endpoint FIFO data port.
#[repr(transparent)]
pub struct Fifo(UnsafeCell<FifoWord>);

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
        assert_eq!(size_of::<RegisterBlock>(), 0x09c);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn access_widths_match_the_bus_registers() {
        assert_eq!(size_of::<RW<Power>>(), 1);
        assert_eq!(size_of::<RO<DeviceControl>>(), 1);
        assert_eq!(size_of::<WriteOneToClear<TransmitInterruptStatus>>(), 2);
        assert_eq!(size_of::<IndexedControlStatusRegister>(), 2);
        assert_eq!(size_of::<ReceiveControlStatusRegister>(), 2);
        assert_eq!(size_of::<Fifo>(), 4);
    }

    #[test]
    fn power_and_bus_controls_preserve_unrelated_bits() {
        let power = Power(0xff)
            .set_high_speed_enabled(false)
            .set_iso_update_enabled(false)
            .set_soft_connected(false);
        assert_eq!(power.0, 0x1f);
        assert_eq!(power.set_soft_connected(true).0, 0x5f);

        assert!(DeviceControl(0x18).is_vbus_valid());
        assert!(!DeviceControl(0x10).is_vbus_valid());
        assert_eq!(EndpointIndex::new(3).endpoint(), 3);
        assert_eq!(VendorControl(0xff).select_pio_bus().0, 0xfe);
    }

    #[test]
    fn interrupt_masks_are_endpoint_typed() {
        let tx = TransmitInterruptStatus((1 << 0) | (1 << 3));
        assert!(tx.endpoint_pending(0));
        assert!(tx.endpoint_pending(3));
        assert_eq!(tx.without_endpoint(0).0, 1 << 3);
        assert_eq!(TransmitInterruptStatus::for_endpoint(2).0, 1 << 2);

        let rx = ReceiveInterruptStatus::for_endpoint(3);
        assert!(rx.endpoint_pending(3));
        assert!(!rx.is_empty());

        let tx_enable = TransmitInterruptEnable::default()
            .enable_endpoint(0)
            .enable_endpoint(2);
        assert!(tx_enable.endpoint_enabled(0));
        assert!(tx_enable.endpoint_enabled(2));

        let rx_enable = ReceiveInterruptEnable::default().enable_endpoint(2);
        assert!(rx_enable.endpoint_enabled(2));

        assert!(BusInterruptStatus(1 << 2).reset_pending());
        assert_eq!(
            BusInterruptEnable::default()
                .enable_suspend()
                .enable_resume()
                .enable_reset()
                .0,
            0x07
        );
    }

    #[test]
    fn packet_and_fifo_encodings_match_the_controller() {
        assert_eq!(MaximumPacketSize::new(64).bytes(), 64);
        assert_eq!(ReceiveByteCount(31).bytes(), 31);
        assert_eq!(FifoSize::single_512().0, 0x06);
        assert_eq!(FifoSize::single_512().total_bytes(), 512);
        assert_eq!(FifoSize::double_512().0, 0x16);
        assert_eq!(FifoSize::double_512().total_bytes(), 1024);
        assert_eq!(FifoAddress::from_byte_offset(0x0e00).0, 0x01c0);
        assert_eq!(FifoAddress::from_byte_offset(0x0e00).byte_offset(), 0x0e00);
        assert_eq!(FunctionAddress::new(127).address(), 127);
    }

    #[test]
    fn endpoint_control_commands_use_exact_encodings() {
        let ep0 = EndpointZeroControlStatus(0x17);
        assert!(ep0.received_packet_ready());
        assert!(ep0.transmit_packet_ready());
        assert!(ep0.sent_stall());
        assert!(ep0.setup_end());
        assert_eq!(EndpointZeroControlStatus::service_setup_end().0, 0x80);
        assert_eq!(
            EndpointZeroControlStatus::service_received_packet_and_setup_end().0,
            0xc0
        );
        assert_eq!(
            EndpointZeroControlStatus::service_received_packet_and_complete().0,
            0x48
        );
        assert_eq!(
            EndpointZeroControlStatus::service_received_packet_and_stall().0,
            0x60
        );
        assert_eq!(EndpointZeroControlStatus::stall().0, 0x20);
        assert_eq!(
            EndpointZeroControlStatus::queue_transmit_packet(false).0,
            0x02
        );
        assert_eq!(
            EndpointZeroControlStatus::queue_transmit_packet(true).0,
            0x0a
        );

        assert!(TransmitControlStatus(0).can_accept_packet());
        assert!(!TransmitControlStatus(1).can_accept_packet());
        assert!(TransmitControlStatus(0x10).is_stalled());
        assert_eq!(TransmitControlStatus::stall().0, 0x2010);
        assert_eq!(TransmitControlStatus::queue_packet().0, 0x2001);
        assert_eq!(
            TransmitControlStatus::flush_and_clear_data_toggle().0,
            0x2048
        );

        assert!(ReceiveControlStatus(1).packet_ready());
        assert!(ReceiveControlStatus(0x40).is_stalled());
        assert_eq!(ReceiveControlStatus::stall().0, 0x0020);
        assert_eq!(
            ReceiveControlStatus::flush_and_clear_data_toggle().0,
            0x0090
        );
    }

    #[test]
    fn specialized_accessors_write_exact_typed_values() {
        let mut interrupt_backing = TransmitInterruptStatus(0x55aa);
        // SAFETY: both types are transparent wrappers with identical size and
        // alignment; the reference is used only for this stack-backed test.
        let interrupt = unsafe {
            &*((&mut interrupt_backing as *mut TransmitInterruptStatus)
                .cast::<WriteOneToClear<TransmitInterruptStatus>>())
        };
        assert_eq!(interrupt.status(), TransmitInterruptStatus(0x55aa));
        interrupt.acknowledge(TransmitInterruptStatus::for_endpoint(3));
        assert_eq!(interrupt_backing, TransmitInterruptStatus(1 << 3));

        let mut indexed_backing = RawIndexedControlStatus(0);
        // SAFETY: same transparent-wrapper layout argument as above.
        let indexed = unsafe {
            &*((&mut indexed_backing as *mut RawIndexedControlStatus)
                .cast::<IndexedControlStatusRegister>())
        };
        indexed
            .write_endpoint_zero(EndpointZeroControlStatus::service_received_packet_and_complete());
        assert_eq!(indexed_backing.0, 0x48);
        indexed.write_transmit(TransmitControlStatus::queue_packet());
        assert_eq!(indexed_backing.0, 0x2001);

        let mut receive_backing = ReceiveControlStatus::clear();
        // SAFETY: same transparent-wrapper layout argument as above.
        let receive = unsafe {
            &*((&mut receive_backing as *mut ReceiveControlStatus)
                .cast::<ReceiveControlStatusRegister>())
        };
        receive.write(ReceiveControlStatus::flush_and_clear_data_toggle());
        assert_eq!(receive_backing.0, 0x0090);

        let mut fifo_backing = FifoWord(0);
        // SAFETY: Fifo is a transparent wrapper over the same four-byte word.
        let fifo = unsafe { &*((&mut fifo_backing as *mut FifoWord).cast::<Fifo>()) };
        fifo.write_byte(0x5a);
        assert_eq!(fifo.read_byte(), 0x5a);
    }
}
