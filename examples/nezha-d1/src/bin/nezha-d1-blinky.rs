#![no_std]
#![no_main]

// use allwinner_hal::prelude::*;
use allwinner_rt::{Clocks, Peripherals, entry};

#[entry]
fn main(p: Peripherals, _c: Clocks) {
    // TODO implement using LEDC peripheral
}
