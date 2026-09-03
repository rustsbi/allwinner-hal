//! GPIO peripheral in A333 and A537 series.

pub use super::v3::PioPowV3 as PioPowV4;
use super::{AnyRegisterBlock, Eint, PortRegisters};
use volatile_register::RW;

/// GPIO port register group, version 4.
#[repr(C)]
pub struct PortV4 {
    /// Mode configuration registers.
    pub cfg: [RW<u32>; 4],
    /// Data register.
    pub dat: RW<u32>,
    _reserved0: [u32; 3],
    /// Drive strength registers.
    pub drv: [RW<u32>; 4],
    /// Pull direction registers.
    pub pull: [RW<u32>; 2],
    _reserved1: [u32; 2],
    /// External interrupt registers for this port.
    pub eint: Eint,
    _reserved2: [u32; 8],
}

/// General Purpose Input/Output registers, version 4.
#[repr(C)]
pub struct RegisterBlockV4 {
    _reserved0: [u32; 16],
    /// System-domain input/output power registers.
    pub sys_pio_pow: PioPowV4,
    _reserved1: [u32; 3],
    /// System-domain GPIO port register groups.
    ///
    /// This covers every main-domain address slot from PA through PK. An
    /// individual chip can leave entries such as PI and PJ unimplemented.
    pub sys_port: [PortV4; 11],
    _reserved2: [u32; 16],
    /// RTC-domain input/output power registers.
    pub rtc_pio_pow: PioPowV4,
    _reserved3: [u32; 3],
    /// RTC-domain GPIO port register groups.
    pub rtc_port: [PortV4; 2],
}

impl RegisterBlockV4 {
    #[inline(always)]
    pub(in crate::gpio) fn as_any(&self) -> &AnyRegisterBlock {
        // SAFETY: `AnyRegisterBlock` is an opaque zero-sized view of the same
        // GPIO register-block base address, and the returned borrow cannot
        // outlive this concrete register block.
        unsafe { &*(self as *const Self).cast() }
    }

    #[inline]
    pub(in crate::gpio) const fn port(&self, p: char) -> PortRegisters<'_> {
        let port = match p {
            'A'..='K' => &self.sys_port[p as usize - b'A' as usize],
            'L'..='M' => &self.rtc_port[p as usize - b'L' as usize],
            _ => panic!("unsupported GPIO port"),
        };
        PortRegisters::new(&port.cfg, &port.dat)
    }

    #[inline]
    pub(in crate::gpio) const fn eint(&self, p: char) -> &Eint {
        match p {
            'A'..='K' => &self.sys_port[p as usize - b'A' as usize].eint,
            'L'..='M' => &self.rtc_port[p as usize - b'L' as usize].eint,
            _ => panic!("unsupported GPIO port"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PortV4, RegisterBlockV4};
    use core::mem::{MaybeUninit, align_of, offset_of, size_of};

    fn zeroed_register_block() -> RegisterBlockV4 {
        // SAFETY: every register payload and reserved field is a `u32`, for
        // which the all-zero bit pattern is valid. The value is only used by
        // host-side layout tests.
        unsafe { MaybeUninit::zeroed().assume_init() }
    }

    #[test]
    fn offset_port_v4() {
        assert_eq!(offset_of!(PortV4, cfg), 0x00);
        assert_eq!(offset_of!(PortV4, dat), 0x10);
        assert_eq!(offset_of!(PortV4, drv), 0x20);
        assert_eq!(offset_of!(PortV4, pull), 0x30);
        assert_eq!(offset_of!(PortV4, eint), 0x40);
        assert_eq!(size_of::<PortV4>(), 0x80);
        assert_eq!(align_of::<PortV4>(), 4);
    }

    #[test]
    fn offset_gpio_v4() {
        assert_eq!(offset_of!(RegisterBlockV4, sys_pio_pow), 0x040);
        assert_eq!(offset_of!(RegisterBlockV4, sys_port), 0x080);
        assert_eq!(offset_of!(RegisterBlockV4, rtc_pio_pow), 0x640);
        assert_eq!(offset_of!(RegisterBlockV4, rtc_port), 0x680);
        assert_eq!(size_of::<RegisterBlockV4>(), 0x780);
        assert_eq!(align_of::<RegisterBlockV4>(), 4);
    }

    #[test]
    fn offset_get_gpio_port_eint_v4() {
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
            ('L', 0x680, 0x6c0),
            ('M', 0x700, 0x740),
        ];

        for (p, port_offset, eint_offset) in test_cases {
            let port = block.port(p);
            let offset = unsafe { port.cfg.as_ptr().cast::<u8>().offset_from(base_addr) };
            assert_eq!(offset, port_offset, "incorrect port {p} offset");

            let eint_ref = block.eint(p);
            let offset = unsafe { (eint_ref as *const _ as *const u8).offset_from(base_addr) };
            assert_eq!(offset, eint_offset, "incorrect port {p} EINT offset");
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
