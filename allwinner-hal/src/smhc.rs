//! SD/MMC Host Controller peripheral.
use core::arch::asm;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx};
use volatile_register::{RO, RW};

use crate::ccu::{self, Clocks, SmhcClockSource};

/// SD/MMC Host Controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// 0x00 - SMC Global Control Register.
    pub global_control: RW<GlobalControl>,
    /// 0x04 - SMC Clock Control Register.
    pub clock_control: RW<ClockControl>,
    /// 0x08 - SMC Time Out Register.
    pub timeout: RW<TimeOut>,
    /// 0x0C - SMC Card Type (Bus Width) Register.
    pub card_type: RW<CardType>,
    /// 0x10 - SMC Block Size Register.
    pub block_size: RW<BlockSize>,
    /// 0x14 - SMC Byte Count Register.
    pub byte_count: RW<ByteCount>,
    /// 0x18 - SMC Command Register.
    pub command: RW<Command>,
    /// 0x1C - SMC Argument Register.
    pub argument: RW<Argument>,
    /// 0x20 ..= 0x2C - SMC Response Registers 0..=3.
    pub responses: [RO<u32>; 4],
    /// 0x30 - SMC Interrupt Mask Register.
    pub interrupt_mask: RW<InterruptMask>,
    /// 0x34 - SMC Masked Interrupt Status Register.
    pub interrupt_state_masked: RO<InterruptStateMasked>,
    /// 0x38 - SMC Raw Interrupt Status Register.
    pub interrupt_state_raw: RW<InterruptStateRaw>,
    /// 0x3C - SMC Status Register.
    pub status: RO<Status>,
    /// 0x40 - SMC FIFO Water Level Register.
    pub fifo_water_level: RW<FifoWaterLevel>,
    _reserved0: [u32; 6],
    /// 0x5c - SMC New Timing Set Register.
    pub new_timing_set: RW<NewTimingSet>,
    _reserved1: [u32; 8],
    /// 0x80 - SMC IDMAC Control Register.
    pub dma_control: RW<u32>,
    /// 0x84 - SMC IDMAC Descriptor List Base Address Register.
    pub dma_descriptor_base: RW<u32>,
    /// 0x88 - SMC IDMAC Status Register.
    pub dma_state: RW<u32>,
    /// 0x8C - SMC IDMAC Interrupt Enable Register.
    pub dma_interrupt_enable: RW<u32>,
    _reserved2: [u32; 44],
    /// 0x140 - Drive Delay Control register.
    pub drive_delay_control: RW<DriveDelayControl>,
    /// 0x144 - Sample Delay Control Register
    pub sample_delay_control: RW<SampleDelayControl>,
    _reserved3: [u32; 15],
    /// 0x184 - deskew control control register.
    pub skew_control: RW<u32>,
    _reserved4: [u32; 30],
    /// 0x200 - SMC FIFO Access Address.
    pub fifo: RW<u32>,
}

/// Global control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GlobalControl(u32);

/// FIFO access mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AccessMode {
    /// Dma bus.
    Dma,
    /// Ahb bus.
    Ahb,
}

/// DDR mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DdrMode {
    /// SDR mode.
    Sdr,
    /// DDR mode.
    Ddr,
}

impl GlobalControl {
    const FIFO_AC_MOD: u32 = 1 << 31;
    const DDR_MOD: u32 = 1 << 10;
    const DMA_ENB: u32 = 1 << 5;
    const INT_ENB: u32 = 1 << 4;
    const DMA_RST: u32 = 1 << 2;
    const FIFO_RST: u32 = 1 << 1;
    const SOFT_RST: u32 = 1 << 0;

    /// Get FIFO access mode.
    #[inline]
    pub const fn access_mode(self) -> AccessMode {
        match (self.0 & Self::FIFO_AC_MOD) >> 31 {
            0 => AccessMode::Dma,
            1 => AccessMode::Ahb,
            _ => unreachable!(),
        }
    }
    /// Set FIFO access mode.
    #[inline]
    pub const fn set_access_mode(self, mode: AccessMode) -> Self {
        let mode = match mode {
            AccessMode::Dma => 0x0,
            AccessMode::Ahb => 0x1,
        };
        Self((self.0 & !Self::FIFO_AC_MOD) | (mode << 31))
    }
    /// Get DDR mode.
    #[inline]
    pub const fn ddr_mode(self) -> DdrMode {
        match (self.0 & Self::DDR_MOD) >> 10 {
            0x0 => DdrMode::Sdr,
            0x1 => DdrMode::Ddr,
            _ => unreachable!(),
        }
    }
    /// Set DDR mode.
    #[inline]
    pub const fn set_ddr_mode(self, mode: DdrMode) -> Self {
        let mode = match mode {
            DdrMode::Sdr => 0x0,
            DdrMode::Ddr => 0x1,
        };
        Self((self.0 & !Self::DDR_MOD) | (mode << 10))
    }
    /// Is DMA enabled?
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        self.0 & Self::DMA_ENB != 0
    }
    /// Enable DMA.
    #[inline]
    pub const fn enable_dma(self) -> Self {
        Self(self.0 | Self::DMA_ENB)
    }
    /// Disable DMA.
    #[inline]
    pub const fn disable_dma(self) -> Self {
        Self(self.0 & !Self::DMA_ENB)
    }
    /// Is interrupt enabled?
    #[inline]
    pub const fn is_interrupt_enabled(self) -> bool {
        self.0 & Self::INT_ENB != 0
    }
    /// Enable interrupt.
    #[inline]
    pub const fn enable_interrupt(self) -> Self {
        Self(self.0 | Self::INT_ENB)
    }
    /// Disable interrupt.
    #[inline]
    pub const fn disable_interrupt(self) -> Self {
        Self(self.0 & !Self::INT_ENB)
    }
    /// DMA Reset.
    #[inline]
    pub const fn set_dma_reset(self) -> Self {
        Self(self.0 | Self::DMA_RST)
    }
    /// FIFO Reset.
    #[inline]
    pub const fn set_fifo_reset(self) -> Self {
        Self(self.0 | Self::FIFO_RST)
    }
    /// Software Reset.
    #[inline]
    pub const fn set_software_reset(self) -> Self {
        Self(self.0 | Self::SOFT_RST)
    }
    /// Is DMA Reset signal cleared by hardware?
    #[inline]
    pub const fn is_dma_reset_cleared(self) -> bool {
        (self.0 & Self::DMA_RST) == 0
    }
    /// Is FIFO Reset signal cleared by hardware?
    #[inline]
    pub const fn is_fifo_reset_cleared(self) -> bool {
        (self.0 & Self::FIFO_RST) == 0
    }
    /// Is Software Reset signal cleared by hardware?
    #[inline]
    pub const fn is_software_reset_cleared(self) -> bool {
        (self.0 & Self::SOFT_RST) == 0
    }
}

/// Clock control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClockControl(u32);

impl ClockControl {
    const MASK_DATA0: u32 = 1 << 31;
    const CCLK_CTRL: u32 = 1 << 16;
    const CCLK_DIV: u32 = 0xFF << 0;
    /// If mask data0 is enabled.
    #[inline]
    pub const fn is_mask_data0_enabled(self) -> bool {
        self.0 & Self::MASK_DATA0 != 0
    }
    /// Enable mask data0.
    #[inline]
    pub const fn enable_mask_data0(self) -> Self {
        Self(self.0 | Self::MASK_DATA0)
    }
    /// Disable mask data0.
    #[inline]
    pub const fn disable_mask_data0(self) -> Self {
        Self(self.0 & !Self::MASK_DATA0)
    }
    /// If card clock is enabled.
    pub const fn is_card_clock_enabled(self) -> bool {
        self.0 & Self::CCLK_CTRL != 0
    }
    /// Enable card clock.
    #[inline]
    pub const fn enable_card_clock(self) -> Self {
        Self(self.0 | Self::CCLK_CTRL)
    }
    /// Disable card clock.
    #[inline]
    pub const fn disable_card_clock(self) -> Self {
        Self(self.0 & !Self::CCLK_CTRL)
    }
    /// Get card clock divider.
    #[inline]
    pub const fn card_clock_divider(self) -> u8 {
        ((self.0 & Self::CCLK_DIV) >> 0) as u8
    }
    /// Set card clock divider.
    #[inline]
    pub const fn set_card_clock_divider(self, divider: u8) -> Self {
        Self((self.0 & !Self::CCLK_DIV) | ((divider as u32) << 0))
    }
}

/// Time out register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TimeOut(u32);

impl TimeOut {
    const DTO_LMT: u32 = 0xFFFFFF << 8;

    /// Get data timeout limit.
    #[inline]
    pub const fn data_timeout_limit(self) -> u32 {
        (self.0 & Self::DTO_LMT) >> 8
    }
    /// Set data timeout limit.
    #[inline]
    pub const fn set_data_timeout_limit(self, limit: u32) -> Self {
        Self((self.0 & !Self::DTO_LMT) | (limit << 8))
    }
}

