//! RISC-V Platform-Level Interrupt Controller.

pub use crate::wafer::interrupt::*;
use crate::PLIC;
use base_address::{Dynamic, Static};
pub use plic::Plic;

impl<const B: usize> PLIC<Static<B>> {
    /// Create a peripheral instance from statically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub const unsafe fn steal_static() -> PLIC<Static<B>> {
        PLIC { base: Static::<B> }
    }
}

impl PLIC<Dynamic> {
    /// Create a peripheral instance from dynamically known address.
    ///
    /// This function is unsafe for it forces to seize ownership from possible
    /// wrapped peripheral group types. Users should normally retrieve ownership
    /// from wrapped types.
    #[inline]
    pub unsafe fn steal_dynamic(base: *const ()) -> PLIC<Dynamic> {
        PLIC {
            base: Dynamic::new(base as usize),
        }
    }
}
