//! GPIO peripheral in T153, T536, MR536, A733 and T736 series.

use super::{AnyRegisterBlock, Eint, PioPow, PortRegisters};
use volatile_register::{RW, WO};

/// Input/output power register group used by version-3 system GPIO.
#[repr(C)]
pub struct PioPowV3 {
    /// Power mode selection register.
    pub mod_sel: RW<u32>,
    _reserved0: [u32; 1],
    /// Power mode control and status register.
    pub ctl_val: RW<u32>,
    _reserved1: [u32; 9],
    /// Input/output power control register.
    pub pow_ctrl: RW<u32>,
}

/// System-domain GPIO port register group, version 3.
#[repr(C)]
pub struct PortV3 {
    /// Mode configuration registers.
    pub cfg: [RW<u32>; 4],
    /// Data register.
    pub dat: RW<u32>,
    /// Atomic data-set register.
    pub dat_set: WO<u32>,
    /// Atomic data-clear register.
    pub dat_clr: WO<u32>,
    _reserved0: [u32; 1],
    /// Drive strength registers.
    pub drv: [RW<u32>; 4],
    /// Pull direction registers.
    pub pull: [RW<u32>; 2],
    _reserved1: [u32; 2],
    /// External interrupt registers for this port.
    pub eint: Eint,
    _reserved2: [u32; 8],
}

/// RTC-domain GPIO configuration register group, version 3.
#[repr(C)]
pub struct RtcPortV3 {
    /// Mode configuration registers.
    pub cfg: [RW<u32>; 4],
    _reserved0: [u32; 1],
    /// Drive strength registers.
    pub drv: [RW<u32>; 4],
    /// Pull direction registers.
    pub pull: [RW<u32>; 2],
    _reserved1: [u32; 1],
}

/// RTC-domain GPIO data register group, version 3.
#[repr(C)]
pub struct RtcDataV3 {
    /// Data register.
    pub dat: RW<u32>,
    /// Atomic data-set register.
    pub dat_set: WO<u32>,
    /// Atomic data-clear register.
    pub dat_clr: WO<u32>,
    _reserved0: [u32; 1],
}

/// General Purpose Input/Output registers, version 3.
#[repr(C)]
pub struct RegisterBlockV3 {
    _reserved0: [u32; 16],
    /// System-domain input/output power registers.
    pub sys_pio_pow: PioPowV3,
    _reserved1: [u32; 3],
    /// System-domain GPIO port register groups.
    ///
    /// This covers the union of main-domain ports used by version-3 SoCs.
    /// Individual chips can leave entries unimplemented.
    pub sys_port: [PortV3; 11],
    /// RTC-domain GPIO configuration register groups.
    pub rtc_port: [RtcPortV3; 2],
    _reserved2: [u32; 104],
    /// RTC-domain external interrupt register groups.
    pub rtc_eint: [Eint; 2],
    _reserved3: [u32; 64],
    /// RTC-domain input/output power registers.
    pub rtc_pio_pow: PioPow,
    _reserved4: [u32; 107],
    /// RTC-domain GPIO data register groups.
    pub rtc_data: [RtcDataV3; 2],
}

impl RegisterBlockV3 {
    #[inline(always)]
    pub(in crate::gpio) fn as_any(&self) -> &AnyRegisterBlock {
        // SAFETY: `AnyRegisterBlock` is an opaque zero-sized view of the same
        // GPIO register-block base address, and the returned borrow cannot
        // outlive this concrete register block.
        unsafe { &*(self as *const Self).cast() }
    }

