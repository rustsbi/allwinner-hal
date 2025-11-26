//! LEDC peripheral registers.

use volatile_register::{RO, RW, WO};

/// LEDC Controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// LEDC Control Register.
    pub ledc_control: RW<LedcControl>,
    /// LEDC T0 & T1 Timing Control Register.
    pub led_t01_timing_ctrl_reg: RW<LedT01TimingControl>,
    /// LEDC Data Finish Counter Register.
    pub ledc_data_finish_cnt_reg: RW<LedcDataFinishCntReg>,
    /// LEDC Reset Timing Control Register.
    pub led_reset_timing_ctrl_reg: RW<LedResetTimingCtrlReg>,
    /// LEDC Wait Time0 Control Register.
    pub ledc_wait_time0_ctrl_reg: RW<LedcWaitTime0CtrlReg>,
    /// LEDC Data Register.
    /// LEDC Display Data.(The low 24-bit is valid.)
    pub ledc_data_reg: WO<u32>,
    /// LEDC DMA Control Register.
    pub ledc_dma_ctrl_reg: RW<LedcDmaCtrlReg>,
    /// LEDC Interrupt Control Register.
    pub ledc_interrupt_ctrl_reg: RW<LedcInterruptCtrlReg>,
    /// LEDC Interrupt Status Register.
    pub ledc_int_sts_reg: RW<LedcInterruptStatusReg>,
    _reserved0: [u8; 4],
    /// LEDC Wait Time1 Control Register.
    pub ledc_wait_time1_ctrl_reg: RW<LedcWaitTime1CtrlReg>,
    /// LEDC FIFO Data Registers.
    /// Internal FIFO data of LEDC.
    /// The low 24-bit is valid.
    pub fifo: [RO<u32>; 32],
}

/// LEDC RGB mode.
///
/// By default, the software configures data to LEDC according to
/// GRB (MSB) mode, the LEDC internal combines data to output to
/// the external LED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum RgbMode {
    GRB = 0b000,
    GBR = 0b001,
    RGB = 0b010,
    RBG = 0b011,
    BGR = 0b100,
    BRG = 0b101,
}

/// LEDC Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedcControl(u32);

impl LedcControl {
    /// That the bit is enabled indicates LEDC can be started when LEDC
    /// data finished transmission or LEDC_EN is cleared to 0 by
    /// hardware in LEDC_SOFT_RESET situation.
    const LED_EN: u32 = 1 << 0;
    /// Write 1 to clear it automatically.
    /// The ranges of LEDC soft reset include the following points: all
    /// internal status registers, the control state machine returns to in
    /// idle status, the LEDC FIFO read & write point is cleared to 0, the
    /// LEDC interrupt is cleared; and the affected registers are
    /// followed.
    const LED_SOFT_RST: u32 = 1 << 1;
    /// MSB control for Blue data.
    const LED_MSB_B: u32 = 1 << 2;
    /// MSB control for Red data.
    const LED_MSB_R: u32 = 1 << 3;
    /// MSB control for Green data.
    const LED_MSB_G: u32 = 1 << 4;
    /// Adjust sequence of the combined GRB data.
    const LED_MSB_TOP: u32 = 1 << 5;
    /// The software writes 1 to the bit, the CPU triggers LEDC to
    /// transfer a reset to LED.
    /// Only when LEDC is in IDLE status, the reset can be performed.
    /// After the reset finished, the control state machine returns to
    /// the IDLE status. To return LEDC to the IDLE status, it also needs
    /// to be used with SOFT_RESET.
    /// When the software sets the bit, the software can read the bit
    /// to check if the reset is complete.
    const RESET_LED_EN: u32 = 1 << 10;