/// Card type register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CardType(u32);

/// Bus width bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BusWidth {
    /// 1 bit.
    OneBit,
    /// 4 bit.
    FourBit,
    /// 8 bit.
    EightBit,
}

impl CardType {
    const CARD_WID: u32 = 0x3 << 0;
    /// Get bus width.
    #[inline]
    pub const fn bus_width(self) -> BusWidth {
        match (self.0 & Self::CARD_WID) >> 0 {
            0x0 => BusWidth::OneBit,
            0x1 => BusWidth::FourBit,
            0x2 | 0x3 => BusWidth::EightBit,
            _ => unreachable!(),
        }
    }
    /// Set bus width.
    #[inline]
    pub const fn set_bus_width(self, width: BusWidth) -> Self {
        Self((self.0 & !Self::CARD_WID) | ((width as u32) << 0))
    }
}

impl Default for CardType {
    #[inline]
    fn default() -> Self {
        Self(0x0000_0000)
    }
}

/// Block size register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockSize(u32);

impl BlockSize {
    const BLK_SZ: u32 = 0x0000FFFF << 0;
    /// Get block size.
    #[inline]
    pub const fn block_size(self) -> u16 {
        ((self.0 & Self::BLK_SZ) >> 0) as u16
    }
    /// Set block size.
    #[inline]
    pub const fn set_block_size(self, size: u16) -> Self {
        Self((self.0 & !Self::BLK_SZ) | ((size as u32) << 0))
    }
}

impl Default for BlockSize {
    #[inline]
    fn default() -> Self {
        Self(0x0000_0200)
    }
}

/// Byte count register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ByteCount(u32);

impl ByteCount {
    const BYTE_CNT: u32 = 0xFFFFFFFF << 0;
    /// Get byte count.
    #[inline]
    pub const fn byte_count(self) -> u32 {
        (self.0 & Self::BYTE_CNT) >> 0
    }
    /// Set byte count.
    #[inline]
    pub const fn set_byte_count(self, count: u32) -> Self {
        Self((self.0 & !Self::BYTE_CNT) | (count << 0))
    }
}

/// Command register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Command(u32);

/// Transfer direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferDirection {
    /// Read from card.
    Read,
    /// Write to card.
    Write,
}

impl Command {
    const CMD_LOAD: u32 = 0x1 << 31;
    const PRG_CLK: u32 = 0x1 << 21;
    const SEND_INIT_SEQ: u32 = 0x1 << 15;
    const STOP_ABT_CMD: u32 = 0x1 << 14;
    const WAIT_PRE_OVER: u32 = 0x1 << 13;
    const STOP_CMD_FLAG: u32 = 0x1 << 12;
    const TRANS_DIR: u32 = 0x1 << 10;
    const DATA_TRANS: u32 = 0x1 << 9;
    const CHK_RESP_CRC: u32 = 0x1 << 8;
    const LONG_RESP: u32 = 0x1 << 7;
    const RESP_RCV: u32 = 0x1 << 6;
    const CMD_IDX: u32 = 0x3F << 0;

    /// Start command.
    #[inline]
    pub const fn set_command_start(self) -> Self {
        Self(self.0 | Self::CMD_LOAD)
    }
    /// If change clock is enabled.
    #[inline]
    pub const fn is_change_clock_enabled(self) -> bool {
        (self.0 & Self::PRG_CLK) != 0
    }
    /// Enable change clock.
    #[inline]
    pub const fn enable_change_clock(self) -> Self {
        Self(self.0 | Self::PRG_CLK)
    }
    /// Disable change clock.
    #[inline]
    pub const fn disable_change_clock(self) -> Self {
        Self(self.0 & !Self::PRG_CLK)
    }
    /// If send init sequence is enabled.
    #[inline]
    pub const fn is_send_init_seq_enabled(self) -> bool {
        (self.0 & Self::SEND_INIT_SEQ) != 0
    }
    /// Enable send init sequence.
    #[inline]
    pub const fn enable_send_init_seq(self) -> Self {
        Self(self.0 | Self::SEND_INIT_SEQ)
    }
    /// Disable send init sequence.
    #[inline]
    pub const fn disable_send_init_seq(self) -> Self {
        Self(self.0 & !Self::SEND_INIT_SEQ)
    }
    /// If stop abort command is enabled.
    #[inline]
    pub const fn is_stop_abort_enabled(self) -> bool {
        (self.0 & Self::STOP_ABT_CMD) != 0
    }
    /// Enable stop abort command.
    #[inline]
    pub const fn enable_stop_abort(self) -> Self {
        Self(self.0 | Self::STOP_ABT_CMD)
    }
    /// Disable stop abort command.
    #[inline]
    pub const fn disable_stop_abort(self) -> Self {
        Self(self.0 & !Self::STOP_ABT_CMD)
    }
    /// If wait for complete is enabled.
    #[inline]
    pub const fn is_wait_for_complete_enabled(self) -> bool {
        (self.0 & Self::WAIT_PRE_OVER) != 0
    }
    /// Enable wait for complete.
    #[inline]
    pub const fn enable_wait_for_complete(self) -> Self {
        Self(self.0 | Self::WAIT_PRE_OVER)
    }
    /// Disable wait for complete.
    #[inline]
    pub const fn disable_wait_for_complete(self) -> Self {
        Self(self.0 & !Self::WAIT_PRE_OVER)
    }
    /// If auto stop is enabled.
    #[inline]
    pub const fn is_auto_stop_enabled(self) -> bool {
        (self.0 & Self::STOP_CMD_FLAG) != 0
    }
    /// Enable auto stop.
    #[inline]
    pub const fn enable_auto_stop(self) -> Self {
        Self(self.0 | Self::STOP_CMD_FLAG)
    }
    /// Disable auto stop.
    #[inline]
    pub const fn disable_auto_stop(self) -> Self {
        Self(self.0 & !Self::STOP_CMD_FLAG)
    }
    /// Get transfer direction.
    #[inline]
    pub const fn transfer_direction(self) -> TransferDirection {
        match (self.0 & Self::TRANS_DIR) >> 10 {
            0 => TransferDirection::Read,
            1 => TransferDirection::Write,
            _ => unreachable!(),
        }
    }
    /// Set transfer direction.
    #[inline]
    pub const fn set_transfer_direction(self, dir: TransferDirection) -> Self {
        Self((self.0 & !Self::TRANS_DIR) | ((dir as u32) << 10))
    }
    /// If data transfer is enabled.
    #[inline]
    pub const fn is_data_transfer_enabled(self) -> bool {
        (self.0 & Self::DATA_TRANS) != 0
    }
    /// Enable data transfer.
    #[inline]
    pub const fn enable_data_transfer(self) -> Self {
        Self(self.0 | Self::DATA_TRANS)
    }
    /// Disable data transfer.
    #[inline]
    pub const fn disable_data_transfer(self) -> Self {
        Self(self.0 & !Self::DATA_TRANS)
    }
    /// If check response CRC is enabled.
    #[inline]
    pub const fn is_check_response_crc_enabled(self) -> bool {
        (self.0 & Self::CHK_RESP_CRC) != 0
    }
    /// Enable check response CRC.
    #[inline]
    pub const fn enable_check_response_crc(self) -> Self {
        Self(self.0 | Self::CHK_RESP_CRC)
    }
    /// Disable check response CRC.
    #[inline]
    pub const fn disable_check_response_crc(self) -> Self {
        Self(self.0 & !Self::CHK_RESP_CRC)
    }
    /// If long response is enabled.
    #[inline]
    pub const fn is_long_response_enabled(self) -> bool {
        (self.0 & Self::LONG_RESP) != 0
    }
    /// Enable long response.
    #[inline]
    pub const fn enable_long_response(self) -> Self {
        Self(self.0 | Self::LONG_RESP)
    }
    /// Disable long response.
    #[inline]
    pub const fn disable_long_response(self) -> Self {
        Self(self.0 & !Self::LONG_RESP)
    }
    /// If response receive enabled.
    #[inline]
    pub const fn is_response_receive_enabled(self) -> bool {
        (self.0 & Self::RESP_RCV) != 0
    }
    /// Enable response receive.
    #[inline]
    pub const fn enable_response_receive(self) -> Self {
        Self(self.0 | Self::RESP_RCV)
    }
    /// Disable response receive.
    #[inline]
    pub const fn disable_response_receive(self) -> Self {
        Self(self.0 & !Self::RESP_RCV)
    }
    /// Get command index.
    #[inline]
    pub const fn command_index(self) -> u8 {
        ((self.0 & Self::CMD_IDX) >> 0) as u8
    }
    /// Set command index.
    #[inline]
    pub const fn set_command_index(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_IDX) | ((val as u32) << 0))
    }
    /// Is command start signal cleared by hardware?
    #[inline]
    pub const fn is_command_start_cleared(self) -> bool {
        (self.0 & Self::CMD_LOAD) == 0
    }
}

