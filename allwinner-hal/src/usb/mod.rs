//! USB device-controller support.
//!
//! [`Usb`] owns one controller MMIO capability. PHY ownership is independent
//! under [`phy`]. [`UsbBus`] consumes the controller owner and adapts it to the
//! Rust USB ecosystem.

mod bus;
mod peripheral;
pub mod phy;
pub mod register;

pub use bus::UsbBus;
pub use peripheral::{Instance, Usb};
pub use register::{PhyRegisterBlock, RegisterBlock};
