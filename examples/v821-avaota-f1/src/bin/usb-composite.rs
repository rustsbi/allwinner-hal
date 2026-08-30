#![no_std]
#![no_main]

use allwinner_rt::{Clocks, Peripherals, entry};
use v821_avaota_f1::{
    console::{Command, Console, InputEvent},
    usb_composite::UsbComposite,
    usb_msc::BLOCK_SIZE,
};

const BLOCK_COUNT: u32 = 2_880;
const README_BLOCK: u32 = 33;
const README_TEXT: &[u8] = b"Avaota F1 V821 USB composite example\r\n\
\r\n\
The CDC-ACM console and this read-only mass-storage volume are active at the same time.\r\n\
Open the virtual serial port and type `help` while browsing this drive.\r\n";
const README_BLOCK_COUNT: u32 = README_TEXT.len().div_ceil(BLOCK_SIZE) as u32;

#[entry]
fn main(_peripherals: Peripherals, _clocks: Clocks) {
    // SAFETY: the runtime gives this single-core payload exclusive ownership;
    // USB0 is polled with interrupts disabled.
    let mut usb = unsafe { UsbComposite::from_v821_mmio(BLOCK_COUNT, CompositeDisk::read_sector) };
    usb.initialize();

    let mut console = Console::<32>::new();
    let mut received = [0u8; 64];
    let mut prompt_visible = false;

    loop {
        let count = usb.poll(&mut received);

        if !usb.is_configured() {
            prompt_visible = false;
            continue;
        }
        if !prompt_visible {
            usb.write(b"Welcome to the V821 CDC + MSC composite example!\r\n> ");
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
                            b"Commands:\r\n  help   show this help\r\n  hello  print hello world\r\n  exit   return to FEL\r\n",
                        ),
                        Command::Exit => {
                            usb.write(b"Bye!\r\n");
                            let _ = usb.flush();
                            return;
                        }
                        Command::Unknown => usb.write(b"unknown command; try help\r\n"),
                    }
                    usb.write(b"> ");
                }
            }
        }
    }
}

struct CompositeDisk;

impl CompositeDisk {
    fn read_sector(lba: u32, sector: &mut [u8; BLOCK_SIZE]) {
        sector.fill(0);
        match lba {
            0 => Self::boot_sector(sector),
            1 | 10 => Self::fat_table(sector),
            19 => Self::root_directory(sector),
            lba if (README_BLOCK..README_BLOCK + README_BLOCK_COUNT).contains(&lba) => {
                let offset = (lba - README_BLOCK) as usize * BLOCK_SIZE;
                let count = (README_TEXT.len() - offset).min(BLOCK_SIZE);
                sector[..count].copy_from_slice(&README_TEXT[offset..offset + count]);
            }
            _ => {}
        }
    }

    fn fat_table(sector: &mut [u8; BLOCK_SIZE]) {
        sector[..3].copy_from_slice(&[0xf0, 0xff, 0xff]);
        for index in 0..README_BLOCK_COUNT as u16 {
            let cluster = index + 2;
            let next = if index + 1 == README_BLOCK_COUNT as u16 {
                0x0fff
            } else {
                cluster + 1
            };
            put_fat12(sector, cluster, next);
        }
    }

    fn boot_sector(sector: &mut [u8; BLOCK_SIZE]) {
        sector[..3].copy_from_slice(&[0xeb, 0x3c, 0x90]);
        sector[3..11].copy_from_slice(b"MSDOS5.0");
        put_u16(sector, 11, BLOCK_SIZE as u16);
        sector[13] = 1;
        put_u16(sector, 14, 1);
        sector[16] = 2;
        put_u16(sector, 17, 224);
        put_u16(sector, 19, BLOCK_COUNT as u16);
        sector[21] = 0xf0;
        put_u16(sector, 22, 9);
        put_u16(sector, 24, 18);
        put_u16(sector, 26, 2);
        sector[38] = 0x29;
        put_u32(sector, 39, 0x0821_f1a3);
        sector[43..54].copy_from_slice(b"V821 CDCMSC");
        sector[54..62].copy_from_slice(b"FAT12   ");
        sector[510..].copy_from_slice(&[0x55, 0xaa]);
    }

    fn root_directory(sector: &mut [u8; BLOCK_SIZE]) {
        sector[..11].copy_from_slice(b"V821 CDCMSC");
        sector[11] = 0x08;

        let file = &mut sector[32..64];
        file[..11].copy_from_slice(b"README  TXT");
        file[11] = 0x21;
        file[12] = 0x10;
        put_u16(file, 16, 0x5d1e);
        put_u16(file, 18, 0x5d1e);
        put_u16(file, 24, 0x5d1e);
        put_u16(file, 26, 2);
        put_u32(file, 28, README_TEXT.len() as u32);
    }
}

fn put_u16(output: &mut [u8], offset: usize, value: u16) {
    output[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(output: &mut [u8], offset: usize, value: u32) {
    output[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_fat12(output: &mut [u8], cluster: u16, value: u16) {
    let offset = usize::from(cluster) * 3 / 2;
    if cluster & 1 == 0 {
        output[offset] = value as u8;
        output[offset + 1] = (output[offset + 1] & 0xf0) | (value >> 8) as u8;
    } else {
        output[offset] = (output[offset] & 0x0f) | (value << 4) as u8;
        output[offset + 1] = (value >> 4) as u8;
    }
}
