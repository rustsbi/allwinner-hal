//! Allwinner GPIO controller.
use volatile_register::RW;

/// Generic Purpose Input/Output registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u32; 12],
    /// Gpio port register group.
    pub port: [Port; 6],
    _reserved1: [u32; 52],
    /// External interrupt register group.
    pub eint: [Eint; 6],
    _reserved2: [u32; 24],
    /// Input/output power register group.
    pub pio_pow: PioPow,
}

/// Gpio port register group.
#[repr(C)]
pub struct Port {
    /// Mode configuration register
    pub cfg: [RW<u32>; 4],
    /// Data register.
    pub dat: RW<u32>,
    /// Drive strength register.
    pub drv: [RW<u32>; 4],
    /// Pull direction register.
    pub pull: [RW<u32>; 2],
    _reserved0: [u32; 1],
}

/// External interrupt register group.
#[repr(C)]
pub struct Eint {
    /// Interrupt mode configuration.
    pub cfg: [RW<u32>; 4],
    /// Enable or disable interrupt.
    pub ctl: RW<u32>,
    /// Status register.
    pub status: RW<u32>,
    /// Debounce register.
    pub deb: RW<u32>,
    _reserved0: [u32; 1],
}

/// Input/Output Power register group.
#[repr(C)]
pub struct PioPow {
    pub mod_sel: RW<u32>,
    pub ms_ctl: RW<u32>,
    pub val: RW<u32>,
    _reserved0: [u32; 1],
    pub vol_sel_ctl: RW<u32>,
}

/// External interrupt event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    PositiveEdge,
    NegativeEdge,
    HighLevel,
    LowLevel,
    BothEdges,
}

#[allow(unused)]
macro_rules! impl_gpio_pins {
    ($($px: ident:($P: expr, $N: expr, $M: ident);)+) => {
/// GPIO pads in current platform.
pub struct Pads<'a> {
    $(
    pub $px: $crate::gpio::$M<'a, $P, $N>,
    )+
}
    };
}

#[inline]
const fn port_index(p: char) -> usize {
    assert!(p as usize >= b'B' as usize && p as usize <= b'G' as usize);
    p as usize - b'B' as usize
}

/// Input mode pad.
pub struct Input<'a, const P: char, const N: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8> Input<'a, P, N> {
    /// Configures the pad to operate as an output pad.
    #[inline]
    pub fn into_output(self) -> Output<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an alternate function pad.
    #[inline]
    pub fn into_function<const F: u8>(self) -> Function<'a, P, N, F> {
        set_mode(self)
    }
    /// Configures the pad to operate as an external interrupt pad.
    #[inline]
    pub fn into_eint(self) -> EintPad<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as a disabled pad.
    #[inline]
    pub fn into_disabled(self) -> Disabled<'a, P, N> {
        set_mode(self)
    }
    /// Borrows the pad to temporarily use it as an output pad.
    #[inline]
    pub fn with_output<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Output<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
    /// Borrows the pad to temporarily use it an alternate function pad.
    #[inline]
    pub fn with_function<const G: u8, F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Function<'a, P, N, G>) -> T,
    {
        borrow_with_mode(self, f)
    }
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::ErrorType for Input<'a, P, N> {
    type Error = core::convert::Infallible;
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::InputPin for Input<'a, P, N> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port[const { port_index(P) }].dat.read() & (1 << N) != 0)
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port[const { port_index(P) }].dat.read() & (1 << N) == 0)
    }
}

/// Output mode pad.
pub struct Output<'a, const P: char, const N: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8> Output<'a, P, N> {
    /// Configures the pad to operate as an input pad.
    #[inline]
    pub fn into_input(self) -> Input<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an alternate function pad.
    #[inline]
    pub fn into_function<const F: u8>(self) -> Function<'a, P, N, F> {
        set_mode(self)
    }
    /// Configures the pad to operate as an external interrupt pad.
    #[inline]
    pub fn into_eint(self) -> EintPad<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as a disabled pad.
    #[inline]
    pub fn into_disabled(self) -> Disabled<'a, P, N> {
        set_mode(self)
    }
    /// Borrows the pad to temporarily use it as an input pad.
    #[inline]
    pub fn with_input<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Input<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
    /// Borrows the pad to temporarily use it an alternate function pad.
    #[inline]
    pub fn with_function<const G: u8, F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Function<'a, P, N, G>) -> T,
    {
        borrow_with_mode(self, f)
    }
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::ErrorType for Output<'a, P, N> {
    type Error = core::convert::Infallible;
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::OutputPin for Output<'a, P, N> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let idx = const { port_index(P) };
        unsafe { self.gpio.port[idx].dat.modify(|value| value & !(1 << N)) };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let idx = const { port_index(P) };
        unsafe { self.gpio.port[idx].dat.modify(|value| value | (1 << N)) };
        Ok(())
    }
}

