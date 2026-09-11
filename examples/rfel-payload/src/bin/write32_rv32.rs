//! Write a 32-bit value using the shared F101/V821 payload protocol.
//! FEL appends the address and value words immediately after the image.
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

use core::ptr::{read_volatile, write_volatile};

#[cfg(target_os = "none")]
use panic_never as _;

// Preserve the original CSR and instruction-cache synchronization before Rust
// executes. Jump without changing ra so the C-ABI function returns to FEL.
#[cfg(all(target_os = "none", target_arch = "riscv32"))]
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
    fence.i
.Lparameters:
    auipc a0, %pcrel_hi(__payload_end)
    addi a0, a0, %pcrel_lo(.Lparameters)
    jal zero, {body}
.option pop
"#,
    body = sym write32,
);

// RV32's default code model uses absolute addresses for linker symbols, so the
// entry computes the parameter pointer relative to its PC and passes it in a0.
#[cfg_attr(
    not(all(target_os = "none", target_arch = "riscv32")),
    allow(dead_code)
)]
unsafe extern "C" fn write32(parameters: *mut u32) {
    // SAFETY: FEL supplies two initialized, aligned SRAM words and an aligned
    // CPU-writable address. It owns the buffer and target access during this call.
    unsafe {
        let address = read_volatile(parameters) as usize as *mut u32;
        let value = read_volatile(parameters.add(1));
        write_volatile(address, value);
    }
}

#[cfg(not(target_os = "none"))]
fn main() {}
