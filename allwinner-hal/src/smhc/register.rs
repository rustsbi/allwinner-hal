use volatile_register::{RO, RW};

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
    pub byte_count: RW<u32>,
    /// 0x18 - SMC Command Register.
    pub command: RW<Command>,
    /// 0x1C - SMC Argument Register.
    pub argument: RW<u32>,
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
    /// 0x44 - SMC FIFO Function Select Register.
    pub fifo_function: RW<FifoFunction>,
    /// 0x48 - SMC Transferred Byte Count Between Controller And Card.
    /// This register should be accessed in full to avoid read-coherency problems,
    ///  and read only after the data transfer completes.
    pub transferred_byte_count0: RO<u32>,
    /// 0x4C - SMC Transferred Byte Count Between Host And FIFO.
    /// This register should be accessed in full to avoid read-coherency problems,
    ///  and read only after the data transfer completes.
    pub transferred_byte_count1: RO<u32>,
    /// 0x50 - SMC Debug Control Register.
    pub debug_control: RW<DebugControl>,
    /// 0x54 - SMC CRC Status Detect Control Register.
    pub crc_status_detect: RW<CrcStatusDetect>,
    /// 0x58 - SMC Auto Command 12 Argument Register.
    pub auto_cmd12_arg: RW<AutoCmd12Arg>,
    /// 0x5C - SMC New Timing Set Register.
    pub new_timing_set: RW<NewTimingSet>,
    _reserved0: [u8; 24],
    /// 0x78 - SMC Hardware Reset Register.
    pub hardware_reset: RW<HardWareReset>,
    _reserved1: [u32; 1],
    /// 0x80 - SMC IDMAC Control Register.
    pub dma_control: RW<DmaControl>,
    /// 0x84 - SMC IDMAC Descriptor List Base Address Register.
    pub dma_descriptor_base: RW<u32>,
    /// 0x88 - SMC IDMAC Status Register.
    pub dma_state: RW<DmaState>,
    /// 0x8C - SMC IDMAC Interrupt Enable Register.
    pub dma_interrupt_enable: RW<DmaInterruptEnable>,
    _reserved2: [u8; 110],
    /// 0x100 - SMC Card Threshold Control Register.
    pub card_threshold_control: RW<CardThresholdControl>,
    /// 0x104 - SMC Sample Fifo Control Register.
    pub sample_fifo_control: RW<SampleFifoControl>,
    /// 0x108 - SMC Auto Command 23 Argument Register.
    pub auto_cmd23_arg: RW<u32>,
    /// 0x10c - SMC eMMC4.5 DDR Start Bit Detection Control Register.
    pub ddr_start_bit_detection: RW<DdrStartBitDetectionControl>,
    _reserved3: [u32; 10],
    /// 0x138 - SMC Extended Command Register.
    pub extended_command: RW<ExtendedCommand>,
    /// 0x13c - SMC Extended Response Register (for auto cmd 23).
    pub extended_response: RW<u32>,
    /// 0x140 - SMC Drive Delay Control Register.
    pub drive_delay_control: RW<DriveDelayControl>,
    /// 0x144 - SMC Sample Delay Control Register.
    pub sample_delay_control: RW<SampleDelayControl>,
    /// 0x148 - SMC Data Strobe Delay Control Register.
    pub data_strobe_delay_control: RW<DataStrobeDelayControl>,
    /// 0x14c - SMC HS400 Delay Control Register.
    pub hs400_delay_control: RW<Hs400DelayControl>,
    _reserved4: [u32; 13],
    /// 0x184 - SMC Deskew Control register.
    pub skew_control: RW<u32>,
    _reserved5: [u32; 30],
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

/// Card clock time uint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    /// 1 card clock period.
    Clk1,
    /// 256 card clock period.
    Clk256,
}

impl GlobalControl {
    const FIFO_AC_MOD: u32 = 1 << 31;
    const TIME_UNIT_CMD: u32 = 1 << 12;
    const TIME_UNIT_DAT: u32 = 1 << 11;
    const DDR_MOD: u32 = 1 << 10;
    const CARD_DE_BOUNCE: u32 = 1 << 8;
    const DMA_ENB: u32 = 1 << 5;
    const INT_ENB: u32 = 1 << 4;
    const DMA_RST: u32 = 1 << 2;
    const FIFO_RST: u32 = 1 << 1;
    const SOFT_RST: u32 = 1 << 0;

    /// Get fifo access mode.
    #[inline]
    pub const fn access_mode(self) -> AccessMode {
        match (self.0 & Self::FIFO_AC_MOD) >> 31 {
            0 => AccessMode::Dma,
            1 => AccessMode::Ahb,
            _ => unreachable!(),
        }
    }
    /// Set fifo access mode.
    #[inline]
    pub const fn set_access_mode(self, mode: AccessMode) -> Self {
        let mode = match mode {
            AccessMode::Dma => 0x0,
            AccessMode::Ahb => 0x1,
        };
        Self((self.0 & !Self::FIFO_AC_MOD) | (mode << 31))
    }
    /// Set time unit for command.
    #[inline]
    pub const fn set_time_unit_cmd(self, unit: TimeUnit) -> Self {
        Self((self.0 & !Self::TIME_UNIT_CMD) | (Self::TIME_UNIT_CMD & ((unit as u32) << 12)))
    }
    /// Get time unit for command.
    #[inline]
    pub const fn time_unit_cmd(self) -> TimeUnit {
        match (self.0 & Self::TIME_UNIT_CMD) >> 12 {
            0x0 => TimeUnit::Clk1,
            0x1 => TimeUnit::Clk256,
            _ => unreachable!(),
        }
    }
    /// Set time unit for data.
    #[inline]
    pub const fn set_time_unit_data(self, unit: TimeUnit) -> Self {
        Self((self.0 & !Self::TIME_UNIT_DAT) | (Self::TIME_UNIT_DAT & ((unit as u32) << 11)))
    }
    /// Get time unit for data.
    #[inline]
    pub const fn time_unit_data(self) -> TimeUnit {
        match (self.0 & Self::TIME_UNIT_DAT) >> 11 {
            0x0 => TimeUnit::Clk1,
            0x1 => TimeUnit::Clk256,
            _ => unreachable!(),
        }
    }
    /// Enable card de-bounce.
    #[inline]
    pub const fn enable_card_debounce(self) -> Self {
        Self(self.0 | Self::CARD_DE_BOUNCE)
    }
    /// Disable card de-bounce.
    #[inline]
    pub const fn disable_card_debounce(self) -> Self {
        Self(self.0 & !Self::CARD_DE_BOUNCE)
    }
    /// Check if card de-bounce is enabled.
    #[inline]
    pub const fn is_card_debounce_enabled(self) -> bool {
        self.0 & Self::CARD_DE_BOUNCE != 0
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

/// Card clock mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardClockMode {
    /// Always on.
    AlwaysOn,
    /// Turn off card clock when FSM is in IDLE state.
    TurnOffConditionally,
}

/// Clock control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClockControl(u32);

impl ClockControl {
    const MASK_DATA0: u32 = 1 << 31;
    const CCLK_CTRL: u32 = 1 << 17;
    const CCLK_ENB: u32 = 1 << 16;
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
    /// Set card clock mode.
    #[inline]
    pub const fn set_card_clock_mode(self, mode: CardClockMode) -> Self {
        Self((self.0 & !Self::CCLK_CTRL) | (Self::CCLK_CTRL & ((mode as u32) << 17)))
    }
    /// Get card clock mode.
    #[inline]
    pub const fn card_clock_mode(self) -> CardClockMode {
        match (self.0 & Self::CCLK_CTRL) >> 17 {
            0x0 => CardClockMode::AlwaysOn,
            0x1 => CardClockMode::TurnOffConditionally,
            _ => unreachable!(),
        }
    }
    /// If card clock is enabled.
    pub const fn is_card_clock_enabled(self) -> bool {
        self.0 & Self::CCLK_ENB != 0
    }
    /// Enable card clock.
    #[inline]
    pub const fn enable_card_clock(self) -> Self {
        Self(self.0 | Self::CCLK_ENB)
    }
    /// Disable card clock.
    #[inline]
    pub const fn disable_card_clock(self) -> Self {
        Self(self.0 & !Self::CCLK_ENB)
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
    const RESP_LMT: u32 = 0xFF;

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
    /// Set response timeout limit.
    #[inline]
    pub const fn set_response_timeout_limit(self, limit: u8) -> Self {
        Self((self.0 & !Self::RESP_LMT) | (Self::RESP_LMT & (limit as u32)))
    }
    /// Get response timeout limit.
    #[inline]
    pub const fn response_timeout_limit(self) -> u8 {
        (self.0 & Self::RESP_LMT) as u8
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
        Self((self.0 & !Self::BLK_SZ) | (Self::BLK_SZ & (size as u32)))
    }
}

impl Default for BlockSize {
    #[inline]
    fn default() -> Self {
        Self(0x0000_0200)
    }
}

/// Voltage switch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VoltageSwitch {
    /// Normal command.
    Normal,
    /// Voltage switch command, set for CMD11 only.
    Switch,
}

/// Transfer direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferDirection {
    /// Read from card.
    Read,
    /// Write to card.
    Write,
}

/// Boot mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BootMode {
    /// Normal command.
    NormalCmd,
    /// Mandatory boot operation.
    MandatoryBoot,
    /// Alternative boot operation.
    AlternativeBoot,
}

/// Transfer mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferMode {
    /// Block data transfer.
    Block,
    /// Stream data transfer.
    Stream,
}

/// Command register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Command(u32);

