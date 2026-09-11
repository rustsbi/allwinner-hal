//! Read a 32-bit value using the shared F101/V821 payload protocol.
//! FEL appends the address and result words immediately after the image.
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
    body = sym read32,
);

// RV32's default code model uses absolute addresses for linker symbols, so the
// entry computes the parameter pointer relative to its PC and passes it in a0.
#[cfg_attr(
    not(all(target_os = "none", target_arch = "riscv32")),
    allow(dead_code)
)]
unsafe extern "C" fn read32(parameters: *mut u32) {
    // SAFETY: FEL supplies two aligned SRAM words and an aligned CPU-readable
    // address. It owns the parameter/result buffer throughout this invocation.
    unsafe {
        let address = read_volatile(parameters) as usize as *const u32;
        let value = read_volatile(address);
        write_volatile(parameters.add(1), value);
    }
}

#[cfg(not(target_os = "none"))]
fn main() {}
