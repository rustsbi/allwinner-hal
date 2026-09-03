//! V821 chip platforms.

use allwinner_hal::{
    ccu::v821::{AonRegisterBlock, ApbSpecialClockSource, AppRegisterBlock, E907ClockSource},
    gpio::PadExt,
    uart::UartExt,
    usb::{
        Instance as UsbInstance,
        phy::v821::{Instance as UsbPhyInstance, Oscillator},
    },
};
use embedded_hal::delay::DelayNs;
use embedded_time::rate::{Extensions, Hertz};

/// ROM runtime peripheral ownership and configurations.
pub struct Peripherals {
    /// General Purpose Input/Output peripheral.
    pub gpio: Pads,
    /// Application-domain Clock Control Unit peripheral.
    pub ccu: CCU,
    /// Always-on Clock Control Unit peripheral.
    pub aon_ccu: AON_CCU,
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub uart0: UART0,
    /// Universal Asynchronous Receiver/Transmitter 1.
    pub uart1: UART1,
    /// Universal Asynchronous Receiver/Transmitter 2.
    pub uart2: UART2,
    /// Universal Asynchronous Receiver/Transmitter 3.
    pub uart3: UART3,
    /// USB On-The-Go device controller 0.
    pub usb0: USB0,
    /// USB physical layer peripheral 0.
    pub usb_phy0: USB_PHY0,
}

soc! {
    /// General Purpose Input/Output peripheral for PA, PC, PD and PL pads.
    pub struct GPIO => 0x42000000, allwinner_hal::gpio::v2::RegisterBlockV2;
    /// Application-domain Clock Control Unit peripheral.
    pub struct CCU => 0x42001000, AppRegisterBlock;
    // TODO pub struct GPADC => 0x42009000
    // TODO pub struct TMR => 0x42050000
    /// Universal Asynchronous Receiver/Transmitter 0.
    pub struct UART0 => 0x42500000, allwinner_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 1.
    pub struct UART1 => 0x42500400, allwinner_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 2.
    pub struct UART2 => 0x42500800, allwinner_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 3.
    pub struct UART3 => 0x42500C00, allwinner_hal::uart::RegisterBlock;
    /// USB On-The-Go device controller 0.
    pub struct USB0 => 0x44100000, allwinner_hal::usb::RegisterBlock;
    /// USB physical layer peripheral 0.
    pub struct USB_PHY0 => 0x44100400, allwinner_hal::usb::PhyRegisterBlock;
    // TODO pub struct TWI0 => 0x42502000
    // TODO pub struct TWI1 => 0x42502400
    // TODO pub struct TWI2 => 0x42502800
    // TODO pub struct DMAC => 0x43001000
    // TODO pub struct WDT => 0x43031000
    // TODO pub struct RTC => 0x4A000C00
    // TODO pub struct WUPTIMER => 0x4A000400
    // TODO pub struct RTC_WDG => 0x4A001000
    /// Always-on Clock Control Unit peripheral.
    pub struct AON_CCU => 0x4A010000, AonRegisterBlock;
}

impl_uart! {
    0 => UART0,
}

// SAFETY: the runtime constructs exactly one `USB0` token in
// `__rom_init_params`; its fixed address is aligned for and mapped as the V821
// USB controller register block for the lifetime of the firmware.
unsafe impl UsbInstance<'static> for USB0 {
    #[inline]
    fn register_block(self) -> &'static allwinner_hal::usb::RegisterBlock {
        // SAFETY: consuming the sole runtime token grants exclusive access.
        unsafe { &*Self::ptr() }
    }
}

// SAFETY: the mutable token borrow remains active for the full returned MMIO
// capability lifetime, preventing another safe controller construction.
unsafe impl<'a> UsbInstance<'a> for &'a mut USB0 {
    #[inline]
    fn register_block(self) -> &'a allwinner_hal::usb::RegisterBlock {
        // SAFETY: `self` is the unique mutable borrow of the singleton token.
        unsafe { &*USB0::ptr() }
    }
}

// SAFETY: the runtime constructs exactly one `USB_PHY0` token in
// `__rom_init_params`; it owns the independent V821 USB PHY mapping paired with
// USB0 and uses the verified `PhyRegisterBlock` layout.
unsafe impl UsbPhyInstance<'static> for USB_PHY0 {
    #[inline]
    fn register_block(self) -> &'static allwinner_hal::usb::PhyRegisterBlock {
        // SAFETY: consuming the sole runtime token grants exclusive access.
        unsafe { &*Self::ptr() }
    }
}

// SAFETY: the mutable token borrow remains active for the full returned MMIO
// capability lifetime, preventing another safe PHY construction.
unsafe impl<'a> UsbPhyInstance<'a> for &'a mut USB_PHY0 {
    #[inline]
    fn register_block(self) -> &'a allwinner_hal::usb::PhyRegisterBlock {
        // SAFETY: `self` is the unique mutable borrow of the singleton token.
        unsafe { &*USB_PHY0::ptr() }
    }
}

