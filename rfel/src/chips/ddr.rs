//! Shared DDR upload protocol and board parameters.

use super::{ChipError, util};
use crate::{Fel, read_all, write_all};

/// Upload a position-independent image, its 24 parameter words, and a zeroed
/// result. The entry returns the detected size in MiB in the final word.
pub(super) fn run(
    fel: &Fel<'_>,
    base: u32,
    payload: &[u8],
    parameters: &[u32; 24],
    synchronize: impl FnOnce() -> Result<(), ChipError>,
) -> Result<(), ChipError> {
    let (request, result_address) = request(base, payload, parameters)?;
    write_all(fel, base, &request)?;
    synchronize()?;
    fel.exec(base)?;
    let mut result = [0; 4];
    read_all(fel, result_address, &mut result)?;
    let size = result_size(result)?;
    log::info!("DDR initialized: {size} MiB");
    Ok(())
}

fn request(base: u32, payload: &[u8], parameters: &[u32; 24]) -> Result<(Vec<u8>, u32), ChipError> {
    if payload.is_empty() {
        return Err(ChipError::NotImplemented("DDR payload is empty"));
    }
    let length = u32::try_from(payload.len())
        .ok()
        .and_then(|length| length.checked_add(100))
        .ok_or(ChipError::Other("DDR image exceeds address space"))?;
    let end = base
        .checked_add(length)
        .ok_or(ChipError::Other("DDR image exceeds address space"))?;
    let mut request = payload.to_vec();
    request.extend_from_slice(&util::u32_params_le(parameters));
    request.extend_from_slice(&0u32.to_le_bytes());
    Ok((request, end - 4))
}

fn result_size(result: [u8; 4]) -> Result<u32, ChipError> {
    match u32::from_le_bytes(result) {
        0 => Err(ChipError::Other("DDR initialization failed")),
        size => Ok(size),
    }
}

pub(super) const D1: [u32; 24] = [
    0x00000318, 0x00000003, 0x007b7bfb, 0x00000001, 0x000010d2, 0x00000000, 0x00001c70, 0x00000042,
    0x00000018, 0x00000000, 0x004a2195, 0x02423190, 0x0008b061, 0xb4787896, 0x00000000, 0x48484848,
    0x00000048, 0x1620121e, 0x00000000, 0x00000000, 0x00000000, 0x00870000, 0x00000024, 0x34050100,
];

pub(super) const F133: [u32; 24] = [
    0x00000210, 0x00000002, 0x007b7bf9, 0x00000000, 0x000000d2, 0x00000000, 0x00000e73, 0x00000002,
    0x00000000, 0x00000000, 0x00471992, 0x0131a10c, 0x00057041, 0xb4787896, 0x00000000, 0x48484848,
    0x00000048, 0x1621121e, 0x00000000, 0x00000000, 0x00000000, 0x00030010, 0x00000035, 0x34000000,
];

pub(super) const F101_S2: [u32; 24] = [
    0x000000e4, 0x00000001, 0x007bfbfb, 0x00000000, 0x00000008, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];

pub(super) const F101_S3: [u32; 24] = [
    0x000000fc, 0x00000003, 0x007bfbfb, 0x00000000, 0x00000010, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];

pub(super) const V821: [u32; 24] = [
    0x00000210, 0x00000002, 0x007b7bf9, 0x00000000, 0x000000d2, 0x00400000, 0x00000e73, 0x00000002,
    0x00000000, 0x00000000, 0x00471992, 0x0131a10c, 0x00057041, 0xb4787896, 0x00000000, 0x48484848,
    0x00000048, 0x1621121e, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x34000100,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unaligned_parameters_follow_image_without_padding() {
        let payload = [0x13, 0, 0, 0, 0xaa];
        let (bytes, result) = request(0x20000, &payload, &D1).unwrap();
        assert_eq!(result, 0x20065);
        assert_eq!(&bytes[..5], &payload);
        assert_eq!(&bytes[5..9], &792u32.to_le_bytes());
        assert_eq!(&bytes[97..101], &0x34050100u32.to_le_bytes());
        assert_eq!(&bytes[101..], &[0; 4]);
        assert!(request(u32::MAX - 100, &payload, &D1).is_err());
        assert!(request(0, &[], &D1).is_err());
    }

    #[test]
    fn initialization_failure_is_reported() {
        assert!(result_size([0; 4]).is_err());
        assert_eq!(result_size(512u32.to_le_bytes()).unwrap(), 512);
    }
}
