//! Version 1 USB PHY registers.

use volatile_register::RW;

use super::InterfaceStatusControlRegister;

/// Version 1 USB PHY register block.
#[repr(C)]
pub struct RegisterBlockV1 {
    /// 0x000 - interface status and control register (ISCR).
    pub interface_status_control: InterfaceStatusControlRegister,
    /// 0x004 - PHY control register.
    pub phy_control: RW<PhyControl>,
    /// 0x008 - PHY built-in self-test register.
    pub phy_bist: RW<PhyBist>,
    /// 0x00c - PHY tuning register.
    pub phy_tune: RW<PhyTune>,
}

/// Version 1 PHY control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PhyControl(u32);

impl PhyControl {
    const VC_CLOCK: u32 = 1 << 0;
    const SIDDQ: u32 = 1 << 1;

    /// Drive the PHY VC clock high or low.
    #[inline]
    pub const fn set_vc_clock_high(self, high: bool) -> Self {
        Self((self.0 & !Self::VC_CLOCK) | if high { Self::VC_CLOCK } else { 0 })
    }

    /// Clear SIDDQ and power up the PHY.
    #[inline]
    pub const fn power_up(self) -> Self {
        Self(self.0 & !Self::SIDDQ)
    }

    /// Set SIDDQ and power down the PHY.
    #[inline]
    pub const fn power_down(self) -> Self {
        Self(self.0 | Self::SIDDQ)
    }
}

/// Version 1 PHY built-in self-test value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PhyBist(u32);

/// Version 1 PHY tuning value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PhyTune(u32);

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlockV1, interface_status_control), 0x00);
        assert_eq!(offset_of!(RegisterBlockV1, phy_control), 0x04);
        assert_eq!(offset_of!(RegisterBlockV1, phy_bist), 0x08);
        assert_eq!(offset_of!(RegisterBlockV1, phy_tune), 0x0c);
        assert_eq!(size_of::<RegisterBlockV1>(), 0x10);
        assert_eq!(align_of::<RegisterBlockV1>(), 4);
    }

    #[test]
    fn phy_control_uses_version_1_bits() {
        assert_eq!(PhyControl::default().set_vc_clock_high(true).0, 1 << 0);
        assert_eq!(PhyControl::default().power_down().0, 1 << 1);
        assert_eq!(PhyControl(u32::MAX).power_up().0 & (1 << 1), 0);
    }
}