    /// LEDC is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        (self.0 & Self::LED_EN) != 0
    }

    /// Enable the LEDC.
    #[inline]
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::LED_EN)
    }

    /// Disable the LEDC.
    #[inline]
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::LED_EN)
    }

    /// Get the red LEDC MSB control bit.
    #[inline]
    pub const fn is_red_msb(self) -> bool {
        (self.0 & Self::LED_MSB_R) != 0
    }
    /// Get the blue LEDC MSB control bit.
    #[inline]
    pub const fn is_blue_msb(self) -> bool {
        (self.0 & Self::LED_MSB_B) != 0
    }
    /// Get the green LEDC MSB control bit.
    #[inline]
    pub const fn is_green_msb(self) -> bool {
        (self.0 & Self::LED_MSB_G) != 0
    }

    /// Set or clear the red LEDC MSB control bit.
    #[inline]
    pub const fn set_red_msb(self, msb: bool) -> Self {
        let mut value = self.0;
        if msb {
            value |= Self::LED_MSB_R;
        } else {
            value &= !Self::LED_MSB_R;
        }
        Self(value)
    }
    /// Set or clear the blue LEDC MSB control bit.
    #[inline]
    pub const fn set_blue_msb(self, msb: bool) -> Self {
        let mut value = self.0;
        if msb {
            value |= Self::LED_MSB_B;
        } else {
            value &= !Self::LED_MSB_B;
        }
        Self(value)
    }
    /// Set or clear the green LEDC MSB control bit.
    #[inline]
    pub const fn set_green_msb(self, msb: bool) -> Self {
        let mut value = self.0;
        if msb {
            value |= Self::LED_MSB_G;
        } else {
            value &= !Self::LED_MSB_G;
        }
        Self(value)
    }

    /// LEDC MSB_TOP control bit.
    #[inline]
    pub const fn is_msb_top(self) -> bool {
        (self.0 & Self::LED_MSB_TOP) != 0
    }

    /// Set or clear LEDC MSB_TOP control bit.
    #[inline]
    pub const fn set_msb_top(self, msb_top: bool) -> Self {
        let mut value = self.0;
        if msb_top {
            value |= Self::LED_MSB_TOP;
        } else {
            value &= !Self::LED_MSB_TOP;
        }
        Self(value)
    }

    /// Reset LED Enable bit status.
    #[inline]
    pub const fn set_reset_led_enable(self) -> Self {
        Self(self.0 | Self::RESET_LED_EN)
    }
    /// Check whether reset is done.
    #[inline]
    pub const fn is_reset_done(self) -> bool {
        (self.0 & Self::RESET_LED_EN) == 0
    }

    /// RGB mode selection.
    #[inline]
    pub const fn rgb_mode(self) -> RgbMode {
        let raw_mode = self.0 >> 6 & 0b111;
        match raw_mode {
            0b000 => RgbMode::GRB,
            0b001 => RgbMode::GBR,
            0b010 => RgbMode::RGB,
            0b011 => RgbMode::RBG,
            0b100 => RgbMode::BGR,
            0b101 => RgbMode::BRG,
            _ => unreachable!(),
        }
    }

    /// Set RGB mode.
    #[inline]
    pub const fn set_rgb_mode(self, mode: RgbMode) -> Self {
        // RGB mode is in bits [8:6].
        let mut value = self.0;
        value &= !(0b111 << 6);
        value |= (mode as u32) << 6;
        Self(value)
    }

    /// LEDC soft reset status.
    #[inline]
    pub const fn soft_reset(self) -> bool {
        (self.0 & Self::LED_SOFT_RST) != 0
    }

    /// Clear LEDC soft reset.
    #[inline]
    pub const fn clear_soft_reset(self) -> Self {
        // Write 1 to clear it.
        Self(self.0 | Self::LED_SOFT_RST)
    }

    /// Total length of transfer data.
    #[inline]
    pub const fn total_data_length(self) -> u32 {
        (self.0 >> 16) & 0xFFF
    }

    /// Set total length of transfer data.
    #[inline]
    pub const fn set_total_data_length(self, length: u32) -> Self {
        let mut value = self.0;
        value &= !(0xFFF << 16);
        value |= (length & 0xFFF) << 16;
        Self(value)
    }
}

/// LEDC T0 & T1 Timing Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedT01TimingControl(u32);

impl LedT01TimingControl {
    /// LED T1H time.
    /// Unit: cycle (24MHz), T1H_TIME = 42ns * (N+1).
    /// The default value is 882ns, the range is 80ns~2560ns.
    /// N: 1~3F. When is 0, T1H_TIME = 4F.
    const LED_T1H_TIME_SHIFT: u32 = 21;
    const LED_T1H_TIME_MASK: u32 = 0x3F;
    /// LED T1L time.
    /// Unit: cycle (24MHz), T1H_TIME = 42ns * (N+1).
    /// The default value is 294ns, the range is 80ns~1280ns.
    /// N: 1~1F. When is 0, T1H_TIME = 1F.
    const LED_T1L_TIME_SHIFT: u32 = 16;
    const LED_T1L_TIME_MASK: u32 = 0x1F;

