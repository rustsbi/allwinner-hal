#![no_std]
#![no_main]

#[path = "usb-msc.rs"]
mod usb_msc;

use allwinner_hal::usb::{Usb, UsbBus as AllwinnerUsbBus, phy::v821::UsbPhy};
use allwinner_rt::{Clocks, Peripherals, entry};
use embedded_hal::delay::DelayNs;
use riscv::delay::McycleDelay;
use usb_device::{
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid},
};
use usb_msc::{BLOCK_SIZE, UsbMassStorage};

const BLOCK_COUNT: u32 = 2_880;
const README_BLOCK: u32 = 33;
const README_TEXT: &[u8] = r#"翻过这座山，他们就会听到你们的故事，故事的结局就是——我们是冠军!

6年，不多不长，英雄联盟对于陌生人可能只是一款游戏，一堆没有用的数据，
可对我们可却是信仰!六年的信仰，六年的青春!
从开始玩英雄联盟的时候就梦想那一场金色的大雨撒向我们的lpl!

但是很可惜，S2倒在马拉松大战面前的WE；S3小狗无助迷茫失落的眼神；
S4omg基地50滴血翻盘fnc给我们带来的无限感动和震撼，以及之后皇族又一次进入决赛却面对三星白队无尽的绝望；
S5那年lpl耻辱的上单，被各种碾压；S6留给我们最深的记忆是厂长那一串耻辱的数据；
S7那句“香锅快走啊”“小狗也倒下了”...

然而，今天，2018年11月3日，我们lpl做到了!我们ig做到了!属于我们lpl的冠军!
属于我们每个lpl英雄联盟玩家的冠军!这一场大雨带走了悲哀，带来了重新开始的青春!
今天lpl获得了所有世界的冠军!今年是lpl最旺盛最强大的一年!

ig牛逼!lpl牛逼!翻过那座山，他们听到了你的故事，等来了那场金色的雨。"#
    .as_bytes();
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
    let mut storage = UsbMassStorage::new(&usb_bus, BLOCK_COUNT, VirtualDisk::read_sector);
    let strings = [StringDescriptors::default()
        .manufacturer("RustSBI")
        .product("Avaota F1")
        .serial_number("0821F100000001")];
    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1f3a, 0x8211))
        .strings(&strings)
        .unwrap()
        .max_packet_size_0(64)
        .unwrap()
        .build();

    loop {
        usb_device.poll(&mut [&mut storage]);
        if storage.poll() {
            delay.delay_ms(10);
            return;
        }
    }
}

struct VirtualDisk;

impl VirtualDisk {
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
        sector[13] = 1; // sectors per cluster
        put_u16(sector, 14, 1); // reserved sectors
        sector[16] = 2; // FAT copies
        put_u16(sector, 17, 224); // root entries
        put_u16(sector, 19, BLOCK_COUNT as u16);
        sector[21] = 0xf0;
        put_u16(sector, 22, 9); // sectors per FAT
        put_u16(sector, 24, 18); // sectors per track
        put_u16(sector, 26, 2); // heads
        sector[38] = 0x29;
        put_u32(sector, 39, 0x0821_f1a0);
        sector[43..54].copy_from_slice(b"Avaota F1  ");
        sector[54..62].copy_from_slice(b"FAT12   ");
        sector[510..].copy_from_slice(&[0x55, 0xaa]);
    }

    fn root_directory(sector: &mut [u8; BLOCK_SIZE]) {
        sector[..11].copy_from_slice(b"Avaota F1  ");
        sector[11] = 0x08; // volume label

        let file = &mut sector[32..64];
        file[..11].copy_from_slice(b"README  TXT");
        file[11] = 0x21; // read-only and archive
        file[12] = 0x10; // lowercase extension: README.txt
        put_u16(file, 16, 0x5d1e); // 2026-08-30
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
