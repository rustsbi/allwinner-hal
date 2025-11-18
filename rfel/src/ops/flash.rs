use std::error::Error;
use std::fmt;

use crate::chips::Chip;
use crate::fel::Fel;
use crate::ops::{spinand, spinor};
use crate::progress::Progress;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashKind {
    Spinand,
    Spinor,
}

impl FlashKind {
    pub fn display_name(self) -> &'static str {
        match self {
            FlashKind::Spinand => "spi nand",
            FlashKind::Spinor => "spi nor",
        }
    }

    pub fn progress_tag_read(self) -> &'static str {
        match self {
            FlashKind::Spinand => "NDRD",
            FlashKind::Spinor => "NORRD",
        }
    }

    pub fn progress_tag_write(self) -> &'static str {
        match self {
            FlashKind::Spinand => "NDWR",
            FlashKind::Spinor => "NORWR",
        }
    }

    pub fn progress_tag_erase(self) -> &'static str {
        match self {
            FlashKind::Spinand => "NDER",
            FlashKind::Spinor => "NORER",
        }
    }
}

pub struct FlashAccess<'chip> {
    chip: &'chip dyn Chip,
    pub kind: FlashKind,
    pub name: String,
    pub capacity: u64,
}

impl<'chip> FlashAccess<'chip> {
    pub fn detect(chip: &'chip dyn Chip, fel: &Fel<'_>) -> Result<Self, FlashDetectError> {
        match spinand::detect(chip, fel) {
            Ok(info) => Ok(Self::from_spinand(chip, info)),
            Err(spinand_err) => match spinor::detect(chip, fel) {
                Ok(info) => Ok(Self::from_spinor(chip, info)),
                Err(spinor_err) => Err(FlashDetectError::NoFlash {
                    spinand: spinand_err,
                    spinor: spinor_err,
                }),
            },
        }
    }

    fn from_spinand(chip: &'chip dyn Chip, info: spinand::DetectInfo) -> Self {
        Self {
            chip,
            kind: FlashKind::Spinand,
            name: info.name,
            capacity: info.capacity,
        }
    }

    fn from_spinor(chip: &'chip dyn Chip, info: spinor::DetectInfo) -> Self {
        Self {
            chip,
            kind: FlashKind::Spinor,
            name: info.name,
            capacity: info.capacity,
        }
    }

    pub fn read(
        &self,
        fel: &Fel<'_>,
        address: u64,
        buffer: &mut [u8],
        progress: Option<&mut Progress>,
    ) -> Result<(), FlashIoError> {
        match self.kind {
            FlashKind::Spinand => spinand::read(self.chip, fel, address, buffer, progress)
                .map_err(FlashIoError::Spinand),
            FlashKind::Spinor => spinor::read(self.chip, fel, address, buffer, progress)
                .map_err(FlashIoError::Spinor),
        }
    }

    pub fn write(
        &self,
        fel: &Fel<'_>,
        address: u64,
        data: &[u8],
        progress: Option<&mut Progress>,
    ) -> Result<(), FlashIoError> {
        match self.kind {
            FlashKind::Spinand => spinand::write(self.chip, fel, address, data, progress)
                .map_err(FlashIoError::Spinand),
            FlashKind::Spinor => {
                spinor::write(self.chip, fel, address, data, progress).map_err(FlashIoError::Spinor)
            }
        }
    }

    pub fn erase(
        &self,
        fel: &Fel<'_>,
        address: u64,
        length: u64,
        progress: Option<&mut Progress>,
    ) -> Result<(), FlashIoError> {
        match self.kind {
            FlashKind::Spinand => spinand::erase(self.chip, fel, address, length, progress)
                .map_err(FlashIoError::Spinand),
            FlashKind::Spinor => spinor::erase(self.chip, fel, address, length, progress)
                .map_err(FlashIoError::Spinor),
        }
    }
}

impl fmt::Debug for FlashAccess<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlashAccess")
            .field("kind", &self.kind)
            .field("name", &self.name)
            .field("capacity", &self.capacity)
            .finish()
    }
}

pub enum FlashDetectError {
    NoFlash {
        spinand: spinand::SpinandError,
        spinor: spinor::SpinorError,
    },
}

impl fmt::Debug for FlashDetectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlashDetectError::NoFlash { spinand, spinor } => f
                .debug_struct("FlashDetectError::NoFlash")
                .field("spinand", spinand)
                .field("spinor", spinor)
                .finish(),
        }
    }
}

impl fmt::Display for FlashDetectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlashDetectError::NoFlash { spinand, spinor } => write!(
                f,
                "failed to detect spi nand ({spinand}) or spi nor ({spinor}) flash"
            ),
        }
    }
}

impl Error for FlashDetectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FlashDetectError::NoFlash { .. } => None,
        }
    }
}

#[derive(Debug)]
pub enum FlashIoError {
    Spinand(spinand::SpinandError),
    Spinor(spinor::SpinorError),
}

impl fmt::Display for FlashIoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlashIoError::Spinand(err) => write!(f, "spi nand error: {err}"),
            FlashIoError::Spinor(err) => write!(f, "spi nor error: {err}"),
        }
    }
}

impl Error for FlashIoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FlashIoError::Spinand(err) => Some(err),
            FlashIoError::Spinor(err) => Some(err),
        }
    }
}
