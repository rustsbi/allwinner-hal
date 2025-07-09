use volatile_register::{RO, RW};

/// Direct Memory Access Controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// DMAC IRQ Enable Register 0.
    pub irq_enable0: RW<IrqEnable0>,
    /// DMAC IRQ Enable Register 1.
    pub irq_enable1: RW<IrqEnable1>,
    _reserved0: [u8; 0x8],
    /// DMAC IRQ Pending Register 0.
    pub irq_pending0: RW<IrqPending0>,
    /// DMAC IRQ Pending Register 1.
    pub irq_pending1: RW<IrqPending1>,
    _reserved1: [u8; 0x10],
    /// DMAC Auto Gating Register.
    pub auto_gating: RW<AutoGating>,
    _reserved2: [u8; 0x4],
    /// DMAC Status Register.
    pub status: RO<Status>,
    _reserved3: [u8; 0xCC],
    /// DMAC Channels' Registers.
    pub channels: [ChannelRegisterBlock; 16],
}

/// DMAC channel registers.
#[repr(C)]
pub struct ChannelRegisterBlock {
    /// DMAC Channel Enable Register.
    pub enable: RW<ChannelEnable>,
    /// DMAC Channel Pause Register.
    pub pause: RW<ChannelPause>,
    /// DMAC Channel Start Address Register.
    pub start_addr: RW<ChannelStartAddr>,
    /// DMAC Channel Configuration Register.
    pub config: RO<ChannelConfig>,
    /// DMAC Channel Current Source Register.
    pub current_src_addr: RO<ChannelCurrentSrcAddr>,
    /// DMAC Channel Current Destination Register.
    pub current_destination: RO<ChannelCurrentDestAddr>,
    /// DMAC Channel Byte Counter Left Register.
    pub byte_counter_left: RO<ChannelByteCounterLeft>,
    /// DMAC Channel Parameter Register.
    pub parameter: RO<ChannelParameter>,
    _reserved0: [u8; 0x8],
    /// DMAC Mode Register.
    pub mode: RW<ChannelMode>,
    /// DMAC Former Descriptor Address Register.
    pub former_desc_addr: RO<ChannelFormerDescAddr>,
    /// DMAC Package Number Register.
    pub package_num: RO<ChannelPackageNum>,
    _reserved1: [u8; 0xC],
}

/// Represents the DMAC IRQ Enable Register 0.
///
/// This register controls the interrupt enable settings for DMA channels 0 to 7.
/// Each channel has three interrupt types: Half Package, Package End, and Queue End.
/// - Address offset: 0x0
/// - Default value: 0x0000_0000
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IrqEnable0(u32);

/// Enumerates the types of interrupts supported by each DMA channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptType {
    /// Half Package Interrupt, triggered when half of a package is transferred.
    HalfPackage,
    /// Package End Interrupt, triggered when a complete package is transferred.
    PackageEnd,
    /// Queue End Interrupt, triggered when a complete queue is transferred.
    QueueEnd,
}

impl IrqEnable0 {
    /// Calculates the bit offset for a given channel and interrupt type.
    #[inline]
    const fn get_bit_offset(channel: u8, interrupt_type: InterruptType) -> u32 {
        assert!(channel <= 7, "Channel must be 0-7 for IrqEnable0");

        let base = channel as u32 * 4;
        match interrupt_type {
            InterruptType::HalfPackage => base,
            InterruptType::PackageEnd => base + 1,
            InterruptType::QueueEnd => base + 2,
        }
    }

    /// Enables the specified interrupt for the given channel.
    #[inline]
    pub const fn enable_interrupt(self, channel: u8, interrupt_type: InterruptType) -> Self {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        Self(self.0 | (1 << offset))
    }

    /// Disables the specified interrupt for the given channel.
    #[inline]
    pub const fn disable_interrupt(self, channel: u8, interrupt_type: InterruptType) -> Self {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        Self(self.0 & !(1 << offset))
    }

