#![no_std]
#![no_main]

#[path = "usb-msc.rs"]
mod usb_msc;

use allwinner_hal::usb::{Usb, UsbBus as AllwinnerUsbBus, phy::v821::UsbPhy};
use allwinner_rt::{Clocks, Peripherals, entry};
use embedded_hal::delay::DelayNs;
use riscv::delay::McycleDelay;
use usb_device::{
    UsbError,
    bus::{UsbBus, UsbBusAllocator},
    device::{StringDescriptors, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
};
use usb_msc::{BLOCK_SIZE, UsbMassStorage};
use usbd_serial::SerialPort;
use v821_avaota_f1::console::{Command, Console, InputEvent};

const BLOCK_COUNT: u32 = 2_880;
const README_BLOCK: u32 = 33;
const CDC_TX_CAPACITY: usize = 128;
const GREETING: &[u8] = b"Welcome to the V821 CDC + MSC composite example!\r\n> ";
const HELP_REPLY: &[u8] = b"\r\nCommands:\r\n  help   show this help\r\n  hello  print hello world\r\n  exit   return to FEL\r\n> ";
const README_TEXT: &[u8] = b"Avaota F1 V821 USB composite example\r\n\
\r\n\
The CDC-ACM console and this read-only mass-storage volume are active at the same time.\r\n\
Open the virtual serial port and type `help` while browsing this drive.\r\n";
const README_BLOCK_COUNT: u32 = README_TEXT.len().div_ceil(BLOCK_SIZE) as u32;

#[entry]
fn main(peripherals: Peripherals, clocks: Clocks) {
    let mut usb0 = peripherals.usb0;
    let mut usb_phy0 = peripherals.usb_phy0;
    let mut ccu = peripherals.ccu;
    let aon_ccu = peripherals.aon_ccu;
    let mut delay = McycleDelay::new(clocks.mcycle_ticks_second(&aon_ccu).unwrap());
    let oscillator = clocks.enable_usb(&mut usb0, &mut usb_phy0, &mut ccu, &aon_ccu, &mut delay);

    let usb = Usb::new(usb0, &mut delay);
    let mut _usb_phy = UsbPhy::new(usb_phy0, oscillator, &mut delay);
    if !usb.is_vbus_valid() {
        _usb_phy.force_vbus_valid();
    }

    let usb_bus = UsbBusAllocator::new(AllwinnerUsbBus::new(usb));
    // Allocate CDC first so interfaces 0/1 form its IAD; MSC then becomes
    // interface 2 and receives the next bulk endpoint pair.
    let mut serial = SerialPort::new(&usb_bus);
    let mut storage = UsbMassStorage::new(&usb_bus, BLOCK_COUNT, CompositeDisk::read_sector);
    let strings = [StringDescriptors::default()
        .manufacturer("RustSBI")
        .product("Avaota F1 CDC + MSC")
        .serial_number("0821F100000003")];
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1f3a, 0x8213))
        .strings(&strings)
        .unwrap()
        .composite_with_iads()
        .device_release(0x0100)
        .max_packet_size_0(64)
        .unwrap()
        .build();

    let mut console = Console::<32>::new();
    let mut received = [0u8; 1];
    let mut tx = CdcTx::new();
    let mut greeting_visible = false;

    loop {
        usb_device.poll(&mut [&mut serial, &mut storage]);
        // Safe eject only completes the MSC command. The CDC console and the
        // composite USB device deliberately remain active.
        let _ = storage.poll();

        if usb_device.state() != UsbDeviceState::Configured {
            greeting_visible = false;
            tx.clear();
            continue;
        }

        match tx.poll(&mut serial) {
            TxProgress::Pending => continue,
            TxProgress::Exit => {
                delay.delay_ms(10);
                return;
            }
            TxProgress::Error => {
                greeting_visible = false;
                continue;
            }
            TxProgress::Ready => {}
        }

        if !greeting_visible {
            tx.queue(GREETING, false);
            greeting_visible = true;
            continue;
        }

        let count = match serial.read(&mut received) {
            Ok(count) => count,
            Err(UsbError::WouldBlock) => 0,
            Err(_) => {
                greeting_visible = false;
                continue;
            }
        };
        if count == 0 {
            continue;
        }

        match console.push(received[0]) {
            InputEvent::None => {}
            InputEvent::Echo(byte) => tx.queue(&[byte], false),
            InputEvent::Erase => tx.queue(b"\x08 \x08", false),
            InputEvent::Bell => tx.queue(b"\x07", false),
            InputEvent::Command(command) => match command {
                Command::Empty => tx.queue(b"\r\n> ", false),
                Command::Hello => tx.queue(b"\r\nhello world\r\n> ", false),
                Command::Help => tx.queue(HELP_REPLY, false),
                Command::Exit => tx.queue(b"\r\nBye!\r\n", true),
                Command::Unknown => tx.queue(b"\r\nunknown command; try help\r\n> ", false),
            },
        }
    }
}

struct CdcTx {
    bytes: [u8; CDC_TX_CAPACITY],
    len: usize,
    offset: usize,
    exit_after_flush: bool,
}

impl CdcTx {
    const fn new() -> Self {
        Self {
            bytes: [0; CDC_TX_CAPACITY],
            len: 0,
            offset: 0,
            exit_after_flush: false,
        }
    }

    fn queue(&mut self, bytes: &[u8], exit_after_flush: bool) {
        debug_assert!(self.len == 0);
        assert!(bytes.len() <= self.bytes.len());
        self.bytes[..bytes.len()].copy_from_slice(bytes);
        self.len = bytes.len();
        self.offset = 0;
        self.exit_after_flush = exit_after_flush;
    }

    fn clear(&mut self) {
        self.len = 0;
        self.offset = 0;
        self.exit_after_flush = false;
    }

    /// Performs at most one non-blocking CDC write or flush attempt.
    fn poll<B: UsbBus>(&mut self, serial: &mut SerialPort<'_, B>) -> TxProgress {
        if self.len == 0 {
            return TxProgress::Ready;
        }
        if self.offset < self.len {
            match serial.write(&self.bytes[self.offset..self.len]) {
                Ok(count) => self.offset += count,
                Err(UsbError::WouldBlock) => {}
                Err(_) => {
                    self.clear();
                    return TxProgress::Error;
                }
            }
            return TxProgress::Pending;
        }

        match serial.flush() {
            Ok(()) => {
                let exit = self.exit_after_flush;
                self.clear();
                if exit {
                    TxProgress::Exit
                } else {
                    TxProgress::Ready
                }
            }
            Err(UsbError::WouldBlock) => TxProgress::Pending,
            Err(_) => {
                self.clear();
                TxProgress::Error
            }
        }
    }
}

enum TxProgress {
    Ready,
    Pending,
    Exit,
    Error,
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
