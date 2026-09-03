//! GPIO peripheral in H313, H616, H618, A133 and R818 series.

use super::{AnyRegisterBlock, Eint, PioPow, PortRegisters};
use volatile_register::RW;

/// GPIO port register group, version 1.
#[repr(C)]
pub struct PortV1 {
    /// Mode configuration registers.
    pub cfg: [RW<u32>; 4],
    /// Data register.
    pub dat: RW<u32>,
    /// Drive strength registers.
    pub drv: [RW<u32>; 2],
    /// Pull direction registers.
    pub pull: [RW<u32>; 2],
}

/// Generic Purpose Input/Output registers, version 1.
#[repr(C)]
pub struct RegisterBlockV1 {
    /// System domain GPIO port register group.
    ///
    /// This covers the union of ports used by version-1 SoCs. Individual
    /// chips can leave entries unimplemented; for example, H616 has no PJ,
    /// while R818 has PJ but no PA.
    pub sys_port: [PortV1; 10],
    _reserved1: [u32; 38],
    /// System domain external interrupt register group.
    pub sys_eint: [Eint; 9],
    _reserved2: [u32; 8],
    /// System domain input/output power register group.
    pub sys_pio_pow: PioPow,
    _reserved3: [u32; 123],
    /// RTC domain GPIO port register group.
    pub rtc_port: PortV1,
    _reserved4: [u32; 7],
    /// RTC domain external interrupt register group.
    pub rtc_eint: Eint,
}

impl RegisterBlockV1 {
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
            'A'..='J' => &self.sys_port[p as usize - b'A' as usize],
            'L' => &self.rtc_port,
            _ => panic!("unsupported GPIO port"),
        };
        PortRegisters::new(&port.cfg, &port.dat)
    }

    #[inline]
    pub(in crate::gpio) const fn eint(&self, p: char) -> &Eint {
        match p {
            // H313/H616/H618/A133 use PA as EINT bank 0 and have no PB;
            // R818 uses PB as EINT bank 0 and has no PA. From PC onwards,
            // both layouts use the port letter minus 'B' as the bank index.
            'A' | 'B' => &self.sys_eint[0],
            'C'..='J' => &self.sys_eint[p as usize - b'B' as usize],
            'L' => &self.rtc_eint,
            _ => panic!("unsupported GPIO port"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PortV1, RegisterBlockV1};
    use core::mem::{MaybeUninit, align_of, offset_of, size_of};

    fn zeroed_register_block() -> RegisterBlockV1 {
        // SAFETY: every register payload and reserved field is a `u32`, for
        // which the all-zero bit pattern is valid. The value is only used by
        // host-side layout tests.
        unsafe { MaybeUninit::zeroed().assume_init() }
    }

    #[test]
    fn offset_port_v1() {
        assert_eq!(offset_of!(PortV1, cfg), 0x00);
        assert_eq!(offset_of!(PortV1, dat), 0x10);
        assert_eq!(offset_of!(PortV1, drv), 0x14);
        assert_eq!(offset_of!(PortV1, pull), 0x1c);
        assert_eq!(size_of::<PortV1>(), 0x24);
        assert_eq!(align_of::<PortV1>(), 4);
    }

    #[test]
    fn offset_gpio_v1() {
        assert_eq!(offset_of!(RegisterBlockV1, sys_port), 0x000);
        assert_eq!(offset_of!(RegisterBlockV1, sys_eint), 0x200);
        assert_eq!(offset_of!(RegisterBlockV1, sys_pio_pow), 0x340);
        assert_eq!(offset_of!(RegisterBlockV1, rtc_port), 0x540);
        assert_eq!(offset_of!(RegisterBlockV1, rtc_eint), 0x580);
        assert_eq!(size_of::<RegisterBlockV1>(), 0x5a0);
        assert_eq!(align_of::<RegisterBlockV1>(), 4);
    }

    #[test]
    fn offset_get_gpio_port_eint_v1() {
        let block = zeroed_register_block();
        let base_addr = (&raw const block).cast::<u8>();
        let test_cases = [
            ('A', 0x000, 0x200),
            ('B', 0x024, 0x200),
            ('C', 0x048, 0x220),
            ('D', 0x06c, 0x240),
            ('E', 0x090, 0x260),
            ('F', 0x0b4, 0x280),
            ('G', 0x0d8, 0x2a0),
            ('H', 0x0fc, 0x2c0),
            ('I', 0x120, 0x2e0),
            ('J', 0x144, 0x300),
            ('L', 0x540, 0x580),
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
        let _ = block.port('K');
    }

    #[test]
    #[should_panic(expected = "unsupported GPIO port")]
    fn invalid_eint_panics() {
        let block = zeroed_register_block();
        let _ = block.eint('K');
    }

    #[test]
    fn any_register_block_view_preserves_address() {
        let block = zeroed_register_block();
        let concrete = (&raw const block).cast::<u8>();

        assert_eq!(block.as_any() as *const _ as usize, concrete as usize);
    }
}
