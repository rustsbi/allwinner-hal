//! PLL registers.

/// CPU PLL Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllCpuControl(u32);

impl PllCpuControl {
    const PLL_ENABLE: u32 = 1 << 31;
    const PLL_LDO_ENABLE: u32 = 1 << 30;
    const LOCK_ENABLE: u32 = 1 << 29;
    const LOCK: u32 = 1 << 28;
    const PLL_OUTPUT_GATE: u32 = 1 << 27;
    const PLL_N: u32 = 0xff << 8;
    const PLL_M: u32 = 0x3 << 0;

    /// Get if PLL is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        self.0 & Self::PLL_ENABLE != 0
    }
    /// Enable PLL.
    #[inline]
    pub const fn enable_pll(self) -> Self {
        Self(self.0 | Self::PLL_ENABLE)
    }
    /// Disable PLL.
    #[inline]
    pub const fn disable_pll(self) -> Self {
        Self(self.0 & !Self::PLL_ENABLE)
    }
    /// Get if PLL LDO is enabled.
    #[inline]
    pub const fn is_pll_ldo_enabled(self) -> bool {
        self.0 & Self::PLL_LDO_ENABLE != 0
    }
    /// Enable PLL LDO.
    #[inline]
    pub const fn enable_pll_ldo(self) -> Self {
        Self(self.0 | Self::PLL_LDO_ENABLE)
    }
    /// Disable PLL LDO.
    #[inline]
    pub const fn disable_pll_ldo(self) -> Self {
        Self(self.0 & !Self::PLL_LDO_ENABLE)
    }
    /// Get if PLL lock is enabled.
    #[inline]
    pub const fn is_lock_enabled(self) -> bool {
        self.0 & Self::LOCK_ENABLE != 0
    }
    /// Enable PLL lock.
    #[inline]
    pub const fn enable_lock(self) -> Self {
        Self(self.0 | Self::LOCK_ENABLE)
    }
    /// Disable PLL lock.
    #[inline]
    pub const fn disable_lock(self) -> Self {
        Self(self.0 & !Self::LOCK_ENABLE)
    }
    /// Get if the PLL locked state is set by hardware.
    #[inline]
    pub const fn is_locked(self) -> bool {
        self.0 & Self::LOCK != 0
    }
    /// Unmask (enable) PLL output.
    pub const fn unmask_pll_output(self) -> Self {
        Self(self.0 | Self::PLL_OUTPUT_GATE)
    }
    /// Mask (disable) PLL output.
    #[inline]
    pub const fn mask_pll_output(self) -> Self {
        Self(self.0 & !Self::PLL_OUTPUT_GATE)
    }
    /// Get if PLL output is unmasked.
    #[inline]
    pub const fn is_pll_output_unmasked(self) -> bool {
        self.0 & Self::PLL_OUTPUT_GATE != 0
    }
    /// Get PLL N factor.
    #[inline]
    pub const fn pll_n(self) -> u8 {
        ((self.0 & Self::PLL_N) >> 8) as u8
    }
    /// Set PLL N factor.
    #[inline]
    pub const fn set_pll_n(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_N) | ((val as u32) << 8))
    }
    /// Get PLL M factor.
    #[inline]
    pub const fn pll_m(self) -> u8 {
        (self.0 & Self::PLL_M) as u8
    }
    /// Set PLL M factor.
    #[inline]
    pub const fn set_pll_m(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_M) | val as u32)
    }
}

impl Default for PllCpuControl {
    #[inline]
    fn default() -> Self {
        Self(0x4a00_1000)
    }
}

/// DDR PLL Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllDdrControl(u32);

impl PllDdrControl {
    const PLL_ENABLE: u32 = 1 << 31;
    const PLL_LDO_ENABLE: u32 = 1 << 30;
    const LOCK_ENABLE: u32 = 1 << 29;
    const LOCK: u32 = 1 << 28;
    const PLL_OUTPUT_GATE: u32 = 1 << 27;
    const PLL_N: u32 = 0xff << 8;
    const PLL_M1: u32 = 0x1 << 1;
    const PLL_M0: u32 = 0x1 << 0;

