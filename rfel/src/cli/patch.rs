use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::{debug, error};
use same_file::is_same_file;
use std::io::{ErrorKind, Seek, SeekFrom};

#[derive(Debug, thiserror::Error)]
pub enum PatchError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Input file too small to be a valid eGON image")]
    InputTooSmall,
    #[error("Invalid stamp in input file")]
    InputInvalidStamp,
}
type Result<T> = core::result::Result<T, PatchError>;

const EGON_HEADER_LENGTH: u64 = 0x60;
const STAMP: u32 = 0x5F0A6C39;

// TODO: add some high-level abstraction for binary to image conversion
// TODO: for example we could pass internal logic as a function and distribute the overall logic in a library
/// Patch an binary file into a bootable image format
pub fn patch_image(
    input_path: impl AsRef<std::path::Path>,
    output_path: impl AsRef<std::path::Path>,
) -> Result<()> {
    let mut input_file = std::fs::OpenOptions::new()
        .read(true)
        .open(&input_path)
        .map_err(|e| PatchError::IoError(e))?;
    debug!("opened input file: {}", input_path.as_ref().display());

    // Check input file length
    let input_metadata = input_file.metadata().map_err(|e| PatchError::IoError(e))?;
    let total_length = input_metadata.len();
    if total_length < EGON_HEADER_LENGTH {
        error!(
            "objcopy binary size less than eGON header length, expected >= {} but is {}",
            EGON_HEADER_LENGTH, total_length
        );
        return Err(PatchError::InputTooSmall);
    }
    debug!("input file length: {} bytes, passed", total_length);

    // Check input file stamp
    input_file.seek(SeekFrom::Start(0x0C)).unwrap();
    let stamp = input_file.read_u32::<LittleEndian>().unwrap();
    if stamp != STAMP {
        error!("wrong stamp value; check your generated blob and try again");
        return Err(PatchError::InputInvalidStamp);
    }
    debug!("input file stamp: 0x{:08X}, passed", stamp);

    // to maintain the consistency for both same and different input and output files, we operate on a file with modifications instead of creating new files
    // so we copy the file first then open it for read and write
    // Do NOT create a new file when opening output file path since it leads to creating an empty file and will break the consistency when input and output paths not same
    // If `is_same_file` fails, we just assume they are different and proceed
    let same_path = is_same_file(&input_path, &output_path).unwrap_or_else(|_| false);
    debug!(
        "input and output file same check: {}",
        if same_path { "same" } else { "different" }
    );
    if !same_path {
        debug!("input and output files are different, copying input file to output file");
        // Copy input binary to output path
        std::fs::copy(&input_path, &output_path)?;
        debug!(
            "copied input file {} to output file {}",
            input_path.as_ref().display(),
            output_path.as_ref().display()
        );
    }

    drop(input_file); // close input file before reopening for read and write

    let mut output_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&output_path)
        .map_err(|e| PatchError::IoError(e))?;
    debug!("opened output file: {}", output_path.as_ref().display());

    let new_len = align_up_to(total_length, 16 * 1024); // align up to 16KB
    output_file.set_len(new_len).unwrap();
    output_file.seek(SeekFrom::Start(0x10)).unwrap();
    output_file
        .write_u32::<LittleEndian>(new_len as u32)
        .unwrap();

    let mut checksum: u32 = 0;
    output_file.seek(SeekFrom::Start(0)).unwrap();
    loop {
        match output_file.read_u32::<LittleEndian>() {
            Ok(val) => checksum = checksum.wrapping_add(val),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => error!("io error while calculating checksum: {:?}", e),
        }
    }
    output_file.seek(SeekFrom::Start(0x0C)).unwrap();
    output_file.write_u32::<LittleEndian>(checksum).unwrap();
    output_file.sync_all().unwrap(); // save file before automatic closing

    Ok(())
}

fn align_up_to(len: u64, target_align: u64) -> u64 {
    let (div, rem) = (len / target_align, len % target_align);
    if rem != 0 {
        (div + 1) * target_align
    } else {
        len
    }
}
