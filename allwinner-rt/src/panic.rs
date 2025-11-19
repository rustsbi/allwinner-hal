//! Allwinner-RT panic handler.

/// Default panic-halt handler; can be disabled under `no-default-features`.
#[cfg(all(feature = "panic-halt", target_os = "none"))]
#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use crate::halt;
    unsafe { halt() }
}
