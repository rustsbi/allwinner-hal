//! V861 E907 runtime peripherals.
//!
//! Clock helpers assume the 24 MHz HOSC used by Avaota F2. Frequencies are
//! snapshots: keep the clock tree unchanged while using a returned UART clock
//! or cycle-based delay. Only the E907 hart may access these peripherals.
//! CCU fields follow the vendor sun252iw1 register definitions referenced in
//! `examples/v861-avaota-f2/README.md`.

use allwinner_hal::{
    gpio::PadExt,
    uart::UartExt,
    usb::{
        Instance as UsbInstance,
        phy::v2::{Instance as UsbPhyInstance, Oscillator},
    },
};
use embedded_hal::delay::DelayNs;
use embedded_time::rate::{Extensions, Hertz};

/// Singleton peripherals available to the E907 SRAM runtime.
pub struct Peripherals {
    /// GPIO pads.
    pub gpio: Pads,
    /// Clock controller.
    pub ccu: CCU,
    /// UART0.
    pub uart0: UART0,
    /// USB OTG controller.
    pub usb0: USB0,
    /// Independent USB PHY.
    pub usb_phy0: USB_PHY0,
}

/// Clock configuration inherited from the BootROM.
pub struct Clocks;

const HOSC_HZ: u32 = 24_000_000;
const USB_PHY_RESET_RELEASE: u32 = 1 << 30;
const UART_SOURCE_AND_DIVIDERS: u32 = (7 << 24) | (3 << 8) | 0x1f;
const PLL_ENABLE: u32 = 1 << 31;
const PLL_OUTPUT_GATE: u32 = 1 << 27;

/// Reset the SoC into FEL, including USB re-enumeration on the host.
///
/// Unlike returning from a FEL call, this also recovers the USB controller
/// after firmware has replaced the ROM's USB device configuration.
pub fn reset_to_fel() -> ! {
    // SAFETY: the V861 runtime owns the SoC. These are the RTC boot-mode and
    // CPU watchdog registers, following the same sequence as rfel's V861
    // reset implementation. The key fields authorize these register writes.
    unsafe {
        let boot_mode = 0x0709_02a0 as *mut u32;
        boot_mode.write_volatile(boot_mode.read_volatile() | (1 << 1) | (0x429b << 16));
        let watchdog = 0x0800_9018 as *mut u32;
        watchdog.write_volatile(0x16aa << 16);
        watchdog.write_volatile((0x16aa << 16) | 0x21);
    }
    loop {
        core::hint::spin_loop();
    }
}

impl Clocks {
    /// Reset USB0 and its PHY while both peripheral tokens are idle.
    /// The board supplies a 24 MHz oscillator.
    pub fn enable_usb(
        &self,
        _usb: &mut USB0,
        _phy: &mut USB_PHY0,
        ccu: &mut CCU,
        delay: &mut impl DelayNs,
    ) -> Oscillator {
        // SAFETY: exclusive clock and peripheral borrows serialize reset and
        // gate changes. USB OTG uses BGR gate 8 / reset 24, PHY reset bit 30.
        unsafe {
            ccu.usb_bgr
                .modify(|v| v.gate_mask::<8>().assert_reset::<8>());
            ccu.usb0_clk.modify(|v| v & !USB_PHY_RESET_RELEASE);
        }
        delay.delay_us(20);
        // SAFETY: the same exclusive borrows remain live through clock setup.
        unsafe {
            ccu.usb0_clk.modify(|v| v | USB_PHY_RESET_RELEASE);
        }
        delay.delay_us(50);
        // SAFETY: release the OTG controller only after the PHY reset delay.
        unsafe {
            ccu.usb_bgr
                .modify(|v| v.deassert_reset::<8>().gate_pass::<8>());
        }
        delay.delay_us(100);
        Oscillator::Mhz24
    }

