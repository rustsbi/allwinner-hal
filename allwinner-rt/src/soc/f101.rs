//! F101 chip platform.
//!
//! The SRAM runtime uses the C907 in the BootROM's RV32 machine mode.
//! Clock helpers assume a 24 MHz HOSC. Keep the clock tree unchanged while
//! using a cycle-based delay. The linker reserves the USB FIFO and ROM SRAM.

use allwinner_hal::usb::{
    Instance as UsbInstance,
    phy::v2::{Instance as UsbPhyInstance, Oscillator},
};
use allwinner_hal::{gpio::PadExt, uart::UartExt};
use embedded_hal::delay::DelayNs;
use embedded_time::rate::{Extensions, Hertz};

/// Singleton peripherals for the F101 SRAM runtime.
pub struct Peripherals {
    /// GPIO pads.
    pub gpio: Pads,
    /// Clock controller.
    pub ccu: CCU,
    /// SRAM mapping controller.
    pub sysctl: SYSCTL,
    /// UART1, available on the Yuzuki Neko headers.
    pub uart1: UART1,
    /// USB OTG controller.
    pub usb0: USB0,
    /// Independent USB PHY.
    pub usb_phy0: USB_PHY0,
}

/// Clock configuration inherited from the BootROM.
pub struct Clocks;

const HOSC_HZ: u32 = 24_000_000;
const PLL_ENABLE: u32 = 1 << 31;
const PLL_OUTPUT_GATE: u32 = 1 << 27;

impl Clocks {
    /// Release UART1 reset and pass its clock gate.
    ///
    /// This uses the BootROM's inherited 24 MHz UART parent clock.
    pub fn enable_uart(&self, _uart: &mut UART1, ccu: &mut CCU) -> UartClock {
        // SAFETY: exclusive UART and CCU borrows prevent a live driver from
        // racing gate/reset changes. F101 UART1 uses gate 1 and reset 17.
        unsafe {
            ccu.uart_bgr
                .modify(|v| v.deassert_reset::<1>().gate_pass::<1>());
        }
        UartClock
    }

    /// Assign FIFO SRAM, reset USB0 and enable its PHY clock.
    ///
    /// The runtime linker excludes SRAM reserved for USB and the BootROM.
    /// Keep both peripheral tokens idle throughout this operation.
    pub fn enable_usb(
        &self,
        _usb: &mut USB0,
        _phy: &mut USB_PHY0,
        ccu: &mut CCU,
        sysctl: &mut SYSCTL,
        delay: &mut impl DelayNs,
    ) -> Oscillator {
        // SAFETY: exclusive tokens serialize SRAM ownership and clock/reset.
        // The F101 runtime places all CPU allocations below 0x2c000. SYSCTRL
        // bits 25/27 assign the peripheral banks to the USB FIFO; the board
        // definitions put OTG gate/reset at 8/24 and PHY clock/reset at 31/30.
        unsafe {
            ccu.usb_bgr
                .modify(|v| v.gate_mask::<8>().assert_reset::<8>());
            ccu.usb0_clk.modify(|v| v.disable());
            sysctl.sram_remap.modify(|v| v.use_usb_fifo());
        }
        delay.delay_us(20);
        // SAFETY: the controller is still held in reset under exclusive borrows.
        unsafe {
            ccu.usb0_clk.modify(|v| v.enable());
        }
        delay.delay_us(50);
        // SAFETY: release the controller after the PHY clock settles.
        unsafe {
            ccu.usb_bgr
                .modify(|v| v.deassert_reset::<8>().gate_pass::<8>());
        }
        delay.delay_us(100);
        Oscillator::Mhz24
    }

    /// Cycle-counter frequency for HOSC, peripheral PLL and the ROM CPU PLL.
    ///
    /// Returns `None` for disabled/gated PLLs, unsupported parents, or CPU PLL
    /// divider settings other than the ROM's M0=M1=P=1. AXI division does not
    /// divide the CPU cycle counter.
    pub fn mcycle_ticks_second(&self, ccu: &CCU) -> Option<u32> {
        c907_frequency(
            ccu.riscv_clk.read(),
            ccu.pll_cpu_ctrl.read(),
            ccu.pll_peri_ctrl.read(),
        )
    }
}

/// UART1 clock inherited from the BootROM (24 MHz).
pub struct UartClock;

impl allwinner_hal::uart::Clock<1> for UartClock {
    fn uart_clock(&self) -> Hertz {
        HOSC_HZ.Hz()
    }
}

impl allwinner_hal::uart::Clock<1> for &UartClock {
    fn uart_clock(&self) -> Hertz {
        HOSC_HZ.Hz()
    }
}