    /// Checks if the specified interrupt is enabled for the given channel.
    #[inline]
    pub const fn is_interrupt_enabled(self, channel: u8, interrupt_type: InterruptType) -> bool {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        (self.0 & (1 << offset)) != 0
    }
}

/// Represents the DMAC IRQ Enable Register 1 (Channels 8-15).
///
/// This register controls the interrupt enable settings for DMA channels 8 to 15.
/// Each channel has three interrupt types: Half Package, Package End, and Queue End.
/// - Address offset: 0x4
/// - Default value: 0x0000_0000
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IrqEnable1(u32);

impl IrqEnable1 {
    /// Calculates the bit offset for a given channel and interrupt type.
    #[inline]
    const fn get_bit_offset(channel: u8, interrupt_type: InterruptType) -> u32 {
        assert!(
            channel >= 8 && channel <= 15,
            "Channel must be 8-15 for IrqEnable1"
        );

        let base = (channel - 8) as u32 * 4;
        match interrupt_type {
            InterruptType::HalfPackage => base,
            InterruptType::PackageEnd => base + 1,
            InterruptType::QueueEnd => base + 2,
        }
    }

    /// Enables the specified interrupt for the given channel.
    #[inline]
    pub const fn enable_interrupt(self, channel: u8, interrupt_type: InterruptType) -> Self {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        Self(self.0 | (1 << offset))
    }

    /// Disables the specified interrupt for the given channel.
    #[inline]
    pub const fn disable_interrupt(self, channel: u8, interrupt_type: InterruptType) -> Self {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        Self(self.0 & !(1 << offset))
    }

    /// Checks if the specified interrupt is enabled for the given channel.
    #[inline]
    pub const fn is_interrupt_enabled(self, channel: u8, interrupt_type: InterruptType) -> bool {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        (self.0 & (1 << offset)) != 0
    }
}

/// DMAC IRQ Pending Register 0 (Channels 0-7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IrqPending0(u32);

impl IrqPending0 {
    /// Calculates the bit offset for a given channel and interrupt type.
    #[inline]
    const fn get_bit_offset(channel: u8, interrupt_type: InterruptType) -> u32 {
        assert!(channel <= 7, "Channel must be 0-7");
        let base = channel as u32 * 4;
        match interrupt_type {
            InterruptType::HalfPackage => base,
            InterruptType::PackageEnd => base + 1,
            InterruptType::QueueEnd => base + 2,
        }
    }

    /// Checks if the specified interrupt is pending for the given channel.
    #[inline]
    pub const fn if_irq_pending(self, channel: u8, interrupt_type: InterruptType) -> bool {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        (self.0 & (1 << offset)) != 0
    }

    /// Clears the specified interrupt for the given channel.
    #[inline]
    pub const fn clear_irq(self, channel: u8, interrupt_type: InterruptType) -> Self {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        Self(self.0 | (1 << offset))
    }
}

/// DMAC IRQ Pending Register 1 (Channels 8-15).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IrqPending1(u32);

impl IrqPending1 {
    /// Calculates the bit offset for a given channel and interrupt type.
    #[inline]
    const fn get_bit_offset(channel: u8, interrupt_type: InterruptType) -> u32 {
        assert!(channel >= 8 && channel <= 15, "Channel must be 8-15");
        let base = (channel - 8) as u32 * 4;
        match interrupt_type {
            InterruptType::HalfPackage => base,
            InterruptType::PackageEnd => base + 1,
            InterruptType::QueueEnd => base + 2,
        }
    }

    /// Checks if the specified interrupt is pending for the given channel.
    #[inline]
    pub const fn if_irq_pending(self, channel: u8, interrupt_type: InterruptType) -> bool {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        (self.0 & (1 << offset)) != 0
    }

    /// Clears the specified interrupt for the given channel.
    #[inline]
    pub const fn clear_irq(self, channel: u8, interrupt_type: InterruptType) -> Self {
        let offset = Self::get_bit_offset(channel, interrupt_type);
        Self(self.0 | (1 << offset))
    }
}

