pub mod error;
mod protocol;

pub use protocol::{Chip, FelRequest, UsbRequest, Version};

use std::cell::RefCell;

use error::{FelError, FelResult, check_fel_ack, check_usb_read_length};
use futures::executor::block_on;
use log::{debug, error, trace};
use nusb::descriptors::TransferType;
use nusb::transfer::{Buffer, Bulk, In, Out};

/// Maximum chunk size for a single FEL read or write operation.
pub const CHUNK_SIZE: usize = 65_536;

pub struct Fel<'a> {
    _iface: &'a mut nusb::Interface,
    endpoint_in: RefCell<nusb::Endpoint<Bulk, In>>,
    endpoint_out: RefCell<nusb::Endpoint<Bulk, Out>>,
    version: Option<Version>,
}

impl<'a> Fel<'a> {
    #[inline]
    pub fn open_interface(iface: &'a mut nusb::Interface) -> FelResult<Self> {
        let mut endpoint_in = None;
        let mut endpoint_out = None;
        for descriptor in iface.descriptors() {
            for endpoint in descriptor.endpoints() {
                if endpoint.transfer_type() != TransferType::Bulk {
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
            return Err(FelError::MissingBulkEndpoints);
        };

        debug!(
            "Endpoint in ID 0x{:x}, out ID 0x{:x}",
            endpoint_in, endpoint_out
        );

        let open_endpoint_error = |address: u8, error: nusb::Error| FelError::OpenBulkEndpoint {
            address,
            kind: error.kind(),
        };
        let endpoint_in = iface
            .endpoint::<Bulk, In>(endpoint_in)
            .map_err(|error| open_endpoint_error(endpoint_in, error))?;
        let endpoint_out = iface
            .endpoint::<Bulk, Out>(endpoint_out)
            .map_err(|error| open_endpoint_error(endpoint_out, error))?;

        Ok(Self {
            _iface: iface,
            endpoint_in: RefCell::new(endpoint_in),
            endpoint_out: RefCell::new(endpoint_out),
            version: None,
        })
    }

    pub fn get_version(&self) -> FelResult<Version> {
        if let Some(version) = self.version {
            return Ok(version);
        }

        let mut buf = [0u8; 32];
        self.send_fel_request(FelRequest::get_version())?;
        self.usb_read(&mut buf)?;
        self.read_fel_status()?;
        Ok(buf.into())
    }

    pub fn read_address(&self, address: u32, buf: &mut [u8]) -> FelResult<()> {
        trace!("read_address(single chunk)");
        debug_assert!(
            buf.len() <= CHUNK_SIZE,
            "read_address expects a single chunk (<= {CHUNK_SIZE} bytes)"
        );
        self.send_fel_request(FelRequest::read_raw(address, buf.len() as u32))?;
        self.usb_read(buf)?;
        self.read_fel_status()
    }

    pub fn write_address(&self, address: u32, buf: &[u8]) -> FelResult<()> {
        trace!("write_address(single chunk)");
        debug_assert!(
            buf.len() <= CHUNK_SIZE,
            "write_address expects a single chunk (<= {CHUNK_SIZE} bytes)"
        );
        self.send_fel_request(FelRequest::write_raw(address, buf.len() as u32))?;
        self.usb_write(buf)?;
        self.read_fel_status()
    }

    pub fn exec(&self, address: u32) -> FelResult<()> {
        trace!("exec");
        self.send_fel_request(FelRequest::exec(address))?;
        self.read_fel_status()?;
        log::debug!("Execution started at 0x{:08x},", address);
        Ok(())
    }

    fn send_fel_request(&self, request: FelRequest) -> FelResult<()> {
        trace!("send_fel_request");
        let buf: [u8; 16] = request.into();
        self.usb_write(&buf)
    }

    fn read_fel_status(&self) -> FelResult<()> {
        trace!("read_fel_status");
        let mut buf = [0u8; 8];
        self.usb_read(&mut buf)?;
        check_fel_ack(buf)
    }

    fn usb_read(&self, buf: &mut [u8]) -> FelResult<()> {
        trace!("usb_read");
        let buf_1: [u8; 32] = UsbRequest::usb_read(buf.len() as u32).into();
        self.bulk_out(buf_1.to_vec(), "sending a USB read request")?;
        let data = self.bulk_in(buf.len(), "receiving USB read data")?;
        let response = self.bulk_in(13, "receiving the USB read response")?;
        if response != *b"AWUS\0\0\0\0\0\0\0\0\0" {
            return Err(FelError::InvalidUsbResponse);
        }
        check_usb_read_length(&data, buf.len())?;
        buf.copy_from_slice(&data);
        Ok(())
    }

    fn usb_write(&self, buf: &[u8]) -> FelResult<()> {
        trace!("usb_write");
        let buf_1: [u8; 32] = UsbRequest::usb_write(buf.len() as u32).into();
        self.bulk_out(buf_1.to_vec(), "sending a USB write request")?;
        self.bulk_out(buf.to_vec(), "sending USB write data")?;
        let response = self.bulk_in(13, "receiving the USB write response")?;
        if response != *b"AWUS\0\0\0\0\0\0\0\0\0" {
            return Err(FelError::InvalidUsbResponse);
        }
        Ok(())
    }

    fn bulk_in(&self, length: usize, stage: &'static str) -> FelResult<Vec<u8>> {
        let mut endpoint = self.endpoint_in.borrow_mut();
        let packet_size = endpoint.max_packet_size();
        let request_length = length.div_ceil(packet_size) * packet_size;
        endpoint.submit(Buffer::new(request_length));
        let completion = block_on(endpoint.next_complete());
        completion
            .status
            .map_err(|source| FelError::UsbTransfer { stage, source })?;
        Ok(completion.buffer.into_vec())
    }

    fn bulk_out(&self, data: Vec<u8>, stage: &'static str) -> FelResult<()> {
        let mut endpoint = self.endpoint_out.borrow_mut();
        endpoint.submit(data.into());
        block_on(endpoint.next_complete())
            .status
            .map_err(|source| FelError::UsbTransfer { stage, source })
    }
}
