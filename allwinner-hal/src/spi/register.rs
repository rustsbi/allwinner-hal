use core::cell::UnsafeCell;
use embedded_hal::spi::Mode;
use volatile_register::{RO, RW};

/// Serial Peripheral Interface registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: u32,
    pub gcr: RW<GlobalControl>,
    pub tcr: RW<TransferControl>,
    _reserved1: u32,
    pub ier: RW<u32>,
    pub isr: RW<u32>,
    pub fcr: RW<u32>,
    /// FIFO status register.
    pub fsr: RO<FifoStatus>,
    pub wcr: RW<u32>,
    _reserved2: u32,
    pub samp_dl: RW<u32>,
    _reserved3: u32,
    /// Master burst counter register.
    ///
    /// In master mode, this field specifies the total burst number.
    /// The totcal transfer data include transmit, receive parts and
    /// dummy burst.
    pub mbc: RW<u32>,
    /// Master transmit counter register.
    pub mtc: RW<u32>,
    /// Burst control counter register.
    pub bcc: RW<BurstControl>,
    _reserved4: u32,
    pub batcr: RW<u32>,
    pub ba_ccr: RW<u32>,
    pub tbr: RW<u32>,
    pub rbr: RW<u32>,
    _reserved5: [u32; 14],
    pub ndma_mode_ctl: RW<u32>,
    _reserved6: [u32; 93],
    pub txd: TXD,
    _reserved7: [u32; 63],
    pub rxd: RXD,
}

/// Global control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GlobalControl(u32);

impl GlobalControl {
    const SRST: u32 = 1 << 31;
    const TP_EN: u32 = 1 << 7;
    // const MODE_SELEC: u32 = 1 << 2;
    const MODE: u32 = 1 << 1;
    const EN: u32 = 1 << 0;
    /// Perform software reset to the SPI controller.
    #[inline]
    pub const fn software_reset(self) -> Self {
        Self(self.0 | Self::SRST)
    }
    /// Check if software reset request has finished.
    #[inline]
    pub const fn is_software_reset_finished(self) -> bool {
        self.0 & Self::SRST != 0
    }
    /// Set transmit pause enable flag.
    ///
    /// In master mode, if this flag is enabled, transmitting data will be
    /// stopped when receive FIFO is full.
    #[inline]
    pub const fn set_transmit_pause_enable(self, val: bool) -> Self {
        Self((self.0 & !Self::TP_EN) | if val { Self::TP_EN } else { 0 })
    }
    /// Check if transmit pause has enabled.
    #[inline]
    pub const fn transmit_pause_enabled(self) -> bool {
        self.0 & Self::TP_EN != 0
    }
    /// Set this peripheral to operate on master mode.
    #[inline]
    pub const fn set_master_mode(self) -> Self {
        Self(self.0 | Self::MODE)
    }
    /// Set this peripheral to operate on slave mode.
    #[inline]
    pub const fn set_slave_mode(self) -> Self {
        Self(self.0 & !Self::MODE)
    }
    /// Check if this peripheral operates on master mode.
    #[inline]
    pub const fn is_master_mode(self) -> bool {
        self.0 & Self::MODE != 0
    }
    /// Check if this peripheral operates on slave mode.
    #[inline]
    pub const fn is_slave_mode(self) -> bool {
        self.0 & Self::MODE == 0
    }
    /// Enable or disable this peripheral.
    #[inline]
    pub const fn set_enabled(self, val: bool) -> Self {
        Self((self.0 & !Self::EN) | if val { Self::EN } else { 0 })
    }
    /// Check if this peripheral is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::EN != 0
    }
}

/// Transfer control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TransferControl(u32);

