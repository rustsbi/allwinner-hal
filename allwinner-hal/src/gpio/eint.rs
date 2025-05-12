use super::{
    function::Function,
    input::Input,
    mode::{FromRegisters, IntoRegisters, set_mode},
    output::Output,
    port_cfg_index, port_index,
    register::RegisterBlock,
};

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
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
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

impl<'a, const P: char, const N: u8> IntoRegisters<'a> for EintPad<'a, P, N> {
    const P: char = P;
    const N: u8 = N;
    #[inline]
    fn gpio(&self) -> &'a RegisterBlock {
        self.gpio
    }
}

impl<'a, const P: char, const N: u8> FromRegisters<'a> for EintPad<'a, P, N> {
    const VALUE: u8 = 14;
    #[inline]
    unsafe fn from_gpio(gpio: &'a RegisterBlock) -> Self {
        Self { gpio }
    }
}