/// DMAC Auto Gating Register (0x0028).
///
/// Controls automatic gating of DMA circuit components for power optimization.
/// - Bit 2: DMA_MCLK_CIRCUIT (0: Auto gating enabled, 1: Auto gating disabled)
/// - Bit 1: DMA_COMMON_CIRCUIT (0: Auto gating enabled, 1: Auto gating disabled)  
/// - Bit 0: DMA_CHAN_CIRCUIT (0: Auto gating enabled, 1: Auto gating disabled)
///
/// NOTE: When initializing the DMA Controller, bit[2] should be set up.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AutoGating(u32);

impl AutoGating {
    const DMA_MCLK_CIRCUIT: u32 = 1 << 2;
    const DMA_COMMON_CIRCUIT: u32 = 1 << 1;
    const DMA_CHAN_CIRCUIT: u32 = 1 << 0;

    // DMA Channel Circuit Auto Gating (bit 0)

    /// Enable auto gating for DMA channel circuit.
    #[inline]
    pub const fn enable_dma_chan_circuit_gating(self) -> Self {
        Self(self.0 & !Self::DMA_CHAN_CIRCUIT) // Clear bit 0
    }

    /// Disable auto gating for DMA channel circuit.
    #[inline]
    pub const fn disable_dma_chan_circuit_gating(self) -> Self {
        Self(self.0 | Self::DMA_CHAN_CIRCUIT) // Set bit 0
    }

    /// Check if DMA channel circuit auto gating is enabled.
    #[inline]
    pub const fn is_dma_chan_circuit_gating_enabled(self) -> bool {
        (self.0 & Self::DMA_CHAN_CIRCUIT) == 0 // 0 = enabled
    }

    // DMA Common Circuit Auto Gating (bit 1)

    /// Enable auto gating for DMA common circuit.
    #[inline]
    pub const fn enable_dma_common_circuit_gating(self) -> Self {
        Self(self.0 & !Self::DMA_COMMON_CIRCUIT) // Clear bit 1
    }

    /// Disable auto gating for DMA common circuit.
    #[inline]
    pub const fn disable_dma_common_circuit_gating(self) -> Self {
        Self(self.0 | Self::DMA_COMMON_CIRCUIT) // Set bit 1
    }

    /// Check if DMA common circuit auto gating is enabled.
    #[inline]
    pub const fn is_dma_common_circuit_gating_enabled(self) -> bool {
        (self.0 & Self::DMA_COMMON_CIRCUIT) == 0 // 0 = enabled
    }

    // DMA MCLK Circuit Auto Gating (bit 2)

    /// Enable auto gating for DMA MCLK interface circuit.
    #[inline]
    pub const fn enable_dma_mclk_circuit_gating(self) -> Self {
        Self(self.0 & !Self::DMA_MCLK_CIRCUIT) // Clear bit 2
    }

    /// Disable auto gating for DMA MCLK interface circuit.
    #[inline]
    pub const fn disable_dma_mclk_circuit_gating(self) -> Self {
        Self(self.0 | Self::DMA_MCLK_CIRCUIT) // Set bit 2
    }

    /// Check if DMA MCLK interface circuit auto gating is enabled.
    #[inline]
    pub const fn is_dma_mclk_circuit_gating_enabled(self) -> bool {
        (self.0 & Self::DMA_MCLK_CIRCUIT) == 0 // 0 = enabled
    }

    /// Initialize with recommended settings.
    /// According to the manual, bit[2] should be set up during initialization.
    #[inline]
    pub const fn init_recommended() -> Self {
        Self(Self::DMA_MCLK_CIRCUIT) // Set bit 2, disable MCLK auto gating during init
    }
}

/// DMAC Status Register bitfields.
///
/// This register provides status information about DMA channels and MBUS FIFO.
/// All fields in this register are read-only.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Status(u32);

