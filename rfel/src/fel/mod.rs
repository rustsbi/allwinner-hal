mod protocol;

pub use protocol::{Chip, FelRequest, UsbRequest, Version};

use futures::executor::block_on;
use log::{debug, error, trace};
use nusb::transfer::EndpointType;

/// Maximum chunk size for a single FEL read or write operation.
pub const CHUNK_SIZE: usize = 65_536;

pub struct Fel<'a> {
    iface: &'a mut nusb::Interface,
    endpoint_in: u8,
    endpoint_out: u8,
    version: Option<Version>,
}

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
            error!(
                "Malformed device. Allwinner USB FEL device should include exactly one bulk in and one bulk out endpoint."
            );
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
            buf.into()
        })
    }

    pub fn read_address(&self, address: u32, buf: &mut [u8]) -> usize {
        trace!("read_address(single chunk)");
        debug_assert!(
            buf.len() <= CHUNK_SIZE,
            "read_address expects a single chunk (<= {CHUNK_SIZE} bytes)"
        );
        self.send_fel_request(FelRequest::read_raw(address, buf.len() as u32));
        self.usb_read(buf);
        self.read_fel_status();
        buf.len()
    }

    pub fn write_address(&self, address: u32, buf: &[u8]) -> usize {
        trace!("write_address(single chunk)");
        debug_assert!(
            buf.len() <= CHUNK_SIZE,
            "write_address expects a single chunk (<= {CHUNK_SIZE} bytes)"
        );
        self.send_fel_request(FelRequest::write_raw(address, buf.len() as u32));
        self.usb_write(buf);
        self.read_fel_status();
        buf.len()
    }

    pub fn exec(&self, address: u32) {
        trace!("exec");
        self.send_fel_request(FelRequest::exec(address));
        self.read_fel_status();
    }

    fn send_fel_request(&self, request: FelRequest) {
        trace!("send_fel_request");
        let buf: [u8; 16] = request.into();
        self.usb_write(&buf);
    }

    fn read_fel_status(&self) {
        trace!("read_fel_status");
        let mut buf = [0u8; 8];
        self.usb_read(&mut buf);
    }

    fn usb_read(&self, buf: &mut [u8]) {
        trace!("usb_read");
        let buf_1: [u8; 36] = UsbRequest::usb_read(buf.len() as u32).into();
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

    fn usb_write(&self, buf: &[u8]) {
        trace!("usb_write");
        let buf_1: [u8; 36] = UsbRequest::usb_write(buf.len() as u32).into();
        block_on(self.iface.bulk_out(self.endpoint_out, buf_1.to_vec()))
            .status
            .expect("send_usb_request on usb_write transfer");
        block_on(self.iface.bulk_out(self.endpoint_out, buf.to_vec()))
            .status
            .expect("usb bulk out on usb_write transfer");
        let buf_3 = nusb::transfer::RequestBuffer::new(13);
        let ans_1 = block_on(self.iface.bulk_in(self.endpoint_in, buf_3));
        ans_1
            .status
            .expect("read_usb_response on usb_write transfer");
        if ans_1.data != *b"AWUS\0\0\0\0\0\0\0\0\0" {
            panic!("invalid data received from read_usb_response")
        }
    }
}
