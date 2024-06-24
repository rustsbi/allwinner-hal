//! SoC peripheral support for Allwinner chips.
//!
//! This package is built under the concept of componentized drivers. It is designed to
//! use in kernels, firmwares and embedded development with both dynamic and static base
//! address support.
//!
//! Most of `allwinner-hal` structures have `embedded-hal` traits implemented. Users may combine
//! this package with `embedded-hal` ecosystem drivers to provide abundant amount of features.
#![no_std]
#[deny(missing_docs)]
pub mod ccu;
pub mod com;
#[macro_use]
pub mod gpio;
pub mod phy;
pub mod spi;
#[macro_use]
pub mod uart;

#[allow(unused)]
macro_rules! impl_pins_trait {
    ($(($p: expr, $i: expr, $f: expr): $Trait: ty;)+) => {
        $(
impl<'a> $Trait for $crate::gpio::Function<'a, $p, $i, $f> {}
        )+
    };
}

/// SoC configurations on peripherals and interrupt contexts.
pub mod wafer {
    pub mod d1;
}
