use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::{debug, error};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use crate::Progress;
use crate::chips;
use crate::fel::Fel;
use crate::ops;

mod util;

#[derive(Parser)]
#[clap(name = "rfel")]
#[clap(about = "Allwinner FEL tool", long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
    /// Read memory into a file: read <address> <length> <file>
    Read {
        address: String,
        length: String,
        file: String,
    },
    /// Write file into memory: write <address> <file>
    Write { address: String, file: String },
    /// Dump raw memory to stdout: dump <address> <length>
    Dump { address: String, length: String },
    /// Execute code at address: exec <address>
    Exec { address: String },
    /// Reset device using watchdog
    Reset,
    /// Show sid information
    Sid,
    /// Enable jtag debug
    Jtag {
        #[clap(long, default_value_t = true)]
        enable: bool,
    },
    /// Initial ddr controller with optional type
    Ddr {
        #[clap(long)]
        profile: Option<String>,
    },
}

#[derive(Debug)]
pub enum CliError {
    DeviceList(nusb::Error),
    NoDevice,
    MultipleDevices,
    OpenDevice(nusb::Error),
    ClaimInterface(nusb::Error),
    FelInterface,
    UnsupportedChip,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::DeviceList(_) => write!(f, "failed to list USB devices"),
            CliError::NoDevice => write!(f, "Cannot find any Allwinner FEL device connected."),
            CliError::MultipleDevices => write!(
                f,
                "rfel does not support connecting to multiple Allwinner FEL devices by now."
            ),
            CliError::OpenDevice(_) => write!(f, "failed to open USB device"),
            CliError::ClaimInterface(_) => write!(f, "failed to claim USB interface 0"),
            CliError::FelInterface => write!(f, "open usb interface as an FEL device"),
            CliError::UnsupportedChip => write!(f, "error: unsupported chip"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CliError::DeviceList(err)
            | CliError::OpenDevice(err)
            | CliError::ClaimInterface(err) => Some(err),
            _ => None,
        }
    }
}

pub fn run(cli: Cli) -> Result<(), CliError> {
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let devices: Vec<_> = nusb::list_devices()
        .map_err(CliError::DeviceList)?
        .filter(|dev| dev.vendor_id() == VENDOR_ALLWINNER && dev.product_id() == PRODUCT_FEL)
        .inspect(|dev| debug!("Allwinner FEL device {:?}", dev))
        .collect();

    if devices.is_empty() {
        error!("Cannot find any Allwinner FEL device connected.");
        return Err(CliError::NoDevice);
    }
    if devices.len() > 1 {
        error!("TODO: rfel does not support connecting to multiple Allwinner FEL devices by now.");
        return Err(CliError::MultipleDevices);
    }

    let device_info = devices.into_iter().next().unwrap();
    let device = device_info.open().map_err(CliError::OpenDevice)?;
    let mut interface = device
        .claim_interface(0)
        .map_err(CliError::ClaimInterface)?;
    let fel = Fel::open_interface(&mut interface).map_err(|_| CliError::FelInterface)?;
    let chip = match chips::detect_from_fel(&fel) {
        Some(chip) => chip,
        None => return Err(CliError::UnsupportedChip),
    };

    execute_command(cli.command, &fel, chip.as_ref());
    Ok(())
}

