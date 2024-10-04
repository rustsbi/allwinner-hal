use volatile_register::{RO, RW};

#[repr(C)]
pub struct RegisterBlock {
    /// 0x00 - SMC Global Control Register.
    pub global_control: RW<GlobalControl>,
    /// 0x04 - SMC Clock Control Register.
    pub clock_control: RW<ClockControl>,
    /// 0x08 - SMC Time Out Register.
    pub timeout: RW<u32>,
    /// 0x0C - SMC Bus Width Register.
    pub bus_width: RW<u32>,
    /// 0x10 - SMC Block Size Register.
    pub block_size: RW<u32>,
    /// 0x14 - SMC Byte Count Register.
    pub byte_count: RW<u32>,
    /// 0x18 - SMC Command Register.
    pub command: RW<u32>,
    /// 0x1C - SMC Argument Register.
    pub argument: RW<u32>,
    /// 0x20 ..= 0x2C - SMC Response Registers 0..=3.
    pub responses: [RO<u32>; 4],
    /// 0x30 - SMC Interrupt Mask Register.
    pub interrupt_mask: RW<u32>,
    /// 0x34 - SMC Masked Interrupt Status Register.
    pub interrupt_state_masked: RO<u32>,
    /// 0x38 - SMC Raw Interrupt Status Register.
    pub interrupt_state_raw: RW<u32>,
    /// 0x3C - SMC Status Register.
    pub status: RO<u32>,
    /// 0x40 - SMC FIFO Threshold Watermark Register.
    pub fifo_threshold: RW<u32>,
    /// 0x5c - SMC New Timing Set Register.
    pub new_timing_set: RW<u32>,
    /// 0x80 - SMC IDMAC Control Register.
    pub dma_control: RW<u32>,
    /// 0x84 - SMC IDMAC Descriptor List Base Address Register.
    pub dma_descriptor_base: RW<u32>,
    /// 0x88 - SMC IDMAC Status Register.
    pub dma_state: RW<u32>,
    /// 0x8C - SMC IDMAC Interrupt Enable Register.
    pub dma_interrupt_enable: RW<u32>,
    /// 0x140 - Drive Delay Control register.
    pub drive_delay_control: RW<u32>,
    /// 0x184 - deskew control control register.
    pub skew_control: RW<u32>,
    /// 0x200 - SMC FIFO Access Address.
    pub fifo: RW<u32>,
}

/// Global control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GlobalControl(u32);

impl GlobalControl {
    // TODO access_mode, set_access_mode, enum AccessMode { Dma, Ahb }
    // TODO ddr_mode, set_ddr_mode, enum DdrMode { Sdr, Ddr }
    // TODO is_dma_enabled, enable_dma, disable_dma
    // TODO is_interrupt_enabled, enable_interrupt, disable_interrupt
    // note: DMA_RST, FIFO_RST and SOFT_RST are write-1-set, auto-cleared by hardware
    // TODO has_dma_reset (DMA_RST == 1), set_dma_reset (self.0 | DMA_RST)
    // TODO has_fifo_reset, set_fifo_reset
    // TODO has_software_reset, set_software_reset
}

/// Clock control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ClockControl(u32);

impl ClockControl {
    // TODO is_mask_data0_enabled, enable_mask_data0, disable_mask_data0
    // TODO is_card_clock_enabled, enable_card_clock, disable_card_clock
    // TODO card_clock_divider, set_card_clock_divider (u8)
}

// TODO pub struct Timeout

// TODO data_timeout_limit, set_data_timeout_limit (u32)

// TODO pub struct BusWidth

// TODO bus_width, set_bus_width, enum BusWidth { OneBit, FourBit, EightBit }

// TODO pub struct BlockSize

// TODO block_size, set_block_size (u16)

// TODO pub struct ByteCount

// TODO byte_count, set_byte_count (u32)

// TODO pub struct Command

