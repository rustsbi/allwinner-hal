use volatile_register::{RO, RW};

/// Direct Memory Access Controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// DMAC IRQ Enable Register 0.
    pub irq_enable0: RW<u32>,
    /// DMAC IRQ Enable Register 1.
    pub irq_enable1: RW<u32>,
    _reserved0: [u8; 0x8],
    /// DMAC IRQ Pending Register 0.
    pub irq_pending0: RW<u32>,
    /// DMAC IRQ Pending Register 1.
    pub irq_pending1: RW<u32>,
    _reserved1: [u8; 0x10],
    /// DMAC Auto Gating Register.
    pub auto_gating: RW<u32>,
    _reserved2: [u8; 0x4],
    /// DMAC Status Register.
    pub status: RO<u32>,
    _reserved3: [u8; 0xCC],
    /// DMAC Channels' Registers.
    pub channels: [ChannelRegisterBlock; 16],
}

/// DMAC channel registers.
#[repr(C)]
pub struct ChannelRegisterBlock {
    /// DMAC Channel Enable Register.
    pub enable: RW<u32>,
    /// DMAC Channel Pause Register.
    pub pause: RW<u32>,
    /// DMAC Channel Start Address Register.
    pub start_addr: RW<u32>,
    /// DMAC Channel Configuration Register.
    pub config: RO<u32>,
    /// DMAC Channel Current Source Register.
    pub current_src_addr: RO<u32>,
    /// DMAC Channel Current Destination Register.
    pub current_destination: RO<u32>,
    /// DMAC Channel Byte Counter Left Register.
    pub byte_counter_left: RO<u32>,
    /// DMAC Channel Parameter Register.
    pub parameter: RO<u32>,
    _reserved0: [u8; 0x8],
    /// DMAC Mode Register.
    pub mode: RW<u32>,
    /// DMAC Former Descriptor Address Register.
    pub former_desc_addr: RO<u32>,
    /// DMAC Package Number Register.
    pub package_num: RO<u32>,
    _reserved1: [u8; 0xC],
}

#[cfg(test)]
mod tests {
    use super::{ChannelRegisterBlock, RegisterBlock};
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
}
