use core::convert::TryFrom;
use std::fmt;
use std::time::{Duration, Instant};

use crate::chips::Chip;
use crate::fel::Fel;
use crate::progress::Progress;
use crate::spi::{self, SpiError, SpiSession};

use super::{erase_span, range_in_bounds};

const OPCODE_SFDP: u8 = 0x5a;
const OPCODE_RDID: u8 = 0x9f;
const OPCODE_READ: u8 = 0x03;
const OPCODE_PAGE_PROGRAM: u8 = 0x02;
const OPCODE_WRITE_ENABLE: u8 = 0x06;
const OPCODE_WRSR: u8 = 0x01;
const OPCODE_RDSR: u8 = 0x05;
const OPCODE_ENTER_4B: u8 = 0xb7;
const OPCODE_RESET_ENABLE: u8 = 0x66;
const OPCODE_RESET_MEMORY: u8 = 0x99;
const OPCODE_GLOBAL_UNLOCK: u8 = 0x98;
const WAIT_TIMEOUT: Duration = Duration::from_secs(5);
const SFDP_MAX_PARAMETERS: usize = 6;
const FLASH_TRANSFER_CHUNK: usize = 64 * 1024;

#[derive(Debug)]
pub enum SpinorError {
    Spi(SpiError),
    Unsupported(&'static str),
    InvalidResponse(&'static str),
    AddressOverflow,
    Timeout,
}

impl fmt::Display for SpinorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpinorError::Spi(err) => write!(f, "spi error: {err}"),
            SpinorError::Unsupported(msg) => write!(f, "unsupported: {msg}"),
            SpinorError::InvalidResponse(msg) => write!(f, "invalid response: {msg}"),
            SpinorError::AddressOverflow => write!(f, "address out of range for device"),
            SpinorError::Timeout => write!(f, "operation timed out waiting for device"),
        }
    }
}

impl std::error::Error for SpinorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpinorError::Spi(err) => Some(err),
            _ => None,
        }
    }
}

impl From<SpiError> for SpinorError {
    fn from(value: SpiError) -> Self {
        SpinorError::Spi(value)
    }
}

type SpinorResult<T> = Result<T, SpinorError>;

pub struct DetectInfo {
    pub name: String,
    pub capacity: u64,
}

pub fn detect(chip: &dyn Chip, fel: &Fel<'_>) -> SpinorResult<DetectInfo> {
    let state = SpinorState::new(chip, fel)?;
    Ok(DetectInfo {
        name: state.info.name.clone(),
        capacity: state.info.capacity,
    })
}

pub fn erase(
    chip: &dyn Chip,
    fel: &Fel<'_>,
    address: u64,
    length: u64,
    progress: Option<&mut Progress>,
) -> SpinorResult<()> {
    let mut state = SpinorState::new(chip, fel)?;
    state.erase_range(fel, address, length, progress)
}

pub fn read(
    chip: &dyn Chip,
    fel: &Fel<'_>,
    address: u64,
    buffer: &mut [u8],
    progress: Option<&mut Progress>,
) -> SpinorResult<()> {
    let mut state = SpinorState::new(chip, fel)?;
    state.read_range(fel, address, buffer, progress)
}

pub fn write(
    chip: &dyn Chip,
    fel: &Fel<'_>,
    address: u64,
    data: &[u8],
    progress: Option<&mut Progress>,
) -> SpinorResult<()> {
    let mut state = SpinorState::new(chip, fel)?;
    state.erase_range(fel, address, data.len() as u64, None)?;
    state.write_range(fel, address, data, progress)
}

struct SpinorState<'chip> {
    session: SpiSession<'chip>,
    info: SpinorInfo,
}

impl<'chip> SpinorState<'chip> {
    fn new(chip: &'chip dyn Chip, fel: &Fel<'_>) -> SpinorResult<Self> {
        let session = spi::begin(chip, fel)?;
        let info = SpinorInfo::detect(fel, &session)?;
        let mut state = Self { session, info };
        state.initialise(fel)?;
        Ok(state)
    }

    fn initialise(&mut self, fel: &Fel<'_>) -> SpinorResult<()> {
        self.reset(fel)?;
        self.wait_ready(fel)?;
        self.write_enable(fel)?;
        self.global_unlock(fel)?;
        self.wait_ready(fel)?;
        self.write_enable(fel)?;
        self.write_status(fel, 0x00)?;
        self.wait_ready(fel)?;
        if self.info.address_length == 4 {
            self.write_enable(fel)?;
            self.enter_4byte(fel)?;
            self.wait_ready(fel)?;
        }
        Ok(())
    }

