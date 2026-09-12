//! DDR initialization with 24 parameter words and an appended size result.

pub mod d1;
pub mod f101;
pub mod v821;

/// Run V821 DDR initialization, returning its size in MiB or zero.
///
/// # Safety
/// Requires V821 M-mode FEL execution from SRAM, an SRAM stack, exclusive
/// clock/PHY/DRAM/SID access, and board-correct parameters. DRAM data is destroyed.
pub unsafe fn run_v821(parameters: &mut Parameters) -> u32 {
    #[cfg(all(target_os = "none", target_arch = "riscv32"))]
    {
        let mut io = mmio::Mmio { ticks_per_us: 24 };
        io.ticks_per_us = v821::timer_init(&mut io);
        v821::init(&mut io, parameters)
    }
    #[cfg(not(all(target_os = "none", target_arch = "riscv32")))]
    {
        let _ = parameters;
        0
    }
}

#[cfg(all(
    target_os = "none",
    any(target_arch = "riscv32", target_arch = "riscv64")
))]
mod mmio;

/// Run F101-S2/S3 PSRAM initialization, returning its size in MiB or zero.
///
/// # Safety
/// Requires F101 M-mode FEL execution from SRAM, an SRAM stack, exclusive
/// CCU/PHY/PSRAM/SID access, and board-correct parameters. PSRAM data is destroyed.
pub unsafe fn run_f101(config: &mut f101::Configuration) -> u32 {
    #[cfg(all(target_os = "none", target_arch = "riscv32"))]
    {
        f101::init(&mut mmio::Mmio { ticks_per_us: 24 }, config)
    }
    #[cfg(not(all(target_os = "none", target_arch = "riscv32")))]
    {
        let _ = config;
        0
    }
}

/// Run D1/F133 memory initialization and return its size in MiB, or zero.
///
/// # Safety
/// Requires D1/F133 M-mode FEL execution from SRAM, an SRAM stack, exclusive
/// access to CCU/DRAM/PRCM/SID, and board-correct memory parameters. DRAM data
/// is destroyed by geometry scanning and by the optional memory test.
pub unsafe fn run_d1(parameters: &mut Parameters) -> u32 {
    #[cfg(all(target_os = "none", target_arch = "riscv64"))]
    {
        d1::init(&mut mmio::Mmio { ticks_per_us: 24 }, parameters)
    }
    #[cfg(not(all(target_os = "none", target_arch = "riscv64")))]
    {
        let _ = parameters;
        0
    }
}

/// The 24-word parameter prefix consumed by the vendor memory routines.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Parameters {
    pub clock: u32,
    pub kind: u32,
    pub zq: u32,
    pub odt: u32,
    pub para1: u32,
    pub para2: u32,
    pub mr: [u32; 4],
    pub tpr: [u32; 14],
}

/// Ordered register accesses and microsecond delays.
/// Implementations must preserve volatile access order and required I/O fences.
pub trait Io {
    fn read(&mut self, address: usize) -> u32;
    fn write(&mut self, address: usize, value: u32);
    fn delay_us(&mut self, microseconds: u32);
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    #[test]
    fn parameter_layout() {
        assert_eq!(core::mem::size_of::<Parameters>(), 96);
        assert_eq!(core::mem::offset_of!(Parameters, mr), 24);
        assert_eq!(core::mem::offset_of!(Parameters, tpr), 40);
        assert_eq!(core::mem::offset_of!(Parameters, tpr) + 13 * 4, 92);
    }
}

#[cfg(test)]
mod tests;
