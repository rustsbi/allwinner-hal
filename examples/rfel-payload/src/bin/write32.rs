//! Write a 32-bit value using the address and value appended after the image.
//! The cache prefix matches xfel; Rust locates parameters through `__payload_end`.
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

use core::ptr::{read_volatile, write_volatile};

#[cfg(target_os = "none")]
use panic_never as _;

// Keep cache synchronization at the very beginning, before any compiler-generated
// instructions. Jump without changing ra so the Rust C-ABI function returns to FEL.
#[cfg(all(target_os = "none", target_arch = "riscv64"))]
core::arch::global_asm!(
    r#"
.section .text.payload,"ax"
.global _start
.option push
.option norelax
.option norvc
_start:
    lui t1, 0x400
    csrrs zero, 0x7c0, t1
    lui t1, 0x30
    addiw t1, t1, 19
    csrrs zero, 0x7c2, t1
    jal zero, {body}
.option pop
"#,
    body = sym write32,
);

unsafe extern "C" {
    // The linker marks the end of the image; FEL appends these two words in SRAM.
    static mut __payload_end: [u32; 2];
}

#[cfg_attr(
    not(all(target_os = "none", target_arch = "riscv64")),
    allow(dead_code)
)]
unsafe extern "C" fn write32() {
    let parameters = &raw mut __payload_end;
    // SAFETY: FEL supplies two initialized, aligned SRAM words and an aligned
    // CPU-writable address. It owns the buffer and target access during this call.
    unsafe {
        // Match RV64 lw: sign-extend the 32-bit address before dereferencing it.
        let address = read_volatile(parameters.cast::<i32>()) as isize as *mut u32;
        let value = read_volatile(parameters.cast::<u32>().add(1));
        write_volatile(address, value);
    }
}

#[cfg(not(target_os = "none"))]
fn main() {}
