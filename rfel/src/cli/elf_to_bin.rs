use std::{
    fs,
    path::{Path, PathBuf},
};

use object::{Object as _, ObjectSection as _, SectionFlags, SectionKind};

type Result<T> = core::result::Result<T, Elf2BinError>;

#[derive(Debug, thiserror::Error)]
pub enum Elf2BinError {
    #[error("Failed to parse ELF file: {0}")]
    ParseError(String),
    #[error("I/O error: {0}")]
    IoError(std::io::Error),
    #[error("Object error: {0}")]
    ObjectError(object::Error),
    #[error("Section size overflow: {0}")]
    SectionSizeOverflow(u64),
}

// since the reference of PathBuf cannot be easily passed around as &Path and the return value should be PathBuf (for conveniently modify the extension), we let the `input` to be &Path
pub(crate) fn resolve_output_path(
    input: &Path,
    output: Option<PathBuf>,
    default_extension: &str,
) -> PathBuf {
    output.unwrap_or_else(|| input.with_extension(default_extension))
}

// TODO: In some cases the zero-padding might be incorrect

// The following functions are for elf2bin module
// Most of the code is adapted from `https://github.com/llvm/llvm-project/tree/main/llvm/lib/ObjCopy/ELF`

/// Main logic for converting ELF to binary, adapted from LLVM's objcopy
///
/// Ref: https://github.com/llvm/llvm-project/blob/main/llvm/lib/ObjCopy/ELF/ELFObjcopy.cpp  `Error
/// objcopy::elf::executeObjcopyOnBinary()` method
pub fn elf_to_bin_bytes(elf_data: &[u8]) -> Result<Vec<u8>> {
    // Parse the ELF file
    let elf_file = object::File::parse(elf_data).map_err(|e| Elf2BinError::ObjectError(e))?;

    // Get loadable sections
    let mut sections = get_loadable_sections(&elf_file);
    // Sort sections by their offset in the file
    sort_sections_with_offset(&mut sections);

    // Log section information
    log_section_info(&sections);

    // Create final binary output
    let output_data = process_sections(sections)?;

    Ok(output_data)
}

/// Wrapper function for converting ELF to binary, takes input and output file paths
pub fn elf_to_bin(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()> {
    // Read the ELF file
    let elf_data = fs::read(input_path).map_err(|e| Elf2BinError::IoError(e))?;

    // Convert ELF to binary
    let bin_data = elf_to_bin_bytes(&elf_data)?;

    // Write the binary data to the output file
    fs::write(output_path, bin_data).map_err(|e| Elf2BinError::IoError(e))?;

    Ok(())
}

// The following functions are helpers for elf2bin module

/// Log section information using `println`
fn log_section_info(sections: &[object::Section]) {
    println!("Found {} loadable sections", sections.len());

    for section in sections {
        println!(
            "Section: {} at address 0x{:x} with size 0x{:x}, align 0x{:x}",
            section.name().unwrap_or("<unnamed>"),
            section.address(),
            section.size(),
            section.align(),
        );
    }
}

/// Get loadable sections from the ELF file
///
/// Loadable sections are those with the ALLOC section header flag set
///
/// Ref: https://github.com/llvm/llvm-project/blob/main/llvm/lib/ObjCopy/ELF/ELFObject.cpp `Error BinaryWriter::finalize()` method
fn get_loadable_sections<'a>(elf_file: &'a object::File) -> Vec<object::Section<'a, 'a>> {
    // Collect sections with ALLOC flag. We keep NOBITS (.bss) out for objcopy parity.
    // GNU/LLVM objcopy -O binary does NOT emit .bss contents (they are zeroed at runtime).
    let mut sections: Vec<_> = elf_file
        .sections()
        .filter(|s| {
            let alloc = match s.flags() {
                SectionFlags::Elf { sh_flags } => (sh_flags & object::elf::SHF_ALLOC as u64) != 0,
                _ => false,
            };
            alloc && s.kind() != SectionKind::UninitializedData
        })
        .collect();
    // Sort by file offset (so we can build a contiguous blob of file-backed bytes)
    sections.sort_by_key(|s| get_section_offset(s));
    sections
}

/// Get the offset of a section using the `compressed_file_range` method,
/// panic if this method fails.
fn get_section_offset(section: &object::Section) -> u64 {
    section
        .compressed_file_range()
        .expect("Section file range not found!")
        .offset
}

/// Sort sections by their offset in the file
///
/// Ref:
/// https://github.com/llvm/llvm-project/blob/main/llvm/lib/ObjCopy/ELF/ELFObject.cpp
/// `Error BinaryWriter::write()`
fn sort_sections_with_offset(sections: &mut Vec<object::Section>) {
    sections.sort_by_key(|s| get_section_offset(s));
}

/// Process sections and serialize them into a raw binary similar to `objcopy -O binary`.
///
/// Differences vs previous implementation:
/// - We no longer rely on virtual addresses to create sparse output with padding up to the
///   next section's address. Instead we concatenate ALLOC sections in their file order so the
///   resulting blob only contains the actual section bytes.
/// - NOBITS sections (e.g. .bss) are skipped because objcopy -O binary also omits them (they
///   are zeroed at runtime by startup code).
fn process_sections(sections: Vec<object::Section>) -> Result<Vec<u8>> {
    // Implement an objcopy-like layout: concatenate all ALLOC + !NOBITS sections based on
    // their file offsets. We do NOT synthesize .bss or virtual address gaps. This matches
    // the common expectation for a raw firmware blob where runtime startup code zeroes BSS.
    if sections.is_empty() {
        return Ok(Vec::new());
    }

    // Gather file-backed sections with their raw file ranges & data
    struct Entry<'a> {
        name: String,
        file_off: u64,
        data: &'a [u8],
    }

    let mut entries: Vec<Entry> = Vec::new();
    for s in sections {
        let name = s.name().unwrap_or("<unnamed>").to_string();
        // Skip any residual NOBITS just in case
        if s.kind() == SectionKind::UninitializedData {
            continue;
        }
        let fr = match s.compressed_file_range() {
            Ok(r) => r,
            Err(_) => continue, // no file range => skip
        };
        let data = match s.data() {
            Ok(d) => d,
            Err(_) => continue,
        };
        // Use actual data length rather than uncompressed_size to avoid appending
        // artificial zero padding that objcopy would not synthesize.
        entries.push(Entry {
            name,
            file_off: fr.offset,
            data,
        });
    }

    if entries.is_empty() {
        return Ok(Vec::new());
    }

    entries.sort_by_key(|e| e.file_off);
    let total_len = entries.iter().try_fold(0u64, |acc, e| {
        let len = e.data.len() as u64;
        acc.checked_add(len)
            .ok_or(Elf2BinError::SectionSizeOverflow(acc.saturating_add(len)))
    })?;
    let total_len =
        usize::try_from(total_len).map_err(|_| Elf2BinError::SectionSizeOverflow(total_len))?;

    let mut output = Vec::with_capacity(total_len);
    let mut cursor = 0usize;

    for e in entries {
        let next = cursor + e.data.len();
        println!(
            "Writing section: {} file_off=0x{:x} data_len=0x{:x} -> out[0x{:x}..0x{:x}]",
            e.name,
            e.file_off,
            e.data.len(),
            cursor,
            next
        );
        output.extend_from_slice(e.data);
        cursor = next;
    }

    Ok(output)
}
