//! Polled CDC-ACM plus read-only mass-storage composite device.

use crate::{
    usb::UsbCdcMscTransport,
    usb_msc::{MassStorageTransport, ReadSector, UsbMassStorageDriver},
};

impl MassStorageTransport for UsbCdcMscTransport {
    fn initialize(&mut self) {
        UsbCdcMscTransport::initialize(self);
    }

    fn take_class_reset(&mut self) -> bool {
        UsbCdcMscTransport::take_class_reset(self)
    }

    fn is_configured(&self) -> bool {
        UsbCdcMscTransport::is_configured(self)
    }

    fn poll(&mut self, output: &mut [u8; 64]) -> usize {
        UsbCdcMscTransport::poll(self, output)
    }

    fn stall_bulk_in(&mut self, until_class_reset: bool) {
        UsbCdcMscTransport::stall_bulk_in(self, until_class_reset);
    }

    fn write(&mut self, bytes: &[u8]) {
        UsbCdcMscTransport::write(self, bytes);
    }

    fn write_zero_length_packet(&mut self) {
        UsbCdcMscTransport::write_zero_length_packet(self);
    }

    fn flush(&mut self) -> bool {
        UsbCdcMscTransport::flush(self)
    }
}

/// One USB device exposing a CDC-ACM console and a read-only SCSI disk.
pub struct UsbComposite {
    storage: UsbMassStorageDriver<UsbCdcMscTransport>,
}

impl UsbComposite {
    /// Maps V821 USB0 after the BootROM hands the E907 to this payload.
    ///
    /// # Safety
    ///
    /// USB0, APP-CCU, and AON-CCU must be exclusively owned by this E907
    /// payload with interrupts disabled.
    pub unsafe fn from_v821_mmio(block_count: u32, read_sector: ReadSector) -> Self {
        Self {
            storage: UsbMassStorageDriver::new(
                // SAFETY: forwarded from this function's ownership contract.
                unsafe { UsbCdcMscTransport::from_v821_mmio() },
                block_count,
                read_sector,
            ),
        }
    }

    /// Reconnects USB0 and initializes both functions.
    pub fn initialize(&mut self) {
        self.storage.initialize();
    }

    pub fn is_configured(&self) -> bool {
        self.storage.transport().is_configured()
    }

    /// Services MSC first, then returns at most one CDC bulk-OUT packet.
    pub fn poll(&mut self, output: &mut [u8; 64]) -> usize {
        // A safe eject only ends the host's storage session. The CDC function
        // remains active, so the composite payload keeps running.
        let _ = self.storage.poll();
        self.storage.transport_mut().poll_cdc(output)
    }

    /// Writes bytes through the CDC-ACM bulk-IN endpoint.
    pub fn write(&mut self, bytes: &[u8]) {
        self.storage.transport_mut().write_cdc(bytes);
    }

    /// Waits for the final CDC packet acknowledgement, or reports a reset.
    pub fn flush(&mut self) -> bool {
        self.storage.transport_mut().flush_cdc()
    }
}