    /// Enable UART0 with the 24 MHz oscillator and no APB division.
    pub fn enable_uart(&self, ccu: &mut CCU) -> UartClock {
        // SAFETY: the caller exclusively borrows the UART and CCU tokens.
        // These fields select HOSC, N=1, M=1, then release UART0 and its gate.
        unsafe {
            ccu.apb_uart_clk.modify(|v| v & !UART_SOURCE_AND_DIVIDERS);
            ccu.uart_bgr
                .modify(|v| v.deassert_reset::<0>().gate_pass::<0>());
        }
        UartClock
    }

    /// E907 cycle frequency for the BootROM HOSC or peripheral-PLL clocks.
    ///
    /// Returns `None` when the selected PLL is disabled or gated, the result
    /// cannot fit in `u32`, or the parent is not supported. CLK32K, the internal
    /// RC oscillator, CPU PLL and reserved selectors are not decoded here.
    /// AXI division affects the bus, not the E907 cycle counter.
    pub fn mcycle_ticks_second(&self, ccu: &CCU) -> Option<u32> {
        let clock = ccu.e907_clk.read();
        let pll = match (clock >> 24) & 7 {
            3 | 4 => ccu.pll_peri_ctrl.read(),
            _ => 0,
        };
        e907_frequency(clock, pll)
    }
}

fn e907_frequency(clock: u32, pll: u32) -> Option<u32> {
    let source = (clock >> 24) & 7;
    let core_divisor = u64::from((clock & 0x1f) + 1);
    let frequency = match source {
        0 => u64::from(HOSC_HZ) / core_divisor,
        3 | 4 => {
            let enabled = PLL_ENABLE | PLL_OUTPUT_GATE;
            if pll & enabled != enabled {
                return None;
            }
            let n = u64::from(((pll >> 8) & 0xff) + 1);
            let m = u64::from(((pll >> 1) & 1) + 1);
            // PERI_600M = PLL / P0 / 2; PERI_800M = PLL / P1.
            let (p_shift, output_divisor) = if source == 3 { (16, 2) } else { (20, 1) };
            let p = u64::from(((pll >> p_shift) & 7) + 1);
            u64::from(HOSC_HZ) * n / (m * p * output_divisor * core_divisor)
        }
        _ => return None,
    };
    u32::try_from(frequency).ok()
}

/// UART0's 24 MHz APB clock.
pub struct UartClock;

impl allwinner_hal::uart::Clock<0> for UartClock {
    fn uart_clock(&self) -> Hertz {
        HOSC_HZ.Hz()
    }
}

impl allwinner_hal::uart::Clock<0> for &UartClock {
    fn uart_clock(&self) -> Hertz {
        HOSC_HZ.Hz()
    }
}

/// Construct the singleton runtime parameters.
///
/// # Safety
/// Call at most once per firmware execution, on the sole active E907 hart.
#[doc(hidden)]
pub unsafe fn __rom_init_params() -> (Peripherals, Clocks) {
    (
        Peripherals {
            gpio: Pads::__new(),
            ccu: CCU { _private: () },
            uart0: UART0 { _private: () },
            usb0: USB0 { _private: () },
            usb_phy0: USB_PHY0 { _private: () },
        },
        Clocks,
    )
}

soc! {
    /// Clock Control Unit peripheral.
    pub struct CCU => 0x0200_1000, allwinner_hal::ccu::v861::RegisterBlock;
    /// General Purpose Input/Output peripheral.
    pub struct GPIO => 0x0200_0000, allwinner_hal::gpio::v2::RegisterBlockV2;
    /// UART0.
    pub struct UART0 => 0x0250_0000, allwinner_hal::uart::RegisterBlock;
    /// USB OTG device controller.
    pub struct USB0 => 0x0410_0000, allwinner_hal::usb::UsbRegisterBlock;
    /// USB PHY and interface controls.
    pub struct USB_PHY0 => 0x0410_0400, allwinner_hal::usb::phy_v2::RegisterBlockV2;
}

impl_uart! { 0 => UART0, }
impl_uart_pads! {
    ('H', 9, 5): IntoTransmit, into_uart_transmit, 0;
    ('H', 10, 5): IntoReceive, into_uart_receive, 0;
}

