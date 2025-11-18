use core::convert::TryFrom;
use std::fmt;
use std::time::{Duration, Instant};

use crate::chips::Chip;
use crate::fel::Fel;
use crate::progress::Progress;
use crate::spi::{self, SpiError, SpiSession};

const WAIT_TIMEOUT: Duration = Duration::from_secs(5);
const OPCODE_RDID: u8 = 0x9f;
const OPCODE_GET_FEATURE: u8 = 0x0f;
const OPCODE_SET_FEATURE: u8 = 0x1f;
const FEATURE_PROTECT: u8 = 0xa0;
const FEATURE_STATUS: u8 = 0xc0;
const OPCODE_READ_PAGE_TO_CACHE: u8 = 0x13;
const OPCODE_READ_PAGE_FROM_CACHE: u8 = 0x03;
const OPCODE_WRITE_ENABLE: u8 = 0x06;
const OPCODE_BLOCK_ERASE: u8 = 0xd8;
const OPCODE_PROGRAM_LOAD: u8 = 0x02;
const OPCODE_PROGRAM_EXEC: u8 = 0x10;
const OPCODE_RESET: u8 = 0xff;

#[derive(Debug)]
pub enum SpinandError {
    Spi(SpiError),
    Unsupported(&'static str),
    InvalidResponse(&'static str),
    AddressOverflow,
    Timeout,
    InvalidImage(&'static str),
}

impl fmt::Display for SpinandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpinandError::Spi(err) => write!(f, "spi error: {err}"),
            SpinandError::Unsupported(msg) => write!(f, "unsupported: {msg}"),
            SpinandError::InvalidResponse(msg) => write!(f, "invalid response: {msg}"),
            SpinandError::AddressOverflow => write!(f, "address out of range for device"),
            SpinandError::Timeout => write!(f, "operation timed out waiting for device"),
            SpinandError::InvalidImage(msg) => write!(f, "invalid image: {msg}"),
        }
    }
}

impl std::error::Error for SpinandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpinandError::Spi(err) => Some(err),
            _ => None,
        }
    }
}

impl From<SpiError> for SpinandError {
    fn from(value: SpiError) -> Self {
        SpinandError::Spi(value)
    }
}

type SpinandResult<T> = Result<T, SpinandError>;

pub struct DetectInfo {
    pub name: String,
    pub capacity: u64,
}

pub fn detect(chip: &dyn Chip, fel: &Fel<'_>) -> SpinandResult<DetectInfo> {
    let state = SpinandState::new(chip, fel)?;
    Ok(DetectInfo {
        name: state.info.name.clone(),
        capacity: state.info.capacity(),
    })
}

pub fn erase(
    chip: &dyn Chip,
    fel: &Fel<'_>,
    address: u64,
    length: u64,
    progress: Option<&mut Progress>,
) -> SpinandResult<()> {
    let mut state = SpinandState::new(chip, fel)?;
    state.erase_range(fel, address, length, progress)
}

pub fn read(
    chip: &dyn Chip,
    fel: &Fel<'_>,
    address: u64,
    buffer: &mut [u8],
    mut progress: Option<&mut Progress>,
) -> SpinandResult<()> {
    let mut state = SpinandState::new(chip, fel)?;
    let total = buffer.len() as u64;
    let mut processed = 0u64;
    let mut offset = 0usize;
    while offset < buffer.len() {
        let chunk = (buffer.len() - offset).min(state.chunk_limit());
        state.read_range_segment(
            fel,
            address + processed,
            &mut buffer[offset..offset + chunk],
        )?;
        processed += chunk as u64;
        offset += chunk;
        if let Some(p) = &mut progress {
            (**p).inc(chunk as u64);
        }
    }
    if let Some(p) = &mut progress {
        if processed == total {
            (**p).finish();
        }
    }
    Ok(())
}

pub fn write(
    chip: &dyn Chip,
    fel: &Fel<'_>,
    address: u64,
    data: &[u8],
    mut progress: Option<&mut Progress>,
) -> SpinandResult<()> {
    let mut state = SpinandState::new(chip, fel)?;
    let mut processed = 0u64;
    let total = data.len() as u64;
    println!(
        "Writing {} bytes to SPI NAND at address 0x{:x}",
        total, address
    );
    while processed < total {
        let chunk = (total - processed).min(state.chunk_limit() as u64) as usize;
        state.write_range_segment(
            fel,
            address + processed,
            &data[processed as usize..processed as usize + chunk],
        )?;
        log::debug!("  wrote {} bytes at offset 0x{:x}", chunk, processed);
        processed += chunk as u64;
        if let Some(p) = &mut progress {
            (**p).inc(chunk as u64);
        }
    }
    if let Some(p) = &mut progress {
        if processed == total {
            (**p).finish();
        }
    }
    Ok(())
}

