use super::{port_cfg_index, register::RegisterBlock};

/// Internal function to set GPIO pad mode.
#[inline]
pub fn set_mode<'a, T, U>(mut value: T) -> U
where
    T: PortAndNumber<'a>,
    U: FromRegisters<'a>,
{
    // take ownership of pad
    let (port, number) = value.port_number();
    let gpio = value.register_block();
    unsafe { write_mode::<T, U>(&mut value) };
    // return ownership of pad
    unsafe { U::from_gpio(port, number, gpio) }
}

#[inline]
unsafe fn write_mode<'a, T: PortAndNumber<'a>, U: FromRegisters<'a>>(value: &mut T) {
    let gpio = value.register_block();
    // calculate mask, value and register address
    let (mask, value, port_idx, cfg_reg_idx) = {
        let (port, number) = value.port_number();
        let (port_idx, cfg_reg_idx, cfg_field_idx) = port_cfg_index(port, number);
        let mask = !(0xF << cfg_field_idx);
        let value = (U::VALUE as u32) << cfg_field_idx;
        (mask, value, port_idx, cfg_reg_idx)
    };
    // apply configuration
    let cfg_reg = &gpio.port[port_idx].cfg[cfg_reg_idx];
    unsafe { cfg_reg.modify(|cfg| (cfg & mask) | value) };
}

#[inline]
pub fn borrow_with_mode<'a, T, U, F, R>(value: &mut T, f: F) -> R
where
    T: PortAndNumber<'a> + FromRegisters<'a>,
    U: FromRegisters<'a>,
    F: FnOnce(&mut U) -> R,
{
    // retrieve information of pad
    let (port, number) = value.port_number();
    let gpio = value.register_block();
    // set pad to new mode
    unsafe { write_mode::<T, U>(value) };
    let mut pad = unsafe { U::from_gpio(port, number, gpio) };
    let val = f(&mut pad);
    // restore pad to original mode
    unsafe { write_mode::<T, T>(value) };
    val
}

pub trait PortAndNumber<'a> {
    fn port_number(&self) -> (char, u8);
    fn register_block(&self) -> &'a RegisterBlock;
}

pub trait FromRegisters<'a> {
    const VALUE: u8;
    unsafe fn from_gpio(port: char, number: u8, gpio: &'a RegisterBlock) -> Self;
}
