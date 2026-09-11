//! Synchronize instruction fetch after replacing a payload in F101 SRAM.
//! This prefix must match both RV32 register helpers when old code is cached.
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use panic_never as _;

// SAFETY: FEL enters at the image start in machine mode with a valid ra. Keep
// the shared prefix first, avoid a stack frame, and preserve all callee-saved
// registers so execution can return to FEL immediately after synchronization.
#[cfg(all(target_os = "none", target_arch = "riscv32"))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.payload")]
unsafe extern "C" fn _start() {
    core::arch::naked_asm!(
        ".option push",
        ".option norelax",
        ".option norvc",
        "lui t1, 0x400",
        "csrrs zero, 0x7c0, t1",
        "fence.i",
        ".option rvc",
        "c.jr ra",
        ".option pop",
    );
}

#[cfg(not(target_os = "none"))]
fn main() {}
