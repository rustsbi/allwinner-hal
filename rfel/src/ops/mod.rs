pub mod chip;
pub mod flash;
pub mod spinand;
pub mod spinor;

pub use chip::{
    ChipOpError, ChipOpResult, DdrResult, JtagResult, ResetResult, SidResult, ddr as op_ddr,
    hexdump as op_chip_hexdump, jtag as op_jtag, read as op_chip_read, read32 as op_chip_read32,
    reset as op_reset, sid as op_sid,
};
pub use flash::{FlashAccess, FlashDetectError, FlashIoError, FlashKind};

use crate::Progress;
use crate::fel::error::FelError;
use crate::fel::{CHUNK_SIZE, Fel, Version};
use crate::transfer::{read_to_writer, write_from_reader};
use std::error::Error;
use std::fmt;
use std::io::{Read, Write};

fn erase_span(address: u64, length: u64, block: u64) -> Option<(u64, u64)> {
    let mask = block.checked_sub(1)?;
    let count = (address & mask).checked_add(length)?.checked_add(mask)? & !mask;
    Some((address & !mask, count))
}

fn range_in_bounds(address: u64, length: u64, capacity: u64) -> bool {
    address
        .checked_add(length)
        .is_some_and(|end| end <= capacity)
}

#[derive(Debug)]
pub struct ReadResult {
    pub address: u32,
    pub length: usize,
}

#[derive(Debug)]
pub struct WriteResult {
    pub address: u32,
    pub written: usize,
    pub total_hint: u64,
}

#[derive(Debug)]
pub struct Read32Result {
    pub address: u32,
    pub value: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct VersionInfo {
    pub version: Version,
}

#[derive(Debug)]
pub struct HexdumpLine<'a> {
    pub base: u32,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub enum FelOpError {
    Fel(FelError),
    Io(std::io::Error),
}

impl fmt::Display for FelOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FelOpError::Fel(err) => write!(f, "FEL error: {err}"),
            FelOpError::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl Error for FelOpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FelOpError::Fel(err) => Some(err),
            FelOpError::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for FelOpError {
    fn from(err: std::io::Error) -> Self {
        FelOpError::Io(err)
    }
}

impl From<FelError> for FelOpError {
    fn from(err: FelError) -> Self {
        FelOpError::Fel(err)
    }
}

pub type FelOpResult<T> = Result<T, FelOpError>;

/// Read memory and stream the contents into the provided writer.
pub fn op_read(
    fel: &Fel<'_>,
    address: u32,
    length: usize,
    writer: &mut impl Write,
    progress: Option<&mut Progress>,
) -> FelOpResult<ReadResult> {
    let written = read_to_writer(fel, address, length, writer, progress)?;
    Ok(ReadResult {
        address,
        length: written,
    })
}

/// Write data from the reader into memory.
pub fn op_write(
    fel: &Fel<'_>,
    address: u32,
    reader: &mut impl Read,
    total_hint: u64,
    progress: Option<&mut Progress>,
) -> FelOpResult<WriteResult> {
    let written = write_from_reader(fel, address, reader, progress)?;
    Ok(WriteResult {
        address,
        written,
        total_hint,
    })
}

/// Read a 32-bit value from the specified address.
pub fn op_read32(fel: &Fel<'_>, address: u32) -> FelOpResult<Read32Result> {
    let mut buf = [0u8; 4];
    fel.read_address(address, &mut buf)?;
    Ok(Read32Result {
        address,
        value: u32::from_le_bytes(buf),
    })
}

/// Write a 32-bit value to the specified address.
pub fn op_write32(fel: &Fel<'_>, address: u32, value: u32) -> FelOpResult<()> {
    fel.write_address(address, &value.to_le_bytes())?;
    Ok(())
}

/// Execute code at the given address.
pub fn op_exec(fel: &Fel<'_>, address: u32) -> FelOpResult<()> {
    fel.exec(address)?;
    Ok(())
}

/// Retrieve the device version reported by FEL.
pub fn op_version(fel: &Fel<'_>) -> FelOpResult<VersionInfo> {
    Ok(VersionInfo {
        version: fel.get_version()?,
    })
}

/// Perform a hexdump in 16-byte lines and emit each line through the provided callback.
pub fn op_hexdump<F>(
    fel: &Fel<'_>,
    mut address: usize,
    mut length: usize,
    mut sink: F,
) -> FelOpResult<()>
where
    F: FnMut(HexdumpLine<'_>),
{
    let mut buf = vec![0u8; CHUNK_SIZE];
    while length > 0 {
        let chunk_len = length.min(buf.len());
        fel.read_address(address as u32, &mut buf[..chunk_len])?;
        for line_offset in (0..chunk_len).step_by(16) {
            let line_len = (chunk_len - line_offset).min(16);
            sink(HexdumpLine {
                base: (address + line_offset) as u32,
                data: &buf[line_offset..line_offset + line_len],
            });
        }
        address += chunk_len;
        length -= chunk_len;
    }
    Ok(())
}