impl Default for Command {
    #[inline]
    fn default() -> Self {
        Self(0x0000_0000)
    }
}

/// Argument register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Argument(u32);

impl Argument {
    const CMD_ARG: u32 = 0xFFFFFFFF << 0;

    /// Get argument.
    #[inline]
    pub const fn argument(self) -> u32 {
        (self.0 & Self::CMD_ARG) as u32 >> 0
    }
    /// Set argument.
    #[inline]
    pub const fn set_argument(self, arg: u32) -> Self {
        Self((self.0 & !Self::CMD_ARG) | ((arg as u32) << 0))
    }
}

/// Interrupt mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptMask(u32);

/// Interrupt type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Interrupt {
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
    ResponseError,
}

impl InterruptMask {
    const CARD_REMOVAL_INT_EN: u32 = 1 << 31;
    const CARD_INSERT_INT_EN: u32 = 1 << 30;
    const SDIO_INT_EN: u32 = 1 << 16;
    const DEE_INT_EN: u32 = 1 << 15;
    const ACD_INT_EN: u32 = 1 << 14;
    const DSE_BC_INT_EN: u32 = 1 << 13;
    const CB_IW_INT_EN: u32 = 1 << 12;
    const FU_FO_INT_EN: u32 = 1 << 11;
    const DSTO_VSD_INT_EN: u32 = 1 << 10;
    const DTO_BDS_INT_EN: u32 = 1 << 9;
    const RTO_BACK_INT_EN: u32 = 1 << 8;
    const DCE_INT_EN: u32 = 1 << 7;
    const RCE_INT_EN: u32 = 1 << 6;
    const DRR_INT_EN: u32 = 1 << 5;
    const DTR_INT_EN: u32 = 1 << 4;
    const DTC_INT_EN: u32 = 1 << 3;
    const CC_INT_EN: u32 = 1 << 2;
    const RE_INT_EN: u32 = 1 << 1;

    /// If the interrupt is unmasked.
    pub const fn is_interrupt_unmasked(self, interrupt: Interrupt) -> bool {
        match interrupt {
            Interrupt::CardRemoved => self.0 & Self::CARD_REMOVAL_INT_EN != 0,
            Interrupt::CardInserted => self.0 & Self::CARD_INSERT_INT_EN != 0,
            Interrupt::Sdio => self.0 & Self::SDIO_INT_EN != 0,
            Interrupt::DataEndBitError => self.0 & Self::DEE_INT_EN != 0,
            Interrupt::AutoCommandDone => self.0 & Self::ACD_INT_EN != 0,
            Interrupt::DataStartError => self.0 & Self::DSE_BC_INT_EN != 0,
            Interrupt::CommandBusyAndIllegalWrite => self.0 & Self::CB_IW_INT_EN != 0,
            Interrupt::FifoUnderrunOrOverflow => self.0 & Self::FU_FO_INT_EN != 0,
            Interrupt::DataStarvationTimeout1V8SwitchDone => self.0 & Self::DSTO_VSD_INT_EN != 0,
            Interrupt::DataTimeoutBootDataStart => self.0 & Self::DTO_BDS_INT_EN != 0,
            Interrupt::ResponseTimeoutBootAckReceived => self.0 & Self::RTO_BACK_INT_EN != 0,
            Interrupt::DataCrcError => self.0 & Self::DCE_INT_EN != 0,
            Interrupt::ResponseCrcError => self.0 & Self::RCE_INT_EN != 0,
            Interrupt::DataReceiveRequest => self.0 & Self::DRR_INT_EN != 0,
            Interrupt::DataTransmitRequest => self.0 & Self::DTR_INT_EN != 0,
            Interrupt::DataTransferComplete => self.0 & Self::DTC_INT_EN != 0,
            Interrupt::CommandComplete => self.0 & Self::CC_INT_EN != 0,
            Interrupt::ResponseError => self.0 & Self::RE_INT_EN != 0,
        }
    }
    /// Unmask the specified interrupt.
    #[inline]
    pub const fn unmask_interrupt(self, interrupt: Interrupt) -> Self {
        match interrupt {
            Interrupt::CardRemoved => Self(self.0 | Self::CARD_REMOVAL_INT_EN),
            Interrupt::CardInserted => Self(self.0 | Self::CARD_INSERT_INT_EN),
            Interrupt::Sdio => Self(self.0 | Self::SDIO_INT_EN),
            Interrupt::DataEndBitError => Self(self.0 | Self::DEE_INT_EN),
            Interrupt::AutoCommandDone => Self(self.0 | Self::ACD_INT_EN),
            Interrupt::DataStartError => Self(self.0 | Self::DSE_BC_INT_EN),
            Interrupt::CommandBusyAndIllegalWrite => Self(self.0 | Self::CB_IW_INT_EN),
            Interrupt::FifoUnderrunOrOverflow => Self(self.0 | Self::FU_FO_INT_EN),
            Interrupt::DataStarvationTimeout1V8SwitchDone => Self(self.0 | Self::DSTO_VSD_INT_EN),
            Interrupt::DataTimeoutBootDataStart => Self(self.0 | Self::DTO_BDS_INT_EN),
            Interrupt::ResponseTimeoutBootAckReceived => Self(self.0 | Self::RTO_BACK_INT_EN),
            Interrupt::DataCrcError => Self(self.0 | Self::DCE_INT_EN),
            Interrupt::ResponseCrcError => Self(self.0 | Self::RCE_INT_EN),
            Interrupt::DataReceiveRequest => Self(self.0 | Self::DRR_INT_EN),
            Interrupt::DataTransmitRequest => Self(self.0 | Self::DTR_INT_EN),
            Interrupt::DataTransferComplete => Self(self.0 | Self::DTC_INT_EN),
            Interrupt::CommandComplete => Self(self.0 | Self::CC_INT_EN),
            Interrupt::ResponseError => Self(self.0 | Self::RE_INT_EN),
        }
    }
    /// Mask the specified interrupt.
    #[inline]
    pub const fn mask_interrupt(self, interrupt: Interrupt) -> Self {
        match interrupt {
            Interrupt::CardRemoved => Self(self.0 & !Self::CARD_REMOVAL_INT_EN),
            Interrupt::CardInserted => Self(self.0 & !Self::CARD_INSERT_INT_EN),
            Interrupt::Sdio => Self(self.0 & !Self::SDIO_INT_EN),
            Interrupt::DataEndBitError => Self(self.0 & !Self::DEE_INT_EN),
            Interrupt::AutoCommandDone => Self(self.0 & !Self::ACD_INT_EN),
            Interrupt::DataStartError => Self(self.0 & !Self::DSE_BC_INT_EN),
            Interrupt::CommandBusyAndIllegalWrite => Self(self.0 & !Self::CB_IW_INT_EN),
            Interrupt::FifoUnderrunOrOverflow => Self(self.0 & !Self::FU_FO_INT_EN),
            Interrupt::DataStarvationTimeout1V8SwitchDone => Self(self.0 & !Self::DSTO_VSD_INT_EN),
            Interrupt::DataTimeoutBootDataStart => Self(self.0 & !Self::DTO_BDS_INT_EN),
            Interrupt::ResponseTimeoutBootAckReceived => Self(self.0 & !Self::RTO_BACK_INT_EN),
            Interrupt::DataCrcError => Self(self.0 & !Self::DCE_INT_EN),
            Interrupt::ResponseCrcError => Self(self.0 & !Self::RCE_INT_EN),
            Interrupt::DataReceiveRequest => Self(self.0 & !Self::DRR_INT_EN),
            Interrupt::DataTransmitRequest => Self(self.0 & !Self::DTR_INT_EN),
            Interrupt::DataTransferComplete => Self(self.0 & !Self::DTC_INT_EN),
            Interrupt::CommandComplete => Self(self.0 & !Self::CC_INT_EN),
            Interrupt::ResponseError => Self(self.0 & !Self::RE_INT_EN),
        }
    }
}

/// Masked Interrupt state masked register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptStateMasked(u32);

impl InterruptStateMasked {
    const M_CARD_REMOVAL_INT: u32 = 1 << 31;
    const M_CARD_INSERT_INT: u32 = 1 << 30;
    const M_SDIO_INT: u32 = 1 << 16;
    const M_DEE_INT: u32 = 1 << 15;
    const M_ACD_INT: u32 = 1 << 14;
    const M_DSE_BC_INT: u32 = 1 << 13;
    const M_CB_IW_INT: u32 = 1 << 12;
    const M_FU_FO_INT: u32 = 1 << 11;
    const M_DSTO_VSD_INT: u32 = 1 << 10;
    const M_DTO_BDS_INT: u32 = 1 << 9;
    const M_RTO_BACK_INT: u32 = 1 << 8;
    const M_DCE_INT: u32 = 1 << 7;
    const M_RCE_INT: u32 = 1 << 6;
    const M_DRR_INT: u32 = 1 << 5;
    const M_DTR_INT: u32 = 1 << 4;
    const M_DTC_INT: u32 = 1 << 3;
    const M_CC_INT: u32 = 1 << 2;
    const M_RE_INT: u32 = 1 << 1;