impl Status {
    const MBUS_FIFO_STATUS: u32 = 0x1 << 31;
    const DMA_STATUS: u32 = 0xFFFF;

    // DMA Channel Status (bits 15:0)

    /// Get the status of all DMA channels as a 16-bit mask.
    /// Each bit represents one channel: 0 = Idle, 1 = Busy.
    #[inline]
    pub const fn dma_channels_status(self) -> u16 {
        (self.0 & Self::DMA_STATUS) as u16
    }

    /// Check if the specified DMA channel is busy.
    ///
    /// # Arguments
    /// * `channel` - Channel number (0-15)
    #[inline]
    pub const fn is_dma_channel_busy(self, channel: u8) -> bool {
        assert!(channel < 16, "Channel number must be 0-15");
        (self.0 & (1 << channel)) != 0
    }

    /// Check if the specified DMA channel is idle.
    #[inline]
    pub const fn is_dma_channel_idle(self, channel: u8) -> bool {
        assert!(channel < 16, "Channel number must be 0-15");
        (self.0 & (1 << channel)) == 0
    }

    /// Check if MBUS FIFO is empty.
    /// 0 = MBUS FIFO empty, 1 = MBUS FIFO not empty
    #[inline]
    pub const fn is_mbus_fifo_empty(self) -> bool {
        (self.0 & Self::MBUS_FIFO_STATUS) == 0
    }
}

/// Channel Enable Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelEnable(u32);

impl ChannelEnable {
    // DMA Channel Enable (Bit 0)
    const DMA_EN: u32 = 0x1 << 0;

    /// Check if DMA channel is enabled.
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        (self.0 & Self::DMA_EN) != 0
    }

    /// Enable DMA channel (set bit 0).
    #[inline]
    pub fn enable_dma(mut self) -> Self {
        self.0 |= Self::DMA_EN;
        self
    }

    /// Disable DMA channel (clear bit 0).
    #[inline]
    pub fn disable_dma(mut self) -> Self {
        self.0 &= !Self::DMA_EN;
        self
    }
}

/// Channel Pause Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelPause(u32);

impl ChannelPause {
    // DMA Pause bit (Bit 0)
    const DMA_PAUSE: u32 = 0x1 << 0;

    /// Check if DMA transfer is paused.
    #[inline]
    pub const fn is_dma_pause(self) -> bool {
        (self.0 & Self::DMA_PAUSE) != 0
    }

    /// Pause DMA transfer (set bit 0).
    #[inline]
    pub fn pause_dma(mut self) -> Self {
        self.0 |= Self::DMA_PAUSE;
        self
    }

    /// Resume DMA transfer (clear bit 0).
    #[inline]
    pub fn resume_dma(mut self) -> Self {
        self.0 &= !Self::DMA_PAUSE;
        self
    }
}

/// Channel Start Address Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelStartAddr(u32);

impl ChannelStartAddr {
    // DMA descriptor address lower 30 bits (Bits [31:2])
    const DMA_DESC_ADDR: u32 = 0x3FFFFFFF << 2;

    // DMA descriptor address higher 2 bits (Bits [1:0])
    const DMA_DESC_HIGH_ADDR: u32 = 0x3 << 0;

    /// Get lower 30 bits of the DMA descriptor address.
    #[inline]
    pub const fn dma_desc_addr(self) -> u32 {
        (self.0 & Self::DMA_DESC_ADDR) >> 2
    }

    /// Get higher 2 bits of the DMA descriptor address.
    #[inline]
    pub const fn dma_desc_high_addr(self) -> u32 {
        (self.0 & Self::DMA_DESC_HIGH_ADDR) >> 0
    }

    /// Get full 32-bit DMA descriptor address with alignment.
    #[inline]
    pub const fn full_dma_desc_addr(self) -> u32 {
        (self.dma_desc_high_addr() << 30) | (self.dma_desc_addr() << 2)
    }
}

/// Channel Configuration Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelConfig(u32);