// TODO GPIO_R logic in allwinner-hal

/// Ownership of a V821 GPIO pad.
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

impl<'a, const P: char, const N: u8> allwinner_hal::gpio::PadExt<'a, P, N> for &'a mut Pad<P, N> {
    #[inline]
    fn into_input(self) -> allwinner_hal::gpio::Input<'a> {
        unsafe { allwinner_hal::gpio::Input::__new_v2(P, N, &GPIO { _private: () }) }
    }

    #[inline]
    fn into_output(self) -> allwinner_hal::gpio::Output<'a> {
        unsafe { allwinner_hal::gpio::Output::__new_v2(P, N, &GPIO { _private: () }) }
    }

    #[inline]
    fn into_function<const F: u8>(self) -> allwinner_hal::gpio::Function<'a, P, N, F> {
        unsafe { allwinner_hal::gpio::Function::__new_v2(&GPIO { _private: () }) }
    }

    #[inline]
    fn into_eint(self) -> allwinner_hal::gpio::EintPad<'a> {
        unsafe { allwinner_hal::gpio::EintPad::__new_v2(P, N, &GPIO { _private: () }) }
    }
}

impl<const P: char, const N: u8> allwinner_hal::gpio::PadExt<'static, P, N> for Pad<P, N> {
    #[inline]
    fn into_input(self) -> allwinner_hal::gpio::Input<'static> {
        unsafe { allwinner_hal::gpio::Input::__new_v2(P, N, &GPIO { _private: () }) }
    }

    #[inline]
    fn into_output(self) -> allwinner_hal::gpio::Output<'static> {
        unsafe { allwinner_hal::gpio::Output::__new_v2(P, N, &GPIO { _private: () }) }
    }

    #[inline]
    fn into_function<const F: u8>(self) -> allwinner_hal::gpio::Function<'static, P, N, F> {
        unsafe { allwinner_hal::gpio::Function::__new_v2(&GPIO { _private: () }) }
    }

    #[inline]
    fn into_eint(self) -> allwinner_hal::gpio::EintPad<'static> {
        unsafe { allwinner_hal::gpio::EintPad::__new_v2(P, N, &GPIO { _private: () }) }
    }
}

/// Clock configuration on current SoC.
#[derive(Debug)]
pub struct Clocks;

impl Clocks {
    /// Return the E907 `mcycle` frequency in ticks per second.
    ///
    /// The frequency remains valid while the E907 clock tree is unchanged.
    /// The V821 BootROM enters Boot0 through HOSC or the 1024 MHz peripheral
    /// PLL; other clock parents are not used by this runtime.
    pub fn mcycle_ticks_second(&self, aon_ccu: &AON_CCU) -> Option<u32> {
        let clock = aon_ccu.e907_clock.read();
        let source_frequency = match clock.clock_source() {
            E907ClockSource::Hosc => {
                if aon_ccu.dcxo_status.read().is_24_mhz() {
                    24_000_000
                } else {
                    40_000_000
                }
            }
            E907ClockSource::PeriPll1024M => 1_024_000_000,
            _ => return None,
        };
        Some(source_frequency / u32::from(clock.divisor()))
    }

    /// Select the board HOSC and enable the peripheral clock for UART `I`.
    #[inline]
    pub fn enable_uart<const I: usize>(&self, ccu: &CCU, aon_ccu: &AON_CCU) -> UartClock<I> {
        let hosc_hz = if aon_ccu.dcxo_status.read().is_24_mhz() {
            24_000_000
        } else {
            40_000_000
        };

        // SAFETY: this Boot0 payload is the only active E907 context, the
        // runtime keeps interrupts disabled, and both CCU domains are mapped
        // and powered while their UART clock fields are modified.
        unsafe {
            aon_ccu.apb_special_clock.modify(|value| {
                value
                    .set_clock_source(ApbSpecialClockSource::Hosc)
                    .set_divisor(1)
            });
            ccu.bus_reset0.modify(|value| value.deassert_reset::<I>());
            ccu.bus_clock_gating0.modify(|value| value.gate_mask::<I>());
        }
        short_delay();
        // SAFETY: same exclusive CCU ownership as above. This is the gate
        // pulse used by the V821 SPL after releasing the UART reset.
        unsafe {
            ccu.bus_clock_gating0.modify(|value| value.gate_pass::<I>());
        }

        UartClock {
            frequency: hosc_hz.Hz(),
        }
    }