    /// If the interrupt occurs.
    #[inline]
    pub const fn has_interrupt(self, interrupt: Interrupt) -> bool {
        match interrupt {
            Interrupt::CardRemoved => self.0 & Self::M_CARD_REMOVAL_INT != 0,
            Interrupt::CardInserted => self.0 & Self::M_CARD_INSERT_INT != 0,
            Interrupt::Sdio => self.0 & Self::M_SDIO_INT != 0,
            Interrupt::DataEndBitError => self.0 & Self::M_DEE_INT != 0,
            Interrupt::AutoCommandDone => self.0 & Self::M_ACD_INT != 0,
            Interrupt::DataStartError => self.0 & Self::M_DSE_BC_INT != 0,
            Interrupt::CommandBusyAndIllegalWrite => self.0 & Self::M_CB_IW_INT != 0,
            Interrupt::FifoUnderrunOrOverflow => self.0 & Self::M_FU_FO_INT != 0,
            Interrupt::DataStarvationTimeout1V8SwitchDone => self.0 & Self::M_DSTO_VSD_INT != 0,
            Interrupt::DataTimeoutBootDataStart => self.0 & Self::M_DTO_BDS_INT != 0,
            Interrupt::ResponseTimeoutBootAckReceived => self.0 & Self::M_RTO_BACK_INT != 0,
            Interrupt::DataCrcError => self.0 & Self::M_DCE_INT != 0,
            Interrupt::ResponseCrcError => self.0 & Self::M_RCE_INT != 0,
            Interrupt::DataReceiveRequest => self.0 & Self::M_DRR_INT != 0,
            Interrupt::DataTransmitRequest => self.0 & Self::M_DTR_INT != 0,
            Interrupt::DataTransferComplete => self.0 & Self::M_DTC_INT != 0,
            Interrupt::CommandComplete => self.0 & Self::M_CC_INT != 0,
            Interrupt::ResponseError => self.0 & Self::M_RE_INT != 0,
        }
    }
}

/// Raw Interrupt state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptStateRaw(u32);

impl InterruptStateRaw {
    const CARD_REMOVAL: u32 = 1 << 31;
    const CARD_INSERT: u32 = 1 << 30;
    const SDIO_INT: u32 = 1 << 16;
    const DEE: u32 = 1 << 15;
    const ACD: u32 = 1 << 14;
    const DSE_BC: u32 = 1 << 13;
    const CB_IW: u32 = 1 << 12;
    const FU_FO: u32 = 1 << 11;
    const DSTO_VSD: u32 = 1 << 10;
    const DTO_BDS: u32 = 1 << 9;
    const RTO_BACK: u32 = 1 << 8;
    const DCE: u32 = 1 << 7;
    const RCE: u32 = 1 << 6;
    const DRR: u32 = 1 << 5;
    const DTR: u32 = 1 << 4;
    const DTC: u32 = 1 << 3;
    const CC: u32 = 1 << 2;
    const RE: u32 = 1 << 1;

    /// If the interrupt occurs.
    #[inline]
    pub const fn has_interrupt(self, interrupt: Interrupt) -> bool {
        match interrupt {
            Interrupt::CardRemoved => self.0 & Self::CARD_REMOVAL != 0,
            Interrupt::CardInserted => self.0 & Self::CARD_INSERT != 0,
            Interrupt::Sdio => self.0 & Self::SDIO_INT != 0,
            Interrupt::DataEndBitError => self.0 & Self::DEE != 0,
            Interrupt::AutoCommandDone => self.0 & Self::ACD != 0,
            Interrupt::DataStartError => self.0 & Self::DSE_BC != 0,
            Interrupt::CommandBusyAndIllegalWrite => self.0 & Self::CB_IW != 0,
            Interrupt::FifoUnderrunOrOverflow => self.0 & Self::FU_FO != 0,
            Interrupt::DataStarvationTimeout1V8SwitchDone => self.0 & Self::DSTO_VSD != 0,
            Interrupt::DataTimeoutBootDataStart => self.0 & Self::DTO_BDS != 0,
            Interrupt::ResponseTimeoutBootAckReceived => self.0 & Self::RTO_BACK != 0,
            Interrupt::DataCrcError => self.0 & Self::DCE != 0,
            Interrupt::ResponseCrcError => self.0 & Self::RCE != 0,
            Interrupt::DataReceiveRequest => self.0 & Self::DRR != 0,
            Interrupt::DataTransmitRequest => self.0 & Self::DTR != 0,
            Interrupt::DataTransferComplete => self.0 & Self::DTC != 0,
            Interrupt::CommandComplete => self.0 & Self::CC != 0,
            Interrupt::ResponseError => self.0 & Self::RE != 0,
        }
    }
    /// Clears the specified interrupt.
    #[inline]
    pub const fn clear_interrupt(self, interrupt: Interrupt) -> Self {
        match interrupt {
            Interrupt::CardRemoved => Self(self.0 | Self::CARD_REMOVAL),
            Interrupt::CardInserted => Self(self.0 | Self::CARD_INSERT),
            Interrupt::Sdio => Self(self.0 | Self::SDIO_INT),
            Interrupt::DataEndBitError => Self(self.0 | Self::DEE),
            Interrupt::AutoCommandDone => Self(self.0 | Self::ACD),
            Interrupt::DataStartError => Self(self.0 | Self::DSE_BC),
            Interrupt::CommandBusyAndIllegalWrite => Self(self.0 | Self::CB_IW),
            Interrupt::FifoUnderrunOrOverflow => Self(self.0 | Self::FU_FO),
            Interrupt::DataStarvationTimeout1V8SwitchDone => Self(self.0 | Self::DSTO_VSD),
            Interrupt::DataTimeoutBootDataStart => Self(self.0 | Self::DTO_BDS),
            Interrupt::ResponseTimeoutBootAckReceived => Self(self.0 | Self::RTO_BACK),
            Interrupt::DataCrcError => Self(self.0 | Self::DCE),
            Interrupt::ResponseCrcError => Self(self.0 | Self::RCE),
            Interrupt::DataReceiveRequest => Self(self.0 | Self::DRR),
            Interrupt::DataTransmitRequest => Self(self.0 | Self::DTR),
            Interrupt::DataTransferComplete => Self(self.0 | Self::DTC),
            Interrupt::CommandComplete => Self(self.0 | Self::CC),
            Interrupt::ResponseError => Self(self.0 | Self::RE),
        }
    }
}

/// State register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
// note: read-only register, no write functions
pub struct Status(u32);

impl Status {
    const FIFO_LEVEL: u32 = 0x1FF << 17;
    const CARD_BUSY: u32 = 1 << 9;
    const FIFO_FULL: u32 = 1 << 3;
    const FIFO_EMPTY: u32 = 1 << 2;

    /// Get FIFO level.
    #[inline]
    pub const fn fifo_level(self) -> u16 {
        ((self.0 & Self::FIFO_LEVEL) >> 17) as u16
    }
    /// Is the card busy?
    #[inline]
    pub const fn card_busy(self) -> bool {
        self.0 & Self::CARD_BUSY != 0
    }
    /// Is the FIFO full?
    #[inline]
    pub const fn fifo_full(self) -> bool {
        self.0 & Self::FIFO_FULL != 0
    }
    /// Is the FIFO empty?
    #[inline]
    pub const fn fifo_empty(self) -> bool {
        self.0 & Self::FIFO_EMPTY != 0
    }
}

/// FIFO water level register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoWaterLevel(u32);

/// Burst size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BurstSize {
    /// 1 byte.
    OneBit,
    /// 4 bytes.
    FourBit,
    /// 8 bytes.
    EightBit,
    /// 16 bytes.
    SixteenBit,
}

impl FifoWaterLevel {
    const BSIZE_OF_TRANS: u32 = 0x7 << 28;
    const RX_TL: u32 = 0xFF << 16;
    const TX_TL: u32 = 0xFF << 0;

