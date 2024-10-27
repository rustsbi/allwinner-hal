#![no_std]
#![no_main]

use allwinner_hal::uart::{Config, Serial};
use allwinner_rt::{entry, Clocks, Peripherals};
use embedded_io::{Read, Write};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) {
    let tx = p.gpio.pb8.into_function::<6>();
    let rx = p.gpio.pb9.into_function::<6>();
    let mut serial = Serial::new(p.uart0, (tx, rx), Config::default(), &c, &p.ccu);

    writeln!(serial, "Hello World!").ok();

    let mut buf = [0u8; 64];
    let mut cur = 0;

    writeln!(serial, "Please input your name (maximum 64 bytes):").unwrap();
    write!(serial, "> ").unwrap();
    loop {
        let mut ch = 0u8;
        let _len = serial.read(core::slice::from_mut(&mut ch)).unwrap();
        if ch == b'\r' || ch == b'\n' {
            break;
        }
        buf[cur] = ch;
        cur += 1;
        if cur > buf.len() {
            break;
        }
    }

    let name = core::str::from_utf8(&buf).unwrap();
    writeln!(serial, "Hello, {}!", name).unwrap();
}
