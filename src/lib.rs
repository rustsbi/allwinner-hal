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
    ($(($p: expr, $i: expr, $m: ty): $Trait: ty;)+) => {
        $(
impl<GPIO> $Trait for $crate::gpio::Pad<GPIO, $p, $i, $m> {}
        )+
    };
}

mod wafer {
    #[cfg(any(feature = "d1", test))]
    mod d1;
    pub mod pins {
        #[cfg(any(feature = "d1", test))]
        pub use super::d1::Pads;
    }
    pub mod interrupt {
        #[allow(unused)] // TODO
        #[cfg(any(feature = "d1", test))]
        pub use super::d1::{Interrupt, Machine, Supevisor};
    }
}

pub use wafer::pins::*;