impl ChannelConfig {
    // BMODE_SEL (Bit 30)
    const BMODE_SEL: u32 = 0x1 << 30;

    // DMA_DEST_DATA_WIDTH (Bits [26:25])
    const DMA_DEST_DATA_WIDTH: u32 = 0x3 << 25;

    // DMA_ADDR_MODE (Bit 24)
    const DMA_ADDR_MODE: u32 = 0x1 << 24;

    // DMA_DEST_BLOCK_SIZE (Bits [23:22])
    const DMA_DEST_BLOCK_SIZE: u32 = 0x3 << 22;

    // DMA_DEST_DRQ_TYPE (Bits [21:16])
    const DMA_DEST_DRQ_TYPE: u32 = 0x3F << 16;

    // DMA_SRC_DATA_WIDTH (Bits [10:9])
    const DMA_SRC_DATA_WIDTH: u32 = 0x3 << 9;

    // DMA_SRC_ADDR_MODE (Bit 8)
    const DMA_SRC_ADDR_MODE: u32 = 0x1 << 8;

    // DMA_SRC_BLOCK_SIZE (Bits [7:6])
    const DMA_SRC_BLOCK_SIZE: u32 = 0x3 << 6;

    // DMA_SRC_DRQ_TYPE (Bits [5:0])
    const DMA_SRC_DRQ_TYPE: u32 = 0x3F << 0;

    /// Get the BMODE_SEL bit.
    #[inline]
    pub const fn bmode_sel(self) -> u32 {
        (self.0 & Self::BMODE_SEL) >> 30
    }

    /// Get the DMA_DEST_DATA_WIDTH bits.
    #[inline]
    pub const fn dma_dest_data_width(self) -> u32 {
        (self.0 & Self::DMA_DEST_DATA_WIDTH) >> 25
    }

    /// Get the DMA_ADDR_MODE bit.
    #[inline]
    pub const fn dma_addr_mode(self) -> u32 {
        (self.0 & Self::DMA_ADDR_MODE) >> 24
    }

    /// Get the DMA_DEST_BLOCK_SIZE bits.
    #[inline]
    pub const fn dma_dest_block_size(self) -> u32 {
        (self.0 & Self::DMA_DEST_BLOCK_SIZE) >> 22
    }

    /// Get the DMA_DEST_DRQ_TYPE bits.
    #[inline]
    pub const fn dma_dest_drq_type(self) -> u32 {
        (self.0 & Self::DMA_DEST_DRQ_TYPE) >> 16
    }

    /// Get the DMA_SRC_DATA_WIDTH bits.
    #[inline]
    pub const fn dma_src_data_width(self) -> u32 {
        (self.0 & Self::DMA_SRC_DATA_WIDTH) >> 9
    }

    /// Get the DMA_SRC_ADDR_MODE bit.
    #[inline]
    pub const fn dma_src_addr_mode(self) -> u32 {
        (self.0 & Self::DMA_SRC_ADDR_MODE) >> 8
    }

    /// Get the DMA_SRC_BLOCK_SIZE bits.
    #[inline]
    pub const fn dma_src_block_size(self) -> u32 {
        (self.0 & Self::DMA_SRC_BLOCK_SIZE) >> 6
    }

    /// Get the DMA_SRC_DRQ_TYPE bits.
    #[inline]
    pub const fn dma_src_drq_type(self) -> u32 {
        (self.0 & Self::DMA_SRC_DRQ_TYPE) >> 0
    }
}

/// Channel Current Source Address Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelCurrentSrcAddr(u32);

impl ChannelCurrentSrcAddr {
    // DMA Current Source Address (Bits [31:0])
    const DMA_CUR_SRC: u32 = 0xFFFFFFFF;

    /// Get the DMA current source address.
    #[inline]
    pub const fn dma_cur_src(self) -> u32 {
        self.0 & Self::DMA_CUR_SRC
    }
}

/// Channel Current Destination Address Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelCurrentDestAddr(u32);

