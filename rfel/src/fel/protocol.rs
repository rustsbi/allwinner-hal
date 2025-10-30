use core::fmt;

/// USB request layout used by the FEL protocol transport.
#[repr(C)]
pub struct UsbRequest {
    magic: [u8; 8],
    length: u32,
    unknown1: u32,
    request: u16,
    length2: u32,
    pad: [u8; 10],
}

impl UsbRequest {
    #[inline]
    pub const fn usb_write(length: u32) -> Self {
        Self {
            magic: *b"AWUC\0\0\0\0",
            request: 0x12,
            length,
            length2: length,
            unknown1: 0x0c00_0000,
            pad: [0; 10],
        }
    }

    #[inline]
    pub const fn usb_read(length: u32) -> Self {
        Self {
            magic: *b"AWUC\0\0\0\0",
            request: 0x11,
            length,
            length2: length,
            unknown1: 0x0c00_0000,
            pad: [0; 10],
        }
    }
}

impl From<UsbRequest> for [u8; 36] {
    #[inline]
    fn from(value: UsbRequest) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

/// FEL command request layout.
#[repr(C)]
pub struct FelRequest {
    request: u32,
    address: u32,
    length: u32,
    pad: u32,
}

impl FelRequest {
    #[inline]
    pub const fn get_version() -> Self {
        Self {
            request: 0x001,
            address: 0,
            length: 0,
            pad: 0,
        }
    }

    #[inline]
    pub const fn read_raw(address: u32, length: u32) -> Self {
        Self {
            request: 0x103,
            address,
            length,
            pad: 0,
        }
    }

    #[inline]
    pub const fn write_raw(address: u32, length: u32) -> Self {
        Self {
            request: 0x101,
            address,
            length,
            pad: 0,
        }
    }

    #[inline]
    pub const fn exec(address: u32) -> Self {
        Self {
            request: 0x102,
            address,
            length: 0,
            pad: 0,
        }
    }
}

impl From<FelRequest> for [u8; 16] {
    #[inline]
    fn from(value: FelRequest) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Version {
    magic: [u8; 8],
    id: u32,
    firmware: u32,
    protocol: u16,
    dflag: u8,
    dlength: u8,
    scratchpad: u32,
    pad: [u8; 8],
}

impl Version {
    pub fn chip(self) -> Option<Chip> {
        match self.id {
            0x0018_5900 => Some(Chip::D1),
            _ => None,
        }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub fn scratchpad(&self) -> u32 {
        self.scratchpad
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        map.entry(&"magic", &String::from_utf8_lossy(&self.magic));
        match self.chip() {
            Some(chip) => map.entry(&"chip", &chip),
            None => map.entry(&"id", &self.id),
        };
        map.entry(&"dflag", &self.dflag)
            .entry(&"dlength", &self.dlength)
            .entry(&"scratchpad", &self.scratchpad)
            .finish()
    }
}

impl From<[u8; 32]> for Version {
    #[inline]
    fn from(value: [u8; 32]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

#[derive(Debug)]
#[repr(u32)]
pub enum Chip {
    /// D1-H, D1s or F133 chip.
    D1 = 0x0018_5900,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_endian = "little")]
    fn test_usb_request_pack_read_write() {
        let b_read: [u8; 36] = UsbRequest::usb_read(0x1234).into();
        assert_eq!(&b_read[0..8], b"AWUC\0\0\0\0");
        assert_eq!(
            u32::from_le_bytes(b_read[8..12].try_into().unwrap()),
            0x1234
        );
        assert_eq!(
            u32::from_le_bytes(b_read[12..16].try_into().unwrap()),
            0x0c00_0000
        );
        assert_eq!(u16::from_le_bytes(b_read[16..18].try_into().unwrap()), 0x11);
        assert_eq!(
            u32::from_le_bytes(b_read[20..24].try_into().unwrap()),
            0x1234
        );

        let b_write: [u8; 36] = UsbRequest::usb_write(0x5678).into();
        assert_eq!(&b_write[0..8], b"AWUC\0\0\0\0");
        assert_eq!(
            u32::from_le_bytes(b_write[8..12].try_into().unwrap()),
            0x5678
        );
        assert_eq!(
            u32::from_le_bytes(b_write[12..16].try_into().unwrap()),
            0x0c00_0000
        );
        assert_eq!(
            u16::from_le_bytes(b_write[16..18].try_into().unwrap()),
            0x12
        );
        assert_eq!(
            u32::from_le_bytes(b_write[20..24].try_into().unwrap()),
            0x5678
        );
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn test_fel_request_pack() {
        let r_read: [u8; 16] = FelRequest::read_raw(0xA0B0C0D0, 0x11).into();
        assert_eq!(u32::from_le_bytes(r_read[0..4].try_into().unwrap()), 0x103);
        assert_eq!(
            u32::from_le_bytes(r_read[4..8].try_into().unwrap()),
            0xA0B0C0D0
        );
        assert_eq!(u32::from_le_bytes(r_read[8..12].try_into().unwrap()), 0x11);

        let r_exec: [u8; 16] = FelRequest::exec(0x20000).into();
        assert_eq!(u32::from_le_bytes(r_exec[0..4].try_into().unwrap()), 0x102);
        assert_eq!(
            u32::from_le_bytes(r_exec[4..8].try_into().unwrap()),
            0x20000
        );
        assert_eq!(u32::from_le_bytes(r_exec[8..12].try_into().unwrap()), 0);
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn test_version_from_bytes_and_chip() {
        let mut raw = [0u8; 32];
        raw[0..8].copy_from_slice(b"AWUS\0\0\0\0");
        raw[8..12].copy_from_slice(&0x0018_5900u32.to_le_bytes());
        raw[12..16].copy_from_slice(&0x0102_0304u32.to_le_bytes());
        raw[16..18].copy_from_slice(&0x0201u16.to_le_bytes());
        raw[18] = 0xAA;
        raw[19] = 0xBB;
        raw[20..24].copy_from_slice(&0x20000u32.to_le_bytes());

        let v: Version = raw.into();
        assert_eq!(v.id(), 0x0018_5900);
        assert_eq!(v.scratchpad(), 0x20000);
        assert!(matches!(v.chip(), Some(Chip::D1)));
    }
}
