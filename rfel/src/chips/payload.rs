/// reset/sid payloads
pub const READ32: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/read32.bin"
));
pub const WRITE32: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/write32.bin"
));

// JTAG/DDR payload
pub const JTAG_ENABLE_D1: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/jtag_d1.bin"
));
pub const DDR_INIT_D1: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/ddr_d1.bin"
));
pub const DDR_INIT_F133: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/ddr_f133.bin"
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payloads_present() {
        // The repo includes these payloads under assets/payloads, ensure they got embedded.
        assert!(!READ32.is_empty(), "read32.bin should be embedded");
        assert!(!WRITE32.is_empty(), "write32.bin should be embedded");
        assert!(!JTAG_ENABLE_D1.is_empty(), "jtag_d1.bin should be embedded");
        assert!(!DDR_INIT_D1.is_empty(), "ddr_d1.bin should be embedded");
        assert!(!DDR_INIT_F133.is_empty(), "ddr_f133.bin should be embedded");
    }
}
