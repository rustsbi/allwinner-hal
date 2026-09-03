//! GPIO registers.
use core::marker::PhantomData;

mod commons;
pub use commons::{Eint, PioPow, Port};

pub mod v2;

/// Representation to any GPIO register block.
pub(super) struct AnyRegisterBlock {
    _private: PhantomData<()>,
}

impl AnyRegisterBlock {
    /// Cast this register block to a versioned one.
    ///
    /// # Safety
    ///
    /// Constructors must ensure that `AnyRegisterBlock` is valid for the
    /// corresponding [`GpioVersion`].
    #[inline]
    pub(super) unsafe fn with_version(&self, version: GpioVersion) -> Versioned<'_> {
        match version {
            GpioVersion::V2 => Versioned::V2(unsafe { &*(self as *const _ as *const _) }),
        }
    }
}

/// GPIO register block version.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GpioVersion {
    /// GPIO peripheral in D1, T113, V851, V853, V861, F101 and V821 series.
    V2,
}

/// Reference of possible GPIO register versions.
pub(super) enum Versioned<'a> {
    /// Register block V2.
    V2(&'a v2::RegisterBlockV2),
}
