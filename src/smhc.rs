use volatile_register::RW;

#[repr(C)]
pub struct RegisterBlock {
    /// 0x00 - SMC Global Control Register.
    pub gctrl: RW<u32>,
    /// 0x04 - SMC Clock Control Register.
    pub clkcr: RW<u32>,
    /// 0x08 - SMC Time Out Register.
    pub timeout: RW<u32>,
    /// 0x0C - SMC Bus Width Register.
    pub width: RW<u32>,
    /// 0x10 - SMC Block Size Register.
    pub blksz: RW<u32>,
    /// 0x14 - SMC Byte Count Register.
    pub bytecnt: RW<u32>,
    /// 0x18 - SMC Command Register.
    pub cmd: RW<u32>,
    /// 0x1C - SMC Argument Register.
    pub arg: RW<u32>,
    /// 0x20 - SMC Response Register 0.
    pub resp0: RW<u32>,
    /// 0x24 - SMC Response Register 1.
    pub resp1: RW<u32>,
    /// 0x28 - SMC Response Register 2.
    pub resp2: RW<u32>,
    /// 0x2C - SMC Response Register 3.
    pub resp3: RW<u32>,
    /// 0x8C - SMC IDMAC Interrupt Enable Register.
    pub idie: RW<u32>,
    /// 0x38 - SMC Raw Interrupt Status Register.
    pub rint: RW<u32>,
    /// 0x3C - SMC Status Register.
    pub status: RW<u32>,
    /// 0x40 - SMC FIFO Threshold Watermark Register.
    pub ftrglevel: RW<u32>,
    /// 0x5c - SMC2 Newtiming Set Register.
    pub ntsr: RW<u32>,
    /// 0x80 - SMC IDMAC Control Register.
    pub dmac: RW<u32>,
    /// 0x84 - SMC IDMAC Descriptor List Base Address Register.
    pub dlba: RW<u32>,
    /// 0x88 - SMC IDMAC Status Register.
    pub idst: RW<u32>,
    /// 0x140 - Drive Delay Control register.
    pub drv_dl: RW<u32>,
    /// 0x184 - deskew control control register.
    pub skew_ctrl: RW<u32>,
    /// 0x200 - SMC FIFO Access Address.
    pub fifo: RW<u32>,
}
