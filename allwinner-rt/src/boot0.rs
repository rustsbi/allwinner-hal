/// eGON.BT0 identifying structure.
// TODO verify with original ROM source code
#[repr(C)]
pub struct EgonHead {
    /// Magic number, ="eGON.BT0".
    pub magic: [u8; 8],
    pub checksum: u32,
    pub length: u32,
    pub pub_head_size: u32,
    pub pub_head_version: [u8; 4],
    pub return_addr: u32,
    pub run_addr: u32,
    pub boot_cpu: u32,
    pub platform: [u8; 8],
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".head.egon")]
static EGON_HEAD: EgonHead = EgonHead {
    magic: *b"eGON.BT0",
    checksum: 0x5F0A6C39, // real checksum will be filled by blob generator
    length: 0x8000,
    pub_head_size: 0,
    pub_head_version: *b"3000",
    return_addr: 0,
    run_addr: 0,
    boot_cpu: 0,
    platform: *b"\0\03.0.0\0",
};
