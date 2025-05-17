// Build this example with:
// cargo build --example fn-main --target riscv64imac-unknown-none-elf --release -p allwinner-rt
// Checkout target assembly code:
// rust-objdump -d target/riscv64imac-unknown-none-elf/release/examples/fn-main > target/1.asm

#![no_std]
#![no_main]
use allwinner_hal::{prelude::*, uart::Config};
use allwinner_rt::{Clocks, Peripherals, entry};
use embedded_hal::digital::{InputPin, OutputPin};

#[entry]
fn main(p: Peripherals, c: Clocks) {
    let mut pb0 = p.gpio.pb0.into_input();

    pb0.with_output(|pad| pad.set_high()).unwrap();

    let _input_high = pb0.is_high();

    let tx = p.gpio.pb8.into_function::<6>();
    let rx = p.gpio.pb9.into_function::<6>();
    let mut serial = p.uart0.serial((tx, rx), Config::default(), &c, &p.ccu);

    let _borrow_input_high = serial.pads(|(_, rx)| rx.with_input(|pad| pad.is_high()));
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
