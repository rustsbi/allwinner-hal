//! F101 SRAM ownership selection in the system controller.

use volatile_register::RW;

/// F101 system-control registers through SRAM remapping.
#[repr(C)]
pub struct RegisterBlock {
    _reserved_000: [u8; 4],
    /// 0x004 - SRAM remapping control.
    pub sram_remap: RW<SramRemap>,
}

/// SRAM mapping between the CPU and peripheral FIFO owners.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SramRemap(u32);

impl SramRemap {
    /// Assign the USB FIFO banks to the controller, preserving other banks.
    /// The firmware linker must exclude these banks from CPU allocations.
    pub const fn use_usb_fifo(self) -> Self {
        Self(self.0 & !((1 << 25) | (1 << 27)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, sram_remap), 4);
        assert_eq!(size_of::<RegisterBlock>(), 8);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }

    #[test]
    fn usb_mapping_preserves_other_banks() {
        assert_eq!(SramRemap(u32::MAX).use_usb_fifo().0, 0xf5ff_ffff);
        // The connected F101 BootROM already assigns bank 25 to USB.
        assert_eq!(SramRemap(0x0d00_0000).use_usb_fifo().0, 0x0500_0000);
    }
}
