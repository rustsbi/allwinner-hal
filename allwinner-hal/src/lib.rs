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
pub mod smhc;
pub mod spi;
#[doc(hidden)]
pub mod sysctl;
pub mod uart;

#[doc(hidden)]
pub mod prelude {
    pub use crate::gpio::PadExt as _;
    pub use crate::uart::UartExt as _;
    pub use embedded_hal::{
        digital::{InputPin as _, OutputPin as _, StatefulOutputPin as _},
        spi::SpiBus as _,
    };
    pub use embedded_io::{Read as _, Write as _};
}

#[allow(unused)]
macro_rules! impl_pins_trait {
    ($(($p: expr_2021, $i: expr_2021, $f: expr_2021): $Trait: ty;)+) => {
        $(
impl<'a> $Trait for $crate::gpio::Function<'a, $p, $i, $f> {}
        )+
    };
}

/// SoC configurations on peripherals and interrupt contexts.
pub mod wafer {
    pub mod d1;
}
