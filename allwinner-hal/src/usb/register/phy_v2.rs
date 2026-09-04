//! Version 2 USB PHY registers.

use volatile_register::{RO, RW};

use super::InterfaceStatusControlRegister;

/// Version 2 USB PHY register block.
#[repr(C)]
pub struct RegisterBlockV2 {
    /// 0x000 - interface status and control register (ISCR).
    pub interface_status_control: InterfaceStatusControlRegister,
    _reserved_004: [u8; 0x0c],
    /// 0x010 - PHY and VC-bus serial control register.
    pub phy_control: RW<PhyControl>,
    /// 0x014 - PHY test register.
    pub phy_test: RW<PhyTest>,
    /// 0x018 - PHY tuning register.
    pub phy_tune: RW<PhyTune>,
    _reserved_01c: [u8; 4],
    /// 0x020 - PHY controller path-selection register (PHYSEL).
    pub phy_select: RW<PhySelect>,
    /// 0x024 - PHY status register.
    pub phy_status: RO<PhyStatus>,
}

/// PHY and VC-bus control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PhyControl(u32);

impl PhyControl {
    const VC_CLOCK: u32 = 1 << 0;
    const VC_ENABLE: u32 = 1 << 1;
    const SIDDQ: u32 = 1 << 3;
    const VC_DATA: u32 = 1 << 7;
    const VC_ADDRESS: u32 = 0xff << 8;

    /// Enable the PHY VC-bus interface.
    #[inline]
    pub const fn enable_vc_bus(self) -> Self {
        Self(self.0 | Self::VC_ENABLE)
    }

    /// Drive the VC clock low and clear the address/data inputs.
    #[inline]
    pub const fn prepare_vc_write(self) -> Self {
        Self(self.0 & !(Self::VC_CLOCK | Self::VC_DATA | Self::VC_ADDRESS))
    }

    /// Present a VC selector and one data bit.
    #[inline]
    pub const fn set_vc_address_and_data(self, selector: u8, data_high: bool) -> Self {
        Self(
            (self.0 & !(Self::VC_ADDRESS | Self::VC_DATA))
                | ((selector as u32) << 8)
                | if data_high { Self::VC_DATA } else { 0 },
        )
    }

    /// Raise the VC clock to latch the current selector and data bit.
    #[inline]
    pub const fn raise_vc_clock(self) -> Self {
        Self(self.0 | Self::VC_CLOCK)
    }

    /// Clear SIDDQ and power up the PHY.
    #[inline]
    pub const fn power_up(self) -> Self {
        Self(self.0 & !Self::SIDDQ)
    }
}

/// USB PHY controller-path selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PhySelect(u32);

impl PhySelect {
    const OTG_CONTROLLER: u32 = 1;

    /// Route USB0 through the OTG controller.
    #[inline]
    pub const fn select_otg_controller(self) -> Self {
        Self(self.0 | Self::OTG_CONTROLLER)
    }
}

/// Version 2 PHY test value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PhyTest(u32);

/// Version 2 PHY tuning value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PhyTune(u32);

/// Version 2 PHY status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PhyStatus(u32);

impl PhyStatus {
    const VC_DATA_OUT: u32 = 1;

    /// Return the current VC-bus data output bit.
    #[inline]
    pub const fn vc_data_out(self) -> bool {
        self.0 & Self::VC_DATA_OUT != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlockV2, interface_status_control), 0x00);
        assert_eq!(offset_of!(RegisterBlockV2, phy_control), 0x10);
        assert_eq!(offset_of!(RegisterBlockV2, phy_test), 0x14);
        assert_eq!(offset_of!(RegisterBlockV2, phy_tune), 0x18);
        assert_eq!(offset_of!(RegisterBlockV2, phy_select), 0x20);
        assert_eq!(offset_of!(RegisterBlockV2, phy_status), 0x24);
        assert_eq!(size_of::<RegisterBlockV2>(), 0x28);
        assert_eq!(align_of::<RegisterBlockV2>(), 4);
    }

    #[test]
    fn access_widths_match_the_bus_registers() {
        assert_eq!(size_of::<InterfaceStatusControlRegister>(), 4);
        assert_eq!(size_of::<RW<PhyControl>>(), 4);
        assert_eq!(size_of::<RW<PhySelect>>(), 4);
        assert_eq!(size_of::<RO<PhyStatus>>(), 4);
    }

    #[test]
    fn controls_match_the_initialization_sequence() {
        let control = PhyControl(u32::MAX).enable_vc_bus().prepare_vc_write();
        assert_eq!(control.0, 0xffff_007e);
        let control = control.set_vc_address_and_data(11, true);
        assert_eq!(control.0 & 0x0000_ff80, 0x0000_0b80);
        assert_eq!(control.raise_vc_clock().0 & 1, 1);
        assert_eq!(PhyControl(u32::MAX).power_up().0 & (1 << 3), 0);
        assert_eq!(PhySelect(0).select_otg_controller().0, 1);
        assert!(PhyStatus(1).vc_data_out());
        assert!(!PhyStatus(0).vc_data_out());
    }
}