    fn erase_range(
        &mut self,
        fel: &Fel<'_>,
        address: u64,
        length: u64,
        mut progress: Option<&mut Progress>,
    ) -> SpinorResult<()> {
        self.validate_range(address, length)?;
        if length == 0 {
            return Ok(());
        }
        let mut blocks = self.info.erase_blocks();
        if blocks.is_empty() {
            return Err(SpinorError::Unsupported("no erase opcode available"));
        }
        blocks.sort_by_key(|block| std::cmp::Reverse(block.size));
        let smallest = blocks.iter().map(|b| b.size).min().unwrap_or(4096) as u64;
        let (mut base, mut cnt) =
            erase_span(address, length, smallest).ok_or(SpinorError::AddressOverflow)?;
        while cnt > 0 {
            let mut erased = false;
            for block in &blocks {
                let bsz = block.size as u64;
                if base.is_multiple_of(bsz) && cnt >= bsz {
                    self.erase_block(fel, base, block)?;
                    base += bsz;
                    cnt -= bsz;
                    if let Some(p) = &mut progress {
                        (**p).inc(bsz);
                    }
                    erased = true;
                    break;
                }
            }
            if !erased {
                return Err(SpinorError::Unsupported(
                    "erase range cannot be expressed with supported block sizes",
                ));
            }
        }
        Ok(())
    }

    fn read_range(
        &mut self,
        fel: &Fel<'_>,
        mut address: u64,
        mut out: &mut [u8],
        mut progress: Option<&mut Progress>,
    ) -> SpinorResult<()> {
        self.validate_range(address, out.len() as u64)?;
        while !out.is_empty() {
            let chunk = out
                .len()
                .min(self.session.context().swap_len as usize)
                .min(FLASH_TRANSFER_CHUNK);
            let addr32 = self.addr_to_u32(address)?;
            let mut tx = Vec::with_capacity(1 + self.info.address_length as usize);
            tx.push(OPCODE_READ);
            push_address(&mut tx, addr32, self.info.address_length);
            let (head, tail) = out.split_at_mut(chunk);
            spi::transfer(fel, &self.session, Some(&tx), Some(head))?;
            address = address.wrapping_add(chunk as u64);
            out = tail;
            if let Some(p) = &mut progress {
                (**p).inc(chunk as u64);
            }
        }
        Ok(())
    }

    fn write_range(
        &mut self,
        fel: &Fel<'_>,
        mut address: u64,
        mut data: &[u8],
        mut progress: Option<&mut Progress>,
    ) -> SpinorResult<()> {
        self.validate_range(address, data.len() as u64)?;
        while !data.is_empty() {
            let max_payload = self.session.context().swap_len as usize;
            let overhead = self.info.address_length as usize + 1;
            let available = max_payload
                .checked_sub(overhead)
                .filter(|&limit| limit != 0)
                .ok_or(SpinorError::Unsupported("SPI swap buffer is too small"))?;
            let page_size = self.info.page_size.max(1) as usize;
            let bytes_left_in_page = page_size - (address % page_size as u64) as usize;
            let chunk = data.len().min(bytes_left_in_page).min(available);
            let addr32 = self.addr_to_u32(address)?;
            self.write_enable(fel)?;
            let mut tx = Vec::with_capacity(1 + self.info.address_length as usize + chunk);
            tx.push(OPCODE_PAGE_PROGRAM);
            push_address(&mut tx, addr32, self.info.address_length);
            tx.extend_from_slice(&data[..chunk]);
            spi::transfer(fel, &self.session, Some(&tx), None)?;
            self.wait_ready(fel)?;
            address = address.wrapping_add(chunk as u64);
            data = &data[chunk..];
            if let Some(p) = &mut progress {
                (**p).inc(chunk as u64);
            }
        }
        Ok(())
    }

    fn erase_block(
        &mut self,
        fel: &Fel<'_>,
        address: u64,
        block: &EraseOpcode,
    ) -> SpinorResult<()> {
        let addr32 = self.addr_to_u32(address)?;
        self.write_enable(fel)?;
        let mut tx = Vec::with_capacity(1 + self.info.address_length as usize);
        tx.push(block.opcode);
        push_address(&mut tx, addr32, self.info.address_length);
        spi::transfer(fel, &self.session, Some(&tx), None)?;
        self.wait_ready(fel)
    }

