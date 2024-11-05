use clap::Parser;
use clap_verbosity_flag::Verbosity;
use log::{debug, error};
use rfel::Fel;

#[derive(Parser)]
#[clap(name = "rfel")]
#[clap(about = "Allwinner FEL tool", long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,
}

/// USB vendor ID 0x1f3a: Allwinner Technology Co., Ltd.
const VENDOR_ALLWINNER: u16 = 0x1f3a;
/// Product 0xefe8: sunxi SoC OTG connector in FEL/flashing mode.
const PRODUCT_FEL: u16 = 0xefe8;

fn main() {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    let devices: Vec<_> = nusb::list_devices()
        .expect("list devices")
        .filter(|dev| dev.vendor_id() == VENDOR_ALLWINNER && dev.product_id() == PRODUCT_FEL)
        .inspect(|dev| debug!("Allwinner FEL device {:?}", dev))
        .collect();
    if devices.len() == 0 {
        error!("Cannot find any Allwinner FEL device connected.");
        return;
    }
    if devices.len() > 1 {
        error!("TODO: rfel does not support connecting to multiple Allwinner FEL devices by now.");
        return;
    }
    let device = devices[0].open().expect("open USB device");
    let mut interface = device.claim_interface(0).expect("open USB interface 0");
    let fel = Fel::open_interface(&mut interface).expect("open usb interface as an FEL device");
    let version = fel.get_version();
    println!("{:x?}", version);
    let mut buf = [0; 16];
    fel.read_address(0x20000, &mut buf);
    println!("{:x?}", buf);
}
