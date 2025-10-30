use log::debug;
use std::error::Error;
use std::fmt;

use crate::Fel;

pub mod d1;
pub mod payload;
pub mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DdrProfile {
    /// D1 / D1s / D1-H
    D1,
    /// F133 / T113
    F133,
}

impl core::str::FromStr for DdrProfile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = s.trim().to_ascii_lowercase();
        match t.as_str() {
            "d1" | "d1s" | "d1-h" => Ok(DdrProfile::D1),
            "f133" | "t113" => Ok(DdrProfile::F133),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum ChipError {
    /// NotImplemented
    NotImplemented(&'static str),
    /// Unsupported operation or args
    Unsupported(&'static str),
    /// other
    Other(&'static str),
}

impl fmt::Display for ChipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChipError::NotImplemented(msg) => write!(f, "not implemented: {msg}"),
            ChipError::Unsupported(msg) => write!(f, "unsupported operation: {msg}"),
            ChipError::Other(msg) => write!(f, "chip error: {msg}"),
        }
    }
}

impl Error for ChipError {}

pub trait Chip {
    fn name(&self) -> String;
    fn reset(&self, fel: &Fel<'_>) -> Result<(), ChipError>;
    fn sid(&self, fel: &Fel<'_>) -> Result<Vec<u8>, ChipError>;
    fn jtag(&self, fel: &Fel<'_>, enable: bool) -> Result<(), ChipError>;
    fn ddr(&self, fel: &Fel<'_>, profile: Option<DdrProfile>) -> Result<(), ChipError>;
    fn as_spi(&self) -> Option<&dyn ChipSpi> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpiContext {
    pub payload_base: u32,
    pub command_base: u32,
    pub command_len: u32,
    pub swap_base: u32,
    pub swap_len: u32,
}

pub trait ChipSpi {
    fn spi_init(&self, fel: &Fel<'_>) -> Result<SpiContext, ChipError>;
    fn spi_run(
        &self,
        fel: &Fel<'_>,
        context: &SpiContext,
        commands: &[u8],
    ) -> Result<(), ChipError>;
}

pub fn detect_from_fel(fel: &Fel<'_>) -> Option<Box<dyn Chip>> {
    let v = fel.get_version();
    debug!("detect_from_fel: version = {:x?}", v);
    match v.chip() {
        Some(crate::Chip::D1) => Some(Box::new(d1::D1)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ddr_profile_parse() {
        assert_eq!("d1".parse::<DdrProfile>(), Ok(DdrProfile::D1));
        assert_eq!("D1S".parse::<DdrProfile>(), Ok(DdrProfile::D1));
        assert_eq!("d1-h".parse::<DdrProfile>(), Ok(DdrProfile::D1));
        assert_eq!("f133".parse::<DdrProfile>(), Ok(DdrProfile::F133));
        assert_eq!("T113".parse::<DdrProfile>(), Ok(DdrProfile::F133));
        assert!("abc".parse::<DdrProfile>().is_err());
    }
}