// bit 31 (SMHC_CMD_START, or CMD_LOAD) is write-1-set by software, auto-cleared by hardware
// TODO has_command_start, set_command_start
// bit 21 (SMHC_CMD_UPCLK_ONLY or PRG_CLK)
// TODO is_change_clock_enabled, enable_change_clock, disable_change_clock
// bit 15 (SMHC_CMD_SEND_INIT_SEQUENCE or SEND_INIT_SEQ)
// TODO is_init_sequence_enabled, enable_init_sequence, disable_init_sequence
// bit 14 (SMHC_CMD_STOP_ABORT_CMD or STOP_ABT_CMD)
// TODO is_stop_abort_enabled, enable_stop_abort, disable_stop_abort
// bit 13 (SMHC_CMD_WAIT_PRE_OVER, WAIT_PRE_OVER)
// TODO is_wait_for_complete_enabled, enable_wait_for_complete, disable_wait_for_complete
// bit 12 (SMHC_CMD_SEND_AUTO_STOP, STOP_CMD_FLAG)
// TODO is_auto_stop_enabled, enable_auto_stop, disable_auto_stop
// bit 10 (SMHC_CMD_WRITE or TRANS_DIR)
// TODO transfer_direction, set_transfer_direction, enum TransferDirection { Read, Write }
// bit 9 (SMHC_CMD_DATA_EXPIRE or DATA_TRANS)
// TODO is_data_transfer_enabled, enable_data_transfer, disable_data_transfer
// bit 8 (SMHC_CMD_CHECK_RESPONSE_CRC, CHK_RESP_CRC)
// TODO is_check_response_crc_enabled, enable_check_response_crc, disable_check_response_crc
// bit 7 (SMHC_CMD_LONG_RESPONSE or LONG_RESP)
// TODO is_long_response_enabled, enable_long_response, disable_long_response
// bit 6 (SMHC_CMD_RESP_EXPIRE, or RESP_RCV)
// TODO is_response_receive_enabled, enable_response_receive, disable_response_receive
// bit 5:0 CMD_IDX
// TODO command_index, set_command_index (u8)

// TODO pub struct Argument

// TODO argument, set_argument (u32)

// TODO pub struct InterruptEnable

// TODO is_interrupt_unmasked, mask_interrupt, unmask_interrupt (Interrupt)
/* TODO pub enum Interrupt {
    CardRemoved,
    CardInserted,
    Sdio,
    DataEndBitError,
    AutoCommandDone,
    DataStartError,
    CommandBusyAndIllegalWrite,
    FifoUnderrunOrOverflow,
    DataStarvationTimeout1V8SwitchDone,
    DataTimeoutBootDataStart,
    ResponseTimeoutBootAckReceived,
    DataCrcError,
    ResponseCrcError,
    DataReceiveRequest,
    DataTransmitRequest,
    DataTransferComplete,
    CommandComplete,
    ResponseError
} */

// TODO pub struct InterruptStateMasked

// TODO has_interrupt (Interrupt)

// TODO pub struct InterruptStateRaw

// TODO has_raw_interrupt (Interrupt)
// TODO clear_interrupt (Interrupt) note: write 1 to clear

// TODO pub struct Status

// note: read-only register, no write functions
// TODO fifo_level (u16)
// TODO card_data_busy
// TODO fifo_full
// TODO fifo_empty

// TODO pub struct FifoThreshold

// TODO burst_size, set_burst_size, enum BurstSize { One, Four, Eight, Sixteen }
// TODO receive_trigger_level, set_receive_trigger_level (u8)
// TODO transmit_trigger_level, set_transmit_trigger_level (u8)

// TODO pub struct NewTimingSet

// 31 MODE_SELECT
// TODO is_new_mode_enabled, enable_new_mode, disable_new_mode
// 9:8 DAT_SAMPLE_TIMING_PHASE
// TODO sample_timing_phase, set_sample_timing_phase, enum TimingPhase { Offset90, Offset180, Offset170, Offset0 }

// TODO pub struct DriveDelay

// TODO data_drive_phase, set_data_drive_phase (DrivePhase)
// TODO command_drive_phase, set_command_drive_phase (DrivePhase)
// TODO enum DrivePhase { Sdr90Ddr45, Sdr180Ddr90 }