impl Command {
    const CMD_LOAD: u32 = 0x1 << 31;
    const VOL_SW: u32 = 0x1 << 28;
    const BOOT_ABT: u32 = 0x1 << 27;
    const EXP_BOOT_ACK: u32 = 0x1 << 26;
    const BOOT_MOD: u32 = 0x3 << 24;
    const PRG_CLK: u32 = 0x1 << 21;
    const SEND_INIT_SEQ: u32 = 0x1 << 15;
    const STOP_ABT_CMD: u32 = 0x1 << 14;
    const WAIT_PRE_OVER: u32 = 0x1 << 13;
    const STOP_CMD_FLAG: u32 = 0x1 << 12;
    const TRANS_MOD: u32 = 0x1 << 11;
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
    /// Set voltage switch mode.
    #[inline]
    pub const fn set_voltage_switch(self, mode: VoltageSwitch) -> Self {
        Self((self.0 & !Self::VOL_SW) | (Self::VOL_SW & ((mode as u32) << 28)))
    }
    /// Get voltage switch mode.
    #[inline]
    pub const fn voltage_switch(self) -> VoltageSwitch {
        match (self.0 & Self::VOL_SW) >> 28 {
            0x0 => VoltageSwitch::Normal,
            0x1 => VoltageSwitch::Switch,
            _ => unreachable!(),
        }
    }
    /// Abort boot operation.
    #[inline]
    pub const fn abort_boot(self) -> Self {
        Self(self.0 | Self::BOOT_ABT)
    }
    /// Check if boot operation is aborted.
    #[inline]
    pub const fn is_boot_aborted(self) -> bool {
        (self.0 & Self::BOOT_ABT) != 0
    }
    /// Enable boot ack expected.
    #[inline]
    pub const fn enable_boot_ack_expected(self) -> Self {
        Self(self.0 | Self::EXP_BOOT_ACK)
    }
    /// Disable boot ack expected.
    #[inline]
    pub const fn disable_boot_ack_expected(self) -> Self {
        Self(self.0 & !Self::EXP_BOOT_ACK)
    }
    /// Check if boot ack is received.
    #[inline]
    pub const fn is_boot_ack_received(self) -> bool {
        (self.0 & Self::EXP_BOOT_ACK) != 0
    }
    /// Set boot mode.
    #[inline]
    pub const fn set_boot_mode(self, mode: BootMode) -> Self {
        Self((self.0 & !Self::BOOT_MOD) | ((mode as u32) << 24))
    }
    /// Get boot mode.
    #[inline]
    pub const fn boot_mode(self) -> BootMode {
        match (self.0 & Self::BOOT_MOD) >> 24 {
            0x0 => BootMode::NormalCmd,
            0x1 => BootMode::MandatoryBoot,
            0x2 => BootMode::AlternativeBoot,
            _ => unreachable!(),
        }
    }
    /// Enable change card clock command.
    #[inline]
    pub const fn enable_change_card_clock(self) -> Self {
        Self(self.0 | Self::PRG_CLK)
    }
    /// Disable change card clock command.
    #[inline]
    pub const fn disable_change_card_clock(self) -> Self {
        Self(self.0 & !Self::PRG_CLK)
    }
    /// Check if change card clock command is enabled.
    #[inline]
    pub const fn is_change_card_clock_enabled(self) -> bool {
        (self.0 & Self::PRG_CLK) != 0
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
    /// If auto stop (cmd12) is enabled.
    #[inline]
    pub const fn is_auto_stop_enabled(self) -> bool {
        (self.0 & Self::STOP_CMD_FLAG) != 0
    }
    /// Enable auto stop (cmd12).
    #[inline]
    pub const fn enable_auto_stop(self) -> Self {
        Self(self.0 | Self::STOP_CMD_FLAG)
    }
    /// Disable auto stop (cmd12).
    #[inline]
    pub const fn disable_auto_stop(self) -> Self {
        Self(self.0 & !Self::STOP_CMD_FLAG)
    }
    /// Set transfer data mode.
    #[inline]
    pub const fn set_transfer_mode(self, mode: TransferMode) -> Self {
        Self((self.0 & !Self::TRANS_MOD) | ((mode as u32) << 11))
    }
    /// Get transfer data mode.
    #[inline]
    pub const fn transfer_mode(self) -> TransferMode {
        match (self.0 & Self::TRANS_MOD) >> 11 {
            0 => TransferMode::Block,
            1 => TransferMode::Stream,
            _ => unreachable!(),
        }
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

/// Interrupt mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptMask(u32);

/// Interrupt type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Interrupt {
    /// Card removed.
    CardRemoved,
    /// Card inserted.
    CardInserted,
    /// Sdio interrupt.
    Sdio,
    /// Data end bit error.
    DataEndBitError,
    /// Auto command done.
    AutoCommandDone,
    /// Data start error.
    DataStartError,
    /// Command busy and illegal write.
    CommandBusyAndIllegalWrite,
    /// Fifo underrun or overflow.
    FifoUnderrunOrOverflow,
    /// Data starvation timeout or 1.8V switch done.
    DataStarvationTimeout1V8SwitchDone,
    /// Data timeout or boot data start.
    DataTimeoutBootDataStart,
    /// Response timeout or boot ack received.
    ResponseTimeoutBootAckReceived,
    /// Data CRC error.
    DataCrcError,
    /// Response CRC error.
    ResponseCrcError,
    /// Data receive request.
    DataReceiveRequest,
    /// Data transmit request.
    DataTransmitRequest,
    /// Data transfer complete.
    DataTransferComplete,
    /// Command complete.
    CommandComplete,
    /// Response error.
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

    #[inline]
    pub const fn clear_all_interrupt(self) -> Self {
        Self(0)
    }
}

/// Command FSM states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CmdFsmState {
    /// Idle.
    Idle,
    /// Send init sequence.
    SendInitSeq,
    /// Tx command start bit.
    TxCmdStartBit,
    /// TX command tx bit.
    TxCmdTxBit,
    /// Tx command index + arggument.
    TxCmdIdxArg,
    /// Tx command CRC7.
    TxCmdCrc7,
    /// Tx command end bit.
    TxCmdEndBit,
    /// Rx response start bit.
    RxRespStartBit,
    /// Rx response irq response.
    RxRespIrqResp,
    /// Rx response tx bit.
    RxRespTxBit,
    /// Rx response command index.
    RxRespCmdIdx,
    /// Rx response data.
    RxRespData,
    /// Rx response CRC7.
    RxRespCrc7,
    /// Rx response end bit.
    RxRespEndBit,
    /// Command path wait NCC.
    WaitNcc,
    /// Wait; CMD-to-response turn around.
    WaitCmdToResp,
}

/// State register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
// note: read-only register, no write functions
pub struct Status(u32);

impl Status {
    const DMA_REQ: u32 = 1 << 31;
    const FIFO_LEVEL: u32 = 0x1FF << 17;
    const RESP_IDX: u32 = 0x3F << 11;
    const FSM_BUSY: u32 = 1 << 10;
    const CARD_BUSY: u32 = 1 << 9;
    const CARD_PRESENT: u32 = 1 << 8;
    const FSM_STA: u32 = 0xF << 4;
    const FIFO_FULL: u32 = 1 << 3;
    const FIFO_EMPTY: u32 = 1 << 2;
    const FIFO_TX_LEVEL: u32 = 1 << 1;
    const FIFO_RX_LEVEL: u32 = 1 << 0;

