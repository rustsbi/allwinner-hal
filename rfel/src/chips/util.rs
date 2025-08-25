use log::trace;

use crate::{Fel, read_all, write_all};

use super::{ChipError, payload};

/// Execute a payload with parameters at the scratchpad base address
/// Note: This function hasn't gone through comprehensive upper-level testing yet.
pub fn exec_stub(
    fel: &Fel<'_>,
    payload: &[u8],
    params_le: &[u8],
    out_len: usize, // Length of the output buffer to read after execution
) -> Result<Vec<u8>, ChipError> {
    if payload.is_empty() {
        return Err(ChipError::NotImplemented("payload is empty"));
    }
    let base = fel.get_version().scratchpad();

    trace!(
        "exec_stub: base=0x{base:08x}, payload_len={}, params_len={}, out_len={}",
        payload.len(),
        params_le.len(),
        out_len
    );

    // Write payload in chunks
    write_all(fel, base, payload);
    // Write params in chunks, immediately after the payload
    let params_addr = base.wrapping_add(payload.len() as u32);
    write_all(fel, params_addr, params_le);
    // Execute
    fel.exec(base);

    // Read return buffer (if requested)
    if out_len == 0 {
        return Ok(Vec::new());
    }
    let out_addr = params_addr.wrapping_add(params_le.len() as u32);
    let mut out = vec![0u8; out_len];
    read_all(fel, out_addr, &mut out);
    Ok(out)
}

#[inline]
pub fn u32_params_le(params: &[u32]) -> Vec<u8> {
    let mut b = Vec::with_capacity(params.len() * 4);
    for &p in params {
        b.extend_from_slice(&p.to_le_bytes());
    }
    b
}

/// Read a 32-bit register via read32 stub (executes at scratchpad)
pub fn read32_via_stub(fel: &Fel<'_>, addr: u32) -> Result<u32, ChipError> {
    let payload = payload::READ32;
    if payload.is_empty() {
        return Err(ChipError::NotImplemented(
            "read32 stub missing: put assets/payloads/read32.bin",
        ));
    }
    let out = exec_stub(fel, payload, &u32_params_le(&[addr]), 4)?;
    Ok(u32::from_le_bytes(out.try_into().unwrap()))
}

/// Write a 32-bit register via write32 stub (executes at scratchpad)
pub fn write32_via_stub(fel: &Fel<'_>, addr: u32, val: u32) -> Result<(), ChipError> {
    let payload = payload::WRITE32;
    if payload.is_empty() {
        return Err(ChipError::NotImplemented(
            "write32 stub missing: put assets/payloads/write32.bin",
        ));
    }
    let _ = exec_stub(fel, payload, &u32_params_le(&[addr, val]), 0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_params_le() {
        let v = [0x11223344, 0xAABBCCDD];
        let b = u32_params_le(&v);
        assert_eq!(b, vec![0x44, 0x33, 0x22, 0x11, 0xDD, 0xCC, 0xBB, 0xAA]);
    }
}
