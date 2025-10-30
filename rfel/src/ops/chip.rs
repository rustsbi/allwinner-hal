use crate::chips::{self, DdrProfile};
use crate::fel::Fel;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct ResetResult {
    pub chip_name: String,
}

#[derive(Debug)]
pub struct SidResult {
    pub chip_name: String,
    pub sid: Vec<u8>,
}

#[derive(Debug)]
pub struct JtagResult {
    pub chip_name: String,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct DdrResult {
    pub chip_name: String,
    pub profile: Option<DdrProfile>,
}

#[derive(Debug)]
pub enum ChipOpError {
    Chip(chips::ChipError),
    InvalidArgument(&'static str),
    Io(io::Error),
}

impl fmt::Display for ChipOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChipOpError::Chip(err) => write!(f, "{err}"),
            ChipOpError::InvalidArgument(msg) => write!(f, "invalid argument: {msg}"),
            ChipOpError::Io(err) => write!(f, "I/O error: {err}"),
        }
    }
}

impl Error for ChipOpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChipOpError::Chip(err) => Some(err),
            ChipOpError::InvalidArgument(_) => None,
            ChipOpError::Io(err) => Some(err),
        }
    }
}

impl From<chips::ChipError> for ChipOpError {
    fn from(err: chips::ChipError) -> Self {
        ChipOpError::Chip(err)
    }
}

impl From<io::Error> for ChipOpError {
    fn from(err: io::Error) -> Self {
        ChipOpError::Io(err)
    }
}

pub type ChipOpResult<T> = Result<T, ChipOpError>;

pub fn reset(chip: &dyn chips::Chip, fel: &Fel<'_>) -> ChipOpResult<ResetResult> {
    chip.reset(fel)?;
    Ok(ResetResult {
        chip_name: chip.name(),
    })
}

pub fn sid(chip: &dyn chips::Chip, fel: &Fel<'_>) -> ChipOpResult<SidResult> {
    let sid = chip.sid(fel)?;
    Ok(SidResult {
        chip_name: chip.name(),
        sid,
    })
}

pub fn jtag(chip: &dyn chips::Chip, fel: &Fel<'_>, enable: bool) -> ChipOpResult<JtagResult> {
    chip.jtag(fel, enable)?;
    Ok(JtagResult {
        chip_name: chip.name(),
        enabled: enable,
    })
}

pub fn ddr(
    chip: &dyn chips::Chip,
    fel: &Fel<'_>,
    profile_raw: Option<&str>,
) -> ChipOpResult<DdrResult> {
    let profile = profile_raw
        .and_then(|s| {
            let trimmed = s.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .map(|raw| raw.parse::<DdrProfile>());

    let profile = match profile {
        None => None,
        Some(Ok(p)) => Some(p),
        Some(Err(_)) => return Err(ChipOpError::InvalidArgument("unknown DDR profile")),
    };

    chip.ddr(fel, profile)?;
    Ok(DdrResult {
        chip_name: chip.name(),
        profile,
    })
}