    /// Check if dma request occurs.
    #[inline]
    pub const fn if_dma_request_occurs(self) -> bool {
        self.0 & Self::DMA_REQ != 0
    }
    /// Get FIFO level.
    #[inline]
    pub const fn fifo_level(self) -> u16 {
        ((self.0 & Self::FIFO_LEVEL) >> 17) as u16
    }
    /// Get previous response index.
    #[inline]
    pub const fn response_index(self) -> u8 {
        ((self.0 & Self::RESP_IDX) >> 11) as u8
    }
    /// Check if the FSM (data transfer state machine) is busy.
    #[inline]
    pub const fn fsm_busy(self) -> bool {
        self.0 & Self::FSM_BUSY != 0
    }
    /// Is the card busy?
    #[inline]
    pub const fn card_busy(self) -> bool {
        self.0 & Self::CARD_BUSY != 0
    }
    /// Check if card is present.
    #[inline]
    pub const fn card_present(self) -> bool {
        self.0 & Self::CARD_PRESENT != 0
    }
    /// Get the current FSM state.
    #[inline]
    pub const fn fsm_state(self) -> CmdFsmState {
        match (self.0 & Self::FSM_STA) >> 4 {
            0 => CmdFsmState::Idle,
            1 => CmdFsmState::SendInitSeq,
            2 => CmdFsmState::TxCmdStartBit,
            3 => CmdFsmState::TxCmdTxBit,
            4 => CmdFsmState::TxCmdIdxArg,
            5 => CmdFsmState::TxCmdCrc7,
            6 => CmdFsmState::TxCmdEndBit,
            7 => CmdFsmState::RxRespStartBit,
            8 => CmdFsmState::RxRespIrqResp,
            9 => CmdFsmState::RxRespTxBit,
            10 => CmdFsmState::RxRespCmdIdx,
            11 => CmdFsmState::RxRespData,
            12 => CmdFsmState::RxRespCrc7,
            13 => CmdFsmState::RxRespEndBit,
            14 => CmdFsmState::WaitNcc,
            15 => CmdFsmState::WaitCmdToResp,
            _ => unreachable!(),
        }
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
    /// Check if the FIFO reaches the transmit level.
    #[inline]
    pub const fn fifo_tx_level(self) -> bool {
        self.0 & Self::FIFO_TX_LEVEL != 0
    }
    /// Check if the FIFO reaches the receive level.
    #[inline]
    pub const fn fifo_rx_level(self) -> bool {
        self.0 & Self::FIFO_RX_LEVEL != 0
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

/// FIFO function select register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoFunction(u32);

impl FifoFunction {
    const ABT_RDATA: u32 = 1 << 2;
    const READ_WAIT: u32 = 1 << 1;
    const HOST_IRQRESQ: u32 = 1;

    /// Abort read data.
    #[inline]
    pub const fn abort_read_data(self) -> Self {
        Self(self.0 | Self::ABT_RDATA)
    }
    /// Clear abort read data.
    #[inline]
    pub const fn clear_abort_read_data(self) -> Self {
        Self(self.0 & !Self::ABT_RDATA)
    }
    /// Check if abort read data is set.
    #[inline]
    pub const fn is_abort_read_data_set(self) -> bool {
        (self.0 & Self::ABT_RDATA) != 0
    }
    /// Enable read wait.
    #[inline]
    pub const fn enable_read_wait(self) -> Self {
        Self(self.0 | Self::READ_WAIT)
    }
    /// Disable read wait.
    #[inline]
    pub const fn disable_read_wait(self) -> Self {
        Self(self.0 & !Self::READ_WAIT)
    }
    /// Check if read wait is enabled.
    #[inline]
    pub const fn is_read_wait_enabled(self) -> bool {
        (self.0 & Self::READ_WAIT) != 0
    }
    /// Enable host send mmc irq request.
    #[inline]
    pub const fn enable_host_irq_request(self) -> Self {
        Self(self.0 | Self::HOST_IRQRESQ)
    }
    /// Disable host send mmc irq request.
    #[inline]
    pub const fn disable_host_irq_request(self) -> Self {
        Self(self.0 & !Self::HOST_IRQRESQ)
    }
    /// Check if host send mmc irq request is enabled.
    #[inline]
    pub const fn is_host_irq_request_enabled(self) -> bool {
        (self.0 & Self::HOST_IRQRESQ) != 0
    }
}

/// Debug control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DebugControl(u32);

impl DebugControl {}

/// Crc mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CrcMode {
    /// Other mode.
    Other = 3,
    /// Hs400 mode.
    Hs400 = 6,
}

/// Crc status detect control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CrcStatusDetect(u32);

impl CrcStatusDetect {
    const CRC_MODE: u32 = 0x7 << 28;

    /// Set the CRC mode.
    #[inline]
    pub const fn set_crc_mode(self, mode: CrcMode) -> Self {
        Self((self.0 & !Self::CRC_MODE) | ((mode as u32) << 28))
    }
    /// Get the current CRC mode.
    #[inline]
    pub const fn crc_mode(self) -> CrcMode {
        match (self.0 & Self::CRC_MODE) >> 28 {
            3 => CrcMode::Other,
            6 => CrcMode::Hs400,
            _ => unreachable!(),
        }
    }
}

/// Auto command 12 argument register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AutoCmd12Arg(u32);

impl AutoCmd12Arg {
    const ARG: u32 = 0xFFFF;

    /// Get the argument.
    #[inline]
    pub const fn argument(self) -> u32 {
        (self.0 & Self::ARG) >> 0
    }
    /// Set the argument.
    #[inline]
    pub const fn set_argument(self, arg: u16) -> Self {
        Self((self.0 & !Self::ARG) | (arg as u32))
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
    const CMD_DAT_RX_PHASE_CLR: u32 = 0x1 << 24;
    const DAT_CRC_STATUS_RX_PHASE_CLR: u32 = 0x1 << 22;
    const DAT_TRANS_RX_PHASE_CLR: u32 = 0x1 << 21;
    const DAT_RECV_RX_PHASE_CLR: u32 = 0x1 << 20;
    const CMD_SEND_RX_PHASE_CLR: u32 = 0x1 << 16;
    const DAT_SAMPLE_TIMING_PHASE: u32 = 0x3 << 8;
    const CMD_SAMPLE_TIMING_PHASE: u32 = 0x3 << 4;
    const HS400_NEW_SAMPLE_EN: u32 = 1;

    /// Check if new mode is enabled.
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
    /// Enable clear the input phase of cmd and dat lines during
    /// the update clock operation.
    #[inline]
    pub fn enable_rx_dat_cmd_clear(self) -> Self {
        Self(self.0 | Self::CMD_DAT_RX_PHASE_CLR)
    }
    /// Disable clear the input phase of cmd and dat lines during
    /// the update clock operation.
    #[inline]
    pub fn disable_rx_dat_cmd_clear(self) -> Self {
        Self(self.0 & !Self::CMD_DAT_RX_PHASE_CLR)
    }
    /// Check if clear the input phase of cmd and dat lines during
    /// the update clock operation is enabled.
    #[inline]
    pub const fn is_rx_dat_cmd_clear_enabled(self) -> bool {
        (self.0 & Self::CMD_DAT_RX_PHASE_CLR) != 0
    }
    /// Enable clear the input phase of dat before receiving
    /// CRC status.
    #[inline]
    pub fn enable_rx_dat_crc_status_clear(self) -> Self {
        Self(self.0 | Self::DAT_CRC_STATUS_RX_PHASE_CLR)
    }
    /// Disable clear the input phase of dat before receiving
    /// CRC status.
    #[inline]
    pub fn disable_rx_dat_crc_status_clear(self) -> Self {
        Self(self.0 & !Self::DAT_CRC_STATUS_RX_PHASE_CLR)
    }
    /// Check if clear the input phase of dat before receiving
    /// CRC status is enabled.
    #[inline]
    pub const fn is_rx_dat_crc_status_clear_enabled(self) -> bool {
        (self.0 & Self::DAT_CRC_STATUS_RX_PHASE_CLR) != 0
    }
    /// Enable clear the input phase of dat before transfering
    /// data.
    #[inline]
    pub fn enable_rx_dat_trans_clear(self) -> Self {
        Self(self.0 | Self::DAT_TRANS_RX_PHASE_CLR)
    }
    /// Disable clear the input phase of dat before transfering
    /// data.
    #[inline]
    pub fn disable_rx_dat_trans_clear(self) -> Self {
        Self(self.0 & !Self::DAT_TRANS_RX_PHASE_CLR)
    }
    /// Check if clear the input phase of dat before transfering
    /// data.
    #[inline]
    pub fn is_rx_dat_trans_clear_enabled(self) -> bool {
        (self.0 & Self::DAT_TRANS_RX_PHASE_CLR) != 0
    }
    /// Enable clear the input phase of dat before receiving
    /// data.
    #[inline]
    pub fn enable_rx_dat_recv_clear(self) -> Self {
        Self(self.0 | Self::DAT_RECV_RX_PHASE_CLR)
    }
    /// Disable clear the input phase of dat before receiving
    /// data.
    #[inline]
    pub fn disable_rx_dat_recv_clear(self) -> Self {
        Self(self.0 & !Self::DAT_RECV_RX_PHASE_CLR)
    }
    /// Check if clear the input phase of dat before receiving
    /// data is enabled.
    #[inline]
    pub fn is_rx_dat_recv_clear_enabled(self) -> bool {
        (self.0 & Self::DAT_RECV_RX_PHASE_CLR) != 0
    }
    /// Enable clear cmd rx phase before sending cmd.
    #[inline]
    pub fn enable_rx_cmd_send_clear(self) -> Self {
        Self(self.0 | Self::CMD_SEND_RX_PHASE_CLR)
    }
    /// Disable clear cmd rx phase before sending cmd.
    #[inline]
    pub fn disable_rx_cmd_send_clear(self) -> Self {
        Self(self.0 & !Self::CMD_SEND_RX_PHASE_CLR)
    }
    /// Check if clear cmd rx phase before sending cmd is enabled.
    #[inline]
    pub fn is_rx_cmd_send_clear_enabled(self) -> bool {
        (self.0 & Self::CMD_SEND_RX_PHASE_CLR) != 0
    }
    /// Get data sample timing phase.
    #[inline]
    pub const fn dat_sample_timing_phase(self) -> NtsTimingPhase {
        match (self.0 & Self::DAT_SAMPLE_TIMING_PHASE) >> 8 {
            0x0 => NtsTimingPhase::Offset90,
            0x1 => NtsTimingPhase::Offset180,
            0x2 => NtsTimingPhase::Offset270,
            0x3 => NtsTimingPhase::Offset0,
            _ => unreachable!(),
        }
    }
    /// Set data sample timing phase.
    #[inline]
    pub const fn set_dat_sample_timing_phase(self, phase: NtsTimingPhase) -> Self {
        Self((self.0 & !Self::DAT_SAMPLE_TIMING_PHASE) | ((phase as u32) << 8))
    }
    /// Get command sample timing phase (except offset0).
    #[inline]
    pub const fn cmd_sample_timing_phase(self) -> NtsTimingPhase {
        match (self.0 & Self::CMD_SAMPLE_TIMING_PHASE) >> 4 {
            0x0 => NtsTimingPhase::Offset90,
            0x1 => NtsTimingPhase::Offset180,
            0x2 => NtsTimingPhase::Offset270,
            _ => unreachable!(),
        }
    }
    /// Set command sample timing phase (except offset0).
    #[inline]
    pub const fn set_cmd_sample_timing_phase(self, phase: NtsTimingPhase) -> Self {
        Self((self.0 & !Self::CMD_SAMPLE_TIMING_PHASE) | ((phase as u32) << 4))
    }
    /// Enable hs400 new sample method.
    #[inline]
    pub const fn enable_hs400_new_sample(self) -> Self {
        Self(self.0 | Self::HS400_NEW_SAMPLE_EN)
    }
    /// Disable hs400 new sample method.
    #[inline]
    pub const fn disable_hs400_new_sample(self) -> Self {
        Self(self.0 & !Self::HS400_NEW_SAMPLE_EN)
    }
    /// Check if hs400 new sample method is enabled.
    #[inline]
    pub const fn is_hs400_new_sample_enabled(self) -> bool {
        (self.0 & Self::HS400_NEW_SAMPLE_EN) != 0
    }
}

/// Hardware reset register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HardWareReset(u32);

impl HardWareReset {
    const HW_RESET: u32 = 1;

    /// Reset the hardware.
    #[inline]
    pub const fn reset_hardware(self) -> Self {
        Self(self.0 | Self::HW_RESET)
    }
    /// Active the hardware.
    #[inline]
    pub const fn active_hardware(self) -> Self {
        Self(self.0 & !Self::HW_RESET)
    }
    /// Check if hardware is reset.
    #[inline]
    pub const fn is_hardware_reset_cleared(self) -> bool {
        (self.0 & Self::HW_RESET) == 0
    }
}

/// IDMAC control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DmaControl(u32);

impl DmaControl {
    const DES_LOAD_CTRL: u32 = 0x1 << 31;
    const IDMAC_ENB: u32 = 0x1 << 7;
    const FIX_BURST_CTRL: u32 = 0x1 << 1;
    const IDMAC_RST: u32 = 0x1;

    /// Enable dma descripter refetch.
    #[inline]
    pub const fn enable_dma_desc_refetch(self) -> Self {
        Self(self.0 | Self::DES_LOAD_CTRL)
    }
    /// Disable dma descripter refetch.
    #[inline]
    pub const fn disable_dma_desc_refetch(self) -> Self {
        Self(self.0 & !Self::DES_LOAD_CTRL)
    }
    /// Check if dma descripter refetch is enabled.
    #[inline]
    pub const fn is_dma_desc_refetch_enable(self) -> bool {
        (self.0 & Self::DES_LOAD_CTRL) != 0
    }
    /// Enable dma.
    #[inline]
    pub const fn enable_dma(self) -> Self {
        Self(self.0 | Self::IDMAC_ENB)
    }
    /// Disable dma.
    #[inline]
    pub const fn disable_dma(self) -> Self {
        Self(self.0 & !Self::IDMAC_ENB)
    }
    /// Check if dma is enabled.
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        (self.0 & Self::IDMAC_ENB) != 0
    }
    /// Enable fix burst size.
    #[inline]
    pub const fn enable_fix_burst_size(self) -> Self {
        Self(self.0 | Self::FIX_BURST_CTRL)
    }
    /// Disable fix burst size.
    #[inline]
    pub const fn disable_fix_burst_size(self) -> Self {
        Self(self.0 & !Self::FIX_BURST_CTRL)
    }
    /// Check if fix burst size is enabled.
    #[inline]
    pub const fn is_fix_burst_size_enabled(self) -> bool {
        (self.0 & Self::FIX_BURST_CTRL) != 0
    }
    /// Reset dma.
    #[inline]
    pub const fn reset_dma(self) -> Self {
        Self(self.0 | Self::IDMAC_RST)
    }
    /// Check if dma is reset.
    pub const fn is_dma_reset_cleared(self) -> bool {
        (self.0 & Self::IDMAC_RST) == 0
    }
}

/// Dma abort state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaAbortState {
    /// Host abort received during the transmission.
    Tx = 1,
    /// Host abort received during the reception.
    Rx = 2,
}

/// IDMAC state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DmaState(u32);

impl DmaState {
    const IDMAC_ERR_STA: u32 = 0x7 << 10;
    const ABN_INT_SUM: u32 = 0x1 << 9;
    const NOR_INT_SUM: u32 = 0x1 << 8;
    const ERR_FLAG_SUM: u32 = 0x1 << 5;
    const DES_UNAVL_INT: u32 = 0x1 << 4;
    const FATAL_BERR_INT: u32 = 0x1 << 2;
    const RX_INT: u32 = 0x1 << 1;
    const TX_INT: u32 = 0x1;

