pub mod chips;
pub mod cli;
pub mod consts;
pub mod fel;
pub mod ops;
pub mod progress;
pub mod spi;
pub mod transfer;

pub use fel::{CHUNK_SIZE, Chip, Fel, Version};
pub use ops::{
    ChipOpError, ChipOpResult, DdrResult, FelOpError, FelOpResult, HexdumpLine, JtagResult,
    Read32Result, ReadResult, ResetResult, SidResult, VersionInfo, WriteResult, op_ddr, op_exec,
    op_hexdump, op_jtag, op_read, op_read32, op_reset, op_sid, op_version, op_write, op_write32,
};
pub use progress::Progress;
pub use transfer::{read_all, read_to_writer, write_all, write_from_reader};
