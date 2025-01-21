//! SD/MMC Host Controller peripheral.

mod register;
pub use register::*;
mod pad;
pub use pad::*;
mod structure;
pub use structure::*;

/// Transfer mode.
pub enum TransferMode {
    /// No data transfer.
    Disable,
    /// Read data.
    Read,
    /// Write data.
    Write,
}

/// Response mode.
pub enum ResponseMode {
    /// No response.
    Disable,
    /// Short response.
    Short,
    /// Long response.
    Long,
}

#[derive(Debug)]
pub enum SdCardError {
    Unknown,
    UnexpectedResponse(u8, u128),
}