pub fn spl_write(
    chip: &dyn Chip,
    fel: &Fel<'_>,
    splitsz: u32,
    address: u64,
    data: &[u8],
) -> SpinandResult<()> {
    let mut state = SpinandState::new(chip, fel)?;
    state.write_spl(fel, splitsz, address, data)
}

struct SpinandState<'chip> {
    session: SpiSession<'chip>,
    info: SpinandInfo,
}

impl<'chip> SpinandState<'chip> {
    fn new(chip: &'chip dyn Chip, fel: &Fel<'_>) -> SpinandResult<Self> {
        let session = spi::begin(chip, fel)?;
        let info = SpinandInfo::detect(fel, &session)?;
        let mut state = Self { session, info };
        state.initialise(fel)?;
        Ok(state)
    }

    fn initialise(&mut self, fel: &Fel<'_>) -> SpinandResult<()> {
        self.reset(fel)?;
        self.wait_ready(fel)?;
        let protect = self.get_feature(fel, FEATURE_PROTECT)?;
        if protect != 0 {
            self.set_feature(fel, FEATURE_PROTECT, 0)?;
            self.wait_ready(fel)?;
        }
        Ok(())
    }

    fn chunk_limit(&self) -> usize {
        self.session.context().swap_len as usize
    }

    fn erase_range(
        &mut self,
        fel: &Fel<'_>,
        address: u64,
        length: u64,
        mut progress: Option<&mut Progress>,
    ) -> SpinandResult<()> {
        let block = self.info.block_size();
        let mask = block as u64 - 1;
        let mut base = address & !mask;
        let mut cnt = (address & mask) + length;
        if cnt & mask != 0 {
            cnt = (cnt + mask + 1) & !mask;
        }
        while cnt > 0 {
            self.erase_block(fel, base)?;
            base += block as u64;
            cnt = cnt.saturating_sub(block as u64);
            if let Some(p) = &mut progress {
                (**p).inc(block as u64);
            }
        }
        Ok(())
    }

    fn erase_block(&mut self, fel: &Fel<'_>, address: u64) -> SpinandResult<()> {
        let page_size = self.info.page_size as u64;
        let pa = u32::try_from(address / page_size).map_err(|_| SpinandError::AddressOverflow)?;
        self.write_enable(fel)?;
        self.wait_ready(fel)?;
        let tx = [
            OPCODE_BLOCK_ERASE,
            ((pa >> 16) & 0xff) as u8,
            ((pa >> 8) & 0xff) as u8,
            (pa & 0xff) as u8,
        ];
        spi::transfer(fel, &self.session, Some(&tx), None)?;
        self.wait_ready(fel)
    }

    fn read_range_segment(
        &mut self,
        fel: &Fel<'_>,
        mut address: u64,
        out: &mut [u8],
    ) -> SpinandResult<()> {
        let page_size = self.info.page_size as usize;
        if page_size == 0 {
            return Err(SpinandError::Unsupported("invalid page size"));
        }

        let mut remaining = out;
        while !remaining.is_empty() {
            let page = u32::try_from(address / page_size as u64)
                .map_err(|_| SpinandError::AddressOverflow)?;
            let mut column = (address % page_size as u64) as usize;
            self.load_page(fel, page)?;
            self.wait_ready(fel)?;

            while !remaining.is_empty() && column < page_size {
                let bytes_left_in_page = page_size - column;
                if bytes_left_in_page == 0 {
                    break;
                }
                let chunk = remaining
                    .len()
                    .min(bytes_left_in_page)
                    .min(self.chunk_limit());
                self.read_cache(fel, column as u16, &mut remaining[..chunk])?;
                remaining = &mut remaining[chunk..];
                address += chunk as u64;
                column += chunk;
                if column == page_size {
                    break;
                }
            }
        }
        Ok(())
    }

