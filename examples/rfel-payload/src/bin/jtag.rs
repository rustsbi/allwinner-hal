//! Enable D1/F101 JTAG on PF0, PF1, PF3 and PF5.
#![no_std]
#![no_main]

use allwinner_hal::gpio::v2::RegisterBlockV2;
use rfel_payload::entry;

#[entry]
fn main(_: *mut u32) {
    // SAFETY: FEL exclusively owns D1/F101 GPIO; the host has selected the
    // matching architecture. Perform four ordered PF_CFG0 updates.
    unsafe {
        let gpio = &*(0x0200_0000 as *const RegisterBlockV2);
        for pin in [0, 1, 3, 5] {
            let shift = pin * 4;
            gpio.sys_port[5].cfg[0].modify(|v| (v & !(0xf << shift)) | (4 << shift));
        }
    }
}
