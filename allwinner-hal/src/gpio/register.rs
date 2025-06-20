use volatile_register::RW;

/// Generic Purpose Input/Output registers.
#[repr(C)]
pub struct RegisterBlock {
    /// System domain GPIO port register group.
    pub sys_port: [Port; 7],
    _reserved1: [u32; 44],
    /// System domain external interrupt register group.
    pub sys_eint: [Eint; 7],
    _reserved2: [u32; 24],
    /// System domain input/output power register group.
    pub sys_pio_pow: PioPow,
    _reserved3: [u32; 123],
    /// RTC domain GPIO port register group.
    pub rtc_port: Port,
    _reserved4: [u32; 4],
    /// RTC domain external interrupt register group.
    pub rtc_eint: Eint,
}

impl RegisterBlock {
    #[inline]
    pub(crate) const fn port(&self, p: char) -> &Port {
        assert!((p as usize >= b'A' as usize && p as usize <= b'G' as usize) || p == 'L');
        match p {
            'A'..='G' => &self.sys_port[p as usize - b'A' as usize],
            'L' => &self.rtc_port,
            _ => unreachable!(),
        }
    }
    #[inline]
    pub(crate) const fn eint(&self, p: char) -> &Eint {
        assert!((p as usize >= b'A' as usize && p as usize <= b'G' as usize) || p == 'L');
        match p {
            'A'..='G' => &self.sys_eint[p as usize - b'A' as usize],
            'L' => &self.rtc_eint,
            _ => unreachable!(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::{Eint, PioPow, Port, RegisterBlock};
    use core::mem::offset_of;

    #[test]
    fn offset_gpio() {
        assert_eq!(offset_of!(RegisterBlock, sys_port), 0x0);
        assert_eq!(offset_of!(RegisterBlock, sys_eint), 0x200);
        assert_eq!(offset_of!(RegisterBlock, sys_pio_pow), 0x340);
        assert_eq!(offset_of!(RegisterBlock, rtc_port), 0x540);
        assert_eq!(offset_of!(RegisterBlock, rtc_eint), 0x580);
    }

    #[test]
    fn offset_get_gpio_port_eint() {
        use core::mem::MaybeUninit;
        let block = MaybeUninit::<RegisterBlock>::uninit();
        let block_ptr = block.as_ptr() as *const RegisterBlock;
        let base_addr = block_ptr as *const u8;

        let test_cases = [
            ('A', 0, 0x200),
            ('B', 0x30, 0x220),
            ('C', 0x60, 0x240),
            ('D', 0x90, 0x260),
            ('E', 0xC0, 0x280),
            ('F', 0xF0, 0x2A0),
            ('G', 0x120, 0x2C0),
            ('L', 0x540, 0x580),
        ];

        for (p, port_offset, eint_offset) in test_cases {
            let port_ref = unsafe { (*block_ptr).port(p) };
            let offset = unsafe { (port_ref as *const _ as *const u8).offset_from(base_addr) };
            assert_eq!(
                offset, port_offset,
                "port offset for port {} should be 0x{:0x}",
                p, port_offset
            );

            let eint_ref = unsafe { (*block_ptr).eint(p) };
            let offset = unsafe { (eint_ref as *const _ as *const u8).offset_from(base_addr) };
            assert_eq!(
                offset, eint_offset,
                "eint offset for port {} should be 0x{:0x}",
                p, eint_offset
            );
        }
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
