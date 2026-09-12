//! Memory layout supplied by the FEL host.

/// Return the SPI command buffer at the runtime payload entry plus `0x1000`.
///
/// The D1, F101 and V821 hosts all reserve this offset from the payload's load
/// address. The host must place commands there when relocating the payload too;
/// this is not an offset from the chip's physical SRAM base or the image end.
/// Returns null on targets where the payload entry is unavailable.
#[inline]
pub fn spi_commands() -> *const u8 {
    #[cfg(all(
        target_os = "none",
        any(target_arch = "riscv32", target_arch = "riscv64")
    ))]
    {
        unsafe extern "C" {
            fn _start();
        }
        let commands;
        // SAFETY: compute an address without accessing memory. Local PC-relative
        // addressing avoids a GOT entry containing an unrelocated absolute pointer.
        unsafe {
            core::arch::asm!(
                ".option push",
                ".option norelax",
                "lla {commands}, {entry} + 0x1000",
                ".option pop",
                commands = out(reg) commands,
                entry = sym _start,
                options(pure, nomem, nostack),
            );
        }
        commands
    }
    #[cfg(not(all(
        target_os = "none",
        any(target_arch = "riscv32", target_arch = "riscv64")
    )))]
    {
        core::ptr::null()
    }
}
