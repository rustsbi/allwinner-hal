//! V821 APP and always-on Clock Control Unit registers.

use volatile_register::{RO, RW};

/// V821 application-domain CCU registers used for peripheral bus control.
#[repr(C)]
pub struct AppRegisterBlock {
    _reserved_000: [u8; 0x7c],
    /// 0x07c - USB reference clock register.
    pub usb_clock: RW<UsbClock>,
    /// 0x080 - application bus clock gating register 0.
    pub bus_clock_gating0: RW<BusClockGating0>,
    _reserved_084: [u8; 0x0c],
    /// 0x090 - application bus reset register 0.
    pub bus_reset0: RW<BusReset0>,
}

/// USB reference clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbClock(u32);

impl UsbClock {
    const CLOCK_GATE: u32 = 1 << 3;

    /// Enable the USB reference clock.
    #[inline]
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::CLOCK_GATE)
    }

    /// Disable the USB reference clock.
    #[inline]
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::CLOCK_GATE)
    }

    /// Returns whether the USB reference clock is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::CLOCK_GATE != 0
    }
}

/// V821 always-on CCU registers used by the special APB clock domain.
#[repr(C)]
pub struct AonRegisterBlock {
    _reserved_000: [u8; 0x404],
    /// 0x404 - DCXO clock source status register.
    pub dcxo_status: RO<DcxoStatus>,
    _reserved_408: [u8; 0x178],
    /// 0x580 - special APB clock configuration register.
    pub apb_special_clock: RW<ApbSpecialClock>,
}

/// DCXO clock source status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DcxoStatus(u32);

impl DcxoStatus {
    const CLOCK_24_MHZ: u32 = 1 << 31;

    /// Returns whether the selected DCXO frequency is 24 MHz rather than 40 MHz.
    #[inline]
    pub const fn is_24_mhz(self) -> bool {
        self.0 & Self::CLOCK_24_MHZ != 0
    }
}

/// Application bus clock gating register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BusClockGating0(u32);

impl BusClockGating0 {
    const UART_SHIFT: usize = 15;
    const UART_COUNT: usize = 4;
    const USB_HCLK: u32 = 1 << 19;
    const USB_OTG: u32 = 1 << 20;

    const fn uart_mask<const I: usize>() -> u32 {
        assert!(I < Self::UART_COUNT);
        1 << (Self::UART_SHIFT + I)
    }

    /// Disable the peripheral clock gate for UART `I`.
    #[inline]
    pub const fn gate_mask<const I: usize>(self) -> Self {
        Self(self.0 & !Self::uart_mask::<I>())
    }

    /// Enable the peripheral clock gate for UART `I`.
    #[inline]
    pub const fn gate_pass<const I: usize>(self) -> Self {
        Self(self.0 | Self::uart_mask::<I>())
    }

    /// Returns whether the peripheral clock gate for UART `I` is enabled.
    #[inline]
    pub const fn is_gate_passed<const I: usize>(self) -> bool {
        self.0 & Self::uart_mask::<I>() != 0
    }

    /// Disable the USB HCLK gate.
    #[inline]
    pub const fn mask_usb_hclk(self) -> Self {
        Self(self.0 & !Self::USB_HCLK)
    }

    /// Enable the USB HCLK gate.
    #[inline]
    pub const fn pass_usb_hclk(self) -> Self {
        Self(self.0 | Self::USB_HCLK)
    }

    /// Disable the USB OTG bus clock gate.
    #[inline]
    pub const fn mask_usb_otg(self) -> Self {
        Self(self.0 & !Self::USB_OTG)
    }

    /// Enable the USB OTG bus clock gate.
    #[inline]
    pub const fn pass_usb_otg(self) -> Self {
        Self(self.0 | Self::USB_OTG)
    }
}

/// Application bus reset register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BusReset0(u32);

impl BusReset0 {
    const UART_SHIFT: usize = 15;
    const UART_COUNT: usize = 4;
    const USB_HCLK: u32 = 1 << 19;
    const USB_OTG: u32 = 1 << 20;
    const USB_PHY: u32 = 1 << 23;

    const fn uart_mask<const I: usize>() -> u32 {
        assert!(I < Self::UART_COUNT);
        1 << (Self::UART_SHIFT + I)
    }

    /// Assert the peripheral reset signal for UART `I`.
    #[inline]
    pub const fn assert_reset<const I: usize>(self) -> Self {
        Self(self.0 & !Self::uart_mask::<I>())
    }

    /// Deassert the peripheral reset signal for UART `I`.
    #[inline]
    pub const fn deassert_reset<const I: usize>(self) -> Self {
        Self(self.0 | Self::uart_mask::<I>())
    }

    /// Returns whether the peripheral reset signal for UART `I` is asserted.
    #[inline]
    pub const fn is_reset_asserted<const I: usize>(self) -> bool {
        self.0 & Self::uart_mask::<I>() == 0
    }

    /// Assert the USB HCLK reset signal.
    #[inline]
    pub const fn assert_usb_hclk(self) -> Self {
        Self(self.0 & !Self::USB_HCLK)
    }

    /// Deassert the USB HCLK reset signal.
    #[inline]
    pub const fn deassert_usb_hclk(self) -> Self {
        Self(self.0 | Self::USB_HCLK)
    }

    /// Assert the USB OTG reset signal.
    #[inline]
    pub const fn assert_usb_otg(self) -> Self {
        Self(self.0 & !Self::USB_OTG)
    }