    fn addr_to_u32(&self, address: u64) -> SpinorResult<u32> {
        if self.info.address_length == 3 && address >= (1 << 24) {
            return Err(SpinorError::AddressOverflow);
        }
        u32::try_from(address).map_err(|_| SpinorError::AddressOverflow)
    }

    fn validate_range(&self, address: u64, length: u64) -> SpinorResult<()> {
        if range_in_bounds(address, length, self.info.capacity) {
            Ok(())
        } else {
            Err(SpinorError::AddressOverflow)
        }
    }

    fn reset(&mut self, fel: &Fel<'_>) -> SpinorResult<()> {
        spi::transfer(fel, &self.session, Some(&[OPCODE_RESET_ENABLE]), None)?;
        spi::transfer(fel, &self.session, Some(&[OPCODE_RESET_MEMORY]), None)?;
        Ok(())
    }

    fn global_unlock(&mut self, fel: &Fel<'_>) -> SpinorResult<()> {
        spi::transfer(fel, &self.session, Some(&[OPCODE_GLOBAL_UNLOCK]), None)?;
        Ok(())
    }

    fn write_enable(&mut self, fel: &Fel<'_>) -> SpinorResult<()> {
        spi::transfer(fel, &self.session, Some(&[OPCODE_WRITE_ENABLE]), None)?;
        Ok(())
    }

    fn write_status(&mut self, fel: &Fel<'_>, value: u8) -> SpinorResult<()> {
        spi::transfer(fel, &self.session, Some(&[OPCODE_WRSR, value]), None)?;
        Ok(())
    }

    fn enter_4byte(&mut self, fel: &Fel<'_>) -> SpinorResult<()> {
        spi::transfer(fel, &self.session, Some(&[OPCODE_ENTER_4B]), None)?;
        Ok(())
    }

