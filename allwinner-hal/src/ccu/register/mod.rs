//! Clock Control Unit register blocks.
//!
//! Allwinner CCU layouts are platform-specific. Modules are named after a
//! representative chip rather than the vendor's internal `sunXXiwX` code.
//! The supported SDK matrix currently contains 13 such layouts. Separate
//! controller domains (for example APP, AON, PRCM, RTC, or MCU CCUs) remain
//! register blocks within the corresponding chip module and are not counted as
//! additional layout versions.

mod commons;
pub use commons::{BusGatingReset, SingleBusGatingReset};

/// A133/R818 (`sun50iw10`) CCU registers.
pub mod a133;
/// A537/A333 (`sun65iw1`) CCU registers.
pub mod a537;
/// A733/T736 (`sun60iw2`) CCU registers.
pub mod a733;
/// D1/D1-H/F133 (`sun20iw1`) CCU registers.
pub mod d1;
/// F101 (`sun252iw2`) CCU registers.
pub mod f101;
/// H616/H313/H618 (`sun50iw9`) CCU registers.
pub mod h616;
/// T113 (`sun8iw20`) CCU registers.
pub mod t113;
/// T153 (`sun8iw22`) CCU registers.
pub mod t153;
/// T527/A523/A527 (`sun55iw3`) CCU registers.
pub mod t527;
/// T536/MR536 (`sun55iw6`) CCU registers.
pub mod t536;
/// V821 (`sun300iw1`) CCU registers.
pub mod v821;
/// V853/V851s (`sun8iw21`) CCU registers.
pub mod v853;
/// V861 (`sun252iw1`) CCU registers.
pub mod v861;