    /// Get the burst size of the transmitter. Value is from 0 to 3.(4 to 7 are reserved)
    #[inline]
    pub const fn burst_size(self) -> BurstSize {
        match (self.0 & Self::BSIZE_OF_TRANS) >> 28 {
            0 => BurstSize::OneBit,
            1 => BurstSize::FourBit,
            2 => BurstSize::EightBit,
            3 => BurstSize::SixteenBit,
            _ => unreachable!(),
        }
    }
    /// Set the burst size of the transmitter. Value is from 0 to 3.(4 to 7 are reserved)
    #[inline]
    pub const fn set_burst_size(self, bsize: BurstSize) -> Self {
        Self(self.0 & !Self::BSIZE_OF_TRANS | (bsize as u32) << 28)
    }
    /// Get the receive trigger level(0xFF is reserved).
    #[inline]
    pub const fn receive_trigger_level(self) -> u8 {
        ((self.0 & Self::RX_TL) >> 16) as u8
    }
    /// Set the receive trigger level(0x0 to 0xFE, 0xFF is reserved).
    #[inline]
    pub const fn set_receive_trigger_level(self, level: u8) -> Self {
        Self(self.0 & !Self::RX_TL | (level as u32) << 16)
    }
    /// Get the transmit trigger level(0x1 to 0xFF, 0x0 is no trigger).
    #[inline]
    pub const fn transmit_trigger_level(self) -> u8 {
        ((self.0 & Self::TX_TL) >> 0) as u8
    }
    /// Set the transmit trigger level(0x0 to 0xFF, 0x0 is no trigger).
    #[inline]
    pub const fn set_transmit_trigger_level(self, level: u8) -> Self {
        Self(self.0 & !Self::TX_TL | (level as u32) << 0)
    }
}

/// New timing set register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NewTimingSet(u32);

/// New timing set timing phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NtsTimingPhase {
    Offset90,
    Offset180,
    Offset270,
    Offset0,
}

impl NewTimingSet {
    const MODE_SELECT: u32 = 1 << 31;
    const DAT_SAMPLE_TIMING_PHASE: u32 = 0x3 << 8;

    /// If new mode is enabled.
    #[inline]
    pub const fn is_new_mode_enabled(self) -> bool {
        (self.0 & Self::MODE_SELECT) != 0
    }
    /// Enable new mode.
    #[inline]
    pub const fn enable_new_mode(self) -> Self {
        Self(self.0 | Self::MODE_SELECT)
    }
    /// Disable new mode.
    #[inline]
    pub const fn disable_new_mode(self) -> Self {
        Self(self.0 & !Self::MODE_SELECT)
    }
    /// Get timing phase.
    #[inline]
    pub const fn sample_timing_phase(self) -> NtsTimingPhase {
        match (self.0 & Self::DAT_SAMPLE_TIMING_PHASE) >> 8 {
            0x0 => NtsTimingPhase::Offset90,
            0x1 => NtsTimingPhase::Offset180,
            0x2 => NtsTimingPhase::Offset270,
            0x3 => NtsTimingPhase::Offset0,
            _ => unreachable!(),
        }
    }
    /// Set timing phase.
    #[inline]
    pub const fn set_sample_timing_phase(self, phase: NtsTimingPhase) -> Self {
        Self((self.0 & !Self::DAT_SAMPLE_TIMING_PHASE) | ((phase as u32) << 8))
    }
}

/// Drive Delay Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DriveDelayControl(u32);

/// Drive delay control timing phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DdcTimingPhase {
    /// Offset is 90 degrees at SDR mode, 45 degrees at DDR mode.
    Sdr90Ddr45,
    /// Offset is 180 degrees at SDR mode, 90 degrees at DDR mode.
    Sdr180Ddr90,
}

impl DriveDelayControl {
    const DAT_DRV_PH_SEL: u32 = 1 << 17;
    const CMD_DRV_PH_SEL: u32 = 1 << 16;

    /// Get data drive phase.
    #[inline]
    pub const fn data_drive_phase(self) -> DdcTimingPhase {
        match (self.0 & Self::DAT_DRV_PH_SEL) >> 17 {
            0x0 => DdcTimingPhase::Sdr90Ddr45,
            0x1 => DdcTimingPhase::Sdr180Ddr90,
            _ => unreachable!(),
        }
    }
    /// Set data drive phase.
    #[inline]
    pub const fn set_data_drive_phase(self, phase: DdcTimingPhase) -> Self {
        Self((self.0 & !Self::DAT_DRV_PH_SEL) | ((phase as u32) << 17))
    }
    /// Get command drive phase.
    #[inline]
    pub const fn command_drive_phase(self) -> DdcTimingPhase {
        match (self.0 & Self::CMD_DRV_PH_SEL) >> 16 {
            0x0 => DdcTimingPhase::Sdr90Ddr45,
            0x1 => DdcTimingPhase::Sdr180Ddr90,
            _ => unreachable!(),
        }
    }
    /// Set command drive phase.
    #[inline]
    pub const fn set_command_drive_phase(self, phase: DdcTimingPhase) -> Self {
        Self((self.0 & !Self::CMD_DRV_PH_SEL) | ((phase as u32) << 16))
    }
}

/// Sample Delay Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SampleDelayControl(u32);

impl SampleDelayControl {
    const SAMP_DL_SW: u32 = 0x3F << 0;
    const SAMP_DL_SW_EN: u32 = 1 << 7;

    /// Set sample delay software.
    #[inline]
    pub const fn set_sample_delay_software(self, delay: u8) -> Self {
        Self((self.0 & !Self::SAMP_DL_SW) | ((delay as u32) << 0))
    }
    /// Get sample delay software.
    #[inline]
    pub const fn sample_delay_software(self) -> u8 {
        ((self.0 & Self::SAMP_DL_SW) >> 0) as u8
    }
    /// Enable sample delay software.
    #[inline]
    pub const fn enable_sample_delay_software(self) -> Self {
        Self(self.0 | Self::SAMP_DL_SW_EN)
    }
    /// Disable sample delay software.
    #[inline]
    pub const fn disable_sample_delay_software(self) -> Self {
        Self(self.0 & !Self::SAMP_DL_SW_EN)
    }
    /// Get if sample delay software is enabled.
    #[inline]
    pub const fn is_sample_delay_software_enabled(self) -> bool {
        (self.0 & Self::SAMP_DL_SW_EN) != 0
    }
}

/// Managed SMHC structure with peripheral and pins.
pub struct Smhc<SMHC, PADS> {
    smhc: SMHC,
    pads: PADS,
}

impl<SMHC: AsRef<RegisterBlock>, PADS> Smhc<SMHC, PADS> {
    /// Create an SMHC instance.
    #[inline]
    pub fn new<const SMHC_IDX: usize>(
        smhc: SMHC,
        pads: PADS,
        clocks: &Clocks,
        ccu: &ccu::RegisterBlock,
    ) -> Self {
        let divider = 2;
        let (factor_n, factor_m) =
            ccu::calculate_best_peripheral_factors_nm(clocks.psi.0, 20_000_000);
        unsafe {
            smhc.as_ref()
                .clock_control
                .modify(|val| val.disable_card_clock());
        }
        unsafe {
            ccu.smhc_bgr.modify(|val| val.assert_reset::<SMHC_IDX>());
            ccu.smhc_bgr.modify(|val| val.gate_mask::<SMHC_IDX>());
            ccu.smhc_clk[SMHC_IDX].modify(|val| {
                val.set_clock_source(SmhcClockSource::PllPeri1x)
                    .set_factor_n(factor_n)
                    .set_factor_m(factor_m)
                    .enable_clock_gating()
            });
            ccu.smhc_bgr.modify(|val| val.deassert_reset::<SMHC_IDX>());
            ccu.smhc_bgr.modify(|val| val.gate_pass::<SMHC_IDX>());
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.global_control.modify(|val| val.set_software_reset());
            while !smhc.global_control.read().is_software_reset_cleared() {
                core::hint::spin_loop();
            }
            smhc.global_control.modify(|val| val.set_fifo_reset());
            while !smhc.global_control.read().is_fifo_reset_cleared() {
                core::hint::spin_loop();
            }
            smhc.global_control.modify(|val| val.disable_interrupt());
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.command.modify(|val| {
                val.enable_wait_for_complete()
                    .enable_change_clock()
                    .set_command_start()
            });
            while !smhc.command.read().is_command_start_cleared() {
                core::hint::spin_loop();
            }
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.clock_control
                .modify(|val| val.set_card_clock_divider(divider - 1));
            smhc.sample_delay_control.modify(|val| {
                val.set_sample_delay_software(0)
                    .enable_sample_delay_software()
            });
            smhc.clock_control.modify(|val| val.enable_card_clock());
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.command.modify(|val| {
                val.enable_wait_for_complete()
                    .enable_change_clock()
                    .set_command_start()
            });
            while !smhc.command.read().is_command_start_cleared() {
                core::hint::spin_loop();
            }
        }
        unsafe {
            let smhc = smhc.as_ref();
            smhc.card_type
                .write(CardType::default().set_bus_width(BusWidth::OneBit));
            smhc.block_size
                .write(BlockSize::default().set_block_size(512)); // TODO
        }

        Self { smhc, pads }
    }
    /// Get a temporary borrow on the underlying GPIO pads.
    #[inline]
    pub fn pads<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut PADS) -> T,
    {
        f(&mut self.pads)
    }
    /// Close SMHC and release peripheral.
    #[inline]
    pub fn free(self, ccu: &ccu::RegisterBlock) -> (SMHC, PADS) {
        unsafe {
            const SMHC_IDX: usize = 0; // TODO
            ccu.smhc_bgr.modify(|val| val.assert_reset::<SMHC_IDX>());
            ccu.smhc_bgr.modify(|val| val.gate_mask::<SMHC_IDX>());
        }
        (self.smhc, self.pads)
    }
    /// Send a command to the card.
    #[inline]
    pub fn send_card_command(
        &self,
        cmd: u8,
        arg: u32,
        transfer_mode: TransferMode,
        response_mode: ResponseMode,
        crc_check: bool,
    ) {
        let (data_trans, trans_dir) = match transfer_mode {
            TransferMode::Disable => (false, TransferDirection::Read),
            TransferMode::Read => (true, TransferDirection::Read),
            TransferMode::Write => (true, TransferDirection::Write),
        };
        let (resp_recv, resp_size) = match response_mode {
            ResponseMode::Disable => (false, false),
            ResponseMode::Short => (true, false),
            ResponseMode::Long => (true, true),
        };
        let smhc = self.smhc.as_ref();
        if data_trans {
            unsafe {
                smhc.byte_count.modify(|w| w.set_byte_count(512)); // TODO
                smhc.global_control
                    .modify(|w| w.set_access_mode(AccessMode::Ahb));
            }
        }
        unsafe {
            smhc.argument.modify(|val| val.set_argument(arg));
            smhc.command.write({
                let mut val = Command::default()
                    .set_command_start()
                    .set_command_index(cmd)
                    .set_transfer_direction(trans_dir)
                    .enable_wait_for_complete()
                    .enable_auto_stop();
                if data_trans {
                    val = val.enable_data_transfer();
                }
                if crc_check {
                    val = val.enable_check_response_crc();
                }
                if resp_recv {
                    val = val.enable_response_receive();
                }
                if resp_size {
                    val = val.enable_long_response();
                }
                val
            });
        };
    }
    /// Read the response from the card.
    #[inline]
    pub fn read_response(&self) -> u128 {
        let smhc = self.smhc.as_ref();
        let mut response = 0u128;
        for i in 0..4 {
            response |= (smhc.responses[i].read() as u128) << (32 * i);
        }
        response
    }
    /// Read data from first-in-first-out buffer.
    #[inline]
    pub fn read_data(&self, buf: &mut [u8]) {
        let smhc = self.smhc.as_ref();
        for i in 0..buf.len() / 4 {
            while smhc.status.read().fifo_empty() {
                core::hint::spin_loop();
            }
            let data = smhc.fifo.read();
            buf[i * 4] = (data & 0xff) as u8;
            buf[i * 4 + 1] = ((data >> 8) & 0xff) as u8;
            buf[i * 4 + 2] = ((data >> 16) & 0xff) as u8;
            buf[i * 4 + 3] = ((data >> 24) & 0xff) as u8;
        }
    }
}