impl TransferControl {
    const XCH: u32 = 1 << 31;
    const CPOL: u32 = 1 << 1;
    const CPHA: u32 = 1 << 0;
    /// Check if burst exchange has finished.
    #[inline]
    pub const fn burst_finished(self) -> bool {
        self.0 & Self::XCH == 0
    }
    /// Initiates burst exchange.
    #[inline]
    pub const fn start_burst_exchange(self) -> Self {
        Self(self.0 | Self::XCH)
    }
    /// Sets SPI work mode.
    #[inline]
    pub const fn set_work_mode(self, mode: Mode) -> Self {
        use embedded_hal::spi::{Phase, Polarity};
        let mut bits = self.0;
        match mode.polarity {
            Polarity::IdleLow => bits &= !Self::CPOL,
            Polarity::IdleHigh => bits |= Self::CPOL,
        }
        match mode.phase {
            Phase::CaptureOnFirstTransition => bits &= !Self::CPHA,
            Phase::CaptureOnSecondTransition => bits |= Self::CPHA,
        }
        Self(bits)
    }
}

/// Status of FIFO for current peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct FifoStatus(u32);

impl FifoStatus {
    const TB_WR: u32 = 0x1 << 31;
    const TB_CNT: u32 = 0x7 << 28;
    const TF_CNT: u32 = 0xff << 16;
    const RB_WR: u32 = 0x1 << 15;
    const RB_CNT: u32 = 0x7 << 12;
    const RF_CNT: u32 = 0xff << 0;

    #[inline]
    pub const fn transmit_buffer_write_enable(self) -> bool {
        self.0 & Self::TB_WR != 0
    }

    #[inline]
    pub const fn transmit_buffer_counter(self) -> u8 {
        ((self.0 & Self::TB_CNT) >> 28) as u8
    }

    #[inline]
    pub const fn transmit_fifo_counter(self) -> u8 {
        ((self.0 & Self::TF_CNT) >> 16) as u8
    }

    #[inline]
    pub const fn receive_buffer_write_enable(self) -> bool {
        self.0 & Self::RB_WR != 0
    }

    #[inline]
    pub const fn receive_buffer_counter(self) -> u8 {
        ((self.0 & Self::RB_CNT) >> 12) as u8
    }

    #[inline]
    pub const fn receive_fifo_counter(self) -> u8 {
        (self.0 & Self::RF_CNT) as u8
    }
}

/// Burst control counter for current peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct BurstControl(u32);

impl BurstControl {
    const QUAD_EN: u32 = 0x1 << 29;
    // const DRM: u32 = 0x1 << 28;
    const DBC: u32 = 0xf << 24;
    const STC: u32 = 0xfff << 0;
    /// Enable quad mode.
    #[inline]
    pub const fn quad_mode_enable(self) -> Self {
        Self(self.0 | Self::QUAD_EN)
    }
    /// Disable quad mode.
    #[inline]
    pub const fn quad_mode_disable(self) -> Self {
        Self(self.0 & !Self::QUAD_EN)
    }
    /// Check if quad mode is enabled.
    #[inline]
    pub const fn is_quad_mode_enabled(self) -> bool {
        self.0 & Self::QUAD_EN != 0
    }

    #[inline]
    pub const fn master_dummy_burst_counter(self) -> u8 {
        ((self.0 & Self::DBC) >> 24) as u8
    }

    #[inline]
    pub const fn set_master_dummy_burst_counter(self, val: u8) -> Self {
        Self((self.0 & !Self::DBC) | ((val as u32 & 0xf) << 24))
    }

    #[inline]
    pub const fn master_single_mode_transmit_counter(self) -> u32 {
        self.0 & Self::STC
    }

    #[inline]
    pub const fn set_master_single_mode_transmit_counter(self, val: u32) -> Self {
        Self((self.0 & !Self::STC) | (val & 0xfff))
    }
}

/// Transmit data register.
#[derive(Debug)]
#[repr(transparent)]
pub struct TXD(UnsafeCell<u32>);

