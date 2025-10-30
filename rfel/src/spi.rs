use std::fmt;

use crate::chips::{Chip, ChipError, ChipSpi, SpiContext};
use crate::fel::Fel;
use crate::transfer::{read_all, write_all};

const SPI_CMD_END: u8 = 0x00;
const SPI_CMD_INIT: u8 = 0x01;
const SPI_CMD_SELECT: u8 = 0x02;
const SPI_CMD_DESELECT: u8 = 0x03;
const SPI_CMD_TXBUF: u8 = 0x05;
const SPI_CMD_RXBUF: u8 = 0x06;

#[derive(Debug)]
pub enum SpiError {
    Chip(ChipError),
    Unsupported(&'static str),
    CommandTooLarge(usize),
    LengthOverflow,
}

impl fmt::Display for SpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpiError::Chip(err) => write!(f, "SPI helper error: {err}"),
            SpiError::Unsupported(msg) => write!(f, "unsupported SPI operation: {msg}"),
            SpiError::CommandTooLarge(len) => {
                write!(f, "spi command buffer too large ({len} bytes)")
            }
            SpiError::LengthOverflow => write!(f, "spi transfer length exceeds u32 range"),
        }
    }
}

impl std::error::Error for SpiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpiError::Chip(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ChipError> for SpiError {
    fn from(value: ChipError) -> Self {
        SpiError::Chip(value)
    }
}

pub struct SpiSession<'chip> {
    chip: &'chip dyn ChipSpi,
    context: SpiContext,
}

impl<'chip> SpiSession<'chip> {
    pub fn context(&self) -> &SpiContext {
        &self.context
    }

    fn run_commands(&self, fel: &Fel<'_>, commands: &[u8]) -> Result<(), SpiError> {
        if commands.len() > self.context.command_len as usize {
            return Err(SpiError::CommandTooLarge(commands.len()));
        }
        self.chip
            .spi_run(fel, &self.context, commands)
            .map_err(SpiError::Chip)
    }
}

pub fn begin<'chip>(chip: &'chip dyn Chip, fel: &Fel<'_>) -> Result<SpiSession<'chip>, SpiError> {
    let Some(spi_chip) = chip.as_spi() else {
        return Err(SpiError::Unsupported("chip does not expose SPI helpers"));
    };
    let context = spi_chip.spi_init(fel).map_err(SpiError::Chip)?;
    let session = SpiSession {
        chip: spi_chip,
        context,
    };
    session.run_commands(fel, &[SPI_CMD_INIT, SPI_CMD_END])?;
    Ok(session)
}

pub fn transfer(
    fel: &Fel<'_>,
    session: &SpiSession<'_>,
    tx: Option<&[u8]>,
    mut rx: Option<&mut [u8]>,
) -> Result<(), SpiError> {
    let swap_base = session.context.swap_base;
    let swap_len = session.context.swap_len as usize;
    let tx_len = tx.map(|buf| buf.len()).unwrap_or(0);
    let rx_len = rx.as_ref().map(|buf| buf.len()).unwrap_or(0);

    if tx_len <= swap_len && rx_len <= swap_len {
        let mut commands =
            Vec::with_capacity(2 + if tx_len > 0 { 9 } else { 0 } + if rx_len > 0 { 9 } else { 0 });
        commands.push(SPI_CMD_SELECT);
        if let Some(buf) = tx {
            push_descriptor(
                &mut commands,
                SPI_CMD_TXBUF,
                swap_base,
                ensure_u32(buf.len())?,
            );
            write_all(fel, swap_base, buf);
        }
        if let Some(buf) = rx.as_ref() {
            push_descriptor(
                &mut commands,
                SPI_CMD_RXBUF,
                swap_base,
                ensure_u32(buf.len())?,
            );
        }
        commands.push(SPI_CMD_DESELECT);
        commands.push(SPI_CMD_END);
        session.run_commands(fel, &commands)?;
        if let Some(buf) = rx.as_deref_mut() {
            read_all(fel, swap_base, buf);
        }
        return Ok(());
    }

    session.run_commands(fel, &[SPI_CMD_SELECT, SPI_CMD_END])?;

    if let Some(mut remaining) = tx {
        while !remaining.is_empty() {
            let chunk = remaining.len().min(swap_len);
            push_single_transfer(
                fel,
                session,
                SPI_CMD_TXBUF,
                swap_base,
                ensure_u32(chunk)?,
                &remaining[..chunk],
            )?;
            remaining = &remaining[chunk..];
        }
    }

    if let Some(buf) = rx.as_deref_mut() {
        let mut offset = 0usize;
        while offset < buf.len() {
            let chunk = (buf.len() - offset).min(swap_len);
            let slice = &mut buf[offset..offset + chunk];
            single_receive(fel, session, swap_base, slice)?;
            offset += chunk;
        }
    }

    session.run_commands(fel, &[SPI_CMD_DESELECT, SPI_CMD_END])?;
    Ok(())
}

fn push_descriptor(buf: &mut Vec<u8>, opcode: u8, addr: u32, len: u32) {
    buf.push(opcode);
    buf.extend_from_slice(&addr.to_le_bytes());
    buf.extend_from_slice(&len.to_le_bytes());
}

fn ensure_u32(value: usize) -> Result<u32, SpiError> {
    value.try_into().map_err(|_| SpiError::LengthOverflow)
}

fn push_single_transfer(
    fel: &Fel<'_>,
    session: &SpiSession<'_>,
    opcode: u8,
    addr: u32,
    len: u32,
    payload: &[u8],
) -> Result<(), SpiError> {
    let mut commands = Vec::with_capacity(1 + 4 + 4 + 1);
    commands.push(opcode);
    commands.extend_from_slice(&addr.to_le_bytes());
    commands.extend_from_slice(&len.to_le_bytes());
    commands.push(SPI_CMD_END);
    if opcode == SPI_CMD_TXBUF {
        write_all(fel, addr, payload);
    }
    session.run_commands(fel, &commands)
}

fn single_receive(
    fel: &Fel<'_>,
    session: &SpiSession<'_>,
    addr: u32,
    out: &mut [u8],
) -> Result<(), SpiError> {
    let len = ensure_u32(out.len())?;
    let mut commands = Vec::with_capacity(1 + 4 + 4 + 1);
    commands.push(SPI_CMD_RXBUF);
    commands.extend_from_slice(&addr.to_le_bytes());
    commands.extend_from_slice(&len.to_le_bytes());
    commands.push(SPI_CMD_END);
    session.run_commands(fel, &commands)?;
    read_all(fel, addr, out);
    Ok(())
}