    /// Reset USB0, enable its bus/reference clocks, and return its oscillator.
    ///
    /// The sequence follows the V821 BootROM ordering. Pass the returned
    /// oscillator to the independent V821 USB PHY initialization. The mutable
    /// peripheral-token borrows prove that neither the controller nor PHY has
    /// an active safe owner while their reset lines are toggled; the mutable
    /// CCU borrow serializes the read-modify-write sequence with safe callers.
    pub fn enable_usb(
        &self,
        _usb0: &mut USB0,
        _usb_phy0: &mut USB_PHY0,
        ccu: &mut CCU,
        aon_ccu: &AON_CCU,
        delay: &mut impl DelayNs,
    ) -> Oscillator {
        // SAFETY: `CCU` is the singleton application-domain clock token. Each
        // operation preserves unrelated fields and touches only the USB reset
        // and gate bits. Delays preserve the BootROM's required edge ordering.
        unsafe {
            ccu.bus_reset0.modify(|value| value.assert_usb_phy());
            ccu.bus_clock_gating0.modify(|value| value.mask_usb_otg());
            ccu.bus_reset0.modify(|value| value.assert_usb_otg());
            ccu.bus_reset0.modify(|value| value.assert_usb_hclk());
        }
        delay.delay_us(20);

        // SAFETY: same exclusive CCU ownership and USB-only fields as above.
        unsafe {
            ccu.bus_clock_gating0.modify(|value| value.mask_usb_hclk());
        }
        delay.delay_us(20);

        // SAFETY: same exclusive CCU ownership and USB-only fields as above.
        unsafe {
            ccu.bus_reset0.modify(|value| value.deassert_usb_phy());
        }
        delay.delay_us(50);

        // SAFETY: same exclusive CCU ownership and USB-only fields as above.
        unsafe {
            ccu.bus_reset0.modify(|value| value.deassert_usb_otg());
        }
        delay.delay_us(100);

        // SAFETY: same exclusive CCU ownership and USB-only fields as above.
        unsafe {
            ccu.bus_clock_gating0.modify(|value| value.pass_usb_otg());
        }
        delay.delay_us(50);

        // SAFETY: same exclusive CCU ownership and USB-only fields as above.
        unsafe {
            ccu.bus_reset0.modify(|value| value.deassert_usb_hclk());
        }
        delay.delay_us(20);

        // SAFETY: same exclusive CCU ownership and USB-only fields as above.
        unsafe {
            ccu.bus_clock_gating0.modify(|value| value.pass_usb_hclk());
        }
        delay.delay_us(20);

        // SAFETY: same exclusive CCU ownership. Enabling the reference clock is
        // the final step, after both reset domains and bus gates are live.
        unsafe {
            ccu.usb_clock.modify(|value| value.enable());
        }

        if aon_ccu.dcxo_status.read().is_24_mhz() {
            Oscillator::Mhz24
        } else {
            Oscillator::Mhz40
        }
    }
}

/// Enabled clock input for V821 UART `I`.
pub struct UartClock<const I: usize> {
    frequency: Hertz,
}

impl<const I: usize> allwinner_hal::uart::Clock<I> for UartClock<I> {
    #[inline]
    fn uart_clock(&self) -> Hertz {
        self.frequency
    }
}

impl<const I: usize> allwinner_hal::uart::Clock<I> for &UartClock<I> {
    #[inline]
    fn uart_clock(&self) -> Hertz {
        self.frequency
    }
}

#[doc(hidden)]
#[inline]
/// Construct the singleton runtime parameters.
///
/// # Safety
///
/// This function may be called at most once during one firmware execution.
pub unsafe fn __rom_init_params() -> (Peripherals, Clocks) {
    let peripherals = Peripherals {
        gpio: Pads::__new(),
        ccu: CCU { _private: () },
        aon_ccu: AON_CCU { _private: () },
        uart0: UART0 { _private: () },
        uart1: UART1 { _private: () },
        uart2: UART2 { _private: () },
        uart3: UART3 { _private: () },
        usb0: USB0 { _private: () },
        usb_phy0: USB_PHY0 { _private: () },
    };
    let clocks = Clocks;
    (peripherals, clocks)
}

impl_gpio_pins! {
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
    pc13: ('C', 13);
    pc14: ('C', 14);
    pc15: ('C', 15);
    pc16: ('C', 16);
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
    pd23: ('D', 23);
    pl0: ('L', 0);
    pl1: ('L', 1);
    pl2: ('L', 2);
    pl3: ('L', 3);
    pl4: ('L', 4);
    pl5: ('L', 5);
    pl6: ('L', 6);
    pl7: ('L', 7);
}

impl_uart_pads! {
    ('L', 4, 3): IntoTransmit, into_uart_transmit, 0;
    ('L', 5, 3): IntoReceive, into_uart_receive, 0;
}

#[inline]
fn short_delay() {
    let mut cycles = core::hint::black_box(100u32);
    while cycles != 0 {
        core::hint::spin_loop();
        cycles -= 1;
    }
}