fn c907_frequency(clock: u32, cpu_pll: u32, peri_pll: u32) -> Option<u32> {
    let divisor = u64::from((clock & 0x1f) + 1);
    let enabled = PLL_ENABLE | PLL_OUTPUT_GATE;
    let frequency = match (clock >> 24) & 7 {
        0 => u64::from(HOSC_HZ),
        3 | 4 => {
            if peri_pll & enabled != enabled {
                return None;
            }
            let n = u64::from(((peri_pll >> 8) & 0xff) + 1);
            let m = u64::from(((peri_pll >> 1) & 1) + 1);
            // F101 selectors: 3 = PERI_800M (P1), 4 = PERI_600M (P0 / 2).
            let (shift, output_divisor) = if (clock >> 24) & 7 == 3 {
                (20, 1)
            } else {
                (16, 2)
            };
            let p = u64::from(((peri_pll >> shift) & 7) + 1);
            u64::from(HOSC_HZ) * n / (m * p * output_divisor)
        }
        5 => {
            if cpu_pll & enabled != enabled || cpu_pll & ((3 << 20) | (7 << 16) | 0xf) != 0 {
                return None;
            }
            // Unlike PLL_PERI, PLL_CPU uses N directly, without adding one.
            let n = (cpu_pll >> 8) & 0xff;
            if n == 0 {
                return None;
            }
            u64::from(HOSC_HZ) * u64::from(n)
        }
        _ => return None,
    };
    u32::try_from(frequency / divisor).ok()
}

/// Construct the singleton runtime parameters.
///
/// # Safety
/// Call once per execution on the sole active C907 hart, using the F101 SRAM
/// linker contract. USB FIFO banks must not contain any live CPU allocations.
#[doc(hidden)]
pub unsafe fn __rom_init_params() -> (Peripherals, Clocks) {
    (
        Peripherals {
            gpio: Pads::__new(),
            ccu: CCU { _private: () },
            sysctl: SYSCTL { _private: () },
            uart1: UART1 { _private: () },
            usb0: USB0 { _private: () },
            usb_phy0: USB_PHY0 { _private: () },
        },
        Clocks,
    )
}

/// Re-enter the BootROM's FEL path and re-enumerate its USB device.
///
/// This abandons the firmware. F101 ROM entry 0x00000020 selects FEL directly,
/// resets its stack and USB state, and bypasses the normal storage boot path.
#[cfg(all(feature = "f101", target_arch = "riscv32"))]
pub fn enter_fel() -> ! {
    // SAFETY: this non-returning handoff runs on the sole F101 hart in RV32
    // machine mode. The ROM dump and saved FEL return PC confirm this entry:
    // instruction addresses use the low ROM window; 0x06000000 is its data alias.
    // Clean dirty SRAM before the ROM invalidates caches and remaps its banks.
    // The warm FEL entry bypasses the cold path's BSS clear at 0x060000ca.
    // Restore the CPU SRAM mapping before clearing that state. Stop polling
    // the USB stack and disconnect it before calling this function.
    unsafe {
        core::arch::asm!(
            "csrci mstatus, 8",
            "csrw mie, zero",
            "li t0, 1 << 22",
            "csrs 0x7c0, t0",
            "fence rw, rw",
            ".word 0x0030000b", // dcache.ciall
            ".word 0x01b0000b", // sync.is
            "csrci 0x7c1, 2",   // disable data cache before clearing ROM BSS
            "li t0, 0x03000004",
            "lw t1, 0(t0)",
            "li t2, 0x0f000000",
            "or t1, t1, t2",
            "sw t1, 0(t0)",
            "fence iorw, iorw",
            "li t0, 0x0002ef00",
            "li t1, 0x0002effc",
            "2:",
            "sw zero, 0(t0)",
            "addi t0, t0, 4",
            "bltu t0, t1, 2b",
            // The ROM powers up the PHY but does not exit VC programming mode.
            // Clear its enable bit as in the F101 USB initialization sequence.
            "li t0, 0x04100410",
            "lw t1, 0(t0)",
            "andi t1, t1, -3",
            "sw t1, 0(t0)",
            "fence rw, rw",
            "fence.i",
            "li t0, 0x00000020",
            "jr t0",
            options(noreturn)
        );
    }
}

soc! {
    /// Clock Control Unit peripheral.
    pub struct CCU => 0x0200_1000, allwinner_hal::ccu::f101::RegisterBlock;
    /// General Purpose Input/Output peripheral for the main GPIO domain.
    pub struct GPIO => 0x0200_0000, allwinner_hal::gpio::v2::RegisterBlockV2;
    /// SRAM mapping controller.
    pub struct SYSCTL => 0x0300_0000, allwinner_hal::sysctl::f101::RegisterBlock;
    /// UART1.
    pub struct UART1 => 0x0250_0400, allwinner_hal::uart::RegisterBlock;
    /// USB OTG controller.
    pub struct USB0 => 0x0410_0000, allwinner_hal::usb::UsbRegisterBlock;
    /// USB PHY and interface controls.
    pub struct USB_PHY0 => 0x0410_0400, allwinner_hal::usb::phy_v2::RegisterBlockV2;
}

