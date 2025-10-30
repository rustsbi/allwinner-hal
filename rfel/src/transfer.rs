use std::io::{self, Read, Write};

use crate::fel::{CHUNK_SIZE, Fel};
use crate::progress::Progress;

/// Read `length` bytes in chunks into the provided writer, optionally reporting progress.
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

/// Stream data from the reader into memory in chunks, optionally reporting progress.
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

/// Write the entire buffer to the target address, chunk by chunk.
pub fn write_all(fel: &Fel<'_>, mut addr: u32, mut data: &[u8]) {
    while !data.is_empty() {
        let n = data.len().min(CHUNK_SIZE);
        fel.write_address(addr, &data[..n]);
        addr = addr.wrapping_add(n as u32);
        data = &data[n..];
    }
}

/// Fill the output buffer by reading from the given address in chunks.
pub fn read_all(fel: &Fel<'_>, mut addr: u32, mut out: &mut [u8]) {
    while !out.is_empty() {
        let n = out.len().min(CHUNK_SIZE);
        let (head, tail) = out.split_at_mut(n);
        fel.read_address(addr, head);
        addr = addr.wrapping_add(n as u32);
        out = tail;
    }
}