/// Transfer mode.
pub enum TransferMode {
    /// No data transfer.
    Disable,
    /// Read data.
    Read,
    /// Write data.
    Write,
}

/// Response mode.
pub enum ResponseMode {
    /// No response.
    Disable,
    /// Short response.
    Short,
    /// Long response.
    Long,
}

pub struct SdCard<S: AsRef<RegisterBlock>, P> {
    smhc: Smhc<S, P>,
    block_count: u32,
}

#[derive(Debug)]
pub enum SdCardError {
    Unknown,
    UnexpectedResponse(u8, u128),
}

impl<S: AsRef<RegisterBlock>, P> SdCard<S, P> {
    /// Create an SD card instance.
    #[inline]
    pub fn new(smhc: Smhc<S, P>) -> Result<Self, SdCardError> {
        /// Host supports high capacity
        const OCR_HCS: u32 = 0x40000000;
        /// Card has finished power up routine if bit is high
        const OCR_NBUSY: u32 = 0x80000000;
        /// Valid bits for voltage setting
        const OCR_VOLTAGE_MASK: u32 = 0x007FFF80;

        // CMD0(reset) -> CMD8(check voltage and sdcard version)
        // -> CMD55+ACMD41(init and read OCR)
        smhc.send_card_command(0, 0, TransferMode::Disable, ResponseMode::Disable, false);
        Self::sleep(100); // TODO: wait for interrupt instead of sleep
        smhc.send_card_command(8, 0x1AA, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);
        let data = smhc.read_response();
        if data != 0x1AA {
            return Err(SdCardError::UnexpectedResponse(8, data));
        }
        loop {
            smhc.send_card_command(55, 0, TransferMode::Disable, ResponseMode::Short, true);
            Self::sleep(100);
            smhc.send_card_command(
                41,
                OCR_VOLTAGE_MASK & 0x00ff8000 | OCR_HCS,
                TransferMode::Disable,
                ResponseMode::Short,
                false,
            );
            Self::sleep(100);
            let ocr = smhc.read_response() as u32;
            if (ocr & OCR_NBUSY) == OCR_NBUSY {
                break;
            }
        }

        // Send CMD2 to get CID.
        smhc.send_card_command(2, 0, TransferMode::Disable, ResponseMode::Long, true);
        Self::sleep(100);
        let _cid = smhc.read_response();

        // Send CMD3 to get RCA.
        smhc.send_card_command(3, 0, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);
        let rca = smhc.read_response() as u32;

        // Send CMD9 to get CSD.
        smhc.send_card_command(9, rca, TransferMode::Disable, ResponseMode::Long, true);
        Self::sleep(100);
        let csd_raw = smhc.read_response();
        let fixed_csd_raw = csd_raw >> 8; // FIXME: 8bit shift for long response, why?
        let (csd_structure, c_size) = Self::parse_csd_v2(fixed_csd_raw);
        if csd_structure != 1 {
            return Err(SdCardError::UnexpectedResponse(9, csd_raw));
        }

        // Send CMD7 to select card.
        smhc.send_card_command(7, rca, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);

        // Set 1 data len, CMD55 -> ACMD6.
        smhc.send_card_command(55, rca, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);
        smhc.send_card_command(6, 0, TransferMode::Disable, ResponseMode::Short, true);
        Self::sleep(100);

        Ok(SdCard {
            smhc,
            block_count: (c_size + 1) * 1024,
        })
    }
    /// Get the size of the SD card in kilobytes.
    #[inline]
    pub fn get_size_kb(&self) -> f64 {
        (self.block_count as f64) * (512 as f64) / 1024.0
    }
    /// Read a block from the SD card.
    #[inline]
    pub fn read_block(&self, block: &mut Block, block_idx: u32) {
        self.smhc
            .send_card_command(17, block_idx, TransferMode::Read, ResponseMode::Short, true);
        self.smhc.read_data(&mut block.contents);
    }
    /// Parse CSD register version 2.
    #[inline]
    fn parse_csd_v2(csd: u128) -> (u32, u32) {
        let csd_structure = (((csd >> (32 * 3)) & 0xC00000) >> 22) as u32;
        let c_size = (((csd >> 32) & 0x3FFFFF00) >> 8) as u32;
        (csd_structure, c_size)
    }
    /// Sleep for a number of cycles.
    #[inline]
    fn sleep(n: u32) {
        for _ in 0..n * 100_000 {
            unsafe { asm!("nop") }
        }
    }
}

impl<S: AsRef<RegisterBlock>, P> BlockDevice for SdCard<S, P> {
    type Error = core::convert::Infallible;
    #[inline]
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter_mut().enumerate() {
            self.read_block(block, start_block_idx.0 + i as u32);
        }
        Ok(())
    }
    #[inline]
    fn write(&self, _blocks: &[Block], _start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        todo!();
    }
    #[inline]
    fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
        Ok(embedded_sdmmc::BlockCount(self.block_count))
    }
}

/// Clock signal pad.
pub trait Clk {}

/// Command signal pad.
pub trait Cmd {}

/// Data input and output pad.
///
/// This is documented in the User Manual as `D[3:0]`.
pub trait Data<const I: usize> {}

