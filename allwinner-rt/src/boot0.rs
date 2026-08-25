use crate::start;

/// The 44-byte metadata portion of an eGON.BT0 file header.
///
/// A complete eGON.BT0 file header is 48 bytes long. Its first four bytes are
/// the jump instruction emitted as `head_jump`; this structure starts at file
/// offset 4, immediately after that instruction.
#[repr(C)]
pub struct EgonHead {
    /// Magic number, ="eGON.BT0".
    pub magic: [u8; 8],
    pub checksum: u32,
    pub length: u32,
    pub pub_head_size: u32,
    pub pub_head_version: [u8; 4],
    pub file_head_version: [u8; 4],
    pub boot_version: [u8; 4],
    pub egon_version: [u8; 4],
    pub platform: [u8; 8],
}

const _: [(); 44] = [(); core::mem::size_of::<EgonHead>()];

#[unsafe(no_mangle)]
#[unsafe(link_section = ".head.egon")]
static EGON_HEAD: EgonHead = EgonHead {
    magic: *b"eGON.BT0",
    checksum: 0x5F0A6C39, // real checksum will be filled by blob generator
    length: 0x8000,
    pub_head_size: 0,
    pub_head_version: *b"3000",
    file_head_version: [0; 4],
    boot_version: [0; 4],
    egon_version: [0; 4],
    platform: *b"\0\03.0.0\0",
};

// Keep the linker entry visible and exactly four bytes long: the BootROM reads
// the non-secure image magic from offset 4.
core::arch::global_asm! {
    ".pushsection .text.head, \"ax\", @progbits",
    ".global head_jump",
    ".type head_jump, @function",
    ".option push",
    ".option norvc",
    "head_jump:",
    "j {start}",
    ".option pop",
    ".size head_jump, . - head_jump",
    ".popsection",
    start = sym start,
}
