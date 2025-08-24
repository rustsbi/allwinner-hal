use core::fmt;
use futures::executor::block_on;
use log::{debug, error, trace};
use nusb::transfer::EndpointType;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

pub mod chips;

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

    pub fn exec(&self, address: u32) {
        trace!("exec");
        self.send_fel_request(FelRequest::exec(address));
        self.read_fel_status();
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

impl From<UsbRequest> for [u8; 36] {
    #[inline]
    fn from(value: UsbRequest) -> Self {
        unsafe { core::mem::transmute(value) }
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
    pub const fn read_raw(address: u32, length: u32) -> Self {
        FelRequest {
            request: 0x103,
            address,
            length,
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
    #[inline]
    pub const fn exec(address: u32) -> Self {
        FelRequest {
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
    /// Get chip from version.
    pub fn chip(self) -> Option<Chip> {
        match self.id {
            0x00185900 => Some(Chip::D1),
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
    D1 = 0x00185900,
}

/// A lightweight progress bar that prints to stdout, for use with read/write operations.
pub struct Progress {
    label: &'static str,
    total: u64, // 0 means the total size is unknown
    done: u64,
    start: Instant,
    last: Instant,
    tick_every: Duration,
}

impl Progress {
    pub fn new(label: &'static str, total: u64) -> Self {
        let now = Instant::now();
        Self {
            label,
            total,
            done: 0,
            start: now,
            last: now,
            tick_every: Duration::from_millis(100),
        }
    }
    pub fn inc(&mut self, n: u64) {
        self.done = self.done.saturating_add(n);
        let now = Instant::now();
        if now.duration_since(self.last) >= self.tick_every || self.done >= self.total {
            self.last = now;
            self.draw(now);
        }
    }
    pub fn finish(&mut self) {
        self.draw(Instant::now());
        println!();
    }
    fn draw(&self, now: Instant) {
        let elapsed = now.duration_since(self.start).as_secs_f64().max(1e-6);
        let speed = self.done as f64 / elapsed; // B/s
        let (spd_val, spd_unit) = human_speed(speed);
        let pct = if self.total > 0 {
            (self.done as f64 / self.total as f64 * 100.0).min(100.0)
        } else {
            0.0
        };
        let eta_secs = if speed > 0.0 && self.total > self.done {
            ((self.total - self.done) as f64 / speed) as u64
        } else {
            0
        };
        let bar = render_bar(pct, 30);
        print!(
            "\r{:<5} {} {:6.2}% {:5.1} {}/s ETA {} {}/{}",
            self.label,
            bar,
            pct,
            spd_val,
            spd_unit,
            fmt_eta(eta_secs),
            human_bytes(self.done),
            human_bytes(self.total),
        );
        let _ = io::stdout().flush();
    }
}

fn render_bar(pct: f64, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f64).round() as usize;
    let mut s = String::with_capacity(width + 2);
    s.push('[');
    for i in 0..width {
        s.push(if i < filled { '#' } else { ' ' });
    }
    s.push(']');
    s
}

fn human_bytes(n: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut val = n as f64;
    let mut idx = 0usize;
    while val >= 1024.0 && idx + 1 < UNITS.len() {
        val /= 1024.0;
        idx += 1;
    }
    if idx == 0 {
        format!("{:.0}{}", val, UNITS[idx])
    } else {
        format!("{:.2}{}", val, UNITS[idx])
    }
}

fn human_speed(bps: f64) -> (f64, &'static str) {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut val = bps;
    let mut idx = 0usize;
    while val >= 1024.0 && idx + 1 < UNITS.len() {
        val /= 1024.0;
        idx += 1;
    }
    (val, UNITS[idx])
}

fn fmt_eta(mut secs: u64) -> String {
    let h = secs / 3600;
    secs %= 3600;
    let m = secs / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}

/// Reads `length` bytes in chunks to a writer, with optional progress reporting.
/// Note: This function hasn't gone through comprehensive upper-level testing yet.
pub fn read_to_writer(
    fel: &Fel<'_>,
    mut address: u32,
    mut length: usize,
    writer: &mut impl Write,
    mut progress: Option<&mut Progress>,
) -> io::Result<usize> {
    let mut buf = vec![0u8; CHUNK_SIZE];
    let mut written = 0usize;
    while length > 0 {
        let n = length.min(CHUNK_SIZE);
        let slice = &mut buf[..n];
        fel.read_address(address, slice);
        writer.write_all(slice)?;
        written += n;
        if let Some(p) = progress.as_deref_mut() {
            p.inc(n as u64);
        }
        length -= n;
        address = address.wrapping_add(n as u32);
    }
    Ok(written)
}

/// Writes to memory from a reader in chunks, with optional progress reporting.
/// Note: This function hasn't gone through comprehensive upper-level testing yet.
pub fn write_from_reader(
    fel: &Fel<'_>,
    mut address: u32,
    reader: &mut impl Read,
    mut progress: Option<&mut Progress>,
) -> io::Result<usize> {
    let mut buf = vec![0u8; CHUNK_SIZE];
    let mut total = 0usize;
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        fel.write_address(address, &buf[..n]);
        total += n;
        if let Some(p) = progress.as_deref_mut() {
            p.inc(n as u64);
        }
        address = address.wrapping_add(n as u32);
    }
    Ok(total)
}

/// write by blocks
pub fn write_all(fel: &Fel<'_>, mut addr: u32, mut data: &[u8]) {
    while !data.is_empty() {
        let n = data.len().min(CHUNK_SIZE);
        fel.write_address(addr, &data[..n]);
        addr = addr.wrapping_add(n as u32);
        data = &data[n..];
    }
}

/// read by blocks
pub fn read_all(fel: &Fel<'_>, mut addr: u32, mut out: &mut [u8]) {
    while !out.is_empty() {
        let n = out.len().min(CHUNK_SIZE);
        let (head, tail) = out.split_at_mut(n);
        fel.read_address(addr, head);
        addr = addr.wrapping_add(n as u32);
        out = tail;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_bytes() {
        assert_eq!(human_bytes(0), "0B");
        assert_eq!(human_bytes(1023), "1023B");
        assert_eq!(human_bytes(1024), "1.00KiB");
        assert_eq!(human_bytes(1024 * 1024), "1.00MiB");
    }

    #[test]
    fn test_fmt_eta() {
        assert_eq!(fmt_eta(59), "00:59");
        assert_eq!(fmt_eta(61), "01:01");
        assert_eq!(fmt_eta(3661), "01:01:01");
    }

    #[test]
    fn test_human_speed() {
        let (v, u) = human_speed(500.0);
        assert_eq!(u, "B");
        assert!((v - 500.0).abs() < 1e-9);

        let (v, u) = human_speed(1024.0);
        assert_eq!(u, "KiB");
        assert!((v - 1.0).abs() < 1e-9);

        let (v, u) = human_speed(1536.0);
        assert_eq!(u, "KiB");
        assert!((v - 1.5).abs() < 1e-9);
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn test_usb_request_pack_read_write() {
        let b_read: [u8; 36] = UsbRequest::usb_read(0x1234).into();
        assert_eq!(&b_read[0..8], b"AWUC\0\0\0\0");
        // length
        assert_eq!(
            u32::from_le_bytes(b_read[8..12].try_into().unwrap()),
            0x1234
        );
        assert_eq!(
            u32::from_le_bytes(b_read[12..16].try_into().unwrap()),
            0x0c00_0000
        );
        // request (u16) at offset 16..18
        assert_eq!(u16::from_le_bytes(b_read[16..18].try_into().unwrap()), 0x11);
        // length2 (u32) at offset 20..24
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
        // id = 0x00185900 -> D1
        raw[8..12].copy_from_slice(&0x0018_5900u32.to_le_bytes());
        // firmware
        raw[12..16].copy_from_slice(&0x01020304u32.to_le_bytes());
        // protocol (u16), dflag (u8), dlength (u8)
        raw[16..18].copy_from_slice(&0x0201u16.to_le_bytes());
        raw[18] = 0xAA;
        raw[19] = 0xBB;
        // scratchpad
        raw[20..24].copy_from_slice(&0x20000u32.to_le_bytes());
        // pad[8]:0

        let v: Version = raw.into();
        assert_eq!(v.id(), 0x0018_5900);
        assert_eq!(v.scratchpad(), 0x20000);
        assert!(matches!(v.chip(), Some(Chip::D1)));
    }

    #[test]
    fn test_render_bar() {
        let b0 = render_bar(0.0, 10);
        assert_eq!(b0, "[          ]");
        let b50 = render_bar(50.0, 10);
        assert_eq!(b50, "[#####     ]");
        let b100 = render_bar(100.0, 10);
        assert_eq!(b100, "[##########]");
    }
}
