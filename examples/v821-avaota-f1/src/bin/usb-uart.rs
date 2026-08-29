#![no_std]
#![no_main]

use core::panic::PanicInfo;
use v821_avaota_f1::{
    console::{Command, Console, InputEvent},
    usb::UsbCdcAcm,
};

// V821 BootROM consumes only the 0x30-byte common eGON prefix. The 0x60-byte
// size below deliberately matches the compact U-Boot/rfel header, not Tina's
// unused 0x3c8-byte private DRAM/GPIO Boot0 header.
core::arch::global_asm!(
    r#"
    .section .head, "ax", @progbits
    .globl _boot
    .type _boot, @function
    .option push
    .option norvc
_boot:
    j       _start
    .size _boot, . - _boot
    .ascii  "eGON.BT0"
    .word   0x5f0a6c39
    .word   0
    .zero   0x60 - (. - _boot)
    .option pop

    .section .text.entry, "ax", @progbits
    .globl _start
    .type _start, @function
    .option push
    .option norvc
_start:
    csrci   mstatus, 8
    csrw    mie, zero

    .option push
    .option norelax
    la      gp, __global_pointer$
    .option pop
    la      sp, __stack_top

    la      t0, __bss_start
    la      t1, __bss_end
1:
    bgeu    t0, t1, 2f
    sw      zero, 0(t0)
    addi    t0, t0, 4
    j       1b
2:
    call    rust_main
3:
    wfi
    j       3b
    .option pop
    .size _start, . - _start
"#,
);

#[unsafe(no_mangle)]
extern "C" fn rust_main() -> ! {
    // SAFETY: BootROM has transferred its E907 exclusively to this payload,
    // either from SPI NOR Boot0 or through FEL. `_start` disabled interrupts.
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

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