    fn write_range_segment(
        &mut self,
        fel: &Fel<'_>,
        mut address: u64,
        mut data: &[u8],
    ) -> SpinandResult<()> {
        let page_size = self.info.page_size as usize;
        if page_size == 0 {
            return Err(SpinandError::Unsupported("invalid page size"));
        }

        while !data.is_empty() {
            let page = u32::try_from(address / page_size as u64)
                .map_err(|_| SpinandError::AddressOverflow)?;
            let mut column = (address % page_size as u64) as usize;
            self.write_enable(fel)?;
            log::debug!(
                "  writing page 0x{:x} starting at column 0x{:x}\n  remaining 0x{:x} bytes",
                page,
                column,
                data.len()
            );
            self.wait_ready(fel)?;

            while !data.is_empty() && column < page_size {
                let bytes_left_in_page = page_size - column;
                if bytes_left_in_page == 0 {
                    break;
                }
                let chunk = data.len().min(bytes_left_in_page).min(self.chunk_limit());
                self.program_load(fel, column as u16, &data[..chunk])?;
                self.wait_ready(fel)?;
                data = &data[chunk..];
                address += chunk as u64;
                column += chunk;
                log::debug!(
                    "    programmed 0x{:x} bytes, 0x{:x} bytes remaining, current offset 0x{:x}",
                    chunk,
                    data.len(),
                    column
                );
                if column == page_size {
                    break;
                }
            }

            self.program_exec(fel, page)?;
            self.wait_ready(fel)?;
        }
        Ok(())
    }

    fn write_spl(
        &mut self,
        fel: &Fel<'_>,
        splitsz: u32,
        address: u64,
        data: &[u8],
    ) -> SpinandResult<()> {
        let split = if splitsz == 0 || splitsz > self.info.page_size {
            self.info.page_size
        } else {
            splitsz
        };
        if split & 0x3ff != 0 {
            return Err(SpinandError::Unsupported("split size must align to 1 KiB"));
        }
        if data.len() < 20 {
            return Err(SpinandError::InvalidImage("buffer too small"));
        }
        if &data[4..12] != b"eGON.BT0" {
            return Err(SpinandError::InvalidImage("missing eGON.BT0 signature"));
        }
        let splsz = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
        if splsz as usize > data.len() {
            return Err(SpinandError::InvalidImage(
                "reported SPL size exceeds buffer",
            ));
        }
        let block = self.info.block_size() as u64;
        let page_size = self.info.page_size as u64;
        let emask = block - 1;
        let mut tsplsz = (splsz as u64 * page_size) / split as u64;
        tsplsz = (tsplsz + block) & !emask;
        let mut nbuf: Vec<u8>;
        let nlen: u64;
        if address >= tsplsz {
            let mut copies = 0usize;
            let mut total = 0u64;
            while total < address {
                total += tsplsz;
                copies += 1;
            }
            total += data.len() as u64;
            nlen = total;
            nbuf = vec![0xff; total as usize];
            let mut src_off = 0usize;
            let mut dst_off = 0usize;
            while src_off < splsz as usize {
                let chunk = split as usize;
                let end = src_off + chunk;
                nbuf[dst_off..dst_off + chunk].copy_from_slice(&data[src_off..end]);
                src_off += chunk;
                dst_off += page_size as usize;
            }
            for i in 1..copies {
                let start = i * tsplsz as usize;
                nbuf.copy_within(0..tsplsz as usize, start);
            }
            let tail_start = (total - data.len() as u64) as usize;
            nbuf[tail_start..tail_start + data.len()].copy_from_slice(data);
        } else {
            nlen = tsplsz;
            nbuf = vec![0xff; tsplsz as usize];
            let mut src_off = 0usize;
            let mut dst_off = 0usize;
            while src_off < splsz as usize {
                let chunk = split as usize;
                let end = src_off + chunk;
                nbuf[dst_off..dst_off + chunk].copy_from_slice(&data[src_off..end]);
                src_off += chunk;
                dst_off += page_size as usize;
            }
        }
        let erase_len = (nlen + emask) & !emask;
        self.erase_range(fel, 0, erase_len, None)?;
        let mut written = 0u64;
        while written < nlen {
            let chunk = (nlen - written).min(self.chunk_limit() as u64) as usize;
            self.write_range_segment(
                fel,
                written,
                &nbuf[written as usize..written as usize + chunk],
            )?;
            written += chunk as u64;
        }
        Ok(())
    }

    fn reset(&mut self, fel: &Fel<'_>) -> SpinandResult<()> {
        spi::transfer(fel, &self.session, Some(&[OPCODE_RESET]), None)?;
        Ok(())
    }

