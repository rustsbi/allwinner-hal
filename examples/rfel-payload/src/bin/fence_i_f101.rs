//! Synchronize instruction fetch after replacing a payload in F101 SRAM.
//! This prefix must match both RV32 register helpers when old code is cached.
#![no_std]
#![no_main]

use rfel_payload::entry;

// On F101, the common entry has already synchronized instruction fetch.
#[entry]
fn main(_: *mut u32) {}