    /// LED T0H time.
    /// Unit: cycle (24MHz), T1H_TIME = 42ns * (N+1).
    /// The default value is 336ns, the range is 80ns~1280ns.
    /// N: 1~1F. When is 0, T0H_TIME = 1F.
    const LED_T0H_TIME_SHIFT: u32 = 6;
    const LED_T0H_TIME_MASK: u32 = 0xF;
    /// LED T0L time.
    /// Unit: cycle (24MHz), T1H_TIME = 42ns * (N+1).
    /// The default value is 336ns, the range is 80ns~2560ns.
    /// N: 1~3F. When is 0, T0H_TIME = 3F.
    const LED_T0L_TIME_SHIFT: u32 = 0;
    const LED_T0L_TIME_MASK: u32 = 0x1F;

    /// Get the led T1H time.
    #[inline]
    pub const fn led_t1h_time(self) -> u32 {
        (self.0 >> Self::LED_T1H_TIME_SHIFT) & Self::LED_T1H_TIME_MASK
    }
    /// Get the led T1L time.
    #[inline]
    pub const fn led_t1l_time(self) -> u32 {
        (self.0 >> Self::LED_T1L_TIME_SHIFT) & Self::LED_T1L_TIME_MASK
    }
    /// Get the led T0H time.
    #[inline]
    pub const fn led_t0h_time(self) -> u32 {
        (self.0 >> Self::LED_T0H_TIME_SHIFT) & Self::LED_T0H_TIME_MASK
    }
    /// Get the led T0L time.
    #[inline]
    pub const fn led_t0l_time(self) -> u32 {
        (self.0 >> Self::LED_T0L_TIME_SHIFT) & Self::LED_T0L_TIME_MASK
    }
    /// Set the led T1H time.
    #[inline]
    pub const fn set_led_t1h_time(self, t1h_time: u32) -> Self {
        let mut value = self.0;
        value &= !Self::LED_T1H_TIME_MASK << Self::LED_T1H_TIME_SHIFT;
        value |= (t1h_time & Self::LED_T1H_TIME_MASK) << Self::LED_T1H_TIME_SHIFT;
        Self(value)
    }
    /// Set the led T1L time.
    #[inline]
    pub const fn set_led_t1l_time(self, t1l_time: u32) -> Self {
        let mut value = self.0;
        value &= !Self::LED_T1L_TIME_MASK << Self::LED_T1L_TIME_SHIFT;
        value |= (t1l_time & Self::LED_T1L_TIME_MASK) << Self::LED_T1L_TIME_SHIFT;
        Self(value)
    }
    /// Set the led T0H time.
    #[inline]
    pub const fn set_led_t0h_time(self, t0h_time: u32) -> Self {
        let mut value = self.0;
        value &= !Self::LED_T0H_TIME_MASK << Self::LED_T0H_TIME_SHIFT;
        value |= (t0h_time & Self::LED_T0H_TIME_MASK) << Self::LED_T0H_TIME_SHIFT;
        Self(value)
    }
    /// Set the led T0L time.
    #[inline]
    pub const fn set_led_t0l_time(self, t0l_time: u32) -> Self {
        let mut value = self.0;
        value &= !Self::LED_T0L_TIME_MASK << Self::LED_T0L_TIME_SHIFT;
        value |= (t0l_time & Self::LED_T0L_TIME_MASK) << Self::LED_T0L_TIME_SHIFT;
        Self(value)
    }
}

/// LEDC Data Finish Counter Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedcDataFinishCntReg(u32);

impl LedcDataFinishCntReg {
    /// LED_WAIT_DATA_TIME
    ///
    /// The value is the time that internal FIFO in LEDC is waiting for data.
    /// When the time is exceeded, the LEDC will send the wait_data_timeout_int interrupt.
    /// The value is about 3000 us by default.
    /// The adjust range is 80ns ~ 655us.
    /// led_wait_data_time = 42ns*(N+1).
    /// N: 1~1FFF. When the field is 0, LEDC_WAIT_DATA_TIME=1FFF.
    const LED_WAIT_DATA_TIME_OFFSET: u32 = 16;
    const LED_WAIT_DATA_TIME_MASK: u32 = 0x2FFF;

    /// LED_DATA_FINISH_CNT
    ///
    /// The value is the total LED data that have been sent. (Range: 0~8k).
    const LED_DATA_FINISH_CNT_OFFSET: u32 = 0;
    const LED_DATA_FINISH_CNT_MASK: u32 = 0xFFF;

    /// Get the led wait data time.
    #[inline]
    pub const fn led_wait_data_time(self) -> u32 {
        (self.0 >> Self::LED_WAIT_DATA_TIME_OFFSET) & 0x1FFF
    }