impl ChannelCurrentDestAddr {
    // DMA Current Destination Address (Bits [31:0])
    const DMA_CUR_DEST: u32 = 0xFFFFFFFF;

    /// Get the DMA current destination address.
    #[inline]
    pub const fn dma_cur_dest(self) -> u32 {
        self.0 & Self::DMA_CUR_DEST
    }
}

/// Channel Byte Counter Left Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelByteCounterLeft(u32);

impl ChannelByteCounterLeft {
    // DMA Byte Counter Left (Bits [24:0])
    const DMA_BCNT_LEFT: u32 = 0xFFFFFF;

    /// Get the DMA byte counter left value.
    #[inline]
    pub const fn dma_bcnt_left(self) -> u32 {
        self.0 & Self::DMA_BCNT_LEFT
    }
}

/// Channel Parameter Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelParameter(u32);

impl ChannelParameter {
    // WAIT_CYC (Bits [7:0])
    const WAIT_CYC: u32 = 0xFF << 0;

    /// Get the WAIT_CYC value.
    #[inline]
    pub const fn wait_cyc(self) -> u32 {
        self.0 & Self::WAIT_CYC
    }
}

/// Channel Mode Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelMode(u32);

impl ChannelMode {
    // DMA_DST_MODE (Bit 3)
    const DMA_DST_MODE: u32 = 0x1 << 3;

    // DMA_SRC_MODE (Bit 2)
    const DMA_SRC_MODE: u32 = 0x1 << 2;

    /// Get the DMA destination communication mode.
    #[inline]
    pub const fn dma_dst_mode(self) -> u32 {
        (self.0 & Self::DMA_DST_MODE) >> 3
    }

    /// Get the DMA source communication mode.
    #[inline]
    pub const fn dma_src_mode(self) -> u32 {
        (self.0 & Self::DMA_SRC_MODE) >> 2
    }

    /// Set the DMA destination communication mode.
    #[inline]
    pub fn set_dma_dst_mode(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= Self::DMA_DST_MODE;
        } else {
            self.0 &= !Self::DMA_DST_MODE;
        }
        self
    }

    /// Set the DMA source communication mode.
    #[inline]
    pub fn set_dma_src_mode(mut self, enable: bool) -> Self {
        if enable {
            self.0 |= Self::DMA_SRC_MODE;
        } else {
            self.0 &= !Self::DMA_SRC_MODE;
        }
        self
    }
}

/// Channel Former Descriptor Address Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelFormerDescAddr(u32);

impl ChannelFormerDescAddr {
    // DMA Former Descriptor Address (Bits [31:0])
    const DMA_FDESC_ADDR: u32 = 0xFFFFFFFF;

    /// Get the DMA former descriptor address.
    #[inline]
    pub const fn dma_fdesc_addr(self) -> u32 {
        self.0 & Self::DMA_FDESC_ADDR
    }
}

/// Channel Package Number Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct ChannelPackageNum(u32);

impl ChannelPackageNum {
    // DMA Package Number (Bits [31:0])
    const DMA_PKG_NUM: u32 = 0xFFFFFFFF;

    /// Get the DMA package number.
    #[inline]
    pub const fn dma_pkg_num(self) -> u32 {
        self.0 & Self::DMA_PKG_NUM
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};
    #[test]
    fn offset_registerblock() {
        assert_eq!(offset_of!(RegisterBlock, irq_enable0), 0x0);
        assert_eq!(offset_of!(RegisterBlock, irq_enable1), 0x4);
        assert_eq!(offset_of!(RegisterBlock, irq_pending0), 0x10);
        assert_eq!(offset_of!(RegisterBlock, irq_pending1), 0x14);
        assert_eq!(offset_of!(RegisterBlock, auto_gating), 0x28);
        assert_eq!(offset_of!(RegisterBlock, status), 0x30);
        assert_eq!(offset_of!(RegisterBlock, channels), 0x100);
    }