#[cfg(test)]
mod tests {
    use super::{
        AccessMode, Argument, BlockSize, BurstSize, BusWidth, ByteCount, CardType, ClockControl,
        Command, DdcTimingPhase, DdrMode, DriveDelayControl, FifoWaterLevel, GlobalControl,
        Interrupt, InterruptMask, InterruptStateMasked, InterruptStateRaw, NewTimingSet,
        NtsTimingPhase, RegisterBlock, Status, TimeOut, TransferDirection,
    };
    use memoffset::offset_of;
    #[test]
    fn offset_smhc() {
        assert_eq!(offset_of!(RegisterBlock, global_control), 0x0);
        assert_eq!(offset_of!(RegisterBlock, clock_control), 0x4);
        assert_eq!(offset_of!(RegisterBlock, timeout), 0x08);
        assert_eq!(offset_of!(RegisterBlock, card_type), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, block_size), 0x10);
        assert_eq!(offset_of!(RegisterBlock, byte_count), 0x14);
        assert_eq!(offset_of!(RegisterBlock, command), 0x18);
        assert_eq!(offset_of!(RegisterBlock, argument), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, responses), 0x20);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask), 0x30);
        assert_eq!(offset_of!(RegisterBlock, interrupt_state_masked), 0x34);
        assert_eq!(offset_of!(RegisterBlock, interrupt_state_raw), 0x38);
        assert_eq!(offset_of!(RegisterBlock, status), 0x3C);
        assert_eq!(offset_of!(RegisterBlock, fifo_water_level), 0x40);
        assert_eq!(offset_of!(RegisterBlock, new_timing_set), 0x5C);
        assert_eq!(offset_of!(RegisterBlock, dma_control), 0x80);
        assert_eq!(offset_of!(RegisterBlock, dma_descriptor_base), 0x84);
        assert_eq!(offset_of!(RegisterBlock, dma_state), 0x88);
        assert_eq!(offset_of!(RegisterBlock, dma_interrupt_enable), 0x8C);
        assert_eq!(offset_of!(RegisterBlock, drive_delay_control), 0x140);
        assert_eq!(offset_of!(RegisterBlock, sample_delay_control), 0x144);
        assert_eq!(offset_of!(RegisterBlock, skew_control), 0x184);
        assert_eq!(offset_of!(RegisterBlock, fifo), 0x200);
    }

    #[test]
    fn struct_global_control_functions() {
        let mut val = GlobalControl(0x0);

        val = val.set_access_mode(AccessMode::Ahb);
        assert_eq!(val.access_mode(), AccessMode::Ahb);
        assert_eq!(val.0, 0x80000000);

        val = val.set_access_mode(AccessMode::Dma);
        assert_eq!(val.access_mode(), AccessMode::Dma);
        assert_eq!(val.0, 0x00000000);

        val = val.set_ddr_mode(DdrMode::Ddr);
        assert_eq!(val.ddr_mode(), DdrMode::Ddr);
        assert_eq!(val.0, 0x00000400);

        val = val.set_ddr_mode(DdrMode::Sdr);
        assert_eq!(val.ddr_mode(), DdrMode::Sdr);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_dma();
        assert!(val.is_dma_enabled());
        assert_eq!(val.0, 0x00000020);

        val = val.disable_dma();
        assert!(!val.is_dma_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_interrupt();
        assert!(val.is_interrupt_enabled());
        assert_eq!(val.0, 0x000000010);

        val = val.disable_interrupt();
        assert!(!val.is_interrupt_enabled());
        assert_eq!(val.0, 0x000000000);

        val = val.set_dma_reset();
        assert!(!val.is_dma_reset_cleared());
        assert_eq!(val.0, 0x00000004);

        val = GlobalControl(0x0);
        assert!(val.is_dma_reset_cleared());

        val = GlobalControl(0x0);
        val = val.set_fifo_reset();
        assert!(!val.is_fifo_reset_cleared());
        assert_eq!(val.0, 0x00000002);

        val = GlobalControl(0x0);
        assert!(val.is_fifo_reset_cleared());

        val = GlobalControl(0x0);
        val = val.set_software_reset();
        assert!(!val.is_software_reset_cleared());
        assert_eq!(val.0, 0x00000001);

        val = GlobalControl(0x0);
        assert!(val.is_software_reset_cleared());
    }

    #[test]
    fn struct_clock_control_functions() {
        let mut val = ClockControl(0x0);

        val = val.enable_mask_data0();
        assert!(val.is_mask_data0_enabled());
        assert_eq!(val.0, 0x80000000);

        val = val.disable_mask_data0();
        assert!(!val.is_mask_data0_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_card_clock();
        assert!(val.is_card_clock_enabled());
        assert_eq!(val.0, 0x00010000);

        val = val.disable_card_clock();
        assert!(!val.is_card_clock_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_card_clock_divider(0xFF);
        assert_eq!(val.card_clock_divider(), 0xFF);
        assert_eq!(val.0, 0x000000FF);
    }

    #[test]
    fn struct_timeout_functions() {
        let mut val = TimeOut(0x0);

        val = val.set_data_timeout_limit(0xFFFFFF);
        assert_eq!(val.data_timeout_limit(), 0xFFFFFF);
        assert_eq!(val.0, 0xFFFFFF00);
    }

    #[test]
    fn struct_bus_width_functions() {
        let mut val = CardType(0x0);

        val = val.set_bus_width(BusWidth::OneBit);
        assert_eq!(val.bus_width(), BusWidth::OneBit);
        assert_eq!(val.0, 0x00000000);

        val = val.set_bus_width(BusWidth::FourBit);
        assert_eq!(val.bus_width(), BusWidth::FourBit);
        assert_eq!(val.0, 0x00000001);

        val = val.set_bus_width(BusWidth::EightBit);
        assert_eq!(val.bus_width(), BusWidth::EightBit);
        assert_eq!(val.0, 0x00000002);
    }

    #[test]
    fn struct_block_size_functions() {
        let mut val = BlockSize(0x0);

        val = val.set_block_size(0xFFFF);
        assert_eq!(val.block_size(), 0xFFFF);
        assert_eq!(val.0, 0x0000FFFF);
    }

    #[test]
    fn struct_byte_count_functions() {
        let mut val = ByteCount(0x0);

        val = val.set_byte_count(0xFFFFFFFF);
        assert_eq!(val.byte_count(), 0xFFFFFFFF);
        assert_eq!(val.0, 0xFFFFFFFF);
    }

    #[test]
    fn struct_command_functions() {
        let mut val = Command(0x0);

        val = val.set_command_start();
        assert!(!val.is_command_start_cleared());
        assert_eq!(val.0, 0x80000000);

        val = Command(0x0);
        assert!(val.is_command_start_cleared());

        val = Command(0x0);
        val = val.enable_change_clock();
        assert!(val.is_change_clock_enabled());
        assert_eq!(val.0, 0x00200000);

        val = val.disable_change_clock();
        assert!(!val.is_change_clock_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_send_init_seq();
        assert!(val.is_send_init_seq_enabled());
        assert_eq!(val.0, 0x00008000);

        val = val.disable_send_init_seq();
        assert!(!val.is_send_init_seq_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_stop_abort();
        assert!(val.is_stop_abort_enabled());
        assert_eq!(val.0, 0x00004000);

        val = val.disable_stop_abort();
        assert!(!val.is_stop_abort_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_wait_for_complete();
        assert!(val.is_wait_for_complete_enabled());
        assert_eq!(val.0, 0x00002000);

        val = val.disable_wait_for_complete();
        assert!(!val.is_wait_for_complete_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_auto_stop();
        assert!(val.is_auto_stop_enabled());
        assert_eq!(val.0, 0x00001000);

        val = val.disable_auto_stop();
        assert!(!val.is_auto_stop_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_transfer_direction(TransferDirection::Write);
        assert_eq!(val.transfer_direction(), TransferDirection::Write);
        assert_eq!(val.0, 0x00000400);

        val = val.set_transfer_direction(TransferDirection::Read);
        assert_eq!(val.transfer_direction(), TransferDirection::Read);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_data_transfer();
        assert!(val.is_data_transfer_enabled());
        assert_eq!(val.0, 0x00000200);

        val = val.disable_data_transfer();
        assert!(!val.is_data_transfer_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_check_response_crc();
        assert!(val.is_check_response_crc_enabled());
        assert_eq!(val.0, 0x00000100);

        val = val.disable_check_response_crc();
        assert!(!val.is_check_response_crc_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_long_response();
        assert!(val.is_long_response_enabled());
        assert_eq!(val.0, 0x00000080);

        val = val.disable_long_response();
        assert!(!val.is_long_response_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_response_receive();
        assert!(val.is_response_receive_enabled());
        assert_eq!(val.0, 0x00000040);

        val = val.disable_response_receive();
        assert!(!val.is_response_receive_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_command_index(0x3F);
        assert_eq!(val.command_index(), 0x3F);
        assert_eq!(val.0, 0x0000003F);
    }

    #[test]
    fn struct_argument_functions() {
        let mut val = Argument(0x0);

        val = val.set_argument(0xFFFFFFFF);
        assert_eq!(val.argument(), 0xFFFFFFFF);
        assert_eq!(val.0, 0xFFFFFFFF);
    }

    #[test]
    fn struct_interrupt_mask_functions() {
        let mut val = InterruptMask(0x0);

        for i in 0..18 as u8 {
            let int_tmp = match i {
                0 => Interrupt::CardRemoved,
                1 => Interrupt::CardInserted,
                2 => Interrupt::Sdio,
                3 => Interrupt::DataEndBitError,
                4 => Interrupt::AutoCommandDone,
                5 => Interrupt::DataStartError,
                6 => Interrupt::CommandBusyAndIllegalWrite,
                7 => Interrupt::FifoUnderrunOrOverflow,
                8 => Interrupt::DataStarvationTimeout1V8SwitchDone,
                9 => Interrupt::DataTimeoutBootDataStart,
                10 => Interrupt::ResponseTimeoutBootAckReceived,
                11 => Interrupt::DataCrcError,
                12 => Interrupt::ResponseCrcError,
                13 => Interrupt::DataReceiveRequest,
                14 => Interrupt::DataTransmitRequest,
                15 => Interrupt::DataTransferComplete,
                16 => Interrupt::CommandComplete,
                17 => Interrupt::ResponseError,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0 => 0x80000000,
                1 => 0x40000000,
                2 => 0x00010000,
                3 => 0x00008000,
                4 => 0x00004000,
                5 => 0x00002000,
                6 => 0x00001000,
                7 => 0x00000800,
                8 => 0x00000400,
                9 => 0x00000200,
                10 => 0x00000100,
                11 => 0x00000080,
                12 => 0x00000040,
                13 => 0x00000020,
                14 => 0x00000010,
                15 => 0x00000008,
                16 => 0x00000004,
                17 => 0x00000002,
                _ => unreachable!(),
            };

            val = val.unmask_interrupt(int_tmp);
            assert!(val.is_interrupt_unmasked(int_tmp));
            assert_eq!(val.0, val_tmp);

            val = val.mask_interrupt(int_tmp);
            assert!(!val.is_interrupt_unmasked(int_tmp));
            assert_eq!(val.0, 0x00000000);
        }
    }

    #[test]
    fn struct_interrupt_state_masked_functions() {
        for i in 0..18 as u8 {
            let int_tmp = match i {
                0x0 => Interrupt::CardRemoved,
                1 => Interrupt::CardInserted,
                2 => Interrupt::Sdio,
                3 => Interrupt::DataEndBitError,
                4 => Interrupt::AutoCommandDone,
                5 => Interrupt::DataStartError,
                6 => Interrupt::CommandBusyAndIllegalWrite,
                7 => Interrupt::FifoUnderrunOrOverflow,
                8 => Interrupt::DataStarvationTimeout1V8SwitchDone,
                9 => Interrupt::DataTimeoutBootDataStart,
                10 => Interrupt::ResponseTimeoutBootAckReceived,
                11 => Interrupt::DataCrcError,
                12 => Interrupt::ResponseCrcError,
                13 => Interrupt::DataReceiveRequest,
                14 => Interrupt::DataTransmitRequest,
                15 => Interrupt::DataTransferComplete,
                16 => Interrupt::CommandComplete,
                17 => Interrupt::ResponseError,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0 => 0x80000000,
                1 => 0x40000000,
                2 => 0x00010000,
                3 => 0x00008000,
                4 => 0x00004000,
                5 => 0x00002000,
                6 => 0x00001000,
                7 => 0x00000800,
                8 => 0x00000400,
                9 => 0x00000200,
                10 => 0x00000100,
                11 => 0x00000080,
                12 => 0x00000040,
                13 => 0x00000020,
                14 => 0x00000010,
                15 => 0x00000008,
                16 => 0x00000004,
                17 => 0x00000002,
                _ => unreachable!(),
            };

            let val = InterruptStateMasked(val_tmp);
            assert!(val.has_interrupt(int_tmp));
        }
    }

    #[test]
    fn struct_interrupt_state_raw_functions() {
        for i in 0..18 as u8 {
            let int_tmp = match i {
                0 => Interrupt::CardRemoved,
                1 => Interrupt::CardInserted,
                2 => Interrupt::Sdio,
                3 => Interrupt::DataEndBitError,
                4 => Interrupt::AutoCommandDone,
                5 => Interrupt::DataStartError,
                6 => Interrupt::CommandBusyAndIllegalWrite,
                7 => Interrupt::FifoUnderrunOrOverflow,
                8 => Interrupt::DataStarvationTimeout1V8SwitchDone,
                9 => Interrupt::DataTimeoutBootDataStart,
                10 => Interrupt::ResponseTimeoutBootAckReceived,
                11 => Interrupt::DataCrcError,
                12 => Interrupt::ResponseCrcError,
                13 => Interrupt::DataReceiveRequest,
                14 => Interrupt::DataTransmitRequest,
                15 => Interrupt::DataTransferComplete,
                16 => Interrupt::CommandComplete,
                17 => Interrupt::ResponseError,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0 => 0x80000000,
                1 => 0x40000000,
                2 => 0x00010000,
                3 => 0x00008000,
                4 => 0x00004000,
                5 => 0x00002000,
                6 => 0x00001000,
                7 => 0x00000800,
                8 => 0x00000400,
                9 => 0x00000200,
                10 => 0x00000100,
                11 => 0x00000080,
                12 => 0x00000040,
                13 => 0x00000020,
                14 => 0x00000010,
                15 => 0x00000008,
                16 => 0x00000004,
                17 => 0x00000002,
                _ => unreachable!(),
            };

            let mut val = InterruptStateRaw(val_tmp);
            assert!(val.has_interrupt(int_tmp));

            val = InterruptStateRaw(0x0);
            val = val.clear_interrupt(int_tmp);
            assert!(val.has_interrupt(int_tmp));
        }
    }

    #[test]
    fn struct_status_functions() {
        let mut val = Status(0x03FE0000);
        assert_eq!(val.fifo_level(), 0x1FF);

        val = Status(0x00000200);
        assert!(val.card_busy());

        val = Status(0x00000008);
        assert!(val.fifo_full());

        val = Status(0x00000004);
        assert!(val.fifo_empty());
    }

    #[test]
    fn struct_fifo_water_level_functions() {
        let mut val = FifoWaterLevel(0x0);

        for i in 0..4 as u8 {
            let bs_tmp = match i {
                0x0 => BurstSize::OneBit,
                0x1 => BurstSize::FourBit,
                0x2 => BurstSize::EightBit,
                0x3 => BurstSize::SixteenBit,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0x0 => 0x00000000,
                0x1 => 0x10000000,
                0x2 => 0x20000000,
                0x3 => 0x30000000,
                _ => unreachable!(),
            };

            val = val.set_burst_size(bs_tmp);
            assert_eq!(val.burst_size(), bs_tmp);
            assert_eq!(val.0, val_tmp);
        }

        val = FifoWaterLevel(0x0);
        val = val.set_receive_trigger_level(0xFF);
        assert_eq!(val.receive_trigger_level(), 0xFF);
        assert_eq!(val.0, 0x00FF0000);

        val = FifoWaterLevel(0x0);
        val = val.set_transmit_trigger_level(0xFF);
        assert_eq!(val.transmit_trigger_level(), 0xFF);
        assert_eq!(val.0, 0x000000FF);
    }

    #[test]
    fn struct_new_timing_set_functions() {
        let mut val = NewTimingSet(0x0);

        val = val.enable_new_mode();
        assert!(val.is_new_mode_enabled());
        assert_eq!(val.0, 0x80000000);

        val = val.disable_new_mode();
        assert!(!val.is_new_mode_enabled());
        assert_eq!(val.0, 0x00000000);

        for i in 0..4 as u8 {
            let tp_tmp = match i {
                0x0 => NtsTimingPhase::Offset90,
                0x1 => NtsTimingPhase::Offset180,
                0x2 => NtsTimingPhase::Offset270,
                0x3 => NtsTimingPhase::Offset0,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0x0 => 0x00000000,
                0x1 => 0x00000100,
                0x2 => 0x00000200,
                0x3 => 0x00000300,
                _ => unreachable!(),
            };

            val = val.set_sample_timing_phase(tp_tmp);
            assert_eq!(val.sample_timing_phase(), tp_tmp);
            assert_eq!(val.0, val_tmp);
        }
    }

    #[test]
    fn struct_drive_delay_control_functions() {
        let mut val = DriveDelayControl(0x0);

        val = val.set_data_drive_phase(DdcTimingPhase::Sdr180Ddr90);
        assert_eq!(val.data_drive_phase(), DdcTimingPhase::Sdr180Ddr90);
        assert_eq!(val.0, 0x00020000);

        val = val.set_data_drive_phase(DdcTimingPhase::Sdr90Ddr45);
        assert_eq!(val.data_drive_phase(), DdcTimingPhase::Sdr90Ddr45);
        assert_eq!(val.0, 0x00000000);

        val = val.set_command_drive_phase(DdcTimingPhase::Sdr180Ddr90);
        assert_eq!(val.command_drive_phase(), DdcTimingPhase::Sdr180Ddr90);
        assert_eq!(val.0, 0x00010000);

        val = val.set_command_drive_phase(DdcTimingPhase::Sdr90Ddr45);
        assert_eq!(val.command_drive_phase(), DdcTimingPhase::Sdr90Ddr45);
        assert_eq!(val.0, 0x00000000);
    }
}