    /// Get dma error status.
    #[inline]
    pub const fn dma_error_status(self) -> DmaAbortState {
        match (self.0 & Self::IDMAC_ERR_STA) >> 10 {
            1 => DmaAbortState::Tx,
            2 => DmaAbortState::Rx,
            _ => unreachable!(),
        }
    }
    /// Check if abnormal interrupt summary occurs.
    #[inline]
    pub const fn abn_int_sum_occurs(self) -> bool {
        (self.0 & Self::ABN_INT_SUM) != 0
    }
    /// Clear abnormal interrupt summary.
    #[inline]
    pub const fn clear_abn_int_sum(self) -> Self {
        Self(self.0 | Self::ABN_INT_SUM)
    }
    /// Check if normal interrupt summary occurs.
    #[inline]
    pub const fn nor_int_sum_occurs(self) -> bool {
        (self.0 & Self::NOR_INT_SUM) != 0
    }
    /// Clear normal interrupt summary.
    #[inline]
    pub const fn clear_nor_int_sum(self) -> Self {
        Self(self.0 | Self::NOR_INT_SUM)
    }
    /// Check if card error summary occurs.
    #[inline]
    pub const fn card_err_sum_occurs(self) -> bool {
        (self.0 & Self::ERR_FLAG_SUM) != 0
    }
    /// Clear card error summary.
    #[inline]
    pub const fn clear_card_err_sum(self) -> Self {
        Self(self.0 | Self::ERR_FLAG_SUM)
    }
    /// Check if descriptor unavailable interrupt occurs.
    #[inline]
    pub const fn des_unavl_int_occurs(self) -> bool {
        (self.0 & Self::DES_UNAVL_INT) != 0
    }
    /// Clear descriptor unavailable interrupt.
    #[inline]
    pub const fn clear_des_unavl_int(self) -> Self {
        Self(self.0 | Self::DES_UNAVL_INT)
    }
    /// Check if fatal bus error interrupt occurs.
    #[inline]
    pub const fn fatal_berr_int_occurs(self) -> bool {
        (self.0 & Self::FATAL_BERR_INT) != 0
    }
    /// Clear fatal bus error interrupt.
    #[inline]
    pub const fn clear_fatal_berr_int(self) -> Self {
        Self(self.0 | Self::FATAL_BERR_INT)
    }
    /// Check if receive interrupt occurs.
    #[inline]
    pub const fn rx_int_occurs(self) -> bool {
        (self.0 & Self::RX_INT) != 0
    }
    /// Clear receive interrupt.
    #[inline]
    pub const fn clear_rx_int(self) -> Self {
        Self(self.0 | Self::RX_INT)
    }
    /// Check if transmit interrupt occurs.
    #[inline]
    pub const fn tx_int_occurs(self) -> bool {
        (self.0 & Self::TX_INT) != 0
    }
    /// Clear transmit interrupt.
    #[inline]
    pub const fn clear_tx_int(self) -> Self {
        Self(self.0 | Self::TX_INT)
    }
}

/// IDMAC interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DmaInterruptEnable(u32);

impl DmaInterruptEnable {
    const ERR_SUM_INT_ENB: u32 = 0x1 << 5;
    const DES_UNAVL_INT_ENB: u32 = 0x1 << 4;
    const FATAL_BERR_INT_ENB: u32 = 0x1 << 2;
    const RX_INT_ENB: u32 = 0x1 << 1;
    const TX_INT_ENB: u32 = 0x1;

    /// Enable card error summary interrupt.
    #[inline]
    pub const fn enable_card_err_sum_int(self) -> Self {
        Self(self.0 | Self::ERR_SUM_INT_ENB)
    }
    /// Disable card error summary interrupt.
    #[inline]
    pub const fn disable_card_err_sum_int(self) -> Self {
        Self(self.0 & !Self::ERR_SUM_INT_ENB)
    }
    /// Check if card error summary interrupt is enabled.
    #[inline]
    pub const fn is_card_err_sum_int_enabled(self) -> bool {
        (self.0 & Self::ERR_SUM_INT_ENB) != 0
    }
    /// Enable descriptor unavailable interrupt.
    #[inline]
    pub const fn enable_des_unavl_int(self) -> Self {
        Self(self.0 | Self::DES_UNAVL_INT_ENB)
    }
    /// Disable descriptor unavailable interrupt.
    #[inline]
    pub const fn disable_des_unavl_int(self) -> Self {
        Self(self.0 & !Self::DES_UNAVL_INT_ENB)
    }
    /// Check if descriptor unavailable interrupt is enabled.
    #[inline]
    pub const fn is_des_unavl_int_enabled(self) -> bool {
        (self.0 & Self::DES_UNAVL_INT_ENB) != 0
    }
    /// Enable fatal bus error interrupt.
    #[inline]
    pub const fn enable_fatal_berr_int(self) -> Self {
        Self(self.0 | Self::FATAL_BERR_INT_ENB)
    }
    /// Disable fatal bus error interrupt.
    #[inline]
    pub const fn disable_fatal_berr_int(self) -> Self {
        Self(self.0 & !Self::FATAL_BERR_INT_ENB)
    }
    /// Check if fatal bus error interrupt is enabled.
    #[inline]
    pub const fn is_fatal_berr_int_enabled(self) -> bool {
        (self.0 & Self::FATAL_BERR_INT_ENB) != 0
    }
    /// Enable receive interrupt.
    #[inline]
    pub const fn enable_rx_int(self) -> Self {
        Self(self.0 | Self::RX_INT_ENB)
    }
    /// Disable receive interrupt.
    #[inline]
    pub const fn disable_rx_int(self) -> Self {
        Self(self.0 & !Self::RX_INT_ENB)
    }
    /// Check if receive interrupt is enabled.
    #[inline]
    pub const fn is_rx_int_enabled(self) -> bool {
        (self.0 & Self::RX_INT_ENB) != 0
    }
    /// Enable transmit interrupt.
    #[inline]
    pub const fn enable_tx_int(self) -> Self {
        Self(self.0 | Self::TX_INT_ENB)
    }
    /// Disable transmit interrupt.
    #[inline]
    pub const fn disable_tx_int(self) -> Self {
        Self(self.0 & !Self::TX_INT_ENB)
    }
    /// Check if transmit interrupt is enabled.
    #[inline]
    pub const fn is_tx_int_enabled(self) -> bool {
        (self.0 & Self::TX_INT_ENB) != 0
    }
}

/// Card threshold control register..
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CardThresholdControl(u32);

impl CardThresholdControl {
    const CARD_WR_THLD_MASK: u32 = 0xFFF << 16;
    const CARD_WR_THLD_ENB: u32 = 1 << 2;
    const BCIG: u32 = 1 << 1;
    const CARD_RD_THLD_ENB: u32 = 1 << 0;