    /// Set the led wait data time.
    #[inline]
    pub const fn set_led_wait_data_time(self, led_wait_data_time: u32) -> Self {
        let mut value = self.0;
        value &= !(Self::LED_WAIT_DATA_TIME_MASK << Self::LED_WAIT_DATA_TIME_OFFSET);
        value |=
            (led_wait_data_time & Self::LED_WAIT_DATA_TIME_MASK) << Self::LED_WAIT_DATA_TIME_OFFSET;
        Self(value)
    }

    /// Get the led data finish count.
    #[inline]
    pub const fn get_led_data_finish_cnt(self) -> u32 {
        (self.0 >> Self::LED_DATA_FINISH_CNT_OFFSET) & Self::LED_DATA_FINISH_CNT_MASK
    }
}

/// LEDC Reset Timing Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedResetTimingCtrlReg(u32);

impl LedResetTimingCtrlReg {
    /// TR_TIME
    ///
    /// Reset time control of LED lamp.
    /// Unit: cycle(24MHz), tr_time=42ns*(N+1).
    /// The default value is 300us.
    /// The adjust range is 80ns~327us.
    /// N: 1~1FFF.
    const TR_TIME_OFFSET: u32 = 16;
    const TR_TIME_MASK: u32 = 0x1FFF;

    /// LED_NUM
    ///
    /// The value is the number of external LED lamp. Maximum up to 1024.
    /// The default value 0 indicates that 1 LED lamp is external connected.
    /// The range is 0~1023.
    const LED_NUM_OFFSET: u32 = 0;
    const LED_NUM_MASK: u32 = 0x1FF;

    /// Get the tr time.
    #[inline]
    pub const fn tr_time(self) -> u32 {
        self.0 >> Self::TR_TIME_OFFSET & Self::TR_TIME_MASK
    }

    /// Set the tr time.
    #[inline]
    pub const fn set_tr_time(self, tr_time: u32) -> Self {
        let mut value = self.0;
        value &= !(Self::TR_TIME_MASK << Self::TR_TIME_OFFSET);
        value |= (tr_time & Self::TR_TIME_MASK) << Self::TR_TIME_OFFSET;
        Self(value)
    }

    /// Get the led num.
    #[inline]
    pub const fn led_num(self) -> u32 {
        (self.0 >> Self::LED_NUM_OFFSET) & Self::LED_NUM_MASK
    }

    /// Set the led num.
    #[inline]
    pub const fn set_led_num(self, led_num: u32) -> Self {
        let mut value = self.0;
        value &= !(Self::LED_NUM_MASK << Self::LED_NUM_OFFSET);
        value |= (led_num & Self::LED_NUM_MASK) << Self::LED_NUM_OFFSET;
        Self(value)
    }
}

/// LEDC Wait Time0 Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedcWaitTime0CtrlReg(u32);

impl LedcWaitTime0CtrlReg {
    /// WAIT_TIM0_EN
    ///
    /// WAIT_TIME0 enable
    /// When it is 1, the controller automatically inserts waiting time between LED package data.
    /// 0: Disable.
    /// 1: Enable.
    const WAIT_TIME0_EN_OFFSET: u32 = 8;
    const WAIT_TIME0_EN_MASK: u32 = 1;

    /// TOTAL_WAIT_TIME0
    ///
    /// Waiting time between 2 LED data. THe LEDC output is low level.
    /// The adjust range is 80ns~10us.
    /// wait_time0=42ns*(N+1)
    /// Unit: cycle(24MHz)
    /// N: 1~FF
    const TOTAL_WAIT_TIME0_OFFSET: u32 = 0;
    const TOTAL_WAIT_TIME0_MASK: u32 = 0xFF;

    /// Get the wait time0 enable state.
    #[inline]
    pub const fn is_wait_time0_enabled(self) -> bool {
        ((self.0 >> Self::WAIT_TIME0_EN_OFFSET) & Self::WAIT_TIME0_EN_MASK) != 0
    }

    /// Enable the wait time0.
    #[inline]
    pub const fn enable_wait_time0(self) -> Self {
        let mut value = self.0;
        value |= (1 & Self::WAIT_TIME0_EN_MASK) << Self::WAIT_TIME0_EN_OFFSET;
        Self(value)
    }

    /// Disable the wait time0.
    #[inline]
    pub const fn disable_wait_time0(self) -> Self {
        let mut value = self.0;
        value &= !(Self::WAIT_TIME0_EN_MASK << Self::WAIT_TIME0_EN_OFFSET);
        Self(value)
    }