    #[inline]
    pub(in crate::gpio) const fn port(&self, p: char) -> PortRegisters<'_> {
        match p {
            'A'..='K' => {
                let port = &self.sys_port[p as usize - b'A' as usize];
                PortRegisters::new(&port.cfg, &port.dat)
            }
            'L'..='M' => {
                let index = p as usize - b'L' as usize;
                PortRegisters::new(&self.rtc_port[index].cfg, &self.rtc_data[index].dat)
            }
            _ => panic!("unsupported GPIO port"),
        }
    }

    #[inline]
    pub(in crate::gpio) const fn eint(&self, p: char) -> &Eint {
        match p {
            'A'..='K' => &self.sys_port[p as usize - b'A' as usize].eint,
            'L'..='M' => &self.rtc_eint[p as usize - b'L' as usize],
            _ => panic!("unsupported GPIO port"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PioPowV3, PortV3, RegisterBlockV3, RtcDataV3, RtcPortV3};
    use core::mem::{MaybeUninit, align_of, offset_of, size_of};

    fn zeroed_register_block() -> RegisterBlockV3 {
        // SAFETY: every register payload and reserved field is a `u32`, for
        // which the all-zero bit pattern is valid. The value is only used by
        // host-side layout tests.
        unsafe { MaybeUninit::zeroed().assume_init() }
    }

    #[test]
    fn offset_pio_pow_v3() {
        assert_eq!(offset_of!(PioPowV3, mod_sel), 0x00);
        assert_eq!(offset_of!(PioPowV3, ctl_val), 0x08);
        assert_eq!(offset_of!(PioPowV3, pow_ctrl), 0x30);
        assert_eq!(size_of::<PioPowV3>(), 0x34);
        assert_eq!(align_of::<PioPowV3>(), 4);
    }

    #[test]
    fn offset_port_v3() {
        assert_eq!(offset_of!(PortV3, cfg), 0x00);
        assert_eq!(offset_of!(PortV3, dat), 0x10);
        assert_eq!(offset_of!(PortV3, dat_set), 0x14);
        assert_eq!(offset_of!(PortV3, dat_clr), 0x18);
        assert_eq!(offset_of!(PortV3, drv), 0x20);
        assert_eq!(offset_of!(PortV3, pull), 0x30);
        assert_eq!(offset_of!(PortV3, eint), 0x40);
        assert_eq!(size_of::<PortV3>(), 0x80);
        assert_eq!(align_of::<PortV3>(), 4);
    }

    #[test]
    fn offset_rtc_port_v3() {
        assert_eq!(offset_of!(RtcPortV3, cfg), 0x00);
        assert_eq!(offset_of!(RtcPortV3, drv), 0x14);
        assert_eq!(offset_of!(RtcPortV3, pull), 0x24);
        assert_eq!(size_of::<RtcPortV3>(), 0x30);
        assert_eq!(align_of::<RtcPortV3>(), 4);

        assert_eq!(offset_of!(RtcDataV3, dat), 0x00);
        assert_eq!(offset_of!(RtcDataV3, dat_set), 0x04);
        assert_eq!(offset_of!(RtcDataV3, dat_clr), 0x08);
        assert_eq!(size_of::<RtcDataV3>(), 0x10);
        assert_eq!(align_of::<RtcDataV3>(), 4);
    }

    #[test]
    fn offset_gpio_v3() {
        assert_eq!(offset_of!(RegisterBlockV3, sys_pio_pow), 0x040);
        assert_eq!(offset_of!(RegisterBlockV3, sys_port), 0x080);
        assert_eq!(offset_of!(RegisterBlockV3, rtc_port), 0x600);
        assert_eq!(offset_of!(RegisterBlockV3, rtc_eint), 0x800);
        assert_eq!(offset_of!(RegisterBlockV3, rtc_pio_pow), 0x940);
        assert_eq!(offset_of!(RegisterBlockV3, rtc_data), 0xb00);
        assert_eq!(size_of::<RegisterBlockV3>(), 0xb20);
        assert_eq!(align_of::<RegisterBlockV3>(), 4);
    }

    #[test]
    fn offset_get_gpio_port_eint_v3() {
        let block = zeroed_register_block();
        let base_addr = (&raw const block).cast::<u8>();
        let test_cases = [
            ('A', 0x080, 0x0c0),
            ('B', 0x100, 0x140),
            ('C', 0x180, 0x1c0),
            ('D', 0x200, 0x240),
            ('E', 0x280, 0x2c0),
            ('F', 0x300, 0x340),
            ('G', 0x380, 0x3c0),
            ('H', 0x400, 0x440),
            ('I', 0x480, 0x4c0),
            ('J', 0x500, 0x540),
            ('K', 0x580, 0x5c0),
            ('L', 0x600, 0x800),
            ('M', 0x630, 0x820),
        ];
        let data_offsets = [0xb00, 0xb10];

        for (p, cfg_offset, eint_offset) in test_cases {
            let port = block.port(p);
            let offset = unsafe { port.cfg.as_ptr().cast::<u8>().offset_from(base_addr) };
            assert_eq!(
                offset, cfg_offset,
                "incorrect port {p} configuration offset"
            );

            let eint_ref = block.eint(p);
            let offset = unsafe { (eint_ref as *const _ as *const u8).offset_from(base_addr) };
            assert_eq!(offset, eint_offset, "incorrect port {p} EINT offset");
        }

        for (index, p) in ['L', 'M'].into_iter().enumerate() {
            let port = block.port(p);
            let offset = unsafe { (port.dat as *const _ as *const u8).offset_from(base_addr) };
            assert_eq!(
                offset, data_offsets[index],
                "incorrect port {p} data offset"
            );
        }
    }

    #[test]
    #[should_panic(expected = "unsupported GPIO port")]
    fn invalid_port_panics() {
        let block = zeroed_register_block();
        let _ = block.port('N');
    }

    #[test]
    #[should_panic(expected = "unsupported GPIO port")]
    fn invalid_eint_panics() {
        let block = zeroed_register_block();
        let _ = block.eint('N');
    }

    #[test]
    fn any_register_block_view_preserves_address() {
        let block = zeroed_register_block();
        let concrete = (&raw const block).cast::<u8>();

        assert_eq!(block.as_any() as *const _ as usize, concrete as usize);
    }
}
