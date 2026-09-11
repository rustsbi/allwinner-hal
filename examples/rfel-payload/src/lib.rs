//! FEL register-access, instruction-synchronization and memory-copy payloads.
#![cfg_attr(target_os = "none", no_std)]

pub use rfel_payload_macros::entry;

#[cfg(target_os = "none")]
use panic_never as _;

// Hosted binaries use no_main too; provide their empty native process entry.
#[cfg(all(not(target_os = "none"), not(test)))]
#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    0
}

#[cfg(all(
    target_os = "none",
    any(target_arch = "riscv32", target_arch = "riscv64")
))]
mod boot;
