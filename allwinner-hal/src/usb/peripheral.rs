use embedded_hal::delay::DelayNs;

use super::register::usb::{
    BusInterruptEnable, FunctionAddress, ReceiveInterruptEnable, RegisterBlock,
    TransmitInterruptEnable,
};

/// A uniquely owned USB device-controller instance.
///
/// # Safety
///
/// Consuming one implementor value must yield logically exclusive access to one
/// valid [`RegisterBlock`] for lifetime `'a`. No other safe value may produce
/// an overlapping controller capability while the returned reference is live,
/// and the MMIO mapping must remain accessible in every execution context to
/// which the capability can be moved.
pub unsafe trait Instance<'a> {
    /// Consume the singleton token and borrow its controller registers.
    fn register_block(self) -> &'a RegisterBlock;
}

/// Exclusively owned USB device controller.
///
/// The chip-specific PHY is intentionally not retained here. Configure and
/// keep ownership of it separately, for example with
/// [`crate::usb::phy::v2::UsbPhy`] on V821.
pub struct Usb<'a> {
    registers: &'a RegisterBlock,
}

// SAFETY: `Instance` grants one logically exclusive controller capability and
// requires its MMIO mapping to remain accessible after a context move. `Usb`
// is not `Sync`; the `UsbBus` adapter adds synchronization around every access.
unsafe impl Send for Usb<'_> {}

impl<'a> Usb<'a> {
    /// Consume a controller token and leave the full-speed device detached.
    ///
    /// `delay` is borrowed only long enough to make a previous attachment
    /// visible to the host. Clock/reset must already be enabled; the independent
    /// PHY must be initialized before [`UsbBus`](crate::usb::UsbBus) is enabled.
    pub fn new(instance: impl Instance<'a>, delay: &mut impl DelayNs) -> Self {
        let registers = instance.register_block();
        // SAFETY: consuming `instance` grants exclusive MMIO access.
        unsafe {
            registers
                .power
                .write(registers.power.read().set_soft_connected(false));
        }
        delay.delay_ms(250);

        // Select PIO and start detached with every controller interrupt off.
        // SAFETY: this capability is the sole controller writer.
        unsafe {
            registers
                .vendor_control
                .write(registers.vendor_control.read().select_pio_bus());
            registers
                .interrupt_usb_enable
                .write(BusInterruptEnable::default());
            registers
                .interrupt_tx_enable
                .write(TransmitInterruptEnable::default());
            registers
                .interrupt_rx_enable
                .write(ReceiveInterruptEnable::default());
            registers.function_address.write(FunctionAddress::default());
            registers.power.write(
                registers
                    .power
                    .read()
                    .set_high_speed_enabled(false)
                    .set_iso_update_enabled(false)
                    .set_soft_connected(false),
            );
        }
        acknowledge_pending(registers);

        Self { registers }
    }

    /// Return whether the controller currently observes valid VBUS.
    #[inline]
    pub fn is_vbus_valid(&self) -> bool {
        self.registers.device_control.read().is_vbus_valid()
    }

    pub(crate) fn registers(&self) -> &RegisterBlock {
        self.registers
    }
}

pub(crate) fn acknowledge_pending(registers: &RegisterBlock) {
    let transmit = registers.interrupt_tx.status();
    if !transmit.is_empty() {
        registers.interrupt_tx.acknowledge(transmit);
    }
    let receive = registers.interrupt_rx.status();
    if !receive.is_empty() {
        registers.interrupt_rx.acknowledge(receive);
    }
    let bus = registers.interrupt_usb.status();
    if !bus.is_empty() {
        registers.interrupt_usb.acknowledge(bus);
    }
}