impl TXD {
    /// Write 8-bit data.
    #[inline]
    pub fn write_u8(&self, val: u8) {
        unsafe { (self.0.get() as *mut u8).write_volatile(val) }
    }
    /// Write 16-bit data.
    #[inline]
    pub fn write_u16(&self, val: u16) {
        unsafe { (self.0.get() as *mut u16).write_volatile(val) }
    }
    /// Write 32-bit data.
    #[inline]
    pub fn write_u32(&self, val: u32) {
        unsafe { self.0.get().write_volatile(val) }
    }
}

/// Receive data register.
#[derive(Debug)]
#[repr(transparent)]
pub struct RXD(UnsafeCell<u32>);

impl RXD {
    /// Read 8-bit data.
    #[inline]
    pub fn read_u8(&self) -> u8 {
        unsafe { (self.0.get() as *const u8).read_volatile() }
    }
    /// Read 16-bit data.
    #[inline]
    pub fn read_u16(&self) -> u16 {
        unsafe { (self.0.get() as *const u16).read_volatile() }
    }
    /// Read 32-bit data.
    #[inline]
    pub fn read_u32(&self) -> u32 {
        unsafe { self.0.get().read_volatile() }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BurstControl, FifoStatus, GlobalControl, RXD, RegisterBlock, TXD, TransferControl,
    };
    use core::cell::UnsafeCell;
    use core::mem::offset_of;
    use embedded_hal::spi::Mode as SpiMode;
    #[test]
    fn offset_spi0() {
        assert_eq!(offset_of!(RegisterBlock, ier), 0x10);
        assert_eq!(offset_of!(RegisterBlock, samp_dl), 0x28);
        assert_eq!(offset_of!(RegisterBlock, mbc), 0x30);
        assert_eq!(offset_of!(RegisterBlock, ndma_mode_ctl), 0x88);
        assert_eq!(offset_of!(RegisterBlock, txd), 0x200);
        assert_eq!(offset_of!(RegisterBlock, rxd), 0x300);
    }

    #[test]
    fn test_spi_global_control() {
        let reg = GlobalControl(0x0).set_enabled(true);
        assert!(reg.is_enabled());
        assert_eq!(reg.0, 1 << 0);

        let reg = GlobalControl::default().set_master_mode();
        assert!(reg.is_master_mode());
        assert_eq!(reg.0, 1 << 1);

        let reg = GlobalControl::default().set_slave_mode();
        assert!(reg.is_slave_mode());
        assert_eq!(reg.0, 0x0);

        let reg = GlobalControl::default().set_transmit_pause_enable(true);
        assert!(reg.transmit_pause_enabled());
        assert_eq!(reg.0, 1 << 7);

        let reg = GlobalControl::default().software_reset();
        assert!(reg.is_software_reset_finished());
        assert_eq!(reg.0, 1 << 31);
    }

    #[test]
    fn test_spi_transfer_control() {
        let mut reg = TransferControl(0x0);
        reg = reg.start_burst_exchange();
        assert!(!reg.burst_finished());
        assert_eq!(reg.0, 1 << 31);

        reg = TransferControl(0x0);
        reg = reg.set_work_mode(SpiMode {
            polarity: embedded_hal::spi::Polarity::IdleHigh, // CPOL=1
            phase: embedded_hal::spi::Phase::CaptureOnSecondTransition, // CPHA=1
        });
        assert_eq!(reg.0, 0b11);
    }

