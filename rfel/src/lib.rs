use core::fmt;
use futures::executor::block_on;
use log::{debug, error, trace};
use nusb::transfer::EndpointType;

pub struct Fel<'a> {
    iface: &'a mut nusb::Interface,
    endpoint_in: u8,
    endpoint_out: u8,
    version: Option<Version>,
}

const CHUNK_SIZE: usize = 65536;

impl<'a> Fel<'a> {
    #[inline]
    pub fn open_interface(iface: &'a mut nusb::Interface) -> Result<Self, ()> {
        let mut endpoint_in = None;
        let mut endpoint_out = None;
        for descriptor in iface.descriptors() {
            for endpoint in descriptor.endpoints() {
                if endpoint.transfer_type() != EndpointType::Bulk {
                    continue;
                }
                match endpoint.direction() {
                    nusb::transfer::Direction::In => endpoint_in = Some(endpoint.address()),
                    nusb::transfer::Direction::Out => endpoint_out = Some(endpoint.address()),
                }
            }
        }
        let (Some(endpoint_in), Some(endpoint_out)) = (endpoint_in, endpoint_out) else {
            error!("Malformed device. Allwinner USB FEL device should include exactly one bulk in and one bulk out endpoint.");
            return Err(());
        };
        debug!(
            "Endpoint in ID 0x{:x}, out ID 0x{:x}",
            endpoint_in, endpoint_out
        );
        Ok(Self {
            iface,
            endpoint_in,
            endpoint_out,
            version: None,
        })
    }

    pub fn get_version(&self) -> Version {
        self.version.unwrap_or_else(|| {
            let mut buf = [0u8; 32];
            self.send_fel_request(FelRequest::get_version());
            self.usb_read(&mut buf);
            self.read_fel_status();
            unsafe { core::mem::transmute(buf) }
        })
    }

    pub fn read_address(&self, address: u32, buf: &mut [u8]) -> usize {
        for chunk in buf.chunks_mut(CHUNK_SIZE) {
            self.send_fel_request(FelRequest::write_raw(address, chunk.len() as u32));
            self.usb_read(chunk);
            self.read_fel_status();
        }
        buf.len()
    }

    fn send_fel_request(&self, request: FelRequest) {
        trace!("send_fel_request");
        let buf: [u8; 36] = unsafe {
            core::mem::transmute(UsbRequest::usb_write(
                core::mem::size_of::<FelRequest>() as u32
            ))
        };
        block_on(self.iface.bulk_out(self.endpoint_out, buf.to_vec()))
            .status
            .expect("send_usb_request on send_fel_request transfer");
        let buf: [u8; 16] = unsafe { core::mem::transmute(request) };
        block_on(self.iface.bulk_out(self.endpoint_out, buf.to_vec()))
            .status
            .expect("usb bulk out on send_fel_request transfer");
        let buf = nusb::transfer::RequestBuffer::new(13);
        let ans = block_on(self.iface.bulk_in(self.endpoint_in, buf));
        ans.status
            .expect("read_usb_response on send_fel_request transfer");
        if ans.data != *b"AWUS\0\0\0\0\0\0\0\0\0" {
            panic!("invalid data received from read_usb_response")
        }
    }

    fn usb_read(&self, buf: &mut [u8]) {
        trace!("usb_read");
        let buf_1: [u8; 36] =
            unsafe { core::mem::transmute(UsbRequest::usb_read(buf.len() as u32)) };
        block_on(self.iface.bulk_out(self.endpoint_out, buf_1.to_vec()))
            .status
            .expect("send_usb_request on usb_read transfer");
        let buf_2 = nusb::transfer::RequestBuffer::new(buf.len());
        let ans = block_on(self.iface.bulk_in(self.endpoint_in, buf_2));
        ans.status.expect("usb bulk out on usb_read transfer");
        let buf_3 = nusb::transfer::RequestBuffer::new(13);
        let ans_1 = block_on(self.iface.bulk_in(self.endpoint_in, buf_3));
        ans_1
            .status
            .expect("read_usb_response on usb_read transfer");
        if ans_1.data != *b"AWUS\0\0\0\0\0\0\0\0\0" {
            panic!("invalid data received from read_usb_response")
        }
        buf.copy_from_slice(&ans.data);
    }

    fn read_fel_status(&self) {
        trace!("read_fel_status");
        let mut buf = [0u8; 8];
        self.usb_read(&mut buf);
    }
}

/// USB request.
#[repr(C)]
struct UsbRequest {
    magic: [u8; 8],
    length: u32,
    unknown1: u32,
    request: u16,
    length2: u32,
    pad: [u8; 10],
}

impl UsbRequest {
    #[inline]
    const fn usb_write(length: u32) -> Self {
        UsbRequest {
            magic: *b"AWUC\0\0\0\0",
            request: 0x12,
            length,
            length2: length,
            unknown1: 0x0c00_0000,
            pad: [0; 10],
        }
    }
    #[inline]
    const fn usb_read(length: u32) -> Self {
        UsbRequest {
            magic: *b"AWUC\0\0\0\0",
            request: 0x11,
            length,
            length2: length,
            unknown1: 0x0c00_0000,
            pad: [0; 10],
        }
    }
}

/// FEL request.
#[repr(C)]
struct FelRequest {
    request: u32,
    address: u32,
    length: u32,
    pad: u32,
}

impl FelRequest {
    #[inline]
    pub const fn get_version() -> Self {
        FelRequest {
            request: 0x001,
            address: 0,
            length: 0,
            pad: 0,
        }
    }
    #[inline]
    pub const fn write_raw(address: u32, length: u32) -> Self {
        FelRequest {
            request: 0x101,
            address,
            length,
            pad: 0,
        }
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
    /// Get chip from version.
    pub fn chip(self) -> Option<Chip> {
        match self.id {
            0x00185900 => Some(Chip::D1),
            _ => None,
        }
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

#[derive(Debug)]
#[repr(u32)]
pub enum Chip {
    /// D1-H, D1s or F133 chip.
    D1 = 0x00185900,
}