// SAFETY: the sole runtime token owns the aligned V861 USB controller mapping.
unsafe impl UsbInstance<'static> for USB0 {
    fn register_block(self) -> &'static allwinner_hal::usb::UsbRegisterBlock {
        // SAFETY: consuming this token transfers exclusive controller access.
        unsafe { &*Self::ptr() }
    }
}

// SAFETY: this mutable borrow prevents another safe controller owner for 'a.
unsafe impl<'a> UsbInstance<'a> for &'a mut USB0 {
    #[inline]
    fn register_block(self) -> &'a allwinner_hal::usb::UsbRegisterBlock {
        // SAFETY: the singleton token remains exclusively borrowed for 'a.
        unsafe { &*USB0::ptr() }
    }
}

/// Ownership of a V861 GPIO pad.
pub struct Pad<const P: char, const N: u8> {
    _private: (),
}

impl<const P: char, const N: u8> Pad<P, N> {
    /// Macro internal constructor.
    #[doc(hidden)]
    #[inline]
    pub const fn __new() -> Self {
        Self { _private: () }
    }
}

impl_gpio_pins! {
    __new_v2;
    pa0: ('A', 0);
    pa1: ('A', 1);
    pa2: ('A', 2);
    pa3: ('A', 3);
    pa4: ('A', 4);
    pa5: ('A', 5);
    pa6: ('A', 6);
    pa7: ('A', 7);
    pa8: ('A', 8);
    pa9: ('A', 9);
    pa10: ('A', 10);
    pa11: ('A', 11);
    pa12: ('A', 12);
    pa13: ('A', 13);
    pa14: ('A', 14);
    pa15: ('A', 15);
    pa16: ('A', 16);
    pa17: ('A', 17);
    pa18: ('A', 18);
    pa19: ('A', 19);
    pa20: ('A', 20);
    pa21: ('A', 21);
    pc0: ('C', 0);
    pc1: ('C', 1);
    pc2: ('C', 2);
    pc3: ('C', 3);
    pc4: ('C', 4);
    pc5: ('C', 5);
    pc6: ('C', 6);
    pc7: ('C', 7);
    pc8: ('C', 8);
    pc9: ('C', 9);
    pc10: ('C', 10);
    pc11: ('C', 11);
    pd0: ('D', 0);
    pd1: ('D', 1);
    pd2: ('D', 2);
    pd3: ('D', 3);
    pd4: ('D', 4);
    pd5: ('D', 5);
    pd6: ('D', 6);
    pd7: ('D', 7);
    pd8: ('D', 8);
    pd9: ('D', 9);
    pd10: ('D', 10);
    pd11: ('D', 11);
    pd12: ('D', 12);
    pd13: ('D', 13);
    pd14: ('D', 14);
    pd15: ('D', 15);
    pd16: ('D', 16);
    pd17: ('D', 17);
    pd18: ('D', 18);
    pd19: ('D', 19);
    pd20: ('D', 20);
    pd21: ('D', 21);
    pd22: ('D', 22);
    pe0: ('E', 0);
    pe1: ('E', 1);
    pe2: ('E', 2);
    pe3: ('E', 3);
    pe4: ('E', 4);
    pe5: ('E', 5);
    pe6: ('E', 6);
    pe7: ('E', 7);
    pe8: ('E', 8);
    pe9: ('E', 9);
    pe10: ('E', 10);
    pe11: ('E', 11);
    pe12: ('E', 12);
    pe13: ('E', 13);
    pe14: ('E', 14);
    pe15: ('E', 15);
    pe16: ('E', 16);
    pe17: ('E', 17);
    pf0: ('F', 0);
    pf1: ('F', 1);
    pf2: ('F', 2);
    pf3: ('F', 3);
    pf4: ('F', 4);
    pf5: ('F', 5);
    pf6: ('F', 6);
    pg0: ('G', 0);
    pg1: ('G', 1);
    pg2: ('G', 2);
    pg3: ('G', 3);
    pg4: ('G', 4);
    pg5: ('G', 5);
    pg6: ('G', 6);
    pg7: ('G', 7);
    ph0: ('H', 0);
    ph1: ('H', 1);
    ph2: ('H', 2);
    ph3: ('H', 3);
    ph4: ('H', 4);
    ph5: ('H', 5);
    ph6: ('H', 6);
    ph7: ('H', 7);
    ph8: ('H', 8);
    ph9: ('H', 9);
    ph10: ('H', 10);
    ph11: ('H', 11);
    ph12: ('H', 12);
    ph13: ('H', 13);
    ph14: ('H', 14);
    ph15: ('H', 15);
    pi0: ('I', 0);
    pi1: ('I', 1);
    pi2: ('I', 2);
    pi3: ('I', 3);
    pi4: ('I', 4);
    pl0: ('L', 0);
    pl1: ('L', 1);
    pl2: ('L', 2);
    pl3: ('L', 3);
    pl4: ('L', 4);
    pl5: ('L', 5);
}