    /// Get the total wait time0.
    #[inline]
    pub const fn total_wait_time0(self) -> u32 {
        (self.0 >> Self::TOTAL_WAIT_TIME0_OFFSET) & Self::TOTAL_WAIT_TIME0_MASK
    }

    /// Set the total wait time0.
    #[inline]
    pub const fn set_total_wait_time0(self, total_wait_time0: u32) -> Self {
        let mut value = self.0;
        value &= !(Self::TOTAL_WAIT_TIME0_MASK << Self::TOTAL_WAIT_TIME0_OFFSET);
        value |= (total_wait_time0 & Self::TOTAL_WAIT_TIME0_MASK) << Self::TOTAL_WAIT_TIME0_OFFSET;
        Self(value)
    }
}

/// LEDC DMA Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedcDmaCtrlReg(u32);

impl LedcDmaCtrlReg {
    /// LEDC_DMA_EN
    ///
    /// LEDC DMA request enable.
    /// 0: Disable request for DMA transfer data.
    /// 1: Enable request for DMA transfer data.
    const LEDC_DMA_EN_OFFSET: u32 = 5;
    const LEDC_DMA_EN_MASK: u32 = 1;

    /// LEDC_FIFO_TRIG_LEVEL
    ///
    /// The remaining space of internal FIFO in LEDC.
    /// The internal FIFO in LEDC is 24*32.
    /// When the remaining space of internal FIFO in LEDC is more than or equal to LEDFIFO_TRIG_LEVEL,
    /// the DMA or the CPU request will generate. The default value is 15.
    ///
    /// The adjusted value is from 1 to 31. The recommended configuration is 7 or 15.
    /// When the configuration value is 0, LEDFIFO_TRIG_LEVEL=F.
    const LEDC_FIFO_TRIG_LEVEL_OFFSET: u32 = 0;
    const LEDC_FIFO_TRIG_LEVEL_MASK: u32 = 0x1F;

    /// Get the LEDC DMA enable state.
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        ((self.0 >> Self::LEDC_DMA_EN_OFFSET) & Self::LEDC_DMA_EN_MASK) != 0
    }

    /// Enable the LEDC DMA.
    #[inline]
    pub const fn enable_dma(self) -> Self {
        let mut value = self.0;
        value |= (1 & Self::LEDC_DMA_EN_MASK) << Self::LEDC_DMA_EN_OFFSET;
        Self(value)
    }

    /// Disable the LEDC DMA.
    #[inline]
    pub const fn disable_dma(self) -> Self {
        let mut value = self.0;
        value &= !((1 & Self::LEDC_DMA_EN_MASK) << Self::LEDC_DMA_EN_OFFSET);
        Self(value)
    }

    /// Get the LEDC FIFO trigger level.
    #[inline]
    pub const fn fifo_trig_level(self) -> u32 {
        (self.0 >> Self::LEDC_FIFO_TRIG_LEVEL_OFFSET) & Self::LEDC_FIFO_TRIG_LEVEL_MASK
    }

    /// Set the LEDC FIFO trigger level.
    #[inline]
    pub const fn set_fifo_trig_level(self, fifo_trig_level: u32) -> Self {
        let mut value = self.0;
        value &= !(Self::LEDC_FIFO_TRIG_LEVEL_MASK << Self::LEDC_FIFO_TRIG_LEVEL_OFFSET);
        value |= (fifo_trig_level & Self::LEDC_FIFO_TRIG_LEVEL_MASK)
            << Self::LEDC_FIFO_TRIG_LEVEL_OFFSET;
        Self(value)
    }
}

/// LEDC Interrupt Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedcInterruptCtrlReg(u32);

impl LedcInterruptCtrlReg {
    /// Get the global interrupt enable state.
    #[inline]
    pub const fn is_global_int_enabled(self) -> bool {
        ((self.0 >> 5) & 1) != 0
    }

    /// Enable the global interrupt.
    #[inline]
    pub const fn enable_global_int(self) -> Self {
        let mut value = self.0;
        value |= 1 << 5;
        Self(value)
    }

    /// Disable the global interrupt.
    #[inline]
    pub const fn disable_global_int(self) -> Self {
        let mut value = self.0;
        value &= !(1 << 5);
        Self(value)
    }

    /// Get the fifo overflow interrupt enable state.
    #[inline]
    pub const fn is_fifo_overflow_int_enabled(self) -> bool {
        ((self.0 >> 4) & 1) != 0
    }

