//! Write a 32-bit value using the address and value appended after the image.
//! The entry locates parameters through `__payload_end` on both RV32 and RV64.
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use rfel_payload::entry;

#[entry]
fn main(parameters: *mut u32) {
    // SAFETY: FEL supplies two initialized, aligned SRAM words and an aligned
    // CPU-writable address. It owns the buffer and target access during this call.
    unsafe {
        // Match lw: extend the address sign on RV64 and keep its bits on RV32.
        let address = read_volatile(parameters.cast::<i32>()) as isize as *mut u32;
        let value = read_volatile(parameters.add(1));
        write_volatile(address, value);
    }
}
