//! GPIO peripheral in D1, T113, V851, V853, V861, F101 and V821 series.

use super::{AnyRegisterBlock, Eint, PioPow, Port};

/// Generic Purpose Input/Output registers, version 2.
#[repr(C)]
pub struct RegisterBlockV2 {
    /// System domain GPIO port register group.
    pub sys_port: [Port; 9],
    _reserved1: [u32; 20],
    /// System domain external interrupt register group.
    pub sys_eint: [Eint; 9],
    _reserved2: [u32; 8],
    /// System domain input/output power register group.
    pub sys_pio_pow: PioPow,
    _reserved3: [u32; 123],
    /// RTC domain GPIO port register group.
    pub rtc_port: Port,
    _reserved4: [u32; 4],
    /// RTC domain external interrupt register group.
    pub rtc_eint: Eint,
}

impl RegisterBlockV2 {
    #[inline(always)]
    pub(in crate::gpio) fn as_any(&self) -> &AnyRegisterBlock {
        // SAFETY: `AnyRegisterBlock` is an opaque zero-sized view of the same
        // GPIO register-block base address, and the returned borrow cannot
        // outlive this concrete register block.
        unsafe { &*(self as *const Self).cast() }
    }

    #[inline]
    pub(crate) const fn port(&self, p: char) -> &Port {
        match p {
            'A'..='I' => &self.sys_port[p as usize - b'A' as usize],
            'L' => &self.rtc_port,
            _ => panic!("unsupported GPIO port"),
        }
    }
    #[inline]
    pub(crate) const fn eint(&self, p: char) -> &Eint {
        match p {
            'A'..='I' => &self.sys_eint[p as usize - b'A' as usize],
            'L' => &self.rtc_eint,
            _ => panic!("unsupported GPIO port"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlockV2;
    use core::mem::{MaybeUninit, align_of, offset_of, size_of};

    fn zeroed_register_block() -> RegisterBlockV2 {
        // SAFETY: every register payload and reserved field is a `u32`, for
        // which the all-zero bit pattern is valid. The value is only used by
        // host-side layout tests.
        unsafe { MaybeUninit::zeroed().assume_init() }
    }

    #[test]
    fn offset_gpio_v2() {
        assert_eq!(offset_of!(RegisterBlockV2, sys_port), 0x0);
        assert_eq!(offset_of!(RegisterBlockV2, sys_eint), 0x200);
        assert_eq!(offset_of!(RegisterBlockV2, sys_pio_pow), 0x340);
        assert_eq!(offset_of!(RegisterBlockV2, rtc_port), 0x540);
        assert_eq!(offset_of!(RegisterBlockV2, rtc_eint), 0x580);
        assert_eq!(size_of::<RegisterBlockV2>(), 0x5a0);
        assert_eq!(align_of::<RegisterBlockV2>(), 4);
    }

    #[test]
    fn offset_get_gpio_port_eint() {
        let block = zeroed_register_block();
        let base_addr = (&raw const block).cast::<u8>();

        let test_cases = [
            ('A', 0, 0x200),
            ('B', 0x30, 0x220),
            ('C', 0x60, 0x240),
            ('D', 0x90, 0x260),
            ('E', 0xC0, 0x280),
            ('F', 0xF0, 0x2A0),
            ('G', 0x120, 0x2C0),
            ('H', 0x150, 0x2E0),
            ('I', 0x180, 0x300),
            ('L', 0x540, 0x580),
        ];

        for (p, port_offset, eint_offset) in test_cases {
            let port_ref = block.port(p);
            let offset = unsafe { (port_ref as *const _ as *const u8).offset_from(base_addr) };
            assert_eq!(
                offset, port_offset,
                "port offset for port {} should be 0x{:0x}",
                p, port_offset
            );

            let eint_ref = block.eint(p);
            let offset = unsafe { (eint_ref as *const _ as *const u8).offset_from(base_addr) };
            assert_eq!(
                offset, eint_offset,
                "eint offset for port {} should be 0x{:0x}",
                p, eint_offset
            );
        }
    }

    #[test]
    #[should_panic(expected = "unsupported GPIO port")]
    fn invalid_port_panics() {
        let block = zeroed_register_block();
        let _ = block.port('J');
    }

    #[test]
    #[should_panic(expected = "unsupported GPIO port")]
    fn invalid_eint_panics() {
        let block = zeroed_register_block();
        let _ = block.eint('J');
    }

    #[test]
    fn any_register_block_view_preserves_address() {
        let block = zeroed_register_block();
        let concrete = (&raw const block).cast::<u8>();

        assert_eq!(block.as_any() as *const _ as usize, concrete as usize);
    }
}