impl<'a, const P: char, const N: u8> embedded_hal::digital::StatefulOutputPin for Output<'a, P, N> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port[const { port_index(P) }].dat.read() & (1 << N) != 0)
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.port[const { port_index(P) }].dat.read() & (1 << N) == 0)
    }
}

/// Alternate function pad.
///
/// F should be in 2..=8.
pub struct Function<'a, const P: char, const N: u8, const F: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8, const F: u8> Function<'a, P, N, F> {
    /// Configures the pad to operate as an input pad.
    #[inline]
    pub fn into_input(self) -> Input<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an output pad.
    #[inline]
    pub fn into_output(self) -> Output<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an alternate function pad.
    #[inline]
    pub fn into_function<const F2: u8>(self) -> Function<'a, P, N, F2> {
        set_mode(self)
    }
    /// Configures the pad to operate as an external interrupt pad.
    #[inline]
    pub fn into_eint(self) -> EintPad<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as a disabled pad.
    #[inline]
    pub fn into_disabled(self) -> Disabled<'a, P, N> {
        set_mode(self)
    }
    /// Borrows the pad to temporarily use it as an input pad.
    #[inline]
    pub fn with_input<G, T>(&mut self, f: G) -> T
    where
        G: FnOnce(&mut Input<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
    /// Borrows the pad to temporarily use it as an output pad.
    #[inline]
    pub fn with_output<G, T>(&mut self, f: G) -> T
    where
        G: FnOnce(&mut Output<'a, P, N>) -> T,
    {
        borrow_with_mode(self, f)
    }
}

/// External interrupt mode pad.
pub struct EintPad<'a, const P: char, const N: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8> EintPad<'a, P, N> {
    /// Configures the pad to operate as an input pad.
    #[inline]
    pub fn into_input(self) -> Input<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an output pad.
    #[inline]
    pub fn into_output(self) -> Output<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an alternate function pad.
    #[inline]
    pub fn into_function<const F: u8>(self) -> Function<'a, P, N, F> {
        set_mode(self)
    }
    /// Configures the pad to operate as a disabled pad.
    #[inline]
    pub fn into_disabled(self) -> Disabled<'a, P, N> {
        set_mode(self)
    }
}

impl<'a, const P: char, const N: u8> EintPad<'a, P, N> {
    #[inline]
    pub fn listen(&mut self, event: Event) {
        let event_id = match event {
            Event::PositiveEdge => 0,
            Event::NegativeEdge => 1,
            Event::HighLevel => 2,
            Event::LowLevel => 3,
            Event::BothEdges => 4,
        };
        let (port_idx, cfg_reg_idx, mask, cfg_field_idx) = const {
            let (port_idx, cfg_reg_idx, cfg_field_idx) = port_cfg_index(P, N);
            let mask = !(0xF << cfg_field_idx);
            (port_idx, cfg_reg_idx, mask, cfg_field_idx)
        };
        let value = event_id << cfg_field_idx;
        let cfg_reg = &self.gpio.eint[port_idx].cfg[cfg_reg_idx];
        unsafe { cfg_reg.modify(|cfg| (cfg & mask) | value) };
    }
    #[inline]
    pub fn enable_interrupt(&mut self) {
        let idx = const { port_index(P) };
        unsafe { self.gpio.eint[idx].ctl.modify(|value| value | (1 << N)) }
    }
    #[inline]
    pub fn disable_interrupt(&mut self) {
        let idx = const { port_index(P) };
        unsafe { self.gpio.eint[idx].ctl.modify(|value| value & !(1 << N)) }
    }
    #[inline]
    pub fn clear_interrupt_pending_bit(&mut self) {
        unsafe { self.gpio.eint[const { port_index(P) }].status.write(1 << N) }
    }
    #[inline]
    pub fn check_interrupt(&mut self) -> bool {
        self.gpio.eint[const { port_index(P) }].status.read() & (1 << N) != 0
    }
}