fn execute_command(command: Commands, fel: &Fel<'_>, chip: &dyn chips::Chip) {
    match command {
        Commands::Version => {
            let info = ops::op_version(fel);
            println!("chip: {}", chip.name());
            println!("{:x?}", info.version);
        }
        Commands::Hexdump { address, length } => {
            let address = match util::parse_value::<usize>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return;
                }
            };
            let length = match util::parse_value::<usize>(&length) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid data length: {}", err);
                    return;
                }
            };
            if let Err(err) = ops::op_hexdump(fel, address, length, |line| {
                util::hexdump(line.data, line.base);
            }) {
                println!("error: hexdump: {}", err);
            }
        }
        Commands::Read32 { address } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return;
                }
            };
            match ops::op_read32(fel, address) {
                Ok(result) => println!("0x{:08x}", result.value),
                Err(err) => println!("error: read32: {}", err),
            }
        }
        Commands::Write32 { address, value } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return;
                }
            };
            let value = match util::parse_value::<u32>(&value) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid value: {}", err);
                    return;
                }
            };
            if let Err(err) = ops::op_write32(fel, address, value) {
                println!("error: write32: {}", err);
            }
        }
        Commands::Read {
            address,
            length,
            file,
        } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return;
                }
            };
            let length = match util::parse_value::<usize>(&length) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid length: {}", err);
                    return;
                }
            };
            let file_handle = match File::create(&file) {
                Ok(f) => f,
                Err(e) => {
                    println!("error: create file {}: {}", file, e);
                    return;
                }
            };
            let mut writer = BufWriter::new(file_handle);
            let mut progress = Progress::new("READ", length as u64);
            match ops::op_read(fel, address, length, &mut writer, Some(&mut progress)) {
                Ok(result) => {
                    let _ = writer.flush();
                    progress.finish();
                    println!(
                        "read {} bytes from 0x{:08x} -> {}",
                        result.length, result.address, file
                    );
                }
                Err(err) => println!("error: read -> file: {}", err),
            }
        }
        Commands::Write { address, file } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return;
                }
            };
            let file_handle = match File::open(&file) {
                Ok(f) => f,
                Err(e) => {
                    println!("error: open file {}: {}", file, e);
                    return;
                }
            };
            let total = file_handle.metadata().ok().map(|m| m.len()).unwrap_or(0);
            let mut reader = BufReader::new(file_handle);
            let mut progress = Progress::new("WRITE", total);
            match ops::op_write(fel, address, &mut reader, total, Some(&mut progress)) {
                Ok(result) => {
                    progress.finish();
                    println!(
                        "write {} bytes from {} -> 0x{:08x}",
                        result.written, file, result.address
                    );
                }
                Err(err) => println!("error: file -> write: {}", err),
            }
        }
        Commands::Dump { address, length } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("error: invalid address: {}", err);
                    return;
                }
            };
            let length = match util::parse_value::<usize>(&length) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("error: invalid length: {}", err);
                    return;
                }
            };
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            if let Err(err) = ops::op_read(fel, address, length, &mut handle, None) {
                eprintln!("error: dump to stdout: {}", err);
            }
        }
        Commands::Exec { address } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return;
                }
            };
            if let Err(err) = ops::op_exec(fel, address) {
                println!("error: exec: {}", err);
            } else {
                println!("exec at 0x{:08x}", address);
            }
        }
        Commands::Reset => {
            println!("resetting...");
            match ops::op_reset(chip, fel) {
                Ok(result) => println!("reset done ({})", result.chip_name),
                Err(err) => println!("error: reset: {}", err),
            }
        }
        Commands::Sid => match ops::op_sid(chip, fel) {
            Ok(result) => {
                print!("sid ({}): ", result.chip_name);
                for b in &result.sid {
                    print!("{:02x}", b);
                }
                println!();
            }
            Err(err) => println!("error: sid: {}", err),
        },
        Commands::Jtag { enable } => match ops::op_jtag(chip, fel, enable) {
            Ok(result) => println!(
                "jtag {}abled ({})",
                if result.enabled { "en" } else { "dis" },
                result.chip_name
            ),
            Err(err) => println!("error: jtag: {}", err),
        },
        Commands::Ddr { profile } => match ops::op_ddr(chip, fel, profile.as_deref()) {
            Ok(result) => {
                let profile_label = result
                    .profile
                    .map(|p| format!("{p:?}"))
                    .unwrap_or_else(|| "unknown".to_string());
                println!(
                    "ddr init done (chip: {}, profile: {profile_label})",
                    result.chip_name
                );
            }
            Err(err) => println!("error: ddr init: {}", err),
        },
    }
}

const VENDOR_ALLWINNER: u16 = 0x1f3a;
const PRODUCT_FEL: u16 = 0xefe8;
