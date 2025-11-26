//! D1-H, D1s, F133, F133A/B chip platforms.

mod clock;
mod interrupt;
mod peripheral;

pub use clock::{Clocks, UartClock};
pub use interrupt::{Interrupt, Machine, Supevisor};
pub use peripheral::*;

use embedded_time::rate::Extensions;

#[doc(hidden)]
#[inline]
pub fn __rom_init_params() -> (Peripherals, Clocks) {
    let clocks = Clocks {
        psi: 600_000_000.Hz(),
        apb1: 24_000_000.Hz(),
    };
    (Peripherals::__new(), clocks)
}