    /// Set card write/read threshold value.
    #[inline]
    pub const fn set_card_wr_thld(self, value: u16) -> Self {
        Self(
            (self.0 & !Self::CARD_WR_THLD_MASK)
                | (Self::CARD_WR_THLD_MASK & ((value as u32) << 16)),
        )
    }
    /// Get card write/read threshold value.
    #[inline]
    pub const fn card_wr_thld(self) -> u16 {
        ((self.0 & Self::CARD_WR_THLD_MASK) >> 16) as u16
    }
    /// Enable card write threshold.
    #[inline]
    pub const fn enable_card_write_threshold(self) -> Self {
        Self(self.0 | Self::CARD_WR_THLD_ENB)
    }
    /// Disable card write threshold.
    #[inline]
    pub const fn disable_card_write_threshold(self) -> Self {
        Self(self.0 & !Self::CARD_WR_THLD_ENB)
    }
    /// Check if card write threshold is enabled.
    #[inline]
    pub const fn is_card_write_threshold_enabled(self) -> bool {
        (self.0 & Self::CARD_WR_THLD_ENB) != 0
    }
    /// Enable busy clear interrupt generation.
    #[inline]
    pub const fn enable_busy_clear(self) -> Self {
        Self(self.0 | Self::BCIG)
    }
    /// Disable busy clear interrupt generation.
    #[inline]
    pub const fn disable_busy_clear(self) -> Self {
        Self(self.0 & !Self::BCIG)
    }
    /// Check if busy clear interrupt generation is enabled.
    #[inline]
    pub const fn is_busy_clear_enabled(self) -> bool {
        (self.0 & Self::BCIG) != 0
    }
    /// Enable card read threshold.
    #[inline]
    pub const fn enable_card_read_threshold(self) -> Self {
        Self(self.0 | Self::CARD_RD_THLD_ENB)
    }
    /// Disable card read threshold.
    #[inline]
    pub const fn disable_card_read_threshold(self) -> Self {
        Self(self.0 & !Self::CARD_RD_THLD_ENB)
    }
    /// Check if card read threshold is enabled.
    #[inline]
    pub const fn is_card_read_threshold_enabled(self) -> bool {
        (self.0 & Self::CARD_RD_THLD_ENB) != 0
    }
}

/// Sample fifo control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SampleFifoControl(u32);

impl SampleFifoControl {
    const STOP_CLK_CTRL_MASK: u32 = 0xF << 1;
    const BYPASS_EN: u32 = 1 << 0;

    /// Set stop clock control value.
    #[inline]
    pub const fn set_stop_clk_ctrl(self, value: u8) -> Self {
        Self(
            (self.0 & !Self::STOP_CLK_CTRL_MASK)
                | (Self::STOP_CLK_CTRL_MASK & ((value as u32) << 1)),
        )
    }
    /// Get stop clock control value.
    #[inline]
    pub const fn stop_clk_ctrl(self) -> u8 {
        ((self.0 & Self::STOP_CLK_CTRL_MASK) >> 1) as u8
    }
    /// Enable bypass (data not using FIFO).
    #[inline]
    pub const fn enable_bypass(self) -> Self {
        Self(self.0 | Self::BYPASS_EN)
    }
    /// Disable bypass (data goes through FIFO).
    #[inline]
    pub const fn disable_bypass(self) -> Self {
        Self(self.0 & !Self::BYPASS_EN)
    }
    /// Check if bypass is enabled.
    #[inline]
    pub const fn is_bypass_enabled(self) -> bool {
        (self.0 & Self::BYPASS_EN) != 0
    }
}

/// Ddr start bit detection control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DdrStartBitDetectionControl(u32);

impl DdrStartBitDetectionControl {
    const HS400_MD_EN: u32 = 0x1 << 31;
    const HALF_START_BIT: u32 = 0x1;

    /// Enable HS400 mode.
    #[inline]
    pub const fn enable_hs400_mode(self) -> Self {
        Self((self.0 & !Self::HS400_MD_EN) | (Self::HS400_MD_EN & 0xFFFF_FFFF))
    }
    /// Disable HS400 mode.
    #[inline]
    pub const fn disable_hs400_mode(self) -> Self {
        Self(self.0 & !Self::HS400_MD_EN)
    }
    /// Check if HS400 mode is enabled.
    #[inline]
    pub const fn is_hs400_mode_enabled(self) -> bool {
        (self.0 & Self::HS400_MD_EN) != 0
    }
    /// Set start bit detection to less than one full cycle.
    #[inline]
    pub const fn set_half_start_bit_less(self) -> Self {
        Self(self.0 | Self::HALF_START_BIT)
    }
    /// Set start bit detection to full cycle.
    #[inline]
    pub const fn set_half_start_bit_full(self) -> Self {
        Self(self.0 & !Self::HALF_START_BIT)
    }
    /// Check if start bit detection is less than one full cycle.
    #[inline]
    pub const fn is_half_start_bit_less(self) -> bool {
        (self.0 & Self::HALF_START_BIT) != 0
    }
    /// Check if start bit detection is full cycle.
    #[inline]
    pub const fn is_half_start_bit_full(self) -> bool {
        (self.0 & Self::HALF_START_BIT) == 0
    }
}

/// Extended command register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ExtendedCommand(u32);

impl ExtendedCommand {
    const AUTO_CMD23_EN: u32 = 0x1;

