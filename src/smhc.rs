use volatile_register::RW;

#[repr(C)]
pub struct RegisterBlock {
    /// 0x00 - SMC Global Control Register.
    pub global_control: RW<u32>,
    /// 0x04 - SMC Clock Control Register.
    pub clock_control: RW<u32>,
    /// 0x08 - SMC Time Out Register.
    pub timeout: RW<u32>,
    /// 0x0C - SMC Bus Width Register.
    pub width: RW<u32>,
    /// 0x10 - SMC Block Size Register.
    pub block_size: RW<u32>,
    /// 0x14 - SMC Byte Count Register.
    pub byte_count: RW<u32>,
    /// 0x18 - SMC Command Register.
    pub command: RW<u32>,
    /// 0x1C - SMC Argument Register.
    pub argument: RW<u32>,
    /// 0x20 ..= 0x2C - SMC Response Registers 0..=3.
    pub responses: [RW<u32>; 4],
    /// 0x8C - SMC IDMAC Interrupt Enable Register.
    pub interrupt_enable: RW<u32>,
    /// 0x38 - SMC Raw Interrupt Status Register.
    pub interrupt_state_raw: RW<u32>,
    /// 0x3C - SMC Status Register.
    pub status: RW<u32>,
    /// 0x40 - SMC FIFO Threshold Watermark Register.
    pub fifo_threshold: RW<u32>,
    /// 0x5c - SMC2 Newtiming Set Register.
    pub ntsr: RW<u32>,
    /// 0x80 - SMC IDMAC Control Register.
    pub dma_control: RW<u32>,
    /// 0x84 - SMC IDMAC Descriptor List Base Address Register.
    pub dma_descriptor_base: RW<u32>,
    /// 0x88 - SMC IDMAC Status Register.
    pub dma_state: RW<u32>,
    /// 0x140 - Drive Delay Control register.
    pub drive_delay_control: RW<u32>,
    /// 0x184 - deskew control control register.
    pub skew_control: RW<u32>,
    /// 0x200 - SMC FIFO Access Address.
    pub fifo: RW<u32>,
}
