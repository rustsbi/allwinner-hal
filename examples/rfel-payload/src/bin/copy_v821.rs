//! Aligned SRAM copy used for V821 BootROM reads.
//! Append three LE u32s: source, destination, nonzero byte length divisible by four.
//! Cache maintenance makes the CPU-written destination visible to FEL USB reads.
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use rfel_payload::entry;

#[entry]
fn main(parameters: *mut u32) {
    // SAFETY: FEL supplies three initialized, aligned parameter words and valid
    // source/destination ranges. The byte length is nonzero and divisible by four.
    // FEL owns these ranges during the call; overlapping ranges copy forwards.
    unsafe {
        let mut source = read_volatile(parameters) as usize as *const u32;
        let mut destination = read_volatile(parameters.add(1)) as usize as *mut u32;
        let mut remaining = read_volatile(parameters.add(2));
        loop {
            let value = read_volatile(source);
            write_volatile(destination, value);
            source = source.wrapping_add(1);
            destination = destination.wrapping_add(1);
            remaining = remaining.wrapping_sub(4);
            if remaining == 0 {
                break;
            }
        }

        // Preserve the original V821 cache-maintenance sequence before FEL reads
        // the SRAM through USB. This asm also acts as a compiler memory barrier.
        #[cfg(all(target_os = "none", target_arch = "riscv32"))]
        core::arch::asm!(
            ".option push",
            ".option arch, +xtheadcmo,+xtheadsync",
            "fence rw, rw",
            "th.dcache.ciall",
            "th.sync.is",
            "fence.i",
            ".option pop",
            options(nostack),
        );
    }
}
