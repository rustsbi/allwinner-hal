#![no_std]
#![no_main]

use allwinner_hal::prelude::*;
use allwinner_rt::{Clocks, Peripherals, entry};

#[entry]
fn main(p: Peripherals, _c: Clocks) {
    // light up led
    let mut pb5 = p.gpio.pb5.into_output();
    pb5.set_high().unwrap();
    let mut pc1 = p.gpio.pc1.into_output();
    pc1.set_high().unwrap();
}
