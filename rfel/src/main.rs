use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::{debug, error};
use rfel::Fel;

#[derive(Parser)]
#[clap(name = "rfel")]
#[clap(about = "Allwinner FEL tool", long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show chip version
    Version,
    /// Dumps memory region in hexadecimal format
    Hexdump {
        /// The address to be dumped
        address: String,
        /// Length of memory to be dumped
        length: String,
    },
    /// Read a 32-bit value from chip memory
    Read32 {
        /// The address to be read
        address: String,
    },
    /// Write a 32-bit value into chip memory
    Write32 {
        /// The address to be written
        address: String,
        /// The 32-bit value to be written
        value: String,
    },
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
    match cli.command {
        Commands::Version => {
            let version = fel.get_version();
            println!("{:x?}", version);
        }
        Commands::Hexdump { address, length } => {
            let address: usize = match parse_value(address.trim()) {
                Some(address) => address,
                None => {
                    println!(
                        "error: invalid address, shoule be hexadecimal like 0x40000000, or decimal like 1073741824"
                    );
                    return;
                }
            };
            let length: usize = match parse_value(length.trim()) {
                Some(address) => address,
                None => {
                    println!(
                        "error: invalid data, shoule be hexadecimal like 0x40000000, or decimal like 1073741824"
                    );
                    return;
                }
            };
            const CHUNK_SIZE: usize = 65536;
            let mut buf = Vec::new();
            buf.resize(CHUNK_SIZE, 0);
            for offset in (0..length).step_by(CHUNK_SIZE) {
                let chunk_len = (length - offset).min(CHUNK_SIZE);
                fel.read_address((address + offset) as u32, &mut buf[..chunk_len]);
                hexdump(&buf[..chunk_len], (address + offset) as u32);
            }
        }
        Commands::Read32 { address } => {
            let address: u32 = match parse_value(address.trim()) {
                Some(address) => address,
                None => {
                    println!(
                        "error: invalid address, shoule be hexadecimal like 0x40000000, or decimal like 1073741824"
                    );
                    return;
                }
            };
            let mut buf = [0u8; 4];
            fel.read_address(address, &mut buf);
            let ans = u32::from_le_bytes(buf);
            println!("0x{:08x}", ans);
        }
        Commands::Write32 { address, value } => {
            let address: u32 = match parse_value(address.trim()) {
                Some(address) => address,
                None => {
                    println!(
                        "error: invalid address, shoule be hexadecimal like 0x40000000, or decimal like 1073741824"
                    );
                    return;
                }
            };
            let value: u32 = match parse_value(value.trim()) {
                Some(value) => value,
                None => {
                    println!(
                        "error: invalid address, shoule be hexadecimal like 0x40000000, or decimal like 1073741824"
                    );
                    return;
                }
            };
            fel.write_address(address, &value.to_le_bytes());
        }
    }
}

fn hexdump(buf: &[u8], base_address: u32) {
    for i in (0..buf.len()).step_by(16) {
        print!("{:08x}: ", base_address as usize + i);
        let chunk_len = 16.min(buf.len() - i);
        for j in 0..chunk_len {
            print!("{:02x} ", buf[i + j]);
        }
        print!(" ");
        for _ in chunk_len..16 {
            print!("   ");
        }
        for byte in &buf[i..(i + chunk_len)] {
            if byte.is_ascii_graphic() || *byte == b' ' {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        println!()
    }
}

fn parse_value<T: core::str::FromStr + num_traits::Num>(value: &str) -> Option<T> {
    if value.starts_with("0x") {
        T::from_str_radix(value.strip_prefix("0x").unwrap(), 16).ok()
    } else {
        value.parse::<T>().ok()
    }
}
