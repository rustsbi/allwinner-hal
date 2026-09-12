//! D1 SPI0 interpreter; commands at the payload entry plus 0x1000.
#![no_std]
#![no_main]

use rfel_payload::{entry, env, spi};

#[entry]
fn spi_d1(_parameters: *mut u32) {
    // SAFETY: rfel supplies a terminated command stream, valid SRAM buffers and
    // an ABI-aligned stack, and exclusively owns D1 SPI0, GPIO and clocks.
    unsafe { spi::run(spi::Soc::D1, env::spi_commands()) };
}
