use super::{port_cfg_index, register::RegisterBlock};

/// Internal function to set GPIO pad mode.
#[inline]
pub fn set_mode<'a, T, U>(value: T) -> U
where
    T: IntoRegisters<'a>,
    U: FromRegisters<'a>,
{
    // take ownership of pad
    let gpio = value.gpio();
    unsafe { write_mode::<T, U>(gpio) };
    // return ownership of pad
    unsafe { U::from_gpio(gpio) }
}

#[inline]
pub fn borrow_with_mode<'a, T, U, F, R>(value: &mut T, f: F) -> R
where
    T: IntoRegisters<'a>,
    U: FromRegisters<'a>,
    F: FnOnce(&mut U) -> R,
{
    // take ownership of pad
    let gpio = value.gpio();
    // set pad to new mode
    unsafe { write_mode::<T, U>(gpio) };
    let mut pad = unsafe { U::from_gpio(gpio) };
    let val = f(&mut pad);
    // restore pad to original mode
    unsafe { write_mode::<T, T>(gpio) };
    val
}

#[inline]
unsafe fn write_mode<'a, T: IntoRegisters<'a>, U: FromRegisters<'a>>(gpio: &RegisterBlock) {
    // calculate mask, value and register address
    let (mask, value, port_idx, cfg_reg_idx) = const {
        let (port_idx, cfg_reg_idx, cfg_field_idx) = port_cfg_index(T::P, T::N);
        let mask = !(0xF << cfg_field_idx);
        let value = (U::VALUE as u32) << cfg_field_idx;
        (mask, value, port_idx, cfg_reg_idx)
    };
    // apply configuration
    let cfg_reg = &gpio.port[port_idx].cfg[cfg_reg_idx];
    unsafe { cfg_reg.modify(|cfg| (cfg & mask) | value) };
}

pub trait IntoRegisters<'a>: FromRegisters<'a> {
    const P: char;
    const N: u8;
    fn gpio(&self) -> &'a RegisterBlock;
}

pub trait FromRegisters<'a> {
    const VALUE: u8;
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self;
}