    /// Enable Auto CMD23.
    #[inline]
    pub const fn enable_auto_cmd23(self) -> Self {
        Self(self.0 | Self::AUTO_CMD23_EN)
    }
    /// Disable Auto CMD23.
    #[inline]
    pub const fn disable_auto_cmd23(self) -> Self {
        Self(self.0 & !Self::AUTO_CMD23_EN)
    }
    /// Check if Auto CMD23 is enabled.
    #[inline]
    pub const fn is_auto_cmd23_enabled(self) -> bool {
        (self.0 & Self::AUTO_CMD23_EN) != 0
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
    const SAMP_DL_CAL_START: u32 = 0x1 << 15;
    const SAMP_DL_CAL_DONE: u32 = 0x1 << 14;
    const SAMP_DL: u32 = 0x3F << 8;
    const SAMP_DL_SW_EN: u32 = 0x1 << 7;
    const SAMP_DL_SW: u32 = 0x3F << 0;

    /// Start sample delay calibration.
    #[inline]
    pub const fn start_sample_delay_cal(self) -> Self {
        Self(self.0 | Self::SAMP_DL_CAL_START)
    }
    /// Stop sample delay calibration.
    #[inline]
    pub const fn stop_sample_delay_cal(self) -> Self {
        Self(self.0 & !Self::SAMP_DL_CAL_START)
    }
    /// Check if sample delay calibration is done.
    #[inline]
    pub const fn is_sample_delay_cal_done(self) -> bool {
        (self.0 & Self::SAMP_DL_CAL_DONE) != 0
    }
    /// Get sample delay value.
    #[inline]
    pub const fn sample_delay(self) -> u8 {
        ((self.0 & Self::SAMP_DL) >> 8) as u8
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
}

/// Data strobe delay control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DataStrobeDelayControl(u32);

impl DataStrobeDelayControl {
    const DS_DL_CAL_START: u32 = 0x1 << 15;
    const DS_DL_CAL_DONE: u32 = 0x1 << 14;
    const DS_DL: u32 = 0x3F << 8;
    const DS_DL_SW_EN: u32 = 0x1 << 7;
    const DS_DL_SW: u32 = 0x3F;

    /// Start data strobe delay calibration.
    #[inline]
    pub const fn start_data_strobe_delay_cal(self) -> Self {
        Self(self.0 | Self::DS_DL_CAL_START)
    }
    /// Stop data strobe delay calibration.
    #[inline]
    pub const fn stop_data_strobe_delay_cal(self) -> Self {
        Self(self.0 & !Self::DS_DL_CAL_START)
    }
    /// Check if data strobe delay calibration is done.
    #[inline]
    pub const fn is_data_strobe_delay_cal_done(self) -> bool {
        (self.0 & Self::DS_DL_CAL_DONE) != 0
    }
    /// Get data strobe delay value.
    #[inline]
    pub const fn data_strobe_delay(self) -> u8 {
        ((self.0 & Self::DS_DL) >> 8) as u8
    }
    /// Enable data strobe delay software.
    #[inline]
    pub const fn enable_data_strobe_delay_software(self) -> Self {
        Self(self.0 | Self::DS_DL_SW_EN)
    }
    /// Disable data strobe delay software.
    #[inline]
    pub const fn disable_data_strobe_delay_software(self) -> Self {
        Self(self.0 & !Self::DS_DL_SW_EN)
    }
    /// Get if data strobe delay software is enabled.
    #[inline]
    pub const fn is_data_strobe_delay_software_enabled(self) -> bool {
        (self.0 & Self::DS_DL_SW_EN) != 0
    }
    /// Set data strobe delay software.
    #[inline]
    pub const fn set_data_strobe_delay_software(self, delay: u8) -> Self {
        Self((self.0 & !Self::DS_DL_SW) | ((delay as u32) << 0))
    }
    /// Get data strobe delay software.
    #[inline]
    pub const fn data_strobe_delay_software(self) -> u8 {
        ((self.0 & Self::DS_DL_SW) >> 0) as u8
    }
}

/// Hs400 delay control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Hs400DelayControl(u32);

impl Hs400DelayControl {
    const HS400_DL_CAL_START: u32 = 0x1 << 15;
    const HS400_DL_CAL_DONE: u32 = 0x1 << 14;
    const HS400_DL: u32 = 0xF << 8;
    const HS400_DL_SW_EN: u32 = 0x1 << 7;
    const HS400_DL_SW: u32 = 0xF;

    /// Start HS400 delay calibration.
    #[inline]
    pub const fn start_hs400_delay_cal(self) -> Self {
        Self(self.0 | Self::HS400_DL_CAL_START)
    }
    /// Stop HS400 delay calibration.
    #[inline]
    pub const fn stop_hs400_delay_cal(self) -> Self {
        Self(self.0 & !Self::HS400_DL_CAL_START)
    }
    /// Check if HS400 delay calibration is done.
    #[inline]
    pub const fn is_hs400_delay_cal_done(self) -> bool {
        (self.0 & Self::HS400_DL_CAL_DONE) != 0
    }
    /// Get HS400 delay value.
    #[inline]
    pub const fn hs400_delay(self) -> u8 {
        ((self.0 & Self::HS400_DL) >> 8) as u8
    }
    /// Enable HS400 delay software.
    #[inline]
    pub const fn enable_hs400_delay_software(self) -> Self {
        Self(self.0 | Self::HS400_DL_SW_EN)
    }
    /// Disable HS400 delay software.
    #[inline]
    pub const fn disable_hs400_delay_software(self) -> Self {
        Self(self.0 & !Self::HS400_DL_SW_EN)
    }
    /// Check if HS400 delay software is enabled.
    #[inline]
    pub const fn is_hs400_delay_software_enabled(self) -> bool {
        (self.0 & Self::HS400_DL_SW_EN) != 0
    }
    /// Set HS400 delay software.
    #[inline]
    pub const fn set_hs400_delay_software(self, delay: u8) -> Self {
        Self((self.0 & !Self::HS400_DL_SW) | (delay as u32 & Self::HS400_DL_SW))
    }
    /// Get HS400 delay software.
    #[inline]
    pub const fn hs400_delay_software(self) -> u8 {
        (self.0 & Self::HS400_DL_SW) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AccessMode, AutoCmd12Arg, BlockSize, BootMode, BurstSize, BusWidth, CardClockMode,
        CardThresholdControl, CardType, ClockControl, CmdFsmState, Command, CrcMode,
        CrcStatusDetect, DataStrobeDelayControl, DdcTimingPhase, DdrMode,
        DdrStartBitDetectionControl, DmaAbortState, DmaControl, DmaInterruptEnable, DmaState,
        DriveDelayControl, ExtendedCommand, FifoFunction, FifoWaterLevel, GlobalControl,
        HardWareReset, Hs400DelayControl, Interrupt, InterruptMask, InterruptStateMasked,
        InterruptStateRaw, NewTimingSet, NtsTimingPhase, RegisterBlock, SampleDelayControl,
        SampleFifoControl, Status, TimeOut, TimeUnit, TransferDirection, TransferMode,
        VoltageSwitch,
    };
    use core::mem::offset_of;
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
        assert_eq!(offset_of!(RegisterBlock, fifo_function), 0x44);
        assert_eq!(offset_of!(RegisterBlock, transferred_byte_count0), 0x48);
        assert_eq!(offset_of!(RegisterBlock, transferred_byte_count1), 0x4C);
        assert_eq!(offset_of!(RegisterBlock, debug_control), 0x50);
        assert_eq!(offset_of!(RegisterBlock, crc_status_detect), 0x54);
        assert_eq!(offset_of!(RegisterBlock, auto_cmd12_arg), 0x58);
        assert_eq!(offset_of!(RegisterBlock, new_timing_set), 0x5C);
        assert_eq!(offset_of!(RegisterBlock, hardware_reset), 0x78);
        assert_eq!(offset_of!(RegisterBlock, dma_control), 0x80);
        assert_eq!(offset_of!(RegisterBlock, dma_descriptor_base), 0x84);
        assert_eq!(offset_of!(RegisterBlock, dma_state), 0x88);
        assert_eq!(offset_of!(RegisterBlock, dma_interrupt_enable), 0x8C);
        assert_eq!(offset_of!(RegisterBlock, card_threshold_control), 0x100);
        assert_eq!(offset_of!(RegisterBlock, sample_fifo_control), 0x104);
        assert_eq!(offset_of!(RegisterBlock, auto_cmd23_arg), 0x108);
        assert_eq!(offset_of!(RegisterBlock, ddr_start_bit_detection), 0x10C);
        assert_eq!(offset_of!(RegisterBlock, extended_command), 0x138);
        assert_eq!(offset_of!(RegisterBlock, extended_response), 0x13C);
        assert_eq!(offset_of!(RegisterBlock, drive_delay_control), 0x140);
        assert_eq!(offset_of!(RegisterBlock, sample_delay_control), 0x144);
        assert_eq!(offset_of!(RegisterBlock, data_strobe_delay_control), 0x148);
        assert_eq!(offset_of!(RegisterBlock, hs400_delay_control), 0x14C);
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

        val = val.set_time_unit_cmd(TimeUnit::Clk256);
        assert_eq!(val.time_unit_cmd(), TimeUnit::Clk256);
        assert_eq!(val.0, 0x00001000);

        val = val.set_time_unit_cmd(TimeUnit::Clk1);
        assert_eq!(val.time_unit_cmd(), TimeUnit::Clk1);
        assert_eq!(val.0, 0x00000000);

        val = val.set_time_unit_data(TimeUnit::Clk256);
        assert_eq!(val.time_unit_data(), TimeUnit::Clk256);
        assert_eq!(val.0, 0x00000800);

        val = val.set_time_unit_data(TimeUnit::Clk1);
        assert_eq!(val.time_unit_data(), TimeUnit::Clk1);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_card_debounce();
        assert!(val.is_card_debounce_enabled());
        assert_eq!(val.0, 0x00000100);

        val = val.disable_card_debounce();
        assert!(!val.is_card_debounce_enabled());
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
        assert_eq!(val.0, 0x00000010);

        val = val.disable_interrupt();
        assert!(!val.is_interrupt_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_dma_reset();
        assert!(!val.is_dma_reset_cleared());
        assert_eq!(val.0, 0x00000004);

        val = GlobalControl(0x0);
        assert!(val.is_dma_reset_cleared());

        val = val.set_fifo_reset();
        assert!(!val.is_fifo_reset_cleared());
        assert_eq!(val.0, 0x00000002);

        val = GlobalControl(0x0);
        assert!(val.is_fifo_reset_cleared());

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

        val = val.set_card_clock_mode(CardClockMode::TurnOffConditionally);
        assert_eq!(val.card_clock_mode(), CardClockMode::TurnOffConditionally);
        assert_eq!(val.0, 0x00020000);

        val = val.set_card_clock_mode(CardClockMode::AlwaysOn);
        assert_eq!(val.card_clock_mode(), CardClockMode::AlwaysOn);
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

        val = val.set_card_clock_divider(0x00);
        assert_eq!(val.card_clock_divider(), 0x00);
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_timeout_functions() {
        let mut val = TimeOut(0x0);

        val = val.set_data_timeout_limit(0xFFFFFF);
        assert_eq!(val.data_timeout_limit(), 0xFFFFFF);
        assert_eq!(val.0, 0xFFFFFF00);

        val = val.set_data_timeout_limit(0x000000);
        assert_eq!(val.data_timeout_limit(), 0x000000);
        assert_eq!(val.0, 0x00000000);

        val = val.set_response_timeout_limit(0xFF);
        assert_eq!(val.response_timeout_limit(), 0xFF);
        assert_eq!(val.0, 0x000000FF);

        val = val.set_response_timeout_limit(0x00);
        assert_eq!(val.response_timeout_limit(), 0x00);
        assert_eq!(val.0, 0x00000000);
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
    fn struct_command_functions() {
        let mut val = Command(0x0);

        val = val.set_command_start();
        assert!(!val.is_command_start_cleared());
        assert_eq!(val.0, 0x80000000);

        val = Command(0x0);
        val = val.set_voltage_switch(VoltageSwitch::Switch);
        assert_eq!(val.voltage_switch(), VoltageSwitch::Switch);
        assert_eq!(val.0, 0x10000000);

        val = val.set_voltage_switch(VoltageSwitch::Normal);
        assert_eq!(val.voltage_switch(), VoltageSwitch::Normal);
        assert_eq!(val.0, 0x00000000);

        val = val.abort_boot();
        assert!(val.is_boot_aborted());
        assert_eq!(val.0, 0x08000000);

        val = Command(0x0);
        val = val.enable_boot_ack_expected();
        assert!(val.is_boot_ack_received());
        assert_eq!(val.0, 0x04000000);

        val = val.disable_boot_ack_expected();
        assert!(!val.is_boot_ack_received());
        assert_eq!(val.0, 0x00000000);

        val = val.set_boot_mode(BootMode::AlternativeBoot);
        assert_eq!(val.boot_mode(), BootMode::AlternativeBoot);
        assert_eq!(val.0, 0x02000000);

        val = val.set_boot_mode(BootMode::MandatoryBoot);
        assert_eq!(val.boot_mode(), BootMode::MandatoryBoot);
        assert_eq!(val.0, 0x01000000);

        val = val.set_boot_mode(BootMode::NormalCmd);
        assert_eq!(val.boot_mode(), BootMode::NormalCmd);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_change_card_clock();
        assert!(val.is_change_card_clock_enabled());
        assert_eq!(val.0, 0x00200000);

        val = val.disable_change_card_clock();
        assert!(!val.is_change_card_clock_enabled());
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

        val = val.set_transfer_mode(TransferMode::Stream);
        assert_eq!(val.transfer_mode(), TransferMode::Stream);
        assert_eq!(val.0, 0x00000800);

        val = val.set_transfer_mode(TransferMode::Block);
        assert_eq!(val.transfer_mode(), TransferMode::Block);
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

        val = val.set_command_index(0x00);
        assert_eq!(val.command_index(), 0x00);
        assert_eq!(val.0, 0x00000000);
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
        let val = Status(0x80000000);
        assert!(val.if_dma_request_occurs());

        let val = Status(0x00000000);
        assert!(!val.if_dma_request_occurs());

        let val = Status(0x03FE0000);
        assert_eq!(val.fifo_level(), 0x1FF);

        let val = Status(0x00000000);
        assert_eq!(val.fifo_level(), 0x00);

        let val = Status(0x0000F800);
        assert_eq!(val.response_index(), 0x1F);

        let val = Status(0x00000000);
        assert_eq!(val.response_index(), 0x00);

        let val = Status(0x00000400);
        assert!(val.fsm_busy());

        let val = Status(0x00000000);
        assert!(!val.fsm_busy());

        let val = Status(0x00000200);
        assert!(val.card_busy());

        let val = Status(0x00000000);
        assert!(!val.card_busy());

        let val = Status(0x00000100);
        assert!(val.card_present());

        let val = Status(0x00000000);
        assert!(!val.card_present());

        for i in 0..=15 {
            let state_val = i << 4;
            let val = Status(state_val);

            let expected_state = match i {
                0 => CmdFsmState::Idle,
                1 => CmdFsmState::SendInitSeq,
                2 => CmdFsmState::TxCmdStartBit,
                3 => CmdFsmState::TxCmdTxBit,
                4 => CmdFsmState::TxCmdIdxArg,
                5 => CmdFsmState::TxCmdCrc7,
                6 => CmdFsmState::TxCmdEndBit,
                7 => CmdFsmState::RxRespStartBit,
                8 => CmdFsmState::RxRespIrqResp,
                9 => CmdFsmState::RxRespTxBit,
                10 => CmdFsmState::RxRespCmdIdx,
                11 => CmdFsmState::RxRespData,
                12 => CmdFsmState::RxRespCrc7,
                13 => CmdFsmState::RxRespEndBit,
                14 => CmdFsmState::WaitNcc,
                15 => CmdFsmState::WaitCmdToResp,
                _ => unreachable!(),
            };

            assert_eq!(val.fsm_state(), expected_state);
        }

        let val = Status(0x00000008);
        assert!(val.fifo_full());

        let val = Status(0x00000000);
        assert!(!val.fifo_full());

        let val = Status(0x00000004);
        assert!(val.fifo_empty());

        let val = Status(0x00000000);
        assert!(!val.fifo_empty());

        let val = Status(0x00000002);
        assert!(val.fifo_tx_level());

        let val = Status(0x00000000);
        assert!(!val.fifo_tx_level());

        let val = Status(0x00000001);
        assert!(val.fifo_rx_level());

        let val = Status(0x00000000);
        assert!(!val.fifo_rx_level());
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
    fn struct_fifo_function_functions() {
        let mut val = FifoFunction(0x0);

        val = val.abort_read_data();
        assert!(val.is_abort_read_data_set());
        assert_eq!(val.0, 0x00000004);

        val = val.clear_abort_read_data();
        assert!(!val.is_abort_read_data_set());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_read_wait();
        assert!(val.is_read_wait_enabled());
        assert_eq!(val.0, 0x00000002);

        val = val.disable_read_wait();
        assert!(!val.is_read_wait_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_host_irq_request();
        assert!(val.is_host_irq_request_enabled());
        assert_eq!(val.0, 0x00000001);

        val = val.disable_host_irq_request();
        assert!(!val.is_host_irq_request_enabled());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_auto_cmd12_arg_functions() {
        let mut val = AutoCmd12Arg(0x0);

        val = val.set_argument(0xFFFF);
        assert_eq!(val.argument(), 0xFFFF);
        assert_eq!(val.0, 0x0000FFFF);

        val = val.set_argument(0x1234);
        assert_eq!(val.argument(), 0x1234);
        assert_eq!(val.0, 0x00001234);
    }

    #[test]
    fn struct_crc_status_detect_functions() {
        let mut val = CrcStatusDetect(0x0);

        val = val.set_crc_mode(CrcMode::Other);
        assert_eq!(val.crc_mode(), CrcMode::Other);
        assert_eq!(val.0, 0x30000000);

        val = val.set_crc_mode(CrcMode::Hs400);
        assert_eq!(val.crc_mode(), CrcMode::Hs400);
        assert_eq!(val.0, 0x60000000);
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

        val = val.enable_rx_dat_cmd_clear();
        assert!(val.is_rx_dat_cmd_clear_enabled());
        assert_eq!(val.0, 0x01000000);

        val = val.disable_rx_dat_cmd_clear();
        assert!(!val.is_rx_dat_cmd_clear_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_rx_dat_crc_status_clear();
        assert!(val.is_rx_dat_crc_status_clear_enabled());
        assert_eq!(val.0, 0x00400000);

        val = val.disable_rx_dat_crc_status_clear();
        assert!(!val.is_rx_dat_crc_status_clear_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_rx_dat_trans_clear();
        assert!(val.is_rx_dat_trans_clear_enabled());
        assert_eq!(val.0, 0x00200000);

        val = val.disable_rx_dat_trans_clear();
        assert!(!val.is_rx_dat_trans_clear_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_rx_dat_recv_clear();
        assert!(val.is_rx_dat_recv_clear_enabled());
        assert_eq!(val.0, 0x00100000);

        val = val.disable_rx_dat_recv_clear();
        assert!(!val.is_rx_dat_recv_clear_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_rx_cmd_send_clear();
        assert!(val.is_rx_cmd_send_clear_enabled());
        assert_eq!(val.0, 0x00010000);

        val = val.disable_rx_cmd_send_clear();
        assert!(!val.is_rx_cmd_send_clear_enabled());
        assert_eq!(val.0, 0x00000000);

        for i in 0..4 {
            let phase = match i {
                0 => NtsTimingPhase::Offset90,
                1 => NtsTimingPhase::Offset180,
                2 => NtsTimingPhase::Offset270,
                3 => NtsTimingPhase::Offset0,
                _ => unreachable!(),
            };

            val = NewTimingSet(0x0);
            val = val.set_dat_sample_timing_phase(phase);
            assert_eq!(val.dat_sample_timing_phase(), phase);
            assert_eq!(val.0, (i as u32) << 8);
        }

        for i in 0..3 {
            let phase = match i {
                0 => NtsTimingPhase::Offset90,
                1 => NtsTimingPhase::Offset180,
                2 => NtsTimingPhase::Offset270,
                _ => unreachable!(),
            };

            val = NewTimingSet(0x0);
            val = val.set_cmd_sample_timing_phase(phase);
            assert_eq!(val.cmd_sample_timing_phase(), phase);
            assert_eq!(val.0, (i as u32) << 4);
        }

        val = NewTimingSet(0x0);
        val = val.enable_hs400_new_sample();
        assert!(val.is_hs400_new_sample_enabled());
        assert_eq!(val.0, 0x00000001);

        val = val.disable_hs400_new_sample();
        assert!(!val.is_hs400_new_sample_enabled());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_hardware_reset_functions() {
        let mut val = HardWareReset(0x0);

        val = val.reset_hardware();
        assert!(!val.is_hardware_reset_cleared());
        assert_eq!(val.0, 0x00000001);

        val = val.active_hardware();
        assert!(val.is_hardware_reset_cleared());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_dma_control_functions() {
        let mut val = DmaControl(0x0);

        val = val.enable_dma_desc_refetch();
        assert!(val.is_dma_desc_refetch_enable());
        assert_eq!(val.0, 0x80000000);

        val = val.disable_dma_desc_refetch();
        assert!(!val.is_dma_desc_refetch_enable());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_dma();
        assert!(val.is_dma_enabled());
        assert_eq!(val.0, 0x00000080);

        val = val.disable_dma();
        assert!(!val.is_dma_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_fix_burst_size();
        assert!(val.is_fix_burst_size_enabled());
        assert_eq!(val.0, 0x00000002);

        val = val.disable_fix_burst_size();
        assert!(!val.is_fix_burst_size_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.reset_dma();
        assert!(!val.is_dma_reset_cleared());
        assert_eq!(val.0, 0x00000001);

        val = DmaControl(0x0);
        assert!(val.is_dma_reset_cleared());
    }

    #[test]
    fn struct_dma_state_functions() {
        let mut val = DmaState(0x00000400);
        assert_eq!(val.dma_error_status(), DmaAbortState::Tx);

        val = DmaState(0x00000800);
        assert_eq!(val.dma_error_status(), DmaAbortState::Rx);

        val = DmaState(0x00000200);
        assert!(val.abn_int_sum_occurs());

        val = DmaState(0x00000000);
        assert!(!val.abn_int_sum_occurs());

        val = val.clear_abn_int_sum();
        assert_eq!(val.0, 0x00000200);

        val = DmaState(0x00000100);
        assert!(val.nor_int_sum_occurs());

        val = DmaState(0x00000000);
        assert!(!val.nor_int_sum_occurs());

        val = val.clear_nor_int_sum();
        assert_eq!(val.0, 0x00000100);

        val = DmaState(0x00000020);
        assert!(val.card_err_sum_occurs());

        val = DmaState(0x00000000);
        assert!(!val.card_err_sum_occurs());

        val = val.clear_card_err_sum();
        assert_eq!(val.0, 0x00000020);

        val = DmaState(0x00000010);
        assert!(val.des_unavl_int_occurs());

        val = DmaState(0x00000000);
        assert!(!val.des_unavl_int_occurs());

        val = val.clear_des_unavl_int();
        assert_eq!(val.0, 0x00000010);

        val = DmaState(0x00000004);
        assert!(val.fatal_berr_int_occurs());

        val = DmaState(0x00000000);
        assert!(!val.fatal_berr_int_occurs());

        val = val.clear_fatal_berr_int();
        assert_eq!(val.0, 0x00000004);

        val = DmaState(0x00000002);
        assert!(val.rx_int_occurs());

        val = DmaState(0x00000000);
        assert!(!val.rx_int_occurs());

        val = val.clear_rx_int();
        assert_eq!(val.0, 0x00000002);

        val = DmaState(0x00000001);
        assert!(val.tx_int_occurs());

        val = DmaState(0x00000000);
        assert!(!val.tx_int_occurs());

        val = val.clear_tx_int();
        assert_eq!(val.0, 0x00000001);
    }

    #[test]
    fn struct_dma_interrupt_enable_functions() {
        let mut val = DmaInterruptEnable(0x0);

        val = val.enable_card_err_sum_int();
        assert!(val.is_card_err_sum_int_enabled());
        assert_eq!(val.0, 0x00000020);

        val = val.disable_card_err_sum_int();
        assert!(!val.is_card_err_sum_int_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_des_unavl_int();
        assert!(val.is_des_unavl_int_enabled());
        assert_eq!(val.0, 0x00000010);

        val = val.disable_des_unavl_int();
        assert!(!val.is_des_unavl_int_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_fatal_berr_int();
        assert!(val.is_fatal_berr_int_enabled());
        assert_eq!(val.0, 0x00000004);

        val = val.disable_fatal_berr_int();
        assert!(!val.is_fatal_berr_int_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_rx_int();
        assert!(val.is_rx_int_enabled());
        assert_eq!(val.0, 0x00000002);

        val = val.disable_rx_int();
        assert!(!val.is_rx_int_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_tx_int();
        assert!(val.is_tx_int_enabled());
        assert_eq!(val.0, 0x00000001);

        val = val.disable_tx_int();
        assert!(!val.is_tx_int_enabled());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_card_threshold_control_functions() {
        let mut val = CardThresholdControl(0x00000000);

        val = val.set_card_wr_thld(0xFFF);
        assert_eq!(val.card_wr_thld(), 0xFFF);
        assert_eq!(val.0, 0x0FFF0000);

        val = CardThresholdControl(0x00000000);
        val = val.enable_card_write_threshold();
        assert!(val.is_card_write_threshold_enabled());
        assert_eq!(val.0, 0x00000004);

        val = val.disable_card_write_threshold();
        assert!(!val.is_card_write_threshold_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_busy_clear();
        assert!(val.is_busy_clear_enabled());
        assert_eq!(val.0, 0x00000002);

        val = val.disable_busy_clear();
        assert!(!val.is_busy_clear_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_card_read_threshold();
        assert!(val.is_card_read_threshold_enabled());
        assert_eq!(val.0, 0x00000001);

        val = val.disable_card_read_threshold();
        assert!(!val.is_card_read_threshold_enabled());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_sample_fifo_control_functions() {
        let mut val = SampleFifoControl(0x00000000);

        val = val.set_stop_clk_ctrl(0xF);
        assert_eq!(val.stop_clk_ctrl(), 0xF);
        assert_eq!(val.0, 0x0000001E);

        val = val.set_stop_clk_ctrl(0x0);
        assert_eq!(val.stop_clk_ctrl(), 0x0);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_bypass();
        assert!(val.is_bypass_enabled());
        assert_eq!(val.0, 0x00000001);

        val = val.disable_bypass();
        assert!(!val.is_bypass_enabled());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_ddr_start_bit_detection_control_functions() {
        let mut val = DdrStartBitDetectionControl(0x00000000);

        val = val.enable_hs400_mode();
        assert!(val.is_hs400_mode_enabled());
        assert_eq!(val.0, 0x80000000);

        val = val.disable_hs400_mode();
        assert!(!val.is_hs400_mode_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_half_start_bit_less();
        assert!(val.is_half_start_bit_less());
        assert!(!val.is_half_start_bit_full());
        assert_eq!(val.0, 0x00000001);

        val = val.set_half_start_bit_full();
        assert!(!val.is_half_start_bit_less());
        assert!(val.is_half_start_bit_full());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_extended_command_functions() {
        let mut val = ExtendedCommand(0x00000000);

        val = val.enable_auto_cmd23();
        assert!(val.is_auto_cmd23_enabled());
        assert_eq!(val.0, 0x00000001);

        val = val.disable_auto_cmd23();
        assert!(!val.is_auto_cmd23_enabled());
        assert_eq!(val.0, 0x00000000);
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

    #[test]
    fn struct_sample_delay_control_functions() {
        let mut val = SampleDelayControl(0x00000000);

        val = val.start_sample_delay_cal();
        assert_eq!(val.0, 0x00008000);

        val = val.stop_sample_delay_cal();
        assert_eq!(val.0, 0x00000000);

        val = SampleDelayControl(0x00004000);
        assert!(val.is_sample_delay_cal_done());

        val = SampleDelayControl(0x0);
        assert!(!val.is_sample_delay_cal_done());

        val = SampleDelayControl(0x00003F00);
        assert_eq!(val.sample_delay(), 0x3F);

        val = SampleDelayControl(0x0);
        val = val.enable_sample_delay_software();
        assert!(val.is_sample_delay_software_enabled());
        assert_eq!(val.0, 0x00000080);

        val = val.disable_sample_delay_software();
        assert!(!val.is_sample_delay_software_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_sample_delay_software(0x3F);
        assert_eq!(val.sample_delay_software(), 0x3F);
        assert_eq!(val.0, 0x0000003F);

        val = val.set_sample_delay_software(0x00);
        assert_eq!(val.sample_delay_software(), 0x00);
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_data_strobe_delay_control_functions() {
        let mut val = DataStrobeDelayControl(0x00000000);

        val = val.start_data_strobe_delay_cal();
        assert_eq!(val.0, 0x00008000);

        val = val.stop_data_strobe_delay_cal();
        assert_eq!(val.0, 0x00000000);

        val = DataStrobeDelayControl(0x00004000);
        assert!(val.is_data_strobe_delay_cal_done());

        val = DataStrobeDelayControl(0x0);
        assert!(!val.is_data_strobe_delay_cal_done());

        val = DataStrobeDelayControl(0x00003F00);
        assert_eq!(val.data_strobe_delay(), 0x3F);

        val = DataStrobeDelayControl(0x00000000);
        val = val.enable_data_strobe_delay_software();
        assert!(val.is_data_strobe_delay_software_enabled());
        assert_eq!(val.0, 0x00000080);

        val = val.disable_data_strobe_delay_software();
        assert!(!val.is_data_strobe_delay_software_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_data_strobe_delay_software(0x3F);
        assert_eq!(val.data_strobe_delay_software(), 0x3F);
        assert_eq!(val.0, 0x0000003F);

        val = val.set_data_strobe_delay_software(0x00);
        assert_eq!(val.data_strobe_delay_software(), 0x00);
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_hs400_delay_control_functions() {
        let mut val = Hs400DelayControl(0x00000000);

        val = val.start_hs400_delay_cal();
        assert_eq!(val.0, 0x00008000);

        val = val.stop_hs400_delay_cal();
        assert_eq!(val.0, 0x00000000);

        val = Hs400DelayControl(0x00004000);
        assert!(val.is_hs400_delay_cal_done());

        val = Hs400DelayControl(0x0);
        assert!(!val.is_hs400_delay_cal_done());

        val = Hs400DelayControl(0x00000F00);
        assert_eq!(val.hs400_delay(), 0x0F);

        val = Hs400DelayControl(0x00000000);
        val = val.enable_hs400_delay_software();
        assert!(val.is_hs400_delay_software_enabled());
        assert_eq!(val.0, 0x00000080);

        val = val.disable_hs400_delay_software();
        assert!(!val.is_hs400_delay_software_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_hs400_delay_software(0x0F);
        assert_eq!(val.hs400_delay_software(), 0x0F);
        assert_eq!(val.0, 0x0000000F);

        val = val.set_hs400_delay_software(0x00);
        assert_eq!(val.hs400_delay_software(), 0x00);
        assert_eq!(val.0, 0x00000000);
    }
}
