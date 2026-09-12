//! Embedded FEL payloads; register helpers are shared by CPU architecture.

pub const READ32_RV64: &[u8] = include_bytes!("../../assets/payloads/read32_rv64.bin");
pub const WRITE32_RV64: &[u8] = include_bytes!("../../assets/payloads/write32_rv64.bin");

// F101 and V821 use the same RV32 register helpers and cache prefix.
pub const READ32_RV32: &[u8] = include_bytes!("../../assets/payloads/read32_rv32.bin");
pub const WRITE32_RV32: &[u8] = include_bytes!("../../assets/payloads/write32_rv32.bin");

pub const DDR_INIT_F101_S2: &[u8] = include_bytes!("../../assets/payloads/ddr_f101_s2.bin");
pub const DDR_INIT_F101_S3: &[u8] = include_bytes!("../../assets/payloads/ddr_f101_s3.bin");
pub const SPI_INIT_F101: &[u8] = include_bytes!("../../assets/payloads/spi_f101.bin");

/// Synchronize F101 instruction fetch when switching between SPI and PSRAM
/// helpers at the same SRAM address. Keep the prefix identical to the
/// register helpers so a cached scratchpad entry still reaches `fence.i`.
pub const FENCE_I_F101: &[u8] = include_bytes!("../../assets/payloads/fence_i_f101.bin");

/// Copy memory into an SRAM buffer on V821 and make the result visible to FEL.
///
/// This RV32 helper takes three little-endian `u32` parameters immediately after
/// the payload: source address, destination address, and byte length. The copy
/// uses aligned 32-bit accesses. `dcache.ciall` and `sync.is` are required before
/// returning because FEL reads the destination SRAM through USB, outside the CPU
/// data cache.
pub const COPY_V821: &[u8] = include_bytes!("../../assets/payloads/copy_v821.bin");

// JTAG/DDR payload
pub const JTAG_RV64: &[u8] = include_bytes!("../../assets/payloads/jtag_rv64.bin");
pub const JTAG_RV32: &[u8] = include_bytes!("../../assets/payloads/jtag_rv32.bin");
pub const DDR_INIT_D1: &[u8] = include_bytes!("../../assets/payloads/ddr_d1.bin");
pub const DDR_INIT_F133: &[u8] = include_bytes!("../../assets/payloads/ddr_f133.bin");

pub const SPI_INIT_D1: &[u8] = include_bytes!("../../assets/payloads/spi_d1.bin");
pub const DDR_INIT_V821: &[u8] = include_bytes!("../../assets/payloads/ddr_v821.bin");
pub const SPI_INIT_V821: &[u8] = include_bytes!("../../assets/payloads/spi_v821.bin");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payloads_present() {
        for payload in [
            READ32_RV32,
            WRITE32_RV32,
            READ32_RV64,
            WRITE32_RV64,
            COPY_V821,
            FENCE_I_F101,
            JTAG_RV64,
            JTAG_RV32,
            DDR_INIT_D1,
            DDR_INIT_F133,
            DDR_INIT_F101_S2,
            DDR_INIT_F101_S3,
            DDR_INIT_V821,
            SPI_INIT_D1,
            SPI_INIT_F101,
            SPI_INIT_V821,
        ] {
            assert!(!payload.is_empty());
        }
    }

    #[test]
    fn appended_word_parameters_are_aligned() {
        for payload in [
            READ32_RV32,
            WRITE32_RV32,
            READ32_RV64,
            WRITE32_RV64,
            COPY_V821,
        ] {
            assert_eq!(payload.len() % 4, 0);
        }
    }

    #[test]
    fn copy_preserves_cache_maintenance_sequence() {
        let sequence = [
            0x0f, 0x00, 0x30, 0x03, // fence rw, rw
            0x0b, 0x00, 0x30, 0x00, // dcache.ciall
            0x0b, 0x00, 0xb0, 0x01, // sync.is
            0x0f, 0x10, 0x00, 0x00, // fence.i
        ];
        assert!(
            COPY_V821
                .windows(sequence.len())
                .any(|bytes| bytes == sequence)
        );
    }
}
