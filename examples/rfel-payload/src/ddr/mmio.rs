use super::Io;

pub(super) struct Mmio {
    pub ticks_per_us: u32,
}

impl Io for Mmio {
    #[inline(always)]
    fn read(&mut self, address: usize) -> u32 {
        // SAFETY: the run_* caller supplies the corresponding address space and
        // exclusive control of memory initialization.
        unsafe {
            let value = (address as *const u32).read_volatile();
            core::arch::asm!("fence ir, ir", options(nostack));
            value
        }
    }

    #[inline(always)]
    fn write(&mut self, address: usize, value: u32) {
        // SAFETY: the same memory-map contract applies to writes. A full fence
        // also orders successive cached-value writes in the reset sequences.
        unsafe {
            core::arch::asm!("fence iorw, iorw", options(nostack));
            (address as *mut u32).write_volatile(value);
        }
    }

    fn delay_us(&mut self, microseconds: u32) {
        #[inline(always)]
        fn counter() -> u64 {
            #[cfg(target_arch = "riscv64")]
            {
                let value;
                // SAFETY: D1/F133 FEL executes in M-mode and time ticks at 24 MHz.
                unsafe {
                    core::arch::asm!("rdtime {0}", out(reg) value, options(nostack));
                }
                value
            }
            #[cfg(target_arch = "riscv32")]
            loop {
                let (high, low, check): (u32, u32, u32);
                // SAFETY: the FEL caller executes in M-mode. Retry on rollover.
                unsafe {
                    core::arch::asm!("rdtimeh {0}", "rdtime {1}", "rdtimeh {2}",
                        out(reg) high, out(reg) low, out(reg) check, options(nostack));
                }
                if high == check {
                    return (u64::from(high) << 32) | u64::from(low);
                }
            }
        }
        let ticks = u64::from(microseconds) * u64::from(self.ticks_per_us);
        let start = counter();
        while counter().wrapping_sub(start) <= ticks {}
    }
}
