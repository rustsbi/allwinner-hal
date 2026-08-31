//! Single-hart critical sections for the V821 E907 MCU.

struct V821McuCriticalSection;

::critical_section::set_impl!(V821McuCriticalSection);

// SAFETY: the V821 MCU runtime starts only the E907 hart. Clearing `mstatus.MIE`
// therefore excludes every interrupt execution context that can access the
// protected state. `csrrci` returns the previous MIE state, so nested critical
// sections restore interrupts only when the outermost acquisition found them
// enabled. The inline assembly deliberately carries a compiler memory clobber.
unsafe impl ::critical_section::Impl for V821McuCriticalSection {
    #[inline]
    unsafe fn acquire() -> ::critical_section::RawRestoreState {
        let previous: usize;
        // SAFETY: this module is compiled only for the V821's 32-bit RISC-V MCU
        // target in machine mode, where `mstatus` and its MIE bit are accessible.
        unsafe {
            core::arch::asm!(
                "csrrci {previous}, mstatus, 8",
                previous = out(reg) previous,
                options(nostack)
            );
        }
        previous & (1 << 3) != 0
    }

    #[inline]
    unsafe fn release(restore_state: ::critical_section::RawRestoreState) {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        if restore_state {
            // SAFETY: this release is properly nested with `acquire`; only the
            // acquisition that observed MIE set may restore it.
            unsafe {
                core::arch::asm!("csrrsi zero, mstatus, 8", options(nostack));
            }
        }
    }
}