    /// Deassert the USB OTG reset signal.
    #[inline]
    pub const fn deassert_usb_otg(self) -> Self {
        Self(self.0 | Self::USB_OTG)
    }

    /// Assert the USB PHY reset signal.
    #[inline]
    pub const fn assert_usb_phy(self) -> Self {
        Self(self.0 & !Self::USB_PHY)
    }

    /// Deassert the USB PHY reset signal.
    #[inline]
    pub const fn deassert_usb_phy(self) -> Self {
        Self(self.0 | Self::USB_PHY)
    }
}

/// Clock source for the V821 special APB domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ApbSpecialClockSource {
    /// High-speed oscillator.
    Hosc = 0,
    /// System 32 kHz clock; marked unused by the vendor definition.
    Sys32K = 1,
    /// Internal 1 MHz RC oscillator.
    Rc1M = 2,
    /// 192 MHz peripheral PLL output.
    Peri192M = 3,
}

/// Special APB clock configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ApbSpecialClock(u32);

impl ApbSpecialClock {
    const CLOCK_SOURCE: u32 = 0x3 << 24;
    const DIVIDER: u32 = 0x1f;

    /// Return the selected clock source.
    #[inline]
    pub const fn clock_source(self) -> ApbSpecialClockSource {
        match (self.0 & Self::CLOCK_SOURCE) >> 24 {
            0 => ApbSpecialClockSource::Hosc,
            1 => ApbSpecialClockSource::Sys32K,
            2 => ApbSpecialClockSource::Rc1M,
            3 => ApbSpecialClockSource::Peri192M,
            _ => unreachable!(),
        }
    }

    /// Select a clock source while preserving unrelated fields.
    #[inline]
    pub const fn set_clock_source(self, source: ApbSpecialClockSource) -> Self {
        Self((self.0 & !Self::CLOCK_SOURCE) | ((source as u32) << 24))
    }

    /// Return the clock divisor in the range 1 through 32.
    #[inline]
    pub const fn divisor(self) -> u8 {
        ((self.0 & Self::DIVIDER) + 1) as u8
    }

    /// Set the clock divisor in the range 1 through 32.
    #[inline]
    pub const fn set_divisor(self, divisor: u8) -> Self {
        assert!(divisor >= 1 && divisor <= 32);
        Self((self.0 & !Self::DIVIDER) | (divisor - 1) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(AppRegisterBlock, usb_clock), 0x7c);
        assert_eq!(offset_of!(AppRegisterBlock, bus_clock_gating0), 0x80);
        assert_eq!(offset_of!(AppRegisterBlock, bus_reset0), 0x90);
        assert_eq!(size_of::<AppRegisterBlock>(), 0x94);
        assert_eq!(offset_of!(AonRegisterBlock, dcxo_status), 0x404);
        assert_eq!(offset_of!(AonRegisterBlock, apb_special_clock), 0x580);
        assert_eq!(size_of::<AonRegisterBlock>(), 0x584);
    }

    #[test]
    fn dcxo_frequency_status() {
        assert!(!DcxoStatus(0).is_24_mhz());
        assert!(DcxoStatus(1 << 31).is_24_mhz());
    }

    #[test]
    fn uart_gate_and_reset_fields() {
        let gate = BusClockGating0(0xa5a5_0000).gate_pass::<0>();
        assert_eq!(gate.0, 0xa5a5_8000);
        assert!(gate.is_gate_passed::<0>());
        assert_eq!(gate.gate_mask::<0>().0, 0xa5a5_0000);

        let reset = BusReset0(0xffff_ffff).assert_reset::<3>();
        assert_eq!(reset.0, 0xfffb_ffff);
        assert!(reset.is_reset_asserted::<3>());
        assert_eq!(reset.deassert_reset::<3>().0, 0xffff_ffff);
    }

    #[test]
    fn usb_clock_gate_and_reset_fields() {
        let clock = UsbClock(0).enable();
        assert!(clock.is_enabled());
        assert_eq!(clock.disable().0, 0);

        let gate = BusClockGating0(u32::MAX).mask_usb_hclk().mask_usb_otg();
        assert_eq!(gate.0 & ((1 << 19) | (1 << 20)), 0);
        let gate = gate.pass_usb_hclk().pass_usb_otg();
        assert_eq!(gate.0, u32::MAX);

        let reset = BusReset0(u32::MAX)
            .assert_usb_hclk()
            .assert_usb_otg()
            .assert_usb_phy();
        assert_eq!(reset.0 & ((1 << 19) | (1 << 20) | (1 << 23)), 0);
        let reset = reset
            .deassert_usb_hclk()
            .deassert_usb_otg()
            .deassert_usb_phy();
        assert_eq!(reset.0, u32::MAX);
    }

    #[test]
    fn apb_special_clock_fields() {
        let clock = ApbSpecialClock(0xffff_ffff)
            .set_clock_source(ApbSpecialClockSource::Hosc)
            .set_divisor(1);
        assert_eq!(clock.0, 0xfcff_ffe0);
        assert_eq!(clock.clock_source(), ApbSpecialClockSource::Hosc);
        assert_eq!(clock.divisor(), 1);

        let clock = clock
            .set_clock_source(ApbSpecialClockSource::Peri192M)
            .set_divisor(32);
        assert_eq!(clock.clock_source(), ApbSpecialClockSource::Peri192M);
        assert_eq!(clock.divisor(), 32);
        assert_eq!(clock.0 & 0x0300_001f, 0x0300_001f);
    }
}