    /// Get if PLL is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        self.0 & Self::PLL_ENABLE != 0
    }
    /// Enable PLL.
    #[inline]
    pub const fn enable_pll(self) -> Self {
        Self(self.0 | Self::PLL_ENABLE)
    }
    /// Disable PLL.
    #[inline]
    pub const fn disable_pll(self) -> Self {
        Self(self.0 & !Self::PLL_ENABLE)
    }
    /// Get if PLL LDO is enabled.
    #[inline]
    pub const fn is_pll_ldo_enabled(self) -> bool {
        self.0 & Self::PLL_LDO_ENABLE != 0
    }
    /// Enable PLL LDO.
    #[inline]
    pub const fn enable_pll_ldo(self) -> Self {
        Self(self.0 | Self::PLL_LDO_ENABLE)
    }
    /// Disable PLL LDO.
    #[inline]
    pub const fn disable_pll_ldo(self) -> Self {
        Self(self.0 & !Self::PLL_LDO_ENABLE)
    }
    /// Get if PLL lock is enabled.
    #[inline]
    pub const fn is_lock_enabled(self) -> bool {
        self.0 & Self::LOCK_ENABLE != 0
    }
    /// Enable PLL lock.
    #[inline]
    pub const fn enable_lock(self) -> Self {
        Self(self.0 | Self::LOCK_ENABLE)
    }
    /// Disable PLL lock.
    #[inline]
    pub const fn disable_lock(self) -> Self {
        Self(self.0 & !Self::LOCK_ENABLE)
    }
    /// Get if the PLL locked state is set by hardware.
    #[inline]
    pub const fn is_locked(self) -> bool {
        self.0 & Self::LOCK != 0
    }
    /// Unmask (enable) PLL output.
    pub const fn unmask_pll_output(self) -> Self {
        Self(self.0 | Self::PLL_OUTPUT_GATE)
    }
    /// Mask (disable) PLL output.
    #[inline]
    pub const fn mask_pll_output(self) -> Self {
        Self(self.0 & !Self::PLL_OUTPUT_GATE)
    }
    /// Get if PLL output is unmasked.
    #[inline]
    pub const fn is_pll_output_unmasked(self) -> bool {
        self.0 & Self::PLL_OUTPUT_GATE != 0
    }
    /// Get PLL N factor.
    #[inline]
    pub const fn pll_n(self) -> u8 {
        ((self.0 & Self::PLL_N) >> 8) as u8
    }
    /// Set PLL N factor.
    #[inline]
    pub const fn set_pll_n(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_N) | ((val as u32) << 8))
    }
    /// Get PLL M1 factor.
    #[inline]
    pub const fn pll_m1(self) -> u8 {
        ((self.0 & Self::PLL_M1) >> 1) as u8
    }
    /// Set PLL M1 factor.
    #[inline]
    pub const fn set_pll_m1(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_M1) | ((val as u32) << 1))
    }
    /// Get PLL M0 factor.
    #[inline]
    pub const fn pll_m0(self) -> u8 {
        (self.0 & Self::PLL_M0) as u8
    }
    /// Set PLL M0 factor.
    #[inline]
    pub const fn set_pll_m0(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_M0) | val as u32)
    }
}

// TODO: default value for PllDdrControl is 0x4800_2301

/// Peripheral PLL Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllPeri0Control(u32);

impl PllPeri0Control {
    const PLL_ENABLE: u32 = 1 << 31;
    const PLL_LDO_ENABLE: u32 = 1 << 30;
    const LOCK_ENABLE: u32 = 1 << 29;
    const LOCK: u32 = 1 << 28;
    const PLL_OUTPUT_GATE: u32 = 1 << 27;
    const PLL_P1: u32 = 0x07 << 20;
    const PLL_P0: u32 = 0x07 << 16;
    const PLL_N: u32 = 0xff << 8;
    const PLL_M: u32 = 0x1 << 1;

