use crate::chips::ChipError;
use crate::fel::error::FelError;
use crate::ops::{ChipOpError, FelOpError, FlashDetectError, FlashIoError};
use crate::spi::SpiError;

/// Result type for crate-level operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors exposed by the crate's high-level operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Fel(#[from] FelError),
    #[error(transparent)]
    Chip(#[from] ChipError),
    #[error(transparent)]
    Spi(#[from] SpiError),
    #[error(transparent)]
    FelOperation(#[from] FelOpError),
    #[error(transparent)]
    ChipOperation(#[from] ChipOpError),
    #[error(transparent)]
    FlashDetection(#[from] FlashDetectError),
    #[error(transparent)]
    FlashIo(#[from] FlashIoError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
