//! Common register blocks shared by GPIO peripheral.
use volatile_register::RW;

/// GPIO port register group.
#[repr(C)]
pub struct Port {
    /// Mode configuration registers.
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

#[cfg(test)]
mod tests {
    use super::{Eint, PioPow, Port};
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn offset_port() {
        assert_eq!(offset_of!(Port, cfg), 0x00);
        assert_eq!(offset_of!(Port, dat), 0x10);
        assert_eq!(offset_of!(Port, drv), 0x14);
        assert_eq!(offset_of!(Port, pull), 0x24);
        assert_eq!(size_of::<Port>(), 0x30);
        assert_eq!(align_of::<Port>(), 4);
    }

    #[test]
    fn offset_eint() {
        assert_eq!(offset_of!(Eint, cfg), 0x00);
        assert_eq!(offset_of!(Eint, ctl), 0x10);
        assert_eq!(offset_of!(Eint, status), 0x14);
        assert_eq!(offset_of!(Eint, deb), 0x18);
        assert_eq!(size_of::<Eint>(), 0x20);
        assert_eq!(align_of::<Eint>(), 4);
    }

    #[test]
    fn offset_pio_pow() {
        assert_eq!(offset_of!(PioPow, mod_sel), 0x00);
        assert_eq!(offset_of!(PioPow, ms_ctl), 0x04);
        assert_eq!(offset_of!(PioPow, val), 0x08);
        assert_eq!(offset_of!(PioPow, vol_sel_ctl), 0x10);
        assert_eq!(size_of::<PioPow>(), 0x14);
        assert_eq!(align_of::<PioPow>(), 4);
    }
}
