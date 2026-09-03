//! GPIO registers.
use core::marker::PhantomData;
use volatile_register::RW;

mod commons;
pub use commons::{Eint, PioPow, Port};

pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;

/// Registers needed by the common GPIO pad implementation.
pub(super) struct PortRegisters<'a> {
    pub(super) cfg: &'a [RW<u32>; 4],
    pub(super) dat: &'a RW<u32>,
}

impl<'a> PortRegisters<'a> {
    #[inline(always)]
    const fn new(cfg: &'a [RW<u32>; 4], dat: &'a RW<u32>) -> Self {
        Self { cfg, dat }
    }
}

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
            GpioVersion::V1 => Versioned::V1(unsafe { &*(self as *const _ as *const _) }),
            GpioVersion::V2 => Versioned::V2(unsafe { &*(self as *const _ as *const _) }),
            GpioVersion::V3 => Versioned::V3(unsafe { &*(self as *const _ as *const _) }),
            GpioVersion::V4 => Versioned::V4(unsafe { &*(self as *const _ as *const _) }),
        }
    }
}

/// GPIO register block version.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GpioVersion {
    /// GPIO peripheral in H313, H616, H618, A133 and R818 series.
    V1,
    /// GPIO peripheral in D1, T113, V851, V853, V861, F101 and V821 series.
    V2,
    /// GPIO peripheral in T153, T536, MR536, A733 and T736 series.
    V3,
    /// GPIO peripheral in A333 and A537 series.
    V4,
}

/// Reference of possible GPIO register versions.
pub(super) enum Versioned<'a> {
    /// Register block V1.
    V1(&'a v1::RegisterBlockV1),
    /// Register block V2.
    V2(&'a v2::RegisterBlockV2),
    /// Register block V3.
    V3(&'a v3::RegisterBlockV3),
    /// Register block V4.
    V4(&'a v4::RegisterBlockV4),
}

impl<'a> Versioned<'a> {
    #[inline]
    pub(super) const fn port(self, p: char) -> PortRegisters<'a> {
        match self {
            Self::V1(gpio) => gpio.port(p),
            Self::V2(gpio) => gpio.port(p),
            Self::V3(gpio) => gpio.port(p),
            Self::V4(gpio) => gpio.port(p),
        }
    }

    #[inline]
    pub(super) const fn eint(self, p: char) -> &'a Eint {
        match self {
            Self::V1(gpio) => gpio.eint(p),
            Self::V2(gpio) => gpio.eint(p),
            Self::V3(gpio) => gpio.eint(p),
            Self::V4(gpio) => gpio.eint(p),
        }
    }
}
