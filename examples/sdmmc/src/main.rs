#![no_std]
#![no_main]

use allwinner_hal::{
    prelude::*,
    smhc::{SdCard, Smhc},
    uart::Config,
};
use allwinner_rt::{Clocks, Peripherals, entry};
use embedded_io::Write;
use embedded_sdmmc::VolumeManager;
use panic_halt as _;

struct MyTimeSource {}

impl embedded_sdmmc::TimeSource for MyTimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        // TODO
        embedded_sdmmc::Timestamp::from_calendar(2023, 1, 1, 0, 0, 0).unwrap()
    }
}

#[entry]
fn main(p: Peripherals, c: Clocks) {
    let tx = p.gpio.pb8.into_function::<6>();
    let rx = p.gpio.pb9.into_function::<6>();
    let mut serial = p.uart0.serial((tx, rx), Config::default(), &c);

    writeln!(serial, "Hello World!").ok();

    writeln!(serial, "initialize sdmmc pins...").ok();
    let sdmmc_pins = {
        let sdc0_d1 = p.gpio.pf0.into_function::<2>();
        let sdc0_d0 = p.gpio.pf1.into_function::<2>();
        let sdc0_clk = p.gpio.pf2.into_function::<2>();
        let sdc0_cmd = p.gpio.pf3.into_function::<2>();
        let sdc0_d3 = p.gpio.pf4.into_function::<2>();
        let sdc0_d2 = p.gpio.pf5.into_function::<2>();
        (sdc0_d1, sdc0_d0, sdc0_clk, sdc0_cmd, sdc0_d3, sdc0_d2)
    };

    writeln!(serial, "initialize smhc...").ok();
    let mut smhc = Smhc::new::<0>(p.smhc0, sdmmc_pins, &c, &p.ccu);

    writeln!(serial, "initializing SD card...").ok();
    let sdcard = match SdCard::new(&mut smhc) {
        Ok(card) => card,
        Err(e) => {
            writeln!(serial, "Failed to initialize SD card: {:?}", e).ok();
            loop {}
        }
    };
    writeln!(
        serial,
        "SD card initialized, size: {:.2}GB",
        sdcard.get_size_kb() / 1024.0 / 1024.0
    )
    .ok();

    let time_source = MyTimeSource {};
    let mut volume_mgr = VolumeManager::new(sdcard, time_source);
    let volume_res = volume_mgr.open_raw_volume(embedded_sdmmc::VolumeIdx(0));
    if let Err(e) = volume_res {
        writeln!(serial, "Failed to open volume: {:?}", e).ok();
        loop {}
    }
    let volume0 = volume_res.unwrap();
    let root_dir = volume_mgr.open_root_dir(volume0).unwrap();

    volume_mgr
        .iterate_dir(root_dir, |entry| {
            writeln!(serial, "Entry: {:?}", entry).ok();
        })
        .unwrap();

    volume_mgr.close_dir(root_dir).unwrap();

    loop {}
}