    fn wait_ready(&mut self, fel: &Fel<'_>) -> SpinandResult<()> {
        let deadline = Instant::now() + WAIT_TIMEOUT;
        loop {
            let status = self.get_feature(fel, FEATURE_STATUS)?;
            if status & 0x01 == 0 {
                return Ok(());
            }
            if Instant::now() > deadline {
                return Err(SpinandError::Timeout);
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    fn get_feature(&mut self, fel: &Fel<'_>, addr: u8) -> SpinandResult<u8> {
        let tx = [OPCODE_GET_FEATURE, addr];
        let mut val = [0u8; 1];
        spi::transfer(fel, &self.session, Some(&tx), Some(&mut val))?;
        Ok(val[0])
    }

    fn set_feature(&mut self, fel: &Fel<'_>, addr: u8, value: u8) -> SpinandResult<()> {
        let tx = [OPCODE_SET_FEATURE, addr, value];
        spi::transfer(fel, &self.session, Some(&tx), None)?;
        Ok(())
    }

    fn write_enable(&mut self, fel: &Fel<'_>) -> SpinandResult<()> {
        spi::transfer(fel, &self.session, Some(&[OPCODE_WRITE_ENABLE]), None)?;
        Ok(())
    }

    fn load_page(&mut self, fel: &Fel<'_>, page: u32) -> SpinandResult<()> {
        let tx = [
            OPCODE_READ_PAGE_TO_CACHE,
            ((page >> 16) & 0xff) as u8,
            ((page >> 8) & 0xff) as u8,
            (page & 0xff) as u8,
        ];
        spi::transfer(fel, &self.session, Some(&tx), None)?;
        Ok(())
    }

    fn read_cache(&mut self, fel: &Fel<'_>, column: u16, out: &mut [u8]) -> SpinandResult<()> {
        let tx = [
            OPCODE_READ_PAGE_FROM_CACHE,
            ((column >> 8) & 0xff) as u8,
            (column & 0xff) as u8,
            0x00,
        ];
        spi::transfer(fel, &self.session, Some(&tx), Some(out))?;
        Ok(())
    }

    fn program_load(&mut self, fel: &Fel<'_>, column: u16, data: &[u8]) -> SpinandResult<()> {
        let mut tx = Vec::with_capacity(3 + data.len());
        tx.push(OPCODE_PROGRAM_LOAD);
        tx.push(((column >> 8) & 0xff) as u8);
        tx.push((column & 0xff) as u8);
        tx.extend_from_slice(data);
        spi::transfer(fel, &self.session, Some(&tx), None)?;
        Ok(())
    }

    fn program_exec(&mut self, fel: &Fel<'_>, page: u32) -> SpinandResult<()> {
        let tx = [
            OPCODE_PROGRAM_EXEC,
            ((page >> 16) & 0xff) as u8,
            ((page >> 8) & 0xff) as u8,
            (page & 0xff) as u8,
        ];
        spi::transfer(fel, &self.session, Some(&tx), None)?;
        Ok(())
    }
}

struct SpinandInfo {
    name: String,
    #[allow(dead_code)]
    id: Vec<u8>,
    page_size: u32,
    #[allow(dead_code)]
    spare_size: u32,
    pages_per_block: u32,
    blocks_per_die: u32,
    #[allow(dead_code)]
    planes_per_die: u32,
    ndies: u32,
}

impl SpinandInfo {
    fn detect(fel: &Fel<'_>, session: &SpiSession<'_>) -> SpinandResult<Self> {
        let mut rx = [0u8; 4];
        spi::transfer(fel, session, Some(&[OPCODE_RDID, 0x00]), Some(&mut rx))?;
        if let Some(info) = Self::from_known(&rx) {
            return Ok(info);
        }
        spi::transfer(fel, session, Some(&[OPCODE_RDID]), Some(&mut rx))?;
        if let Some(info) = Self::from_known(&rx) {
            return Ok(info);
        }
        Err(SpinandError::Unsupported("unknown spi nand flash"))
    }

    fn from_known(id: &[u8; 4]) -> Option<Self> {
        for dev in KNOWN_DEVICES {
            if dev.matches(id) {
                return Some(dev.to_info());
            }
        }
        None
    }

    fn capacity(&self) -> u64 {
        self.page_size as u64
            * self.pages_per_block as u64
            * self.blocks_per_die as u64
            * self.ndies as u64
    }

    fn block_size(&self) -> u32 {
        self.page_size * self.pages_per_block
    }
}

struct SpinandKnown {
    name: &'static str,
    id: &'static [u8],
    page_size: u32,
    spare_size: u32,
    pages_per_block: u32,
    blocks_per_die: u32,
    planes_per_die: u32,
    ndies: u32,
}

impl SpinandKnown {
    fn matches(&self, id: &[u8]) -> bool {
        if self.id.len() > id.len() {
            return false;
        }
        self.id.iter().zip(id.iter()).all(|(a, b)| a == b)
    }

    fn to_info(&self) -> SpinandInfo {
        SpinandInfo {
            name: self.name.to_string(),
            id: self.id.to_vec(),
            page_size: self.page_size,
            spare_size: self.spare_size,
            pages_per_block: self.pages_per_block,
            blocks_per_die: self.blocks_per_die,
            planes_per_die: self.planes_per_die,
            ndies: self.ndies,
        }
    }
}

const KNOWN_DEVICES: &[SpinandKnown] = &[
    SpinandKnown {
        name: "W25N512GV",
        id: &[0xef, 0xaa, 0x20],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 512,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "W25N01GV",
        id: &[0xef, 0xaa, 0x21],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "W25M02GV",
        id: &[0xef, 0xab, 0x21],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 2,
    },
    SpinandKnown {
        name: "W25N02KV",
        id: &[0xef, 0xaa, 0x22],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F1GQ4UAWxx",
        id: &[0xc8, 0x10],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F1GQ5UExxG",
        id: &[0xc8, 0x51],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F1GQ4UExIG",
        id: &[0xc8, 0xd1],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F1GQ4UExxH",
        id: &[0xc8, 0xd9],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F1GQ4xAYIG",
        id: &[0xc8, 0xf1],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F2GQ4UExIG",
        id: &[0xc8, 0xd2],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F2GQ5UExxH",
        id: &[0xc8, 0x32],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F2GQ4xAYIG",
        id: &[0xc8, 0xf2],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F4GQ4UBxIG",
        id: &[0xc8, 0xd4],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F4GQ4xAYIG",
        id: &[0xc8, 0xf4],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F2GQ5UExxG",
        id: &[0xc8, 0x52],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F4GQ4UCxIG",
        id: &[0xc8, 0xb4],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "GD5F4GQ4RCxIG",
        id: &[0xc8, 0xa4],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX35LF1GE4AB",
        id: &[0xc2, 0x12],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX35LF1G24AD",
        id: &[0xc2, 0x14],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX31LF1GE4BC",
        id: &[0xc2, 0x1e],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX35LF2GE4AB",
        id: &[0xc2, 0x22],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX35LF2G24AD",
        id: &[0xc2, 0x24],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX35LF2GE4AD",
        id: &[0xc2, 0x26],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX35LF2G14AC",
        id: &[0xc2, 0x20],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX35LF4G24AD",
        id: &[0xc2, 0x35],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MX35LF4GE4AD",
        id: &[0xc2, 0x37],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MT29F1G01AAADD",
        id: &[0x2c, 0x12],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MT29F1G01ABAFD",
        id: &[0x2c, 0x14],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MT29F2G01AAAED",
        id: &[0x2c, 0x9f],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 2,
        ndies: 1,
    },
    SpinandKnown {
        name: "MT29F2G01ABAGD",
        id: &[0x2c, 0x24],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 2,
        ndies: 1,
    },
    SpinandKnown {
        name: "MT29F4G01AAADD",
        id: &[0x2c, 0x32],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 2,
        ndies: 1,
    },
    SpinandKnown {
        name: "MT29F4G01ABAFD",
        id: &[0x2c, 0x34],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "MT29F4G01ADAGD",
        id: &[0x2c, 0x36],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 2,
        ndies: 2,
    },
    SpinandKnown {
        name: "MT29F8G01ADAFD",
        id: &[0x2c, 0x46],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 2,
    },
    SpinandKnown {
        name: "TC58CVG0S3HRAIG",
        id: &[0x98, 0xc2],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "TC58CVG1S3HRAIG",
        id: &[0x98, 0xcb],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "TC58CVG2S0HRAIG",
        id: &[0x98, 0xcd],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "TC58CVG0S3HRAIJ",
        id: &[0x98, 0xe2],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "TC58CVG1S3HRAIJ",
        id: &[0x98, 0xeb],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "TC58CVG2S0HRAIJ",
        id: &[0x98, 0xed],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "TH58CVG3S0HRAIJ",
        id: &[0x98, 0xe4],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "F50L512M41A",
        id: &[0xc8, 0x20],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 512,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "F50L1G41A",
        id: &[0xc8, 0x21],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "F50L1G41LB",
        id: &[0xc8, 0x01],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "F50L2G41LB",
        id: &[0xc8, 0x0a],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 2,
    },
    SpinandKnown {
        name: "CS11G0T0A0AA",
        id: &[0x6b, 0x00],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "CS11G0G0A0AA",
        id: &[0x6b, 0x10],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "CS11G0S0A0AA",
        id: &[0x6b, 0x20],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "CS11G1T0A0AA",
        id: &[0x6b, 0x01],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "CS11G1S0A0AA",
        id: &[0x6b, 0x21],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "CS11G2T0A0AA",
        id: &[0x6b, 0x02],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "CS11G2S0A0AA",
        id: &[0x6b, 0x22],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73B044VCA",
        id: &[0xd5, 0x01],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 512,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044SNB",
        id: &[0xd5, 0x11],
        page_size: 2048,
        spare_size: 120,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044SNF",
        id: &[0xd5, 0x09],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044VCA",
        id: &[0xd5, 0x18],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044SNA",
        id: &[0xd5, 0x19],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 128,
        blocks_per_die: 512,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044VCD",
        id: &[0xd5, 0x1c],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044SND",
        id: &[0xd5, 0x1d],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044SND",
        id: &[0xd5, 0x1e],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044VCC",
        id: &[0xd5, 0x22],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044VCF",
        id: &[0xd5, 0x25],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044SNC",
        id: &[0xd5, 0x31],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044SNC",
        id: &[0xd5, 0x0a],
        page_size: 2048,
        spare_size: 120,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044SNA",
        id: &[0xd5, 0x12],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044SNF",
        id: &[0xd5, 0x10],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044VCA",
        id: &[0xd5, 0x13],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044VCB",
        id: &[0xd5, 0x14],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044VCD",
        id: &[0xd5, 0x17],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044VCH",
        id: &[0xd5, 0x1b],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044SND",
        id: &[0xd5, 0x1d],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044VCG",
        id: &[0xd5, 0x1f],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044VCE",
        id: &[0xd5, 0x20],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044VCL",
        id: &[0xd5, 0x2e],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044SNB",
        id: &[0xd5, 0x32],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73E044SNA",
        id: &[0xd5, 0x03],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73E044SND",
        id: &[0xd5, 0x0b],
        page_size: 4096,
        spare_size: 240,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73E044SNB",
        id: &[0xd5, 0x23],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73E044VCA",
        id: &[0xd5, 0x2c],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73E044VCB",
        id: &[0xd5, 0x2f],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73F044SNA",
        id: &[0xd5, 0x24],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73F044VCA",
        id: &[0xd5, 0x2d],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73E044SNE",
        id: &[0xd5, 0x0e],
        page_size: 4096,
        spare_size: 256,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73C044SNG",
        id: &[0xd5, 0x0c],
        page_size: 2048,
        spare_size: 120,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "EM73D044VCN",
        id: &[0xd5, 0x0f],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "FM35Q1GA",
        id: &[0xe5, 0x71],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "PN26G01A",
        id: &[0xa1, 0xe1],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "PN26G02A",
        id: &[0xa1, 0xe2],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "ATO25D1GA",
        id: &[0x9b, 0x12],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "HYF1GQ4U",
        id: &[0xc9, 0x51],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "HYF2GQ4U",
        id: &[0xc9, 0x52],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "HYF4GQ4U",
        id: &[0xc9, 0x54],
        page_size: 2048,
        spare_size: 128,
        pages_per_block: 64,
        blocks_per_die: 4096,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "F35SQA001G",
        id: &[0xcd, 0x71, 0x71],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 1024,
        planes_per_die: 1,
        ndies: 1,
    },
    SpinandKnown {
        name: "F35SQA002G",
        id: &[0xcd, 0x72, 0x72],
        page_size: 2048,
        spare_size: 64,
        pages_per_block: 64,
        blocks_per_die: 2048,
        planes_per_die: 1,
        ndies: 1,
    },
];
