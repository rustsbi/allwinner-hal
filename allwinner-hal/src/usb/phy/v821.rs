//! V821 28 nm USB PHY ownership and initialization.

use embedded_hal::delay::DelayNs;

use crate::usb::PhyRegisterBlock;

/// Oscillator selected by the V821 BootROM for the PHY reference path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Oscillator {
    /// 24 MHz high-speed oscillator.
    Mhz24,
    /// 40 MHz high-speed oscillator.
    Mhz40,
}

/// A uniquely owned V821 USB PHY instance.
///
/// # Safety
///
/// Consuming one implementor value must yield logically exclusive access to one
/// valid V821 [`PhyRegisterBlock`] for lifetime `'a`. No overlapping safe PHY
/// capability may exist while the returned reference is live.
pub unsafe trait Instance<'a> {
    /// Consume the singleton token and borrow its PHY registers.
    fn register_block(self) -> &'a PhyRegisterBlock;
}

/// Exclusively owned and initialized V821 USB PHY.
///
/// This type is deliberately independent of [`crate::usb::Usb`]. Other
/// Allwinner PHY IP blocks can expose unrelated types and initialization APIs.
pub struct UsbPhy<'a> {
    registers: &'a PhyRegisterBlock,
}

impl<'a> UsbPhy<'a> {
    /// Consume a V821 PHY token and run the BootROM-derived initialization.
    pub fn new(
        instance: impl Instance<'a>,
        oscillator: Oscillator,
        delay: &mut impl DelayNs,
    ) -> Self {
        let registers = instance.register_block();
        let serial_byte = match oscillator {
            Oscillator::Mhz24 => 0x14_u8,
            Oscillator::Mhz40 => 0x0c_u8,
        };

        for selector in 11_u8..19 {
            let data_high = serial_byte & (1 << (selector - 11)) != 0;
            let control = &registers.phy_control_28nm;
            // SAFETY: consuming the PHY token grants exclusive access. These
            // four writes reproduce the V821 VC-bus latch sequence.
            unsafe {
                control.write(control.read().enable_vc_bus());
                control.write(control.read().prepare_vc_write());
                control.write(control.read().set_vc_address_and_data(selector, data_high));
                control.write(control.read().raise_vc_clock());
            }
            delay.delay_us(50);
        }

        // SAFETY: this value exclusively owns the independent PHY peripheral.
        unsafe {
            registers
                .phy_select
                .write(registers.phy_select.read().select_otg_controller());
            registers
                .phy_control_28nm
                .write(registers.phy_control_28nm.read().power_up());
        }
        delay.delay_us(20);

        registers
            .interface_status_control
            .modify_control(|value| value.force_id_high());
        registers.interface_status_control.modify_control(|value| {
            value
                .set_dpdm_pullup_enabled(true)
                .use_all_vbus_valid_sources()
                .use_detected_vbus()
        });
        registers
            .interface_status_control
            .modify_control(|value| value.set_dpdm_pullup_enabled(false));

        Self { registers }
    }

    /// Override the PHY VBUS-valid input after board-level detection fails.
    pub fn force_vbus_valid(&mut self) {
        self.registers
            .interface_status_control
            .modify_control(|value| value.force_vbus_valid_high());
    }
}
