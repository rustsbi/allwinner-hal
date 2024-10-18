//! System power, LDO and calibration controller.

use volatile_register::{RO, RW};

/// System power, LDO and calibration controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// System LDO Control Register.
    // TODO: offset = 0x150
    pub ldo_control: RW<u32>,
    /// Resistor Calibration Control register.
    // TODO: offset = 0x160
    pub zq_resistor_control: RW<u32>,
    /// 240-Ohm Resistor Manual Control register.
    // TODO: offset = 0x168
    pub zq_resistor_240_control: RW<u32>,
    /// Resistor Calibration Status register.
    // TODO: offset = 0x16C
    pub zq_resistor_state: RO<u32>,
}