impl_uart! { 1 => UART1, }
impl_uart_pads! {
    ('B', 0, 4): IntoTransmit, into_uart_transmit, 1;
    ('B', 1, 4): IntoReceive, into_uart_receive, 1;
}

// SAFETY: the runtime constructs one token for the aligned F101 USB mapping.
unsafe impl UsbInstance<'static> for USB0 {
    fn register_block(self) -> &'static allwinner_hal::usb::UsbRegisterBlock {
        // SAFETY: consuming the singleton transfers exclusive controller access.
        unsafe { &*Self::ptr() }
    }
}

// SAFETY: the token is exclusively borrowed for the returned capability's lifetime.
unsafe impl<'a> UsbInstance<'a> for &'a mut USB0 {
    fn register_block(self) -> &'a allwinner_hal::usb::UsbRegisterBlock {
        // SAFETY: no other safe controller owner exists during this borrow.
        unsafe { &*USB0::ptr() }
    }
}

// SAFETY: the unique PHY token owns F101's verified version 2 register mapping.
unsafe impl UsbPhyInstance<'static> for USB_PHY0 {
    fn register_block(self) -> &'static allwinner_hal::usb::phy_v2::RegisterBlockV2 {
        // SAFETY: consuming the singleton transfers exclusive PHY access.
        unsafe { &*Self::ptr() }
    }
}

// SAFETY: the mutable token borrow excludes another PHY owner for its lifetime.
unsafe impl<'a> UsbPhyInstance<'a> for &'a mut USB_PHY0 {
    fn register_block(self) -> &'a allwinner_hal::usb::phy_v2::RegisterBlockV2 {
        // SAFETY: the PHY singleton remains exclusively borrowed.
        unsafe { &*USB_PHY0::ptr() }
    }
}

/// Ownership of an F101 GPIO pad.
pub struct Pad<const P: char, const N: u8> {
    _private: (),
}

impl<const P: char, const N: u8> Pad<P, N> {
    /// Macro internal constructor.
    #[doc(hidden)]
    #[inline]
    pub(crate) const fn __new() -> Self {
        Self { _private: () }
    }
}

impl_gpio_pins! {
    pub(crate),
    __new_v2;
    pa0: ('A', 0);
    pa1: ('A', 1);
    pa2: ('A', 2);
    pa3: ('A', 3);
    pb0: ('B', 0);
    pb1: ('B', 1);
    pb2: ('B', 2);
    pb3: ('B', 3);
    pb4: ('B', 4);
    pb5: ('B', 5);
    pb6: ('B', 6);
    pb7: ('B', 7);
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
    pc12: ('C', 12);
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
    pg8: ('G', 8);
    pg9: ('G', 9);
    pg10: ('G', 10);
    pg11: ('G', 11);
    pg12: ('G', 12);
    pg13: ('G', 13);
    pg14: ('G', 14);
    pg15: ('G', 15);
    pg16: ('G', 16);
    pg17: ('G', 17);
    pg18: ('G', 18);
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROM_CPU_PLL: u32 = 0xf800_1900;
    const ROM_PERI_PLL: u32 = 0xf821_6310;

    #[test]
    fn rom_cpu_clock_and_core_divisor() {
        assert_eq!(
            c907_frequency(0x0500_0000, ROM_CPU_PLL, ROM_PERI_PLL),
            Some(600_000_000)
        );
        assert_eq!(
            c907_frequency(0x0500_0301, ROM_CPU_PLL, 0),
            Some(300_000_000)
        );
        assert_eq!(c907_frequency(0x301, 0, 0), Some(12_000_000));
    }

    #[test]
    fn peripheral_clock_selectors_are_f101_specific() {
        assert_eq!(
            c907_frequency(0x0300_0000, 0, ROM_PERI_PLL),
            Some(800_000_000)
        );
        assert_eq!(
            c907_frequency(0x0400_0000, 0, ROM_PERI_PLL),
            Some(600_000_000)
        );
    }

    #[test]
    fn unknown_and_disabled_clocks_are_rejected() {
        for source in [1, 2, 6, 7] {
            assert_eq!(
                c907_frequency(source << 24, ROM_CPU_PLL, ROM_PERI_PLL),
                None
            );
        }
        for bit in [PLL_ENABLE, PLL_OUTPUT_GATE] {
            assert_eq!(c907_frequency(5 << 24, ROM_CPU_PLL & !bit, 0), None);
            assert_eq!(c907_frequency(3 << 24, 0, ROM_PERI_PLL & !bit), None);
        }
        for dividers in [1, 1 << 16, 1 << 20] {
            assert_eq!(c907_frequency(5 << 24, ROM_CPU_PLL | dividers, 0), None);
        }
    }
}
