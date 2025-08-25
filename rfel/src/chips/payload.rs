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
