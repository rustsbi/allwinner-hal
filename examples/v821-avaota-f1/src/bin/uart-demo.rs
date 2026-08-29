#![no_std]
#![no_main]

use allwinner_rt::{
    Clocks, Peripherals, entry,
    soc::v821::{Pad, UART0},
};
use v821_avaota_f1::console::{Command, Console, InputEvent};

const UART0_BASE: usize = 0x4250_0000;
const UART_RBR_THR_DLL: usize = 0x00;
const UART_IER_DLH: usize = 0x04;
const UART_FCR: usize = 0x08;
const UART_LCR: usize = 0x0c;
const UART_MCR: usize = 0x10;
const UART_USR: usize = 0x7c;

const APB_SPECIAL_CLOCK: usize = 0x4a01_0580;
const APB_SPECIAL_CLOCK_FIELDS: u32 = 0x0300_001f;
const UART_GATE: usize = 0x4200_1080;
const UART_RESET: usize = 0x4200_1090;
const UART0_CLOCK_BIT: u32 = 1 << 15;
const PL_CFG0: usize = 0x4200_0540;
const PL4_PL5_FUNCTION_MASK: u32 = 0xff << 16;
const PL4_PL5_UART0: u32 = 0x33 << 16;

const HOSC_HZ: u32 = 40_000_000;
const BAUD: u32 = 115_200;
const DIVISOR: u32 = (HOSC_HZ + 8 * BAUD) / (16 * BAUD);
const _: () = assert!(DIVISOR == 22);

struct Uart0 {
    _instance: UART0,
    _tx: Pad<'L', 4>,
    _rx: Pad<'L', 5>,
}

impl Uart0 {
    fn new(instance: UART0, tx: Pad<'L', 4>, rx: Pad<'L', 5>) -> Self {
        let uart = Self {
            _instance: instance,
            _tx: tx,
            _rx: rx,
        };

        // APB_SPC uses the board's 40 MHz HOSC directly. This avoids relying
        // on a peripheral PLL state that is not guaranteed at either FEL or
        // cold Boot0 entry.
        modify32(APB_SPECIAL_CLOCK, APB_SPECIAL_CLOCK_FIELDS, 0);

        // Match the V821 SPL sequence: reset pulse, then gate pulse.
        modify32(UART_RESET, UART0_CLOCK_BIT, 0);
        short_delay();
        modify32(UART_RESET, 0, UART0_CLOCK_BIT);
        modify32(UART_GATE, UART0_CLOCK_BIT, 0);
        short_delay();
        modify32(UART_GATE, 0, UART0_CLOCK_BIT);

        // PL4=UART0_TX and PL5=UART0_RX, both mux function 3.
        modify32(PL_CFG0, PL4_PL5_FUNCTION_MASK, PL4_PL5_UART0);

        write32(UART0_BASE + UART_IER_DLH, 0);
        write32(UART0_BASE + UART_MCR, 3);
        write32(UART0_BASE + UART_LCR, 0x80);
        write32(UART0_BASE + UART_RBR_THR_DLL, DIVISOR);
        write32(UART0_BASE + UART_IER_DLH, 0);
        write32(UART0_BASE + UART_LCR, 3);
        write32(UART0_BASE + UART_FCR, 7);

        uart
    }

    fn read(&mut self) -> u8 {
        while read32(UART0_BASE + UART_USR) & (1 << 3) == 0 {
            core::hint::spin_loop();
        }
        read32(UART0_BASE + UART_RBR_THR_DLL) as u8
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            while read32(UART0_BASE + UART_USR) & (1 << 1) == 0 {
                core::hint::spin_loop();
            }
            write32(UART0_BASE + UART_RBR_THR_DLL, u32::from(*byte));
        }
    }
}

#[entry]
fn main(peripherals: Peripherals, _clocks: Clocks) {
    let mut uart = Uart0::new(
        peripherals.uart0,
        peripherals.gpio.pl4,
        peripherals.gpio.pl5,
    );
    let mut console = Console::<32>::new();

    uart.write(b"Welcome to Allwinner-HAL v821-avaota-f1 UART0!\r\n> ");

    loop {
        match console.push(uart.read()) {
            InputEvent::None => {}
            InputEvent::Echo(byte) => uart.write(&[byte]),
            InputEvent::Erase => uart.write(b"\x08 \x08"),
            InputEvent::Bell => uart.write(b"\x07"),
            InputEvent::Command(command) => {
                uart.write(b"\r\n");
                match command {
                    Command::Empty => {}
                    Command::Hello => uart.write(b"hello world\r\n"),
                    Command::Help => uart.write(
                        b"Commands:\r\n  help   show this help\r\n  hello  print hello world\r\n",
                    ),
                    Command::Unknown => uart.write(b"unknown command; try help\r\n"),
                }
                uart.write(b"> ");
            }
        }
    }
}

#[inline]
fn read32(address: usize) -> u32 {
    // SAFETY: All callers use aligned V821 MMIO addresses. The runtime gives
    // this payload exclusive E907 execution with interrupts disabled.
    unsafe { (address as *const u32).read_volatile() }
}

#[inline]
fn write32(address: usize, value: u32) {
    // SAFETY: See read32. UART data/configuration registers are written only
    // after their source clock, reset, and gate have been configured.
    unsafe { (address as *mut u32).write_volatile(value) }
}

#[inline]
fn modify32(address: usize, clear: u32, set: u32) {
    // There are no concurrent CCU/GPIO writers in FEL or this Boot0 payload.
    write32(address, (read32(address) & !clear) | set);
}

#[inline]
fn short_delay() {
    let mut cycles = core::hint::black_box(100u32);
    while cycles != 0 {
        core::hint::spin_loop();
        cycles -= 1;
    }
}
