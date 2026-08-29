#![no_std]
#![no_main]

use allwinner_rt::{Clocks, Peripherals, entry};
use v821_avaota_f1::{
    console::{Command, Console, InputEvent},
    usb::UsbCdcAcm,
};

#[entry]
fn main(_peripherals: Peripherals, _clocks: Clocks) {
    // SAFETY: BootROM has transferred its E907 exclusively to this payload,
    // either from SPI NOR Boot0 or through FEL. The runtime disabled interrupts.
    let mut usb = unsafe { UsbCdcAcm::from_v821_mmio() };
    usb.initialize();

    let mut console = Console::<32>::new();
    let mut received = [0u8; 64];
    let mut prompt_visible = false;

    usb.write(b"Welcome to Allwinner-HAL v821-avaota-f1 example!\r\n");

    loop {
        let count = usb.poll(&mut received);

        if !usb.is_configured() {
            prompt_visible = false;
            continue;
        }
        if !prompt_visible {
            usb.write(b"> ");
            prompt_visible = true;
        }

        for byte in &received[..count] {
            match console.push(*byte) {
                InputEvent::None => {}
                InputEvent::Echo(byte) => usb.write(&[byte]),
                InputEvent::Erase => usb.write(b"\x08 \x08"),
                InputEvent::Bell => usb.write(b"\x07"),
                InputEvent::Command(command) => {
                    usb.write(b"\r\n");
                    match command {
                        Command::Empty => {}
                        Command::Hello => usb.write(b"hello world\r\n"),
                        Command::Help => usb.write(
                            b"Commands:\r\n  help   show this help\r\n  hello  print hello world\r\n",
                        ),
                        Command::Unknown => usb.write(b"unknown command; try help\r\n"),
                    }
                    usb.write(b"> ");
                }
            }
        }
    }
}