// SAFETY: the runtime constructs exactly one `USB_PHY0` token in
// `__rom_init_params`; it owns the independent V861 USB PHY mapping paired with
// USB0 and uses the verified version 2 register layout.
unsafe impl UsbPhyInstance<'static> for USB_PHY0 {
    #[inline]
    fn register_block(self) -> &'static allwinner_hal::usb::phy_v2::RegisterBlockV2 {
        // SAFETY: consuming the sole runtime token grants exclusive access.
        unsafe { &*Self::ptr() }
    }
}

// SAFETY: the mutable token borrow remains active for the full returned MMIO
// capability lifetime, preventing another safe PHY construction.
unsafe impl<'a> UsbPhyInstance<'a> for &'a mut USB_PHY0 {
    #[inline]
    fn register_block(self) -> &'a allwinner_hal::usb::phy_v2::RegisterBlockV2 {
        // SAFETY: `self` is the unique mutable borrow of the singleton token.
        unsafe { &*USB_PHY0::ptr() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Register snapshot from the connected V861 in FEL (600 MHz E907).
    const FEL_PLL: u32 = 0xf821_6310;

    #[test]
    fn hosc_uses_core_divisor_not_axi_divisor() {
        assert_eq!(e907_frequency(0, 0), Some(24_000_000));
        assert_eq!(e907_frequency(0x301, 0), Some(12_000_000));
        assert_eq!(e907_frequency(31, 0), Some(750_000));
    }

    #[test]
    fn peripheral_pll_outputs_and_dividers() {
        assert_eq!(e907_frequency(0x0300_0100, FEL_PLL), Some(600_000_000));
        assert_eq!(e907_frequency(0x0400_0000, FEL_PLL), Some(800_000_000));
        assert_eq!(e907_frequency(0x0300_0002, FEL_PLL), Some(200_000_000));
        assert_eq!(e907_frequency(0x0400_0003, FEL_PLL | 2), Some(100_000_000));
    }

    #[test]
    fn disabled_or_gated_pll_has_no_frequency() {
        for source in [3, 4] {
            assert_eq!(e907_frequency(source << 24, FEL_PLL & !PLL_ENABLE), None);
            assert_eq!(
                e907_frequency(source << 24, FEL_PLL & !PLL_OUTPUT_GATE),
                None
            );
        }
    }

    #[test]
    fn unsupported_parents_are_not_guessed() {
        for source in [1, 2, 5, 6, 7] {
            assert_eq!(e907_frequency(source << 24, FEL_PLL), None);
        }
    }

    #[test]
    fn wide_intermediate_is_divided_before_narrowing() {
        // Arithmetic boundary inputs, not recommended operating frequencies.
        let pll = PLL_ENABLE | PLL_OUTPUT_GATE | (255 << 8);
        assert_eq!(e907_frequency(0x0400_0000, pll), None);
        assert_eq!(e907_frequency(0x0400_0001, pll), Some(3_072_000_000));
    }
}