/// Disabled GPIO pad.
pub struct Disabled<'a, const P: char, const N: u8> {
    gpio: &'a RegisterBlock,
}

impl<'a, const P: char, const N: u8> Disabled<'a, P, N> {
    /// Configures the pad to operate as an input pad.
    #[inline]
    pub fn into_input(self) -> Input<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an output pad.
    #[inline]
    pub fn into_output(self) -> Output<'a, P, N> {
        set_mode(self)
    }
    /// Configures the pad to operate as an alternate function pad.
    #[inline]
    pub fn into_function<const F: u8>(self) -> Function<'a, P, N, F> {
        set_mode(self)
    }
    /// Configures the pad to operate as an external interrupt pad.
    #[inline]
    pub fn into_eint(self) -> EintPad<'a, P, N> {
        set_mode(self)
    }

    /// Internal constructor for ROM runtime. Do not use.
    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn __new(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

/// Internal function to set GPIO pad mode.
#[inline]
fn set_mode<'a, T, U>(value: T) -> U
where
    T: HasMode<'a>,
    U: HasMode<'a>,
{
    // take ownership of pad
    let gpio = value.gpio();
    unsafe { write_mode::<T, U>(gpio) };
    // return ownership of pad
    unsafe { U::from_gpio(gpio) }
}

#[inline]
fn borrow_with_mode<'a, T, U, F, R>(value: &mut T, f: F) -> R
where
    T: HasMode<'a>,
    U: HasMode<'a>,
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
unsafe fn write_mode<'a, T: HasMode<'a>, U: HasMode<'a>>(gpio: &RegisterBlock) {
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

#[inline]
const fn port_cfg_index(p: char, n: u8) -> (usize, usize, u8) {
    assert!(p as usize >= b'B' as usize && p as usize <= b'G' as usize);
    assert!(n <= 31);
    let port_idx = p as usize - b'B' as usize;
    let cfg_reg_idx = (n >> 3) as usize;
    let cfg_field_idx = (n & 0b111) << 2;
    (port_idx, cfg_reg_idx, cfg_field_idx)
}

trait HasMode<'a> {
    const P: char;
    const N: u8;
    const VALUE: u8;
    fn gpio(&self) -> &'a RegisterBlock;
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self;
}

impl<'a, const P: char, const N: u8> HasMode<'a> for Input<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
    const VALUE: u8 = 0;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

impl<'a, const P: char, const N: u8> HasMode<'a> for Output<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
    const VALUE: u8 = 1;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

impl<'a, const P: char, const N: u8, const F: u8> HasMode<'a> for Function<'a, P, N, F> {
    const P: char = P;
    const N: u8 = N;
    const VALUE: u8 = F;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

impl<'a, const P: char, const N: u8> HasMode<'a> for EintPad<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
    const VALUE: u8 = 14;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

impl<'a, const P: char, const N: u8> HasMode<'a> for Disabled<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
    const VALUE: u8 = 15;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}

#[cfg(test)]
mod tests {
    use super::{Eint, PioPow, Port, RegisterBlock};
    use memoffset::offset_of;
    #[test]
    fn offset_gpio() {
        assert_eq!(offset_of!(RegisterBlock, port), 0x30);
        assert_eq!(offset_of!(RegisterBlock, eint), 0x220);
        assert_eq!(offset_of!(RegisterBlock, pio_pow), 0x340);
    }
    #[test]
    fn offset_port() {
        assert_eq!(offset_of!(Port, cfg), 0x00);
        assert_eq!(offset_of!(Port, dat), 0x10);
        assert_eq!(offset_of!(Port, drv), 0x14);
        assert_eq!(offset_of!(Port, pull), 0x24);
    }
    #[test]
    fn offset_eint() {
        assert_eq!(offset_of!(Eint, cfg), 0x00);
        assert_eq!(offset_of!(Eint, ctl), 0x10);
        assert_eq!(offset_of!(Eint, status), 0x14);
        assert_eq!(offset_of!(Eint, deb), 0x18);
    }
    #[test]
    fn offset_pio_pow() {
        assert_eq!(offset_of!(PioPow, mod_sel), 0x00);
        assert_eq!(offset_of!(PioPow, ms_ctl), 0x04);
        assert_eq!(offset_of!(PioPow, val), 0x08);
        assert_eq!(offset_of!(PioPow, vol_sel_ctl), 0x10);
    }
}
