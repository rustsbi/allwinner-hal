#![no_std]
#![no_main]

use allwinner_rt::{Clocks, Peripherals, entry};
use v821_avaota_f1::usb_network::UsbNetwork;

const IPV6_ADDRESS: [u8; 16] = [0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

#[entry]
fn main(_peripherals: Peripherals, _clocks: Clocks) {
    // SAFETY: the runtime gives this single-core payload exclusive ownership;
    // USB0 is polled with interrupts disabled.
    let mut network = unsafe { UsbNetwork::from_v821_mmio(IPV6_ADDRESS) };
    network.initialize();

    loop {
        if network.poll() {
            return;
        }
    }
}