    /// Enable the fifo overflow interrupt.
    #[inline]
    pub const fn enable_fifo_overflow_int(self) -> Self {
        let mut value = self.0;
        value |= 1 << 4;
        Self(value)
    }

    /// Disable the fifo overflow interrupt.
    #[inline]
    pub const fn disable_fifo_overflow_int(self) -> Self {
        let mut value = self.0;
        value &= !(1 << 4);
        Self(value)
    }

    /// Get the waitdata interrupt enable state.
    #[inline]
    pub const fn is_waitdata_int_enabled(self) -> bool {
        ((self.0 >> 3) & 1) != 0
    }

    /// Enable the waitdata interrupt.
    #[inline]
    pub const fn enable_waitdata_int(self) -> Self {
        let mut value = self.0;
        value |= 1 << 3;
        Self(value)
    }

    /// Disable the waitdata interrupt.
    #[inline]
    pub const fn disable_waitdata_int(self) -> Self {
        let mut value = self.0;
        value &= !(1 << 3);
        Self(value)
    }

    /// Get the fifo request cpu data interrupt enable state.
    #[inline]
    pub const fn is_cpureq_int_enabled(self) -> bool {
        ((self.0 >> 1) & 1) != 0
    }

    /// Enable the fifo request cpu data interrupt.
    #[inline]
    pub const fn enable_cpureq_int(self) -> Self {
        let mut value = self.0;
        value |= 1 << 1;
        Self(value)
    }

    /// Disable the fifo request cpu data interrupt.
    #[inline]
    pub const fn disable_cpureq_int(self) -> Self {
        let mut value = self.0;
        value &= !(1 << 1);
        Self(value)
    }

    /// Get the data transmission complete interrupt enable state.
    #[inline]
    pub const fn is_transfer_finish_int_enabled(self) -> bool {
        (self.0 & 1) != 0
    }

    /// Enable the data transmission complete interrupt.
    #[inline]
    pub const fn enable_transfer_finish_int(self) -> Self {
        let mut value = self.0;
        value |= 1;
        Self(value)
    }

    /// Disable the data transmission complete interrupt.
    #[inline]
    pub const fn disable_transfer_finish_int(self) -> Self {
        let mut value = self.0;
        value &= !(1 << 0);
        Self(value)
    }
}

/// LEDC Interrupt Status Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedcInterruptStatusReg(u32);

impl LedcInterruptStatusReg {
    /// FIFO empty status flag.
    const FIFO_EMPTY_OFFSET: u32 = 17;
    const FIFO_EMPTY_MASK: u32 = 1;

    /// FIFO full status flag.
    const FIFO_FULL_OFFSET: u32 = 16;
    const FIFO_FULL_MASK: u32 = 1;

    /// FIFO internal valid data depth.
    ///
    /// It indicates the space FIFO has been occupied.
    const FIFO_WLW_OFFSET: u32 = 10;
    const FIFO_WLW_MASK: u32 = 0x3F;

    /// FIFO overflow interrupt.
    ///
    /// The data written by external is more than the maximum storage space of LED FIFO,
    /// the LEDC will be in the data loss state.
    /// At this time, the software needs to deal with the abnormal situation.
    /// The processnig mode is as follows.
    ///
    /// (1) The software can query LED_FIFO_DATA_REG to determine
    ///     which data has been storedin the internal FIFO of LEDC.
    /// (2) The LEDC performs soft_reset operation to refresh all data.
    ///
    /// 0: FIFO not overflow.
    /// 1: FIFO overflow.
    ///
    /// Write 1 to clear it.
    const FIFO_OVERFLOW_INT_OFFSET: u32 = 4;
    const FIFO_OVERFLOW_INT_MASK: u32 = 1;

    /// FIFO waitdata timeout interrupt.
    ///
    /// When internal FIFO of LEDC cannot get data because of some abnormal situation after led_wait_data_time,
    /// the timeout interrupt is set, the LEDC is in WAIT_DATA state,
    /// the LEDC outputs a level state configured by LED_POLARITY; in the course of wait_data, if the new data arrives,
    /// the LEDC will continue to send data, as this time software needs to notice whether the waiting time of LEDC exceeds the operation time of reset.
    /// If the waiting time of LEDC exceeds the operation time of reset (this is equivalent to reset operation sent by LEDC),
    /// the LED may enter in refresh state, the data has not been sent.
    ///
    /// 0: LEDC not timeout.
    /// 1: LEDC timeout.
    ///
    /// Write 1 to clear it.
    const FIFO_WAITDATA_TIMEOUT_INT_OFFSET: u32 = 3;
    const FIFO_WAITDATA_TIMEOUT_INT_MASK: u32 = 1;

