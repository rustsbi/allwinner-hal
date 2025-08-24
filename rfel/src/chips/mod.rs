use log::debug;

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

pub trait Chip {
    fn name(&self) -> String;
    fn reset(&self, fel: &Fel<'_>) -> Result<(), ChipError>;
    fn sid(&self, fel: &Fel<'_>) -> Result<Vec<u8>, ChipError>;
    fn jtag(&self, fel: &Fel<'_>, enable: bool) -> Result<(), ChipError>;
    fn ddr(&self, fel: &Fel<'_>, profile: Option<DdrProfile>) -> Result<(), ChipError>;
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
