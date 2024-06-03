// Build this example with:
// cargo build --example fn-main --target riscv64imac-unknown-none-elf --release
// Checkout target assembly code:
// rust-objdump -d target/riscv64imac-unknown-none-elf/release/examples/fn-main > target/1.asm

#![no_std]
#![no_main]
use allwinner_rt::{entry, Clocks, Peripherals};

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    drop((p, c));
    loop {
        // TODO: main function contents
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