    /// FIFO request CPU data interrupt.
    ///
    /// When FIFO data is less than the threshold, the interrupt will be reported to the CPU.
    ///
    /// 0: FIFO does not request that CPU transfers data.
    /// 1: FIFO requests that CPU transfers data.
    const FIFO_CPUREQ_INT_OFFSET: u32 = 1;
    const FIFO_CPUREQ_INT_MASK: u32 = 1;

    /// Data transfer complete interrupt.
    ///
    /// The value indicates that the data configured as total_data_length is transferred completely.
    ///
    /// 0: Data is not transferred completely.
    /// 1: Data is transferred completely.
    const FIFO_TRANS_FINISH_INT_OFFSET: u32 = 0;
    const FIFO_TRANS_FINISH_INT_MASK: u32 = 1;

    /// Get the fifo empty flag.
    #[inline]
    pub const fn is_fifo_empty(self) -> bool {
        ((self.0 >> Self::FIFO_EMPTY_OFFSET) & Self::FIFO_EMPTY_MASK) != 0
    }

    /// Get the fifo full flag.
    #[inline]
    pub const fn is_fifo_full(self) -> bool {
        ((self.0 >> Self::FIFO_FULL_OFFSET) & Self::FIFO_FULL_MASK) != 0
    }

    /// Get the fifo internal valid data depth.
    #[inline]
    pub const fn fifo_internal_valid_data_depth(self) -> u32 {
        (self.0 >> Self::FIFO_WLW_OFFSET) & Self::FIFO_WLW_MASK
    }

    /// Get the fifo overflow interrupt status.
    #[inline]
    pub const fn fifo_overflow_interrupt(self) -> bool {
        ((self.0 >> Self::FIFO_OVERFLOW_INT_OFFSET) & Self::FIFO_OVERFLOW_INT_MASK) != 0
    }

    /// Clear the fifo overflow interrupt status.
    #[inline]
    pub const fn clear_fifo_overflow_interrupt(self) -> Self {
        Self(self.0 | (Self::FIFO_OVERFLOW_INT_MASK << Self::FIFO_OVERFLOW_INT_OFFSET))
    }

    /// Get the waitdata timeout interrupt status.
    #[inline]
    pub const fn waitdata_timeout_interrupt(self) -> bool {
        ((self.0 >> Self::FIFO_WAITDATA_TIMEOUT_INT_OFFSET) & Self::FIFO_WAITDATA_TIMEOUT_INT_MASK)
            != 0
    }

    /// Clear the waitdata timeout interrupt status.
    #[inline]
    pub const fn clear_waitdata_timeout_interrupt(self) -> Self {
        Self(
            self.0
                | (Self::FIFO_WAITDATA_TIMEOUT_INT_MASK << Self::FIFO_WAITDATA_TIMEOUT_INT_OFFSET),
        )
    }

    /// Get the fifo request CPU data interrupt status.
    #[inline]
    pub const fn fifo_cpu_req_interrupt(self) -> bool {
        ((self.0 >> Self::FIFO_CPUREQ_INT_OFFSET) & Self::FIFO_CPUREQ_INT_MASK) != 0
    }

    /// Clear the fifo request CPU data interrupt status.
    #[inline]
    pub const fn clear_fifo_cpu_req_interrupt(self) -> Self {
        Self(self.0 | (Self::FIFO_CPUREQ_INT_MASK << Self::FIFO_CPUREQ_INT_OFFSET))
    }

    /// Get the data transfer complete interrupt status.
    #[inline]
    pub const fn transfer_finish_interrupt(self) -> bool {
        ((self.0 >> Self::FIFO_TRANS_FINISH_INT_OFFSET) & Self::FIFO_TRANS_FINISH_INT_MASK) != 0
    }

    /// Clear the data transfer complete interrupt status.
    #[inline]
    pub const fn clear_transfer_finish_interrupt(self) -> Self {
        Self(self.0 | (Self::FIFO_TRANS_FINISH_INT_MASK << Self::FIFO_TRANS_FINISH_INT_OFFSET))
    }
}

/// LEDC Wait Time1 Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LedcWaitTime1CtrlReg(u32);

impl LedcWaitTime1CtrlReg {
    /// Get the wait time1 enable state.
    #[inline]
    pub const fn is_wait_time1_enabled(self) -> bool {
        ((self.0 >> 31) & 1) != 0
    }