    /// Get if PLL is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        self.0 & Self::PLL_ENABLE != 0
    }
    /// Enable PLL.
    #[inline]
    pub const fn enable_pll(self) -> Self {
        Self(self.0 | Self::PLL_ENABLE)
    }
    /// Disable PLL.
    #[inline]
    pub const fn disable_pll(self) -> Self {
        Self(self.0 & !Self::PLL_ENABLE)
    }
    /// Get if PLL LDO is enabled.
    #[inline]
    pub const fn is_pll_ldo_enabled(self) -> bool {
        self.0 & Self::PLL_LDO_ENABLE != 0
    }
    /// Enable PLL LDO.
    #[inline]
    pub const fn enable_pll_ldo(self) -> Self {
        Self(self.0 | Self::PLL_LDO_ENABLE)
    }
    /// Disable PLL LDO.
    #[inline]
    pub const fn disable_pll_ldo(self) -> Self {
        Self(self.0 & !Self::PLL_LDO_ENABLE)
    }
    /// Get if PLL lock is enabled.
    #[inline]
    pub const fn is_lock_enabled(self) -> bool {
        self.0 & Self::LOCK_ENABLE != 0
    }
    /// Enable PLL lock.
    #[inline]
    pub const fn enable_lock(self) -> Self {
        Self(self.0 | Self::LOCK_ENABLE)
    }
    /// Disable PLL lock.
    #[inline]
    pub const fn disable_lock(self) -> Self {
        Self(self.0 & !Self::LOCK_ENABLE)
    }
    /// Get if the PLL locked state is set by hardware.
    #[inline]
    pub const fn is_locked(self) -> bool {
        self.0 & Self::LOCK != 0
    }
    /// Unmask (enable) PLL output.
    pub const fn unmask_pll_output(self) -> Self {
        Self(self.0 | Self::PLL_OUTPUT_GATE)
    }
    /// Mask (disable) PLL output.
    #[inline]
    pub const fn mask_pll_output(self) -> Self {
        Self(self.0 & !Self::PLL_OUTPUT_GATE)
    }
    /// Get if PLL output is unmasked.
    #[inline]
    pub const fn is_pll_output_unmasked(self) -> bool {
        self.0 & Self::PLL_OUTPUT_GATE != 0
    }
    /// Get PLL P1 factor.
    #[inline]
    pub const fn pll_p1(self) -> u8 {
        ((self.0 & Self::PLL_P1) >> 20) as u8
    }
    /// Set PLL P1 factor.
    #[inline]
    pub const fn set_pll_p1(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_P1) | ((val as u32) << 20))
    }
    /// Get PLL P0 factor.
    #[inline]
    pub const fn pll_p0(self) -> u8 {
        ((self.0 & Self::PLL_P0) >> 16) as u8
    }
    /// Set PLL P0 factor.
    #[inline]
    pub const fn set_pll_p0(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_P0) | ((val as u32) << 16))
    }
    /// Get PLL N factor.
    #[inline]
    pub const fn pll_n(self) -> u8 {
        ((self.0 & Self::PLL_N) >> 8) as u8
    }
    /// Set PLL N factor.
    #[inline]
    pub const fn set_pll_n(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_N) | ((val as u32) << 8))
    }
    /// Get PLL M factor.
    #[inline]
    pub const fn pll_m(self) -> u8 {
        ((self.0 & Self::PLL_M) >> 1) as u8
    }
    /// Set PLL M factor.
    #[inline]
    pub const fn set_pll_m(self, val: u8) -> Self {
        Self((self.0 & !Self::PLL_M) | ((val as u32) << 1))
    }
}

// TODO: default value for PllPeriControl is 0x4821_6300

#[cfg(test)]
mod tests {
    use super::{PllCpuControl, PllDdrControl, PllPeri0Control};

    #[test]
    fn struct_pll_cpu_control_functions() {
        let mut val = PllCpuControl(0x0);

        val = val.enable_pll();
        assert_eq!(val.0, 0x80000000);
        assert!(val.is_pll_enabled());

        val = val.disable_pll();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_enabled());

        val = val.enable_pll_ldo();
        assert_eq!(val.0, 0x40000000);
        assert!(val.is_pll_ldo_enabled());

