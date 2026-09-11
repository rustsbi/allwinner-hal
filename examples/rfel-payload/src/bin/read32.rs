//! Read a 32-bit value from the address appended after the image into the next word.
//! The entry locates parameters through `__payload_end` on both RV32 and RV64.
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use rfel_payload::entry;

#[entry]
fn main(parameters: *mut u32) {
    // SAFETY: FEL supplies two aligned SRAM words and an aligned CPU-readable
    // address. It owns the parameter/result buffer throughout this invocation.
    unsafe {
        // Match lw: extend the address sign on RV64 and keep its bits on RV32.
        let address = read_volatile(parameters.cast::<i32>()) as isize as *const u32;
        let value = read_volatile(address);
        write_volatile(parameters.add(1), value);
    }
}
