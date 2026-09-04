//! Register values shared by USB PHY versions.

use volatile_register::RW;

/// USB PHY interface status and control value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct InterfaceStatusControl(u32);

impl InterfaceStatusControl {
    const CHANGE_DETECT: u32 = 0x70;
    const FORCE_ID: u32 = 0x3 << 14;
    const DPDM_PULLUP: u32 = 1 << 16;
    const VBUS_VALID_SOURCE: u32 = 0x3 << 10;
    const FORCE_VBUS_VALID: u32 = 0x3 << 12;

    const fn without_change_detect(self) -> Self {
        Self(self.0 & !Self::CHANGE_DETECT)
    }

    /// Force the PHY ID input high.
    #[inline]
    pub const fn force_id_high(self) -> Self {
        Self((self.0 & !Self::FORCE_ID) | Self::FORCE_ID)
    }

    /// Enable or disable the DP/DM pull-up control.
    #[inline]
    pub const fn set_dpdm_pullup_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !Self::DPDM_PULLUP) | if enabled { Self::DPDM_PULLUP } else { 0 })
    }

    /// Use all available VBUS-valid sources.
    #[inline]
    pub const fn use_all_vbus_valid_sources(self) -> Self {
        Self((self.0 & !Self::VBUS_VALID_SOURCE) | Self::VBUS_VALID_SOURCE)
    }

    /// Use the selected VBUS sources without a force override.
    #[inline]
    pub const fn use_detected_vbus(self) -> Self {
        Self(self.0 & !Self::FORCE_VBUS_VALID)
    }

    /// Force the VBUS-valid input high.
    #[inline]
    pub const fn force_vbus_valid_high(self) -> Self {
        Self((self.0 & !Self::FORCE_VBUS_VALID) | Self::FORCE_VBUS_VALID)
    }
}

/// ISCR register wrapper that protects its W1C change-detect fields.
#[repr(transparent)]
pub struct InterfaceStatusControlRegister(RW<InterfaceStatusControl>);

impl InterfaceStatusControlRegister {
    /// Read the complete status and control value.
    #[inline(always)]
    pub fn read(&self) -> InterfaceStatusControl {
        self.0.read()
    }

    /// Modify control fields without acknowledging change-detect status.
    #[inline(always)]
    pub fn modify_control(&self, f: impl FnOnce(InterfaceStatusControl) -> InterfaceStatusControl) {
        let control = f(self.read().without_change_detect()).without_change_detect();
        // SAFETY: this performs one volatile control write while forcing every
        // W1C change-detect bit to zero.
        unsafe { self.0.write(control) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn access_width_matches_both_phy_versions() {
        assert_eq!(size_of::<InterfaceStatusControlRegister>(), 4);
    }

    #[test]
    fn control_fields_preserve_unrelated_bits() {
        let interface = InterfaceStatusControl(0x70)
            .force_id_high()
            .set_dpdm_pullup_enabled(true)
            .use_all_vbus_valid_sources()
            .force_vbus_valid_high()
            .without_change_detect();
        assert_eq!(interface.0 & 0x70, 0);
        assert_eq!(interface.0 & 0x0001_fc00, 0x0001_fc00);
        assert_eq!(interface.use_detected_vbus().0 & (0x3 << 12), 0);
    }

    #[test]
    fn accessor_does_not_acknowledge_change_detect() {
        let mut backing = InterfaceStatusControl(0x75);
        // SAFETY: both types are transparent wrappers with identical size and
        // alignment; the reference is used only for this stack-backed test.
        let register = unsafe {
            &*((&mut backing as *mut InterfaceStatusControl)
                .cast::<InterfaceStatusControlRegister>())
        };
        register.modify_control(InterfaceStatusControl::force_id_high);
        assert_eq!(backing.0, 0x0000_c005);
    }
}