    fn wait_ready(&mut self, fel: &Fel<'_>) -> SpinorResult<()> {
        let deadline = Instant::now() + WAIT_TIMEOUT;
        loop {
            let mut status = [0u8; 1];
            spi::transfer(fel, &self.session, Some(&[OPCODE_RDSR]), Some(&mut status))?;
            if status[0] & 0x01 == 0 {
                return Ok(());
            }
            if Instant::now() > deadline {
                return Err(SpinorError::Timeout);
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

struct SpinorInfo {
    name: String,
    capacity: u64,
    page_size: u32,
    address_length: u8,
    opcode_erase_4k: Option<u8>,
    opcode_erase_32k: Option<u8>,
    opcode_erase_64k: Option<u8>,
    opcode_erase_256k: Option<u8>,
}

impl SpinorInfo {
    fn detect(fel: &Fel<'_>, session: &SpiSession<'_>) -> SpinorResult<Self> {
        if let Some(info) = Self::from_sfdp(fel, session)? {
            return Ok(info);
        }
        let id = Self::read_id(fel, session)?;
        Self::from_known(id).ok_or(SpinorError::Unsupported("unknown spi nor flash"))
    }

    fn read_id(fel: &Fel<'_>, session: &SpiSession<'_>) -> SpinorResult<u32> {
        let mut rx = [0u8; 3];
        spi::transfer(fel, session, Some(&[OPCODE_RDID]), Some(&mut rx))?;
        Ok(((rx[0] as u32) << 16) | ((rx[1] as u32) << 8) | (rx[2] as u32))
    }

    fn from_known(id: u32) -> Option<Self> {
        KNOWN_DEVICES
            .iter()
            .find(|dev| dev.id == id)
            .map(|dev| dev.to_info())
    }

    fn from_sfdp(fel: &Fel<'_>, session: &SpiSession<'_>) -> SpinorResult<Option<Self>> {
        let mut header = [0u8; 8];
        let tx = [OPCODE_SFDP, 0x00, 0x00, 0x00, 0x00];
        spi::transfer(fel, session, Some(&tx), Some(&mut header))?;
        if &header[0..4] != b"SFDP" {
            return Ok(None);
        }
        log::debug!(
            "SFDP header version {}.{}, {} parameter headers",
            header[5],
            header[4],
            header[6] as usize + 1
        );
        let nph = header[6];
        let param_count = ((nph as usize) + 1).min(SFDP_MAX_PARAMETERS);
        let mut basic: Option<(SfdpParameterHeader, Vec<u8>)> = None;
        for i in 0..param_count {
            let addr = (i * 8 + 8) as u32;
            let mut param_raw = [0u8; 8];
            let tx = [
                OPCODE_SFDP,
                ((addr >> 16) & 0xff) as u8,
                ((addr >> 8) & 0xff) as u8,
                (addr & 0xff) as u8,
                0x00,
            ];
            spi::transfer(fel, session, Some(&tx), Some(&mut param_raw))?;
            let param = SfdpParameterHeader::from_bytes(param_raw);
            if param.id_lsb == 0x00 && param.id_msb == 0xff {
                log::debug!(
                    "SFDP basic parameter table version {}.{}, {} DWORDs",
                    param.major,
                    param.minor,
                    param.length
                );
                let mut table = vec![0u8; param.length as usize * 4];
                let base = ((param.ptp[2] as u32) << 16)
                    | ((param.ptp[1] as u32) << 8)
                    | (param.ptp[0] as u32);
                let tx = [
                    OPCODE_SFDP,
                    ((base >> 16) & 0xff) as u8,
                    ((base >> 8) & 0xff) as u8,
                    (base & 0xff) as u8,
                    0x00,
                ];
                spi::transfer(fel, session, Some(&tx), Some(&mut table))?;
                basic = Some((param, table));
                break;
            }
        }
        let Some((param, table)) = basic else {
            return Ok(None);
        };
        let info = Self::from_sfdp_table(&param, &table)?;
        Ok(Some(info))
    }

    fn from_sfdp_table(param: &SfdpParameterHeader, table: &[u8]) -> SpinorResult<Self> {
        if table.len() < 8 {
            return Err(SpinorError::InvalidResponse("sfdp basic table too short"));
        }
        let capacity_raw = read_dword(table, 4).unwrap();
        let capacity = if capacity_raw & 0x8000_0000 != 0 {
            let exp = (capacity_raw & 0x7fff_ffff)
                .checked_sub(3)
                .ok_or(SpinorError::InvalidResponse("invalid sfdp density"))?;
            1u64.checked_shl(exp)
                .ok_or(SpinorError::InvalidResponse("invalid sfdp density"))?
        } else {
            ((capacity_raw as u64) + 1) / 8
        };
        let dw1 = read_dword(table, 0).unwrap();
        let address_length = if capacity <= 16 * 1024 * 1024 && ((dw1 >> 17) & 0x3) != 0x2 {
            3
        } else {
            4
        };
        let mut erase = [None; 4];
        if (dw1 & 0x3) == 0x1 {
            erase[0] = Some((dw1 >> 8) as u8);
        }
        for &offset in &[28usize, 32usize] {
            let Some(dw) = read_dword(table, offset) else {
                continue;
            };
            for shift in [0, 16] {
                let index = match (dw >> shift) & 0xff {
                    12 => 0,
                    15 => 1,
                    16 => 2,
                    18 => 3,
                    _ => continue,
                };
                erase[index] = Some((dw >> (shift + 8)) as u8);
            }
        }
        let [erase4, erase32, erase64, erase256] = erase;
        let page_size = if param.major == 1 && param.minor >= 5 {
            let dw11 = read_dword(table, 40).ok_or(SpinorError::InvalidResponse(
                "sfdp basic table missing page size",
            ))?;
            1 << ((dw11 >> 4) & 0xf)
        } else {
            // Older basic tables do not report the page size. 256 bytes is
            // the conservative default used by mature SPI NOR stacks.
            256
        };
        Ok(Self {
            name: "SFDP".to_string(),
            capacity,
            page_size,
            address_length,
            opcode_erase_4k: erase4,
            opcode_erase_32k: erase32,
            opcode_erase_64k: erase64,
            opcode_erase_256k: erase256,
        })
    }

    fn erase_blocks(&self) -> Vec<EraseOpcode> {
        [
            (self.opcode_erase_256k, 256 * 1024),
            (self.opcode_erase_64k, 64 * 1024),
            (self.opcode_erase_32k, 32 * 1024),
            (self.opcode_erase_4k, 4 * 1024),
        ]
        .into_iter()
        .filter_map(|(opcode, size)| opcode.map(|opcode| EraseOpcode { size, opcode }))
        .collect()
    }
}

struct EraseOpcode {
    size: u32,
    opcode: u8,
}

struct SpinorKnown {
    name: &'static str,
    id: u32,
    capacity: u32,
    page_size: u32,
    address_length: u8,
    opcode_erase_4k: Option<u8>,
    opcode_erase_32k: Option<u8>,
    opcode_erase_64k: Option<u8>,
    opcode_erase_256k: Option<u8>,
}

impl SpinorKnown {
    fn to_info(&self) -> SpinorInfo {
        SpinorInfo {
            name: self.name.to_string(),
            capacity: self.capacity as u64,
            page_size: self.page_size,
            address_length: self.address_length,
            opcode_erase_4k: self.opcode_erase_4k,
            opcode_erase_32k: self.opcode_erase_32k,
            opcode_erase_64k: self.opcode_erase_64k,
            opcode_erase_256k: self.opcode_erase_256k,
        }
    }
}

struct SfdpParameterHeader {
    id_lsb: u8,
    minor: u8,
    major: u8,
    length: u8,
    ptp: [u8; 3],
    id_msb: u8,
}

fn read_dword(table: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = table.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}

impl SfdpParameterHeader {
    fn from_bytes(raw: [u8; 8]) -> Self {
        Self {
            id_lsb: raw[0],
            minor: raw[1],
            major: raw[2],
            length: raw[3],
            ptp: [raw[4], raw[5], raw[6]],
            id_msb: raw[7],
        }
    }
}

fn push_address(buf: &mut Vec<u8>, address: u32, length: u8) {
    match length {
        3 => {
            buf.push(((address >> 16) & 0xff) as u8);
            buf.push(((address >> 8) & 0xff) as u8);
            buf.push((address & 0xff) as u8);
        }
        4 => {
            buf.push(((address >> 24) & 0xff) as u8);
            buf.push(((address >> 16) & 0xff) as u8);
            buf.push(((address >> 8) & 0xff) as u8);
            buf.push((address & 0xff) as u8);
        }
        _ => {}
    }
}

const KNOWN_DEVICES: &[SpinorKnown] = &[
    SpinorKnown {
        name: "W25X40",
        id: 0xef3013,
        capacity: 512 * 1024,
        page_size: 256,
        address_length: 3,
        opcode_erase_4k: Some(0x20),
        opcode_erase_32k: None,
        opcode_erase_64k: Some(0xd8),
        opcode_erase_256k: None,
    },
    SpinorKnown {
        name: "W25Q128JVEIQ",
        id: 0xefc018,
        capacity: 16 * 1024 * 1024,
        page_size: 256,
        address_length: 3,
        opcode_erase_4k: Some(0x20),
        opcode_erase_32k: Some(0x52),
        opcode_erase_64k: Some(0xd8),
        opcode_erase_256k: None,
    },
    SpinorKnown {
        name: "W25Q256JVEIQ",
        id: 0xef4019,
        capacity: 32 * 1024 * 1024,
        page_size: 256,
        address_length: 4,
        opcode_erase_4k: Some(0x20),
        opcode_erase_32k: Some(0x52),
        opcode_erase_64k: Some(0xd8),
        opcode_erase_256k: None,
    },
    SpinorKnown {
        name: "GD25D10B",
        id: 0xc84011,
        capacity: 128 * 1024,
        page_size: 256,
        address_length: 3,
        opcode_erase_4k: Some(0x20),
        opcode_erase_32k: Some(0x52),
        opcode_erase_64k: Some(0xd8),
        opcode_erase_256k: None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_short_sfdp_1_0_basic_table() {
        let param = SfdpParameterHeader {
            id_lsb: 0,
            minor: 0,
            major: 1,
            length: 9,
            ptp: [0; 3],
            id_msb: 0xff,
        };
        let mut table = vec![0u8; 9 * 4];
        table[0..4].copy_from_slice(&0x0000_2001u32.to_le_bytes());
        table[4..8].copy_from_slice(&0x07ff_ffffu32.to_le_bytes());
        table[28..32].copy_from_slice(&0xd810_200cu32.to_le_bytes());

        let info = SpinorInfo::from_sfdp_table(&param, &table).unwrap();

        assert_eq!(info.capacity, 16 * 1024 * 1024);
        assert_eq!(info.address_length, 3);
        assert_eq!(info.page_size, 256);
        assert_eq!(info.opcode_erase_4k, Some(0x20));
        assert_eq!(info.opcode_erase_64k, Some(0xd8));
    }

    #[test]
    fn requires_page_size_for_sfdp_1_5() {
        let param = SfdpParameterHeader {
            id_lsb: 0,
            minor: 5,
            major: 1,
            length: 9,
            ptp: [0; 3],
            id_msb: 0xff,
        };
        let mut table = vec![0u8; 9 * 4];
        table[4..8].copy_from_slice(&0x07ff_ffffu32.to_le_bytes());

        assert!(matches!(
            SpinorInfo::from_sfdp_table(&param, &table),
            Err(SpinorError::InvalidResponse(
                "sfdp basic table missing page size"
            ))
        ));
    }
}