    #[test]
    fn offset_channel_registerblock() {
        assert_eq!(offset_of!(ChannelRegisterBlock, enable), 0x0);
        assert_eq!(offset_of!(ChannelRegisterBlock, pause), 0x4);
        assert_eq!(offset_of!(ChannelRegisterBlock, start_addr), 0x08);
        assert_eq!(offset_of!(ChannelRegisterBlock, config), 0xC);
        assert_eq!(offset_of!(ChannelRegisterBlock, current_src_addr), 0x10);
        assert_eq!(offset_of!(ChannelRegisterBlock, current_destination), 0x14);
        assert_eq!(offset_of!(ChannelRegisterBlock, byte_counter_left), 0x18);
        assert_eq!(offset_of!(ChannelRegisterBlock, parameter), 0x1C);
        assert_eq!(offset_of!(ChannelRegisterBlock, mode), 0x28);
        assert_eq!(offset_of!(ChannelRegisterBlock, former_desc_addr), 0x2C);
        assert_eq!(offset_of!(ChannelRegisterBlock, package_num), 0x30);
        assert_eq!(size_of::<ChannelRegisterBlock>(), 0x40);
    }

    #[test]
    fn test_irq_enable0() {
        let reg = IrqEnable0(0x0);

        let reg = reg.enable_interrupt(0, InterruptType::HalfPackage);
        assert!(reg.is_interrupt_enabled(0, InterruptType::HalfPackage));
        assert_eq!(reg.0, 0x1);

        let reg = reg.enable_interrupt(0, InterruptType::PackageEnd);
        assert!(reg.is_interrupt_enabled(0, InterruptType::PackageEnd));
        assert_eq!(reg.0, 0x3);

        let reg = reg.disable_interrupt(0, InterruptType::HalfPackage);
        assert!(!reg.is_interrupt_enabled(0, InterruptType::HalfPackage));
        assert_eq!(reg.0, 0x2);

        // Test channel 7, QueueEnd (bit 30)
        let reg = IrqEnable0(0x0);
        let reg = reg.enable_interrupt(7, InterruptType::QueueEnd);
        assert_eq!(reg.0, 0x40000000);
    }

    #[test]
    fn test_irq_enable1() {
        let reg = IrqEnable1(0x0);

        let reg = reg.enable_interrupt(8, InterruptType::HalfPackage);
        assert!(reg.is_interrupt_enabled(8, InterruptType::HalfPackage));
        assert_eq!(reg.0, 0x1);

        let reg = reg.enable_interrupt(15, InterruptType::QueueEnd);
        assert!(reg.is_interrupt_enabled(15, InterruptType::QueueEnd));
        assert_eq!(reg.0, 0x40000001);

        let reg = reg.disable_interrupt(8, InterruptType::HalfPackage);
        assert!(!reg.is_interrupt_enabled(8, InterruptType::HalfPackage));
        assert_eq!(reg.0, 0x40000000);
    }

    #[test]
    fn test_irq_pending0() {
        let reg = IrqPending0(0x0);

        let reg = reg.clear_irq(0, InterruptType::HalfPackage);
        assert!(reg.if_irq_pending(0, InterruptType::HalfPackage));
        assert_eq!(reg.0, 0x1);

        let reg = reg.clear_irq(1, InterruptType::PackageEnd);
        assert!(reg.if_irq_pending(1, InterruptType::PackageEnd));
        assert_eq!(reg.0, 0x21);

        // Test reading existing pending
        let reg = IrqPending0(0x4);
        assert!(reg.if_irq_pending(0, InterruptType::QueueEnd));
        assert!(!reg.if_irq_pending(0, InterruptType::HalfPackage));
    }

