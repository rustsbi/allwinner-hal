use super::{
    cfg_index,
    mode::{FromRegisters, PortAndNumber},
    register::{AnyRegisterBlock, GpioVersion},
};

/// External interrupt mode pad.
pub struct EintPad<'a> {
    port: char,
    number: u8,
    version: GpioVersion,
    gpio: &'a AnyRegisterBlock,
}

impl<'a> EintPad<'a> {
    impl_gpio_constructors!(pad);
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
        let (cfg_reg_idx, mask, cfg_field_idx) = {
            let (cfg_reg_idx, cfg_field_idx) = cfg_index(self.number);
            let mask = !(0xF << cfg_field_idx);
            (cfg_reg_idx, mask, cfg_field_idx)
        };
        let value = event_id << cfg_field_idx;
        let cfg_reg = &unsafe { self.gpio.with_version(self.version) }
            .eint(self.port)
            .cfg[cfg_reg_idx];
        unsafe { cfg_reg.modify(|cfg| (cfg & mask) | value) };
    }
    #[inline]
    pub fn enable_interrupt(&mut self) {
        let ctl = &unsafe { self.gpio.with_version(self.version) }
            .eint(self.port)
            .ctl;
        unsafe { ctl.modify(|value| value | (1 << self.number)) }
    }
    #[inline]
    pub fn disable_interrupt(&mut self) {
        let ctl = &unsafe { self.gpio.with_version(self.version) }
            .eint(self.port)
            .ctl;
        unsafe { ctl.modify(|value| value & !(1 << self.number)) }
    }
    #[inline]
    pub fn clear_interrupt_pending_bit(&mut self) {
        let status = &unsafe { self.gpio.with_version(self.version) }
            .eint(self.port)
            .status;
        unsafe { status.write(1 << self.number) }
    }
    #[inline]
    pub fn check_interrupt(&mut self) -> bool {
        let status = &unsafe { self.gpio.with_version(self.version) }
            .eint(self.port)
            .status;
        status.read() & (1 << self.number) != 0
    }
}

impl<'a> PortAndNumber<'a> for EintPad<'a> {
    #[inline]
    fn port_number(&self) -> (char, u8) {
        (self.port, self.number)
    }
    #[inline]
    fn gpio_version(&self) -> GpioVersion {
        self.version
    }
    #[inline]
    fn register_block(&self) -> &'a AnyRegisterBlock {
        self.gpio
    }
}

impl<'a> FromRegisters<'a> for EintPad<'a> {
    #[inline]
    fn mode_value(version: GpioVersion) -> u8 {
        match version {
            GpioVersion::V1 => 6,
            GpioVersion::V2 | GpioVersion::V3 | GpioVersion::V4 => 14,
        }
    }
    #[inline]
    unsafe fn from_gpio(
        port: char,
        number: u8,
        version: GpioVersion,
        gpio: &'a AnyRegisterBlock,
    ) -> Self {
        Self {
            port,
            number,
            version,
            gpio,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EintPad, FromRegisters, GpioVersion};

    #[test]
    fn mode_value_depends_on_register_version() {
        assert_eq!(
            <EintPad<'_> as FromRegisters<'_>>::mode_value(GpioVersion::V1),
            6
        );
        assert_eq!(
            <EintPad<'_> as FromRegisters<'_>>::mode_value(GpioVersion::V2),
            14
        );
        assert_eq!(
            <EintPad<'_> as FromRegisters<'_>>::mode_value(GpioVersion::V3),
            14
        );
        assert_eq!(
            <EintPad<'_> as FromRegisters<'_>>::mode_value(GpioVersion::V4),
            14
        );
    }
}
