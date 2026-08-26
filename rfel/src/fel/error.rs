/// Result type for FEL protocol operations.
pub type FelResult<T> = core::result::Result<T, FelError>;

/// Errors reported by a FEL status acknowledgment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum FelError {
    /// The claimed USB interface does not expose both FEL bulk endpoints.
    #[error("FEL interface is missing a bulk IN or bulk OUT endpoint")]
    MissingBulkEndpoints,
    /// The response is not a valid FEL status acknowledgment.
    #[error("invalid FEL ACK marker: expected 0xffff, got 0x{marker:04x}")]
    InvalidAckMarker { marker: u16 },
    /// The device rejected the FEL request.
    #[error("FEL request failed: subcommand 0x{subcommand:04x}, status 0x{status:02x}")]
    RequestFailed { subcommand: u16, status: u8 },
    /// A USB read completed with a payload shorter or longer than requested.
    #[error("unexpected FEL USB read length: expected {expected} bytes, received {actual}")]
    UnexpectedReadLength { expected: usize, actual: usize },
}

pub(crate) fn check_fel_ack(bytes: [u8; 8]) -> FelResult<()> {
    let marker = u16::from_le_bytes([bytes[0], bytes[1]]);
    if marker != u16::MAX {
        return Err(FelError::InvalidAckMarker { marker });
    }

    let subcommand = u16::from_le_bytes([bytes[2], bytes[3]]);
    let status = bytes[4];
    if status != 0 {
        return Err(FelError::RequestFailed { subcommand, status });
    }

    Ok(())
}

pub(crate) fn check_usb_read_length(bytes: &[u8], expected: usize) -> FelResult<()> {
    if bytes.len() == expected {
        return Ok(());
    }

    // Some rejected FEL reads return the eight-byte command status where the
    // requested data payload would normally appear. Preserve that useful error
    // instead of reporting only a generic short read.
    if let Ok(ack) = <[u8; 8]>::try_from(bytes)
        && let Err(err @ FelError::RequestFailed { .. }) = check_fel_ack(ack)
    {
        return Err(err);
    }

    Err(FelError::UnexpectedReadLength {
        expected,
        actual: bytes.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_successful_ack() {
        assert_eq!(check_fel_ack([0xff, 0xff, 0x34, 0x12, 0, 0, 0, 0]), Ok(()));
    }

    #[test]
    fn rejects_invalid_ack_marker() {
        assert_eq!(
            check_fel_ack([0x55, 0xaa, 0x34, 0x12, 0, 0, 0, 0]),
            Err(FelError::InvalidAckMarker { marker: 0xaa55 })
        );
    }

    #[test]
    fn reports_ack_status_and_subcommand() {
        assert_eq!(
            check_fel_ack([0xff, 0xff, 0x34, 0x12, 1, 0, 0, 0]),
            Err(FelError::RequestFailed {
                subcommand: 0x1234,
                status: 1,
            })
        );
    }

    #[test]
    fn reports_failed_status_instead_of_short_read() {
        assert_eq!(
            check_usb_read_length(&[0xff, 0xff, 0x34, 0x12, 1, 0, 0, 0], 4096),
            Err(FelError::RequestFailed {
                subcommand: 0x1234,
                status: 1,
            })
        );
    }

    #[test]
    fn reports_unrecognized_short_read() {
        assert_eq!(
            check_usb_read_length(&[1, 2, 3], 4096),
            Err(FelError::UnexpectedReadLength {
                expected: 4096,
                actual: 3,
            })
        );
    }
}
