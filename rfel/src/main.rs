use core::fmt;

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use futures::executor::block_on;
use log::{debug, error, trace};
use nusb::transfer::EndpointType;

#[derive(Parser)]
#[clap(name = "rfel")]
#[clap(about = "Allwinner FEL tool", long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,
}

/// USB vendor ID 0x1f3a: Allwinner Technology Co., Ltd.
const VENDOR_ALLWINNER: u16 = 0x1f3a;
/// Product 0xefe8: sunxi SoC OTG connector in FEL/flashing mode.
const PRODUCT_FEL: u16 = 0xefe8;

fn main() {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();
    let devices: Vec<_> = nusb::list_devices()
        .expect("list devices")
        .filter(|dev| dev.vendor_id() == VENDOR_ALLWINNER && dev.product_id() == PRODUCT_FEL)
        .inspect(|dev| debug!("Allwinner FEL device {:?}", dev))
        .collect();
    if devices.len() == 0 {
        error!("Cannot find any Allwinner FEL device connected.");
        return;
    }
    if devices.len() > 1 {
        error!("TODO: rfel does not support connecting to multiple Allwinner FEL devices by now.");
        return;
    }
    let device = devices[0].open().expect("open USB device");
    let mut interface = device.claim_interface(0).expect("open USB interface 0");
    let fel = Fel::open_usb_interface(&mut interface).expect("open usb interface as an FEL device");
    let version = fel.get_version();
    println!("{:?}", version);
}

struct Fel<'a> {
    iface: &'a mut nusb::Interface,
    endpoint_in: u8,
    endpoint_out: u8,
    version: Option<Version>,
}

impl<'a> Fel<'a> {
    #[inline]
    pub fn open_usb_interface(iface: &'a mut nusb::Interface) -> Result<Self, ()> {
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
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Version {
    magic: [u8; 8],
    id: u32,
    firmware: u32,
    protocol: u16,
    dflag: u8,
    dlength: u8,
    scratchpad: u32,
    pad: [u8; 8],
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entry(&"magic", &String::from_utf8_lossy(&self.magic))
            .entry(&"id", &self.id)
            .entry(&"dflag", &self.dflag)
            .entry(&"dlength", &self.dlength)
            .entry(&"scratchpad", &self.scratchpad)
            .finish()
    }
}
