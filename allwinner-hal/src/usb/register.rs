//! USB controller and physical-layer registers.

mod commons;

pub use commons::{InterfaceStatusControl, InterfaceStatusControlRegister};

/// Version 1 USB PHY registers.
pub mod phy_v1;

/// Version 2 USB PHY registers.
pub mod phy_v2;

/// MUSB-compatible USB device-controller registers.
pub mod usb;