        val = val.disable_pll_ldo();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_ldo_enabled());

        val = val.enable_lock();
        assert_eq!(val.0, 0x20000000);
        assert!(val.is_lock_enabled());

        val = val.disable_lock();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_lock_enabled());

        let val = PllCpuControl(0x10000000);
        assert!(val.is_locked());
        let val = PllCpuControl(0x0);
        assert!(!val.is_locked());

        let mut val = PllCpuControl(0x0);

        val = val.unmask_pll_output();
        assert_eq!(val.0, 0x08000000);
        assert!(val.is_pll_output_unmasked());

        val = val.mask_pll_output();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_output_unmasked());

        val = val.set_pll_n(0xFF);
        assert_eq!(val.0, 0x0000FF00);
        assert_eq!(val.pll_n(), 0xFF);

        val = val.set_pll_n(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_n(), 0x0);

        val = val.set_pll_m(0x03);
        assert_eq!(val.0, 0x00000003);
        assert_eq!(val.pll_m(), 0x03);

        val = val.set_pll_m(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_m(), 0x0);

        let default = PllCpuControl::default();
        assert!(!default.is_pll_enabled());
        assert!(default.is_pll_ldo_enabled());
        assert!(!default.is_lock_enabled());
        assert!(!default.is_locked());
        assert!(default.is_pll_output_unmasked());
        assert_eq!(default.pll_n(), 0x10);
        assert_eq!(default.pll_m(), 0x0);
    }

    #[test]
    fn struct_pll_ddr_control_functions() {
        let mut val = PllDdrControl(0x0);

        val = val.enable_pll();
        assert_eq!(val.0, 0x80000000);
        assert!(val.is_pll_enabled());

        val = val.disable_pll();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_enabled());

        val = val.enable_pll_ldo();
        assert_eq!(val.0, 0x40000000);
        assert!(val.is_pll_ldo_enabled());

        val = val.disable_pll_ldo();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_ldo_enabled());

        val = val.enable_lock();
        assert_eq!(val.0, 0x20000000);
        assert!(val.is_lock_enabled());

        val = val.disable_lock();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_lock_enabled());

        let val = PllDdrControl(0x10000000);
        assert!(val.is_locked());
        let val = PllDdrControl(0x0);
        assert!(!val.is_locked());

        let mut val = PllDdrControl(0x0);

        val = val.unmask_pll_output();
        assert_eq!(val.0, 0x08000000);
        assert!(val.is_pll_output_unmasked());

        val = val.mask_pll_output();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_output_unmasked());

        val = val.set_pll_n(0xFF);
        assert_eq!(val.0, 0x0000FF00);
        assert_eq!(val.pll_n(), 0xFF);

        val = val.set_pll_n(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_n(), 0x0);

        val = val.set_pll_m1(0x01);
        assert_eq!(val.0, 0x00000002);
        assert_eq!(val.pll_m1(), 0x01);

        val = val.set_pll_m1(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_m1(), 0x0);

        val = val.set_pll_m0(0x01);
        assert_eq!(val.0, 0x00000001);
        assert_eq!(val.pll_m0(), 0x01);

        val = val.set_pll_m0(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_m0(), 0x0);
    }

    #[test]
    fn struct_pll_peri0_control_functions() {
        let mut val = PllPeri0Control(0x0);

        val = val.enable_pll();
        assert_eq!(val.0, 0x80000000);
        assert!(val.is_pll_enabled());

        val = val.disable_pll();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_enabled());

        val = val.enable_pll_ldo();
        assert_eq!(val.0, 0x40000000);
        assert!(val.is_pll_ldo_enabled());

        val = val.disable_pll_ldo();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_ldo_enabled());

        val = val.enable_lock();
        assert_eq!(val.0, 0x20000000);
        assert!(val.is_lock_enabled());

        val = val.disable_lock();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_lock_enabled());

        let val = PllPeri0Control(0x10000000);
        assert!(val.is_locked());
        let val = PllPeri0Control(0x0);
        assert!(!val.is_locked());

        let mut val = PllPeri0Control(0x0);

        val = val.unmask_pll_output();
        assert_eq!(val.0, 0x08000000);
        assert!(val.is_pll_output_unmasked());

        val = val.mask_pll_output();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_pll_output_unmasked());

        val = val.set_pll_p1(0x07);
        assert_eq!(val.0, 0x00700000);
        assert_eq!(val.pll_p1(), 0x07);

        val = val.set_pll_p1(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_p1(), 0x0);

        val = val.set_pll_p0(0x07);
        assert_eq!(val.0, 0x00070000);
        assert_eq!(val.pll_p0(), 0x07);

        val = val.set_pll_p0(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_p0(), 0x0);

        val = val.set_pll_n(0xFF);
        assert_eq!(val.0, 0x0000FF00);
        assert_eq!(val.pll_n(), 0xFF);

        val = val.set_pll_n(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_n(), 0x0);

        val = val.set_pll_m(0x01);
        assert_eq!(val.0, 0x00000002);
        assert_eq!(val.pll_m(), 0x01);

        val = val.set_pll_m(0x0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pll_m(), 0x0);
    }
}