    /// Enable the wait time1.
    #[inline]
    pub const fn enable_wait_time1(self) -> Self {
        let mut value = self.0;
        value |= 1 << 31;
        Self(value)
    }

    /// Disable the wait time1.
    #[inline]
    pub const fn disable_wait_time1(self) -> Self {
        let mut value = self.0;
        value &= !(1 << 31);
        Self(value)
    }

    /// Get the total wait time1.
    ///
    /// TOTAL_WAIT_TIME1:
    /// Waiting time between 2 frame data.
    /// The LEDC output is low level.
    /// The adjust range is 80ns~85ns. wait_time1=42ns*(N+1)
    /// Unit cycle(24MHz)
    /// N: 0x80~0x7FFFFFFF
    /// If the value is 0, TOTAL_WAIT_TIME1=0x7FFFFFFF
    #[inline]
    pub const fn total_wait_time1(self) -> u32 {
        self.0 & 0x7FFFFFFF
    }

    /// Set the total wait time1.
    #[inline]
    pub const fn set_total_wait_time1(self, total_wait_time1: u32) -> Self {
        let mut value = self.0;
        value &= 0x7FFFFFFF;
        value |= total_wait_time1 & 0x7FFFFFFF;
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ledc_control_default_value() {
        use super::{LedcControl, RgbMode};

        let reg = LedcControl(0x0000_003C);
        assert!(!reg.is_enabled());
        assert!(!reg.soft_reset());
        assert!(reg.is_blue_msb());
        assert!(reg.is_red_msb());
        assert!(reg.is_green_msb());
        assert!(reg.is_msb_top());
        assert_eq!(reg.rgb_mode(), RgbMode::GRB);
        assert_eq!(reg.total_data_length(), 0);
    }

    #[test]
    fn test_ledc_t01_timing_control_default_value() {
        use super::LedT01TimingControl;

        let reg = LedT01TimingControl(0x0286_01D3);
        assert_eq!(reg.led_t1h_time(), 0x14);
        assert_eq!(reg.led_t1l_time(), 0x6);
        assert_eq!(reg.led_t0h_time(), 0x7);
        assert_eq!(reg.led_t0l_time(), 0x13);
    }

    #[test]
    fn test_ledc_data_finish_cnt_reg_default_value() {
        use super::LedcDataFinishCntReg;

        let reg = LedcDataFinishCntReg(0x1D4C_0000);
        assert_eq!(reg.led_wait_data_time(), 0x1D4C);
        assert_eq!(reg.get_led_data_finish_cnt(), 0x0);
    }

    #[test]
    fn test_ledc_reset_timing_ctrl_reg_default_value() {
        use super::LedResetTimingCtrlReg;

        let reg = LedResetTimingCtrlReg(0x1D4C_0000);
        assert_eq!(reg.tr_time(), 0x1D4C);
        assert_eq!(reg.led_num(), 0x0);
    }

    #[test]
    fn test_ledc_wait_time0_ctrl_reg_default_value() {
        use super::LedcWaitTime0CtrlReg;

        let reg = LedcWaitTime0CtrlReg(0x0000_00FF);
        assert_eq!(reg.is_wait_time0_enabled(), false);
        assert_eq!(reg.total_wait_time0(), 0xFF);
    }

    #[test]
    fn test_ledc_dma_ctrl_reg_default_value() {
        use super::LedcDmaCtrlReg;

        let reg = LedcDmaCtrlReg(0x0000_002F);
        assert_eq!(reg.is_dma_enabled(), true);
        assert_eq!(reg.fifo_trig_level(), 0x0F);
    }

    #[test]
    fn test_ledc_interrupt_status_reg_default_value() {
        use super::LedcInterruptStatusReg;

        let reg = LedcInterruptStatusReg(0x0002_0000);
        assert!(reg.is_fifo_empty());
        assert!(!reg.is_fifo_full());
        assert!(!reg.fifo_overflow_interrupt());
        assert!(!reg.waitdata_timeout_interrupt());
        assert!(!reg.fifo_cpu_req_interrupt());
        assert!(!reg.transfer_finish_interrupt());
        assert_eq!(reg.fifo_internal_valid_data_depth(), 0);
    }

    #[test]
    fn test_ledc_wait_time1_ctrl_reg_default_value() {
        use super::LedcWaitTime1CtrlReg;

        let reg = LedcWaitTime1CtrlReg(0x01FF_FFFF);
        assert!(!reg.is_wait_time1_enabled());
        assert_eq!(reg.total_wait_time1(), 0x01FF_FFFF);
    }
}