    #[test]
    fn test_fifo_status_functions() {
        let mut val = FifoStatus(0x0); // Start with 0 to test all bits

        // Test transmit_buffer_write_enable (bit 31)
        val.0 = 0x80000000; // Set TB_WR
        assert_eq!(val.transmit_buffer_write_enable(), true);
        val.0 = 0x0;
        assert_eq!(val.transmit_buffer_write_enable(), false);

        // Test transmit_buffer_counter (bits 28-30)
        val.0 = 0x10000000; // TB_CNT = 1 (0b001 << 28)
        assert_eq!(val.transmit_buffer_counter(), 1);
        val.0 = 0x70000000; // TB_CNT = 7 (0b111 << 28)
        assert_eq!(val.transmit_buffer_counter(), 7);

        // Test transmit_fifo_counter (bits 16-23)
        val.0 = 0x00010000; // TF_CNT = 1
        assert_eq!(val.transmit_fifo_counter(), 1);
        val.0 = 0x00ff0000; // TF_CNT = 0xff
        assert_eq!(val.transmit_fifo_counter(), 0xff);

        // Test receive_buffer_write_enable (bit 15)
        val.0 = 0x00008000; // Set RB_WR
        assert_eq!(val.receive_buffer_write_enable(), true);

        // Test receive_buffer_counter (bits 12-14)
        val.0 = 0x00001000; // RB_CNT = 1 (0b001 << 12)
        assert_eq!(val.receive_buffer_counter(), 1);
        val.0 = 0x0;
        assert_eq!(val.receive_buffer_counter(), 0);

        // Test receive_fifo_counter (bits 0-7)
        val.0 = 0x000000ff; // RF_CNT = 0xff
        assert_eq!(val.receive_fifo_counter(), 0xff);
        val.0 = 0x0;
        assert_eq!(val.receive_fifo_counter(), 0);
    }

    #[test]
    fn test_spi_burst_control_functions() {
        let mut val = BurstControl(0x0); // Default value 0x0

        // Test Quad Mode Enable (bit 29)
        val = val.quad_mode_enable();
        assert!(val.is_quad_mode_enabled());
        assert_eq!(val.0 & (1 << 29), 1 << 29);

        val = val.quad_mode_disable();
        assert!(!val.is_quad_mode_enabled());
        assert_eq!(val.0 & (1 << 29), 0);

        // Test Master Dummy Burst Counter (bits 24-27)
        val = val.set_master_dummy_burst_counter(5);
        assert_eq!(val.master_dummy_burst_counter(), 5);
        assert_eq!(val.0 & (0xf << 24), 5 << 24);

        val = val.set_master_dummy_burst_counter(0);
        assert_eq!(val.master_dummy_burst_counter(), 0);
        assert_eq!(val.0 & (0xf << 24), 0);

        // Test Single Mode Transmit Counter (bits 0-23)
        val = val.set_master_single_mode_transmit_counter(0x123456);
        assert_eq!(val.master_single_mode_transmit_counter(), 0x0456);
        assert_eq!(val.0 & 0xfff, 0x0456);

        val = val.set_master_single_mode_transmit_counter(0);
        assert_eq!(val.master_single_mode_transmit_counter(), 0);
        assert_eq!(val.0 & 0xfff, 0);
    }

    #[test]
    fn test_spi_tx_data_functions() {
        let val = TXD(UnsafeCell::new(0x15)); // Default value from image

        // Test write_u8
        val.write_u8(0x2a);
        assert_eq!(unsafe { (val.0.get() as *const u8).read_volatile() }, 0x2a);

        // Test write_u16
        val.write_u16(0x1234);
        assert_eq!(
            unsafe { (val.0.get() as *const u16).read_volatile() },
            0x1234
        );

        // Test write_u32
        val.write_u32(0x12345678);
        assert_eq!(unsafe { val.0.get().read_volatile() }, 0x12345678);

        // Reset to default
        val.write_u32(0x15);
        assert_eq!(unsafe { val.0.get().read_volatile() }, 0x15);
    }

    #[test]
    fn test_spi_rx_data_functions() {
        let val = RXD(UnsafeCell::new(0x0)); // Assume default 0x0

        // Test read_u8
        unsafe { val.0.get().write_volatile(0x2a) }; // Simulate hardware write
        assert_eq!(val.read_u8(), 0x2a);

        // Test read_u16
        unsafe { val.0.get().write_volatile(0x1234) };
        assert_eq!(val.read_u16(), 0x1234);

        // Test read_u32
        unsafe { val.0.get().write_volatile(0x12345678) };
        assert_eq!(val.read_u32(), 0x12345678);

        // Reset to default
        unsafe { val.0.get().write_volatile(0x0) };
        assert_eq!(val.read_u32(), 0x0);
    }
}