    #[test]
    fn test_irq_pending1() {
        let reg = IrqPending1(0x0);

        let reg = reg.clear_irq(8, InterruptType::HalfPackage);
        assert!(reg.if_irq_pending(8, InterruptType::HalfPackage));
        assert_eq!(reg.0, 0x1);

        let reg = reg.clear_irq(15, InterruptType::QueueEnd);
        assert!(reg.if_irq_pending(15, InterruptType::QueueEnd));
        assert_eq!(reg.0, 0x40000001);

        // Test reading existing pending
        let reg = IrqPending1(0x2);
        assert!(reg.if_irq_pending(8, InterruptType::PackageEnd));
        assert!(!reg.if_irq_pending(8, InterruptType::HalfPackage));
    }

    #[test]
    fn test_auto_gating() {
        let reg = AutoGating(0x0);

        // Test DMA channel circuit gating
        let reg = reg.disable_dma_chan_circuit_gating();
        assert!(!reg.is_dma_chan_circuit_gating_enabled());
        assert_eq!(reg.0, 0x1);

        let reg = reg.enable_dma_chan_circuit_gating();
        assert!(reg.is_dma_chan_circuit_gating_enabled());
        assert_eq!(reg.0, 0x0);

        // Test DMA common circuit gating
        let reg = reg.disable_dma_common_circuit_gating();
        assert!(!reg.is_dma_common_circuit_gating_enabled());
        assert_eq!(reg.0, 0x2);

        // Test DMA MCLK circuit gating
        let reg = reg.disable_dma_mclk_circuit_gating();
        assert!(!reg.is_dma_mclk_circuit_gating_enabled());
        assert_eq!(reg.0, 0x6);

        // Test init recommended
        let reg = AutoGating::init_recommended();
        assert_eq!(reg.0, 0x4);
        assert!(!reg.is_dma_mclk_circuit_gating_enabled());
        assert!(reg.is_dma_common_circuit_gating_enabled());
        assert!(reg.is_dma_chan_circuit_gating_enabled());
    }

    #[test]
    fn test_status() {
        let reg = Status(0x8000FFFF);
        assert_eq!(reg.0, 0x8000FFFF);

        // Test DMA channels status
        assert_eq!(reg.dma_channels_status(), 0xFFFF);

        // Test individual channel status
        for ch in 0..16 {
            assert!(reg.is_dma_channel_busy(ch));
            assert!(!reg.is_dma_channel_idle(ch));
        }

        // Test specific channels
        let reg = Status(0x0001);
        assert!(reg.is_dma_channel_busy(0));
        assert!(reg.is_dma_channel_idle(1));

        let reg = Status(0x8000);
        assert!(reg.is_dma_channel_busy(15));
        assert!(reg.is_dma_channel_idle(0));

        // Test empty status
        let reg = Status(0x0);
        assert_eq!(reg.dma_channels_status(), 0x0);
        for ch in 0..16 {
            assert!(!reg.is_dma_channel_busy(ch));
            assert!(reg.is_dma_channel_idle(ch));
        }
    }

    #[test]
    #[should_panic(expected = "Channel must be 0-7 for IrqEnable0")]
    fn test_irq_enable0_invalid_channel() {
        let reg = IrqEnable0(0x0);
        reg.enable_interrupt(8, InterruptType::HalfPackage);
    }

    #[test]
    #[should_panic(expected = "Channel must be 8-15 for IrqEnable1")]
    fn test_irq_enable1_invalid_channel() {
        let reg = IrqEnable1(0x0);
        reg.enable_interrupt(7, InterruptType::HalfPackage);
    }

    #[test]
    #[should_panic(expected = "Channel must be 0-7")]
    fn test_irq_pending0_invalid_channel() {
        let reg = IrqPending0(0x0);
        reg.if_irq_pending(8, InterruptType::HalfPackage);
    }

    #[test]
    #[should_panic(expected = "Channel must be 8-15")]
    fn test_irq_pending1_invalid_channel() {
        let reg = IrqPending1(0x0);
        reg.if_irq_pending(7, InterruptType::HalfPackage);
    }

    #[test]
    #[should_panic(expected = "Channel number must be 0-15")]
    fn test_status_invalid_channel() {
        let reg = Status(0x0);
        reg.is_dma_channel_busy(16);
    }
}
