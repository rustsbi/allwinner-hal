/// reset/sid payloads
pub const READ32: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/read32.bin"
));
pub const WRITE32: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/write32.bin"
));

pub const READ32_V821: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/read32_v821.bin"
));
pub const WRITE32_V821: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/write32_v821.bin"
));

/// Copy memory into an SRAM buffer on V821 and make the result visible to FEL.
///
/// This RV32 helper takes three little-endian `u32` parameters immediately after
/// the payload: source address, destination address, and byte length. The copy
/// uses aligned 32-bit accesses. `dcache.ciall` and `sync.is` are required before
/// returning because FEL reads the destination SRAM through USB, outside the CPU
/// data cache.
pub const COPY_V821: &[u8] = &[
    0x37, 0x03, 0x40, 0x00, // lui t1, 0x400
    0x73, 0x20, 0x03, 0x7c, // csrrs zero, mxstatus, t1
    0x0f, 0x10, 0x00, 0x00, // fence.i
    0x09, 0xa0, // j .+2
    0x97, 0x02, 0x00, 0x00, // auipc t0, 0
    0x93, 0x82, 0x62, 0x03, // addi t0, t0, 54 (parameters)
    0x03, 0xa5, 0x02, 0x00, // lw a0, 0(t0)
    0x83, 0xa5, 0x42, 0x00, // lw a1, 4(t0)
    0x03, 0xa6, 0x82, 0x00, // lw a2, 8(t0)
    0x03, 0x23, 0x05, 0x00, // lw t1, 0(a0)
    0x23, 0xa0, 0x65, 0x00, // sw t1, 0(a1)
    0x11, 0x05, // addi a0, 4
    0x91, 0x05, // addi a1, 4
    0x71, 0x16, // addi a2, -4
    0x6d, 0xfa, // bnez a2, copy loop
    0x0f, 0x00, 0x30, 0x03, // fence rw, rw
    0x0b, 0x00, 0x30, 0x00, // dcache.ciall
    0x0b, 0x00, 0xb0, 0x01, // sync.is
    0x0f, 0x10, 0x00, 0x00, // fence.i
    0x82, 0x80, // ret
];

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

pub const SPI_INIT_D1: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/spi_d1.bin"
));
pub const DDR_INIT_V821: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/ddr_v821.bin"
));
pub const SPI_INIT_V821: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/payloads/spi_v821.bin"
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payloads_present() {
        // The repo includes these payloads under assets/payloads, ensure they got embedded.
        assert!(!READ32.is_empty(), "read32.bin should be embedded");
        assert!(!WRITE32.is_empty(), "write32.bin should be embedded");
        assert!(
            !READ32_V821.is_empty(),
            "read32_v821.bin should be embedded"
        );
        assert!(
            !WRITE32_V821.is_empty(),
            "write32_v821.bin should be embedded"
        );
        assert_eq!(COPY_V821.len(), 68);
        assert_eq!(
            &COPY_V821[54..62],
            &[0x0b, 0x00, 0x30, 0x00, 0x0b, 0x00, 0xb0, 0x01]
        );
        assert!(!JTAG_ENABLE_D1.is_empty(), "jtag_d1.bin should be embedded");
        assert!(!DDR_INIT_D1.is_empty(), "ddr_d1.bin should be embedded");
        assert!(!DDR_INIT_F133.is_empty(), "ddr_f133.bin should be embedded");
        assert!(!SPI_INIT_D1.is_empty(), "spi_d1.bin should be embedded");
        assert!(!DDR_INIT_V821.is_empty(), "ddr_v821.bin should be embedded");
        assert!(!SPI_INIT_V821.is_empty(), "spi_v821.bin should be embedded");
    }
}
