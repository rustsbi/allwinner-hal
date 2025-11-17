pub mod elf_to_bin;
pub mod patch;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::{debug, error};
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use elf_to_bin::{elf_to_bin, resolve_output_path};

use crate::Progress;
use crate::chips;
use crate::fel::Fel;
use crate::ops::{self, spinand, spinor};

mod util;

#[derive(Parser)]
#[command(
    name = "rfel",
    about = "Allwinner FEL tool",
    long_about = None,
    version,
    disable_help_subcommand = true,
    help_template = r#"rfel(v{version}) - https://github.com/rustsbi/allwinner-hal
usage:
    rfel version                                        - Show chip version
    rfel elf2bin --input <input-elf> [--output <output-bin>] - Convert ELF to raw binary data
    rfel patch --input <input-bin>  [--output <output-img>] - Patch binary into bootable image
    rfel hexdump <address> <length>                     - Dumps memory region in hex
    rfel dump <address> <length>                        - Binary memory dump to stdout
    rfel read32 <address>                               - Read 32-bits value from device memory
    rfel write32 <address> <value>                      - Write 32-bits value to device memory
    rfel read <address> <length> <file>                 - Read memory to file
    rfel write <address> <file>                         - Write file to memory
    rfel exec <address>                                 - Call function address
    rfel reset                                          - Reset device using watchdog
    rfel sid                                            - Show sid information
    rfel jtag                                           - Enable jtag debug
    rfel ddr [type]                                     - Initial ddr controller with optional type
    rfel sign <public-key> <private-key> <file>         - Generate ecdsa256 signature file for sha256 of sid
    rfel spinor                                         - Detect spi nor flash
    rfel spinor erase <address> <length>                - Erase spi nor flash
    rfel spinor read <address> <length> <file>          - Read spi nor flash to file
    rfel spinor write <address> <file>                  - Write file to spi nor flash
    rfel spinand                                        - Detect spi nand flash
    rfel spinand erase <address> <length>               - Erase spi nand flash
    rfel spinand read <address> <length> <file>         - Read spi nand flash to file
    rfel spinand write <address> <file>                 - Write file to spi nand flash
    rfel spinand splwrite <split-size> <address> <file> - Write file to spi nand flash with split support
    rfel extra [...]                                    - The extra commands
"#
)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Show chip version
    Version,
    /// Convert ELF to raw binary data.
    #[command(name = "elf2bin")]
    Elf2Bin {
        /// Input ELF file path.
        #[arg(long = "input", short = 'i')]
        input: PathBuf,
        /// Output binary file path (optional).
        #[arg(long = "output", short = 'o')]
        output: Option<PathBuf>,
    },
    #[command(name = "patch")]
    Patch {
        /// Input ELF file path.
        #[arg(long = "input", short = 'i')]
        input: PathBuf,
        /// Output binary file path (optional).
        #[arg(long = "output", short = 'o')]
        output: Option<PathBuf>,
    },
    /// Dumps memory region in hexadecimal format
    Hexdump {
        /// The address to be dumped
        address: String,
        /// Length of memory to be dumped
        length: String,
    },
    /// Binary memory dump to stdout
    Dump {
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
    /// Execute code at address: exec <address>
    Exec { address: String },
    /// Reset device using watchdog
    Reset,
    /// Show sid information
    Sid,
    /// Enable JTAG debug (pass --disable to turn it off)
    Jtag {
        #[arg(long, help = "Disable JTAG instead of enabling it")]
        disable: bool,
    },
    /// Initial ddr controller with optional type
    Ddr {
        #[arg(long)]
        profile: Option<String>,
    },
    /// Generate ECDSA signature file for the SID hash
    Sign {
        public_key: String,
        private_key: String,
        file: String,
    },
    /// Operate on SPI NOR flash
    Spinor {
        #[command(subcommand)]
        command: Option<SpinorCommand>,
    },
    /// Operate on SPI NAND flash
    Spinand {
        #[command(subcommand)]
        command: Option<SpinandCommand>,
    },
    /// Placeholder for passthrough extras
    Extra {
        #[arg(num_args = 1.., value_name = "args", trailing_var_arg = true)]
        args: Vec<String>,
    },
}

