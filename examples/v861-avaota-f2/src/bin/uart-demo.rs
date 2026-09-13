#![no_std]
#![no_main]

use allwinner_hal::{
    prelude::*,
    uart::{BlockingSerial, Config},
};
use allwinner_rt::{Clocks, Peripherals, entry};
use v861_avaota_f2::console::{Command, Console, InputEvent};

#[entry]
fn main(mut p: Peripherals, clocks: Clocks) {
    let uart_clock = clocks.enable_uart(&mut p.ccu);
    let mut uart = p
        .uart0
        .serial((p.gpio.ph9, p.gpio.ph10), Config::default(), uart_clock);
    let mut console = Console::<32>::new();

    write(
        &mut uart,
        b"Welcome to Allwinner-HAL v861-avaota-f2 UART0!\r\n> ",
    );

    loop {
        match console.push(read(&mut uart)) {
            InputEvent::None => {}
            InputEvent::Echo(byte) => write(&mut uart, &[byte]),
            InputEvent::Erase => write(&mut uart, b"\x08 \x08"),
            InputEvent::Bell => write(&mut uart, b"\x07"),
            InputEvent::Command(command) => {
                write(&mut uart, b"\r\n");
                match command {
                    Command::Empty => {}
                    Command::Hello => write(&mut uart, b"hello world\r\n"),
                    Command::Help => write(
                        &mut uart,
                        b"Commands:\r\n  help   show this help\r\n  hello  print hello world\r\n  exit   return to FEL\r\n",
                    ),
                    Command::Exit => {
                        write(&mut uart, b"Bye!\r\n");
                        uart.flush().unwrap();
                        return;
                    }
                    Command::Unknown => write(&mut uart, b"unknown command; try help\r\n"),
                }
                write(&mut uart, b"> ");
            }
        }
    }
}

#[inline]
fn read(uart: &mut BlockingSerial<'_>) -> u8 {
    let mut byte = [0];
    uart.read_exact(&mut byte).unwrap();
    byte[0]
}

#[inline]
fn write(uart: &mut BlockingSerial<'_>, bytes: &[u8]) {
    uart.write_all(bytes).unwrap();
}
