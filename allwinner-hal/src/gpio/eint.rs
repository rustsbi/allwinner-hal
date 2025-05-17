use super::{
    mode::{FromRegisters, PortAndNumber, set_mode},
    port_cfg_index, port_index,
    register::RegisterBlock,
};

/// External interrupt mode pad.
pub struct EintPad<'a> {
    port: char,
    number: u8,
    gpio: &'a RegisterBlock,
}

impl<'a> EintPad<'a> {
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(port: char, number: u8, gpio: &'a RegisterBlock) -> Self {
        set_mode(Self { gpio, port, number })
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

impl<'a> EintPad<'a> {
    #[inline]
    pub fn listen(&mut self, event: Event) {
        let event_id = match event {
            Event::PositiveEdge => 0,
            Event::NegativeEdge => 1,
            Event::HighLevel => 2,
            Event::LowLevel => 3,
            Event::BothEdges => 4,
        };
        let (port_idx, cfg_reg_idx, mask, cfg_field_idx) = {
            let (port_idx, cfg_reg_idx, cfg_field_idx) = port_cfg_index(self.port, self.number);
            let mask = !(0xF << cfg_field_idx);
            (port_idx, cfg_reg_idx, mask, cfg_field_idx)
        };
        let value = event_id << cfg_field_idx;
        let cfg_reg = &self.gpio.eint[port_idx].cfg[cfg_reg_idx];
        unsafe { cfg_reg.modify(|cfg| (cfg & mask) | value) };
    }
    #[inline]
    pub fn enable_interrupt(&mut self) {
        let idx = port_index(self.port);
        unsafe {
            self.gpio.eint[idx]
                .ctl
                .modify(|value| value | (1 << self.number))
        }
    }
    #[inline]
    pub fn disable_interrupt(&mut self) {
        let idx = port_index(self.port);
        unsafe {
            self.gpio.eint[idx]
                .ctl
                .modify(|value| value & !(1 << self.number))
        }
    }
    #[inline]
    pub fn clear_interrupt_pending_bit(&mut self) {
        unsafe {
            self.gpio.eint[port_index(self.port)]
                .status
                .write(1 << self.number)
        }
    }
    #[inline]
    pub fn check_interrupt(&mut self) -> bool {
        self.gpio.eint[port_index(self.port)].status.read() & (1 << self.number) != 0
    }
}

impl<'a> PortAndNumber<'a> for EintPad<'a> {
    #[inline]
    fn port_number(&self) -> (char, u8) {
        (self.port, self.number)
    }
    #[inline]
    fn register_block(&self) -> &'a RegisterBlock {
        self.gpio
    }
}

impl<'a> FromRegisters<'a> for EintPad<'a> {
    const VALUE: u8 = 14;
    #[inline]
    unsafe fn from_gpio(port: char, number: u8, gpio: &'a RegisterBlock) -> Self {
        Self { port, number, gpio }
    }
}