impl Commands {
    fn requires_device(&self) -> bool {
        match self {
            Commands::Elf2Bin { .. } | Commands::Patch { .. } => false,
            _ => true,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum SpinorCommand {
    /// Detect SPI NOR flash
    Detect,
    /// Erase a range: erase <address> <length>
    Erase { address: String, length: String },
    /// Read into a file: read <address> <length> <file>
    Read {
        address: String,
        length: String,
        file: String,
    },
    /// Write from a file: write <address> <file>
    Write { address: String, file: String },
}

#[derive(Debug, Subcommand)]
pub enum SpinandCommand {
    /// Detect SPI NAND flash
    Detect,
    /// Erase a range: erase <address> <length>
    Erase { address: String, length: String },
    /// Read into a file: read <address> <length> <file>
    Read {
        address: String,
        length: String,
        file: String,
    },
    /// Write from a file: write <address> <file>
    Write { address: String, file: String },
    /// Write SPL image with split support: splwrite <split-size> <address> <file>
    #[command(name = "splwrite")]
    SplWrite {
        #[arg(value_name = "split-size")]
        split_size: String,
        address: String,
        file: String,
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
    UnimplementedCommand(String),
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
            CliError::UnimplementedCommand(cmd) => {
                write!(f, "command '{cmd}' is not implemented yet")
            }
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
    let Cli { verbose, command } = cli;

    env_logger::Builder::new()
        .filter_level(verbose.log_level_filter())
        .init();

    if !command.requires_device() {
        return execute_host_command(command);
    }

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

    execute_device_command(command, &fel, chip.as_ref())
}

fn execute_host_command(command: Commands) -> Result<(), CliError> {
    match command {
        Commands::Elf2Bin { input, output } => {
            let output_path = resolve_output_path(&input, output, "bin");
            match elf_to_bin(&input, &output_path) {
                Ok(()) => {
                    println!(
                        "converted ELF {} -> binary {}",
                        input.display(),
                        output_path.display()
                    );
                    Ok(())
                }
                Err(err) => {
                    println!("error: elf2bin: {}", err);
                    Ok(())
                }
            }
        }
        Commands::Patch { input, output } => {
            let output = output.unwrap_or_else(|| input.clone());
            match patch::patch_image(&input, &output) {
                Ok(()) => {
                    println!(
                        "patched Bin {} -> image {}",
                        input.display(),
                        output.display()
                    );
                    Ok(())
                }
                Err(err) => {
                    println!("error: patch: {}", err);
                    Ok(())
                }
            }
        }
        _ => unreachable!("host command invoked for device-only variant"),
    }
}

fn execute_device_command(
    command: Commands,
    fel: &Fel<'_>,
    chip: &dyn chips::Chip,
) -> Result<(), CliError> {
    match command {
        Commands::Elf2Bin { .. } => unreachable!("device command invoked for host-only variant"),
        Commands::Patch { .. } => unreachable!("device command invoked for host-only variant"),
        Commands::Version => {
            let info = ops::op_version(fel);
            println!("chip: {}", chip.name());
            println!("{:x?}", info.version);
            Ok(())
        }
        Commands::Hexdump { address, length } => {
            let address = match util::parse_value::<usize>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return Ok(());
                }
            };
            let length = match util::parse_value::<usize>(&length) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid data length: {}", err);
                    return Ok(());
                }
            };
            if let Err(err) = ops::op_hexdump(fel, address, length, |line| {
                util::hexdump(line.data, line.base);
            }) {
                println!("error: hexdump: {}", err);
            }
            Ok(())
        }
        Commands::Dump { address, length } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("error: invalid address: {}", err);
                    return Ok(());
                }
            };
            let length = match util::parse_value::<usize>(&length) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("error: invalid length: {}", err);
                    return Ok(());
                }
            };
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            if let Err(err) = ops::op_read(fel, address, length, &mut handle, None) {
                eprintln!("error: dump to stdout: {}", err);
            }
            Ok(())
        }
        Commands::Read32 { address } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return Ok(());
                }
            };
            match ops::op_read32(fel, address) {
                Ok(result) => println!("0x{:08x}", result.value),
                Err(err) => println!("error: read32: {}", err),
            }
            Ok(())
        }
        Commands::Write32 { address, value } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return Ok(());
                }
            };
            let value = match util::parse_value::<u32>(&value) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid value: {}", err);
                    return Ok(());
                }
            };
            if let Err(err) = ops::op_write32(fel, address, value) {
                println!("error: write32: {}", err);
            }
            Ok(())
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
                    return Ok(());
                }
            };
            let length = match util::parse_value::<usize>(&length) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid length: {}", err);
                    return Ok(());
                }
            };
            let file_handle = match File::create(&file) {
                Ok(f) => f,
                Err(e) => {
                    println!("error: create file {}: {}", file, e);
                    return Ok(());
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
            Ok(())
        }
        Commands::Write { address, file } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return Ok(());
                }
            };
            let file_handle = match File::open(&file) {
                Ok(f) => f,
                Err(e) => {
                    println!("error: open file {}: {}", file, e);
                    return Ok(());
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
            Ok(())
        }
        Commands::Exec { address } => {
            let address = match util::parse_value::<u32>(&address) {
                Ok(v) => v,
                Err(err) => {
                    println!("error: invalid address: {}", err);
                    return Ok(());
                }
            };
            if let Err(err) = ops::op_exec(fel, address) {
                println!("error: exec: {}", err);
            } else {
                println!("exec at 0x{:08x}", address);
            }
            Ok(())
        }
        Commands::Reset => {
            println!("resetting...");
            match ops::op_reset(chip, fel) {
                Ok(result) => println!("reset done ({})", result.chip_name),
                Err(err) => println!("error: reset: {}", err),
            }
            Ok(())
        }
        Commands::Sid => {
            match ops::op_sid(chip, fel) {
                Ok(result) => {
                    print!("sid ({}): ", result.chip_name);
                    for b in &result.sid {
                        print!("{:02x}", b);
                    }
                    println!();
                }
                Err(err) => println!("error: sid: {}", err),
            }
            Ok(())
        }
        Commands::Jtag { disable } => {
            let enable = !disable;
            match ops::op_jtag(chip, fel, enable) {
                Ok(result) => println!(
                    "jtag {}abled ({})",
                    if result.enabled { "en" } else { "dis" },
                    result.chip_name
                ),
                Err(err) => println!("error: jtag: {}", err),
            }
            Ok(())
        }
        Commands::Ddr { profile } => {
            match ops::op_ddr(chip, fel, profile.as_deref()) {
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
            }
            Ok(())
        }
        Commands::Sign { .. } => Err(CliError::UnimplementedCommand("sign".to_string())),
        Commands::Spinor { command } => {
            let sub = command.unwrap_or(SpinorCommand::Detect);
            match sub {
                SpinorCommand::Detect => match spinor::detect(chip, fel) {
                    Ok(info) => print_flash_info("spi nor", &info.name, info.capacity),
                    Err(err) => println!("error: spinor detect: {}", err),
                },
                SpinorCommand::Erase { address, length } => {
                    let address = match util::parse_value::<u64>(&address) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid address: {}", err);
                            return Ok(());
                        }
                    };
                    let length = match util::parse_value::<u64>(&length) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid length: {}", err);
                            return Ok(());
                        }
                    };
                    let mut progress = Progress::new("NORER", length);
                    match spinor::erase(chip, fel, address, length, Some(&mut progress)) {
                        Ok(()) => {
                            progress.finish();
                            println!("erased {} bytes at 0x{:016x}", length, address);
                        }
                        Err(err) => println!("error: spinor erase: {}", err),
                    }
                }
                SpinorCommand::Read {
                    address,
                    length,
                    file,
                } => {
                    let address = match util::parse_value::<u64>(&address) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid address: {}", err);
                            return Ok(());
                        }
                    };
                    let length = match util::parse_value::<usize>(&length) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid length: {}", err);
                            return Ok(());
                        }
                    };
                    let file_handle = match File::create(&file) {
                        Ok(f) => f,
                        Err(err) => {
                            println!("error: create file {}: {}", file, err);
                            return Ok(());
                        }
                    };
                    let mut writer = BufWriter::new(file_handle);
                    let mut data = vec![0u8; length];
                    let mut progress = Progress::new("NORRD", length as u64);
                    match spinor::read(chip, fel, address, &mut data, Some(&mut progress)) {
                        Ok(()) => {
                            progress.finish();
                            if let Err(err) = writer.write_all(&data) {
                                println!("error: write {}: {}", file, err);
                            } else if let Err(err) = writer.flush() {
                                println!("error: flush {}: {}", file, err);
                            } else {
                                println!(
                                    "read {} bytes from 0x{:016x} -> {}",
                                    data.len(),
                                    address,
                                    file
                                );
                            }
                        }
                        Err(err) => println!("error: spinor read: {}", err),
                    }
                }
                SpinorCommand::Write { address, file } => {
                    let address = match util::parse_value::<u64>(&address) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid address: {}", err);
                            return Ok(());
                        }
                    };
                    let data = match fs::read(&file) {
                        Ok(d) => d,
                        Err(err) => {
                            println!("error: read file {}: {}", file, err);
                            return Ok(());
                        }
                    };
                    let mut progress = Progress::new("NORWR", data.len() as u64);
                    match spinor::write(chip, fel, address, &data, Some(&mut progress)) {
                        Ok(()) => {
                            progress.finish();
                            println!(
                                "write {} bytes from {} -> 0x{:016x}",
                                data.len(),
                                file,
                                address
                            );
                        }
                        Err(err) => println!("error: spinor write: {}", err),
                    }
                }
            }
            Ok(())
        }
        Commands::Spinand { command } => {
            let sub = command.unwrap_or(SpinandCommand::Detect);
            match sub {
                SpinandCommand::Detect => match spinand::detect(chip, fel) {
                    Ok(info) => print_flash_info("spi nand", &info.name, info.capacity),
                    Err(err) => println!("error: spinand detect: {}", err),
                },
                SpinandCommand::Erase { address, length } => {
                    let address = match util::parse_value::<u64>(&address) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid address: {}", err);
                            return Ok(());
                        }
                    };
                    let length = match util::parse_value::<u64>(&length) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid length: {}", err);
                            return Ok(());
                        }
                    };
                    match spinand::erase(chip, fel, address, length) {
                        Ok(()) => println!("erased {} bytes at 0x{:016x}", length, address),
                        Err(err) => println!("error: spinand erase: {}", err),
                    }
                }
                SpinandCommand::Read {
                    address,
                    length,
                    file,
                } => {
                    let address = match util::parse_value::<u64>(&address) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid address: {}", err);
                            return Ok(());
                        }
                    };
                    let length = match util::parse_value::<usize>(&length) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid length: {}", err);
                            return Ok(());
                        }
                    };
                    let file_handle = match File::create(&file) {
                        Ok(f) => f,
                        Err(err) => {
                            println!("error: create file {}: {}", file, err);
                            return Ok(());
                        }
                    };
                    let mut writer = BufWriter::new(file_handle);
                    let mut data = vec![0u8; length];
                    let mut progress = Progress::new("NDRD", length as u64);
                    match spinand::read(chip, fel, address, &mut data, Some(&mut progress)) {
                        Ok(()) => {
                            progress.finish();
                            if let Err(err) = writer.write_all(&data) {
                                println!("error: write {}: {}", file, err);
                            } else if let Err(err) = writer.flush() {
                                println!("error: flush {}: {}", file, err);
                            } else {
                                println!(
                                    "read {} bytes from 0x{:016x} -> {}",
                                    data.len(),
                                    address,
                                    file
                                );
                            }
                        }
                        Err(err) => println!("error: spinand read: {}", err),
                    }
                }
                SpinandCommand::Write { address, file } => {
                    let address = match util::parse_value::<u64>(&address) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid address: {}", err);
                            return Ok(());
                        }
                    };
                    let data = match fs::read(&file) {
                        Ok(d) => d,
                        Err(err) => {
                            println!("error: read file {}: {}", file, err);
                            return Ok(());
                        }
                    };
                    let mut progress = Progress::new("NDWR", data.len() as u64);
                    match spinand::write(chip, fel, address, &data, Some(&mut progress)) {
                        Ok(()) => {
                            progress.finish();
                            println!(
                                "write {} bytes from {} -> 0x{:016x}",
                                data.len(),
                                file,
                                address
                            );
                        }
                        Err(err) => println!("error: spinand write: {}", err),
                    }
                }
                SpinandCommand::SplWrite {
                    split_size,
                    address,
                    file,
                } => {
                    let split_size = match util::parse_value::<u32>(&split_size) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid split-size: {}", err);
                            return Ok(());
                        }
                    };
                    if split_size == 0 {
                        println!("error: split-size must be greater than zero");
                        return Ok(());
                    }
                    let address = match util::parse_value::<u64>(&address) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("error: invalid address: {}", err);
                            return Ok(());
                        }
                    };
                    let data = match fs::read(&file) {
                        Ok(d) => d,
                        Err(err) => {
                            println!("error: read file {}: {}", file, err);
                            return Ok(());
                        }
                    };
                    match spinand::spl_write(chip, fel, split_size, address, &data) {
                        Ok(()) => println!(
                            "splwrite {} bytes from {} -> 0x{:016x} (split {})",
                            data.len(),
                            file,
                            address,
                            split_size
                        ),
                        Err(err) => println!("error: spinand splwrite: {}", err),
                    }
                }
            }
            Ok(())
        }
        Commands::Extra { args } => Err(CliError::UnimplementedCommand(format_command(
            "extra", &args,
        ))),
    }
}

fn print_flash_info(kind: &str, name: &str, capacity: u64) {
    let pretty = format_size(capacity);
    println!("{kind}: {name} ({pretty} / {capacity} bytes)");
}

fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut value = bytes as f64;
    let mut idx = 0usize;
    while value >= 1024.0 && idx + 1 < UNITS.len() {
        value /= 1024.0;
        idx += 1;
    }
    if idx == 0 {
        format!("{:.0}{}", value, UNITS[idx])
    } else {
        format!("{:.2}{}", value, UNITS[idx])
    }
}

fn format_command(command: &str, args: &[String]) -> String {
    if args.is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, args.join(" "))
    }
}

const VENDOR_ALLWINNER: u16 = 0x1f3a;
const PRODUCT_FEL: u16 = 0xefe8;
