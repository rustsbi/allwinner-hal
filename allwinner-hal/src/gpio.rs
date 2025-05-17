//! Allwinner GPIO controller.
mod eint;
mod function;
mod input;
mod mode;
mod output;
mod pad_ext;
mod register;

pub use eint::{EintPad, Event};
pub use function::Function;
pub use input::Input;
pub use output::Output;
pub use pad_ext::PadExt;
pub use register::{Eint, PioPow, Port, RegisterBlock};

#[inline]
const fn port_index(p: char) -> usize {
    assert!(p as usize >= b'B' as usize && p as usize <= b'G' as usize);
    p as usize - b'B' as usize
}

#[inline]
const fn port_cfg_index(p: char, n: u8) -> (usize, usize, u8) {
    assert!(p as usize >= b'B' as usize && p as usize <= b'G' as usize);
    assert!(n <= 31);
    let port_idx = p as usize - b'B' as usize;
    let cfg_reg_idx = (n >> 3) as usize;
    let cfg_field_idx = (n & 0b111) << 2;
    (port_idx, cfg_reg_idx, cfg_field_idx)
}
