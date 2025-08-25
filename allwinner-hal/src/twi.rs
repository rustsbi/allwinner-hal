//! Two-Wire Interface (TWI/I2C) peripheral registers.
//! Allwinner D1-H TWI Controller Register Definitions.

use volatile_register::{RO, RW, WO};

/// TWI Register Block Definition
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000: TWI Slave Address Register (TWI_ADDR)
    pub addr: RW<u32>,
    /// 0x0004: TWI Extended Slave Address Register (TWI_XADDR)
    pub xaddr: RW<u32>,
    /// 0x0008: TWI Data Byte Register (TWI_DATA)
    pub data: RW<u32>,
    /// 0x000C: TWI Control Register (TWI_CNTR)
    pub cntr: RW<Control>,
    /// 0x0010: TWI Status Register (TWI_STAT)
    pub stat: RO<Status>,
    /// 0x0014: TWI Clock Control Register (TWI_CCR)
    pub ccr: RW<ClockControl>,
    /// 0x0018: TWI Software Reset Register (TWI_SRST)
    pub srst: RW<SoftReset>,
    /// 0x001C: TWI Enhance Feature Register (TWI_EFR)
    pub efr: RW<EnhanceFeature>,
    /// 0x0020: TWI Line Control Register (TWI_LCR)
    pub lcr: RW<LineControl>,
    _reserved0: [u8; 0x200 - 0x024],
    /// 0x0200: TWI Driver Control Register (TWI_DRV_CTRL)
    pub drv_ctrl: RW<DrvControl>,
    /// 0x0204: TWI Driver Transmission Configuration Register (TWI_DRV_CFG)
    pub drv_cfg: RW<DrvCfg>,
    /// 0x0208: TWI Driver Slave ID Register (TWI_DRV_SLV)
    pub drv_slv: RW<DrvSlv>,
    /// 0x020C: TWI Driver Packet Format Register (TWI_DRV_FMT)
    pub drv_fmt: RW<DrvFmt>,
    /// 0x0210: TWI Driver Bus Control Register (TWI_DRV_BUS_CTRL)
    pub drv_bus_ctrl: RW<DrvBusCtrl>,
    /// 0x0214: TWI Driver Interrupt Control Register (TWI_DRV_INT_CTRL)
    pub drv_int_ctrl: RW<DrvIntCtrl>,
    /// 0x0218: TWI Driver DMA Configuration Register (TWI_DRV_DMA_CFG)
    pub drv_dma_cfg: RW<DrvDmaCfg>,
    /// 0x021C: TWI Driver FIFO Content Register (TWI_DRV_FIFO_CON)
    pub drv_fifo_con: RW<DrvFifoCon>,
    _reserved1: [u8; 0x300 - 0x0220],
    /// 0x0300: TWI Driver Send FIFO Access Register (TWI_DRV_SEND_FIFO_ACC)
    pub drv_send_fifo_acc: WO<u32>,
    /// 0x0304: TWI Driver Receive FIFO Access Register (TWI_DRV_RECV_FIFO_ACC)
    pub drv_recv_fifo_acc: RO<u32>,
}

/// 0x000C: TWI Control Register (Default: 0x0000_0000).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    const INT_EN: u32 = 1 << 7;
    const BUS_EN: u32 = 1 << 6;
    const M_STA: u32 = 1 << 5;
    const M_STP: u32 = 1 << 4;
    const INT_FLAG: u32 = 1 << 3;
    const A_ACK: u32 = 1 << 2;

    //Interrupt Enable
    pub const fn enable_interrupt(self) -> Self {
        Self(self.0 | Self::INT_EN)
    }
    pub const fn disable_interrupt(self) -> Self {
        Self(self.0 & !Self::INT_EN)
    }
    pub const fn is_interrupt_enabled(self) -> bool {
        self.0 & Self::INT_EN != 0
    }

    /// Bus Enable
    pub const fn enable_bus(self) -> Self {
        Self(self.0 | Self::BUS_EN)
    }
    pub const fn disable_bus(self) -> Self {
        Self(self.0 & !Self::BUS_EN)
    }
    pub const fn is_bus_enabled(self) -> bool {
        self.0 & Self::BUS_EN != 0
    }

    /// Master Mode Start
    pub const fn set_start_bit(self) -> Self {
        Self(self.0 | Self::M_STA)
    }
    pub const fn clear_start_bit(self) -> Self {
        Self(self.0 & !Self::M_STA)
    }
    pub const fn start_bit(self) -> bool {
        self.0 & Self::M_STA != 0
    }

    /// Master Mode Stop
    pub const fn set_stop_bit(self) -> Self {
        Self(self.0 | Self::M_STP)
    }
    pub const fn clear_stop_bit(self) -> Self {
        Self(self.0 & !Self::M_STP)
    }
    pub const fn stop_bit(self) -> bool {
        self.0 & Self::M_STP != 0
    }

    /// Interrupt Flag
    pub const fn interrupt_flag(self) -> bool {
        self.0 & Self::INT_FLAG != 0
    }
    pub const fn clear_interrupt_flag(self) -> Self {
        Self(self.0 | Self::INT_FLAG)
    }

    /// Assert Acknowledge
    pub const fn set_ack(self, ack: bool) -> Self {
        Self(if ack {
            self.0 | Self::A_ACK
        } else {
            self.0 & !Self::A_ACK
        })
    }
    pub const fn ack(self) -> bool {
        self.0 & Self::A_ACK != 0
    }
}

/// 0x0010: TWI Status Register (Default: 0x0000_00F8).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Status(u32);

impl Status {
    pub const BUS_ERROR: u32 = 0x00;
    pub const START_TRANSMITTED: u32 = 0x08;
    pub const REPEATED_START_TRANSMITTED: u32 = 0x10;
    pub const ADDRESS_WRITE_ACK: u32 = 0x18;
    pub const ADDRESS_WRITE_NACK: u32 = 0x20;
    pub const DATA_WRITE_ACK: u32 = 0x28;
    pub const DATA_WRITE_NACK: u32 = 0x30;
    pub const ARBITRATION_LOST: u32 = 0x38;
    pub const ADDRESS_READ_ACK: u32 = 0x40;
    pub const ADDRESS_READ_NACK: u32 = 0x48;
    pub const DATA_READ_ACK: u32 = 0x50;
    pub const DATA_READ_NACK: u32 = 0x58;
    pub const IDLE: u32 = 0xF8;

    pub const fn code(self) -> u32 {
        self.0 & 0xFF
    }
}

/// 0x0014: TWI Clock Control Register (Default: 0x0000_0080).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ClockControl(u32);

impl ClockControl {
    const CLK_DUTY: u32 = 1 << 7;
    const CLK_M: u32 = 0xF << 3;
    const CLK_N: u32 = 0x7 << 0;

    pub const fn set_duty_cycle(self, duty_40: bool) -> Self {
        Self(if duty_40 {
            self.0 | Self::CLK_DUTY
        } else {
            self.0 & !Self::CLK_DUTY
        })
    }

    /// Setting duty cycle of clock as master
    pub const fn duty_cycle(self) -> bool {
        self.0 & Self::CLK_DUTY != 0
    }

    pub const fn set_m(self, m: u8) -> Self {
        Self((self.0 & !Self::CLK_M) | ((m as u32 & 0xF) << 3))
    }
    /// The TWI SCL output frequency, in master mode, is F1/10
    pub const fn m(self) -> u8 {
        ((self.0 & Self::CLK_M) >> 3) as u8
    }

    pub const fn set_n(self, n: u8) -> Self {
        Self((self.0 & !Self::CLK_N) | (n as u32 & 0x7))
    }
    /// The TWI bus is sampled by the TWI at the frequency defined by F0
    pub const fn n(self) -> u8 {
        (self.0 & Self::CLK_N) as u8
    }
}

/// 0x0018: TWI Software Reset Register (Default: 0x0000_0000).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SoftReset(u32);

impl SoftReset {
    const SOFT_RST: u32 = 1 << 0;

    /// Software Reset
    pub const fn set_soft_reset(self) -> Self {
        Self(self.0 | Self::SOFT_RST)
    }
    pub const fn clear_soft_reset(self) -> Self {
        Self(self.0 & !Self::SOFT_RST)
    }

    /// Write ‘1’ to this bit to reset the TWI and clear to ‘0’ when completing Soft Reset operation.
    pub const fn soft_reset(self) -> bool {
        self.0 & Self::SOFT_RST != 0
    }
}

/// 0x001C: TWI Enhance Feature Register (Default: 0x0000_0000).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct EnhanceFeature(u32);

impl EnhanceFeature {
    const DBYTE_EN: u32 = 0x3 << 0;

    /// Data Byte Number Follow Read Command Control
    pub const fn set_data_byte(self, bytes: u8) -> Self {
        Self((self.0 & !Self::DBYTE_EN) | (bytes as u32 & 0x3))
    }
    pub const fn data_byte(self) -> u8 {
        (self.0 & Self::DBYTE_EN) as u8
    }
}

/// 0x0020: TWI Line Control Register (Default: 0x0000_003A)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct LineControl(u32);

impl LineControl {
    const SCL_STATE: u32 = 1 << 5;
    const SCL_CTL_EN: u32 = 1 << 2;
    const SCL_CTL: u32 = 1 << 3;
    const SDA_STATE: u32 = 1 << 4;
    const SDA_CTL_EN: u32 = 1 << 0;
    const SDA_CTL: u32 = 1 << 1;

    /// Get the current state of the SCL line (read-only).
    pub const fn scl_state(self) -> bool {
        self.0 & Self::SCL_STATE != 0
    }

    /// Enable manual control of the SCL line.
    pub const fn enable_scl_control(self) -> Self {
        Self(self.0 | Self::SCL_CTL_EN)
    }
    /// Disable manual control of the SCL line.
    pub const fn disable_scl_control(self) -> Self {
        Self(self.0 & !Self::SCL_CTL_EN)
    }
    pub const fn is_scl_control_enabled(self) -> bool {
        self.0 & Self::SCL_CTL_EN != 0
    }

    /// Set the output level of the SCL line in manual mode.
    pub const fn set_scl_control(self, high: bool) -> Self {
        Self(if high {
            self.0 | Self::SCL_CTL
        } else {
            self.0 & !Self::SCL_CTL
        })
    }
    pub const fn scl_control(self) -> bool {
        self.0 & Self::SCL_CTL != 0
    }

    /// Get the current state of the SDA line (read-only).
    pub const fn sda_state(self) -> bool {
        self.0 & Self::SDA_STATE != 0
    }

    /// Enable manual control of the SDA line.
    pub const fn enable_sda_control(self) -> Self {
        Self(self.0 | Self::SDA_CTL_EN)
    }
    /// Disable manual control of the SDA line.
    pub const fn disable_sda_control(self) -> Self {
        Self(self.0 & !Self::SDA_CTL_EN)
    }
    pub const fn is_sda_control_enabled(self) -> bool {
        self.0 & Self::SDA_CTL_EN != 0
    }

    /// Set the output level of the SDA line in manual mode.
    pub const fn set_sda_control(self, high: bool) -> Self {
        Self(if high {
            self.0 | Self::SDA_CTL
        } else {
            self.0 & !Self::SDA_CTL
        })
    }
    pub const fn sda_control(self) -> bool {
        self.0 & Self::SDA_CTL != 0
    }
}

/// 0x0200: TWI Driver Control Register (Default: 0x00F8_1000)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DrvControl(u32);

impl DrvControl {
    const START_TRAN: u32 = 1 << 31;
    const RESTART_MODE: u32 = 1 << 29;
    const TRAN_RESULT: u32 = 0xF << 24;
    const TWI_STA: u32 = 0xFF << 16;
    const TIMEOUT_N: u32 = 0xFF << 8;
    const SOFT_RESET: u32 = 1 << 1;
    const TWI_DRV_EN: u32 = 1 << 0;

    /// Set this bit to start a transmission. It is cleared automatically by hardware upon completion.
    pub const fn start_transmission(self) -> Self {
        Self(self.0 | Self::START_TRAN)
    }
    /// Set the restart mode. `false`: RESTART, `true`: STOP+START.
    pub const fn set_restart_mode(self, stop_then_start: bool) -> Self {
        Self(if stop_then_start {
            self.0 | Self::RESTART_MODE
        } else {
            self.0 & !Self::RESTART_MODE
        })
    }
    /// Get the transmission result (read-only). 0: OK, 1: FAIL.
    pub const fn transmission_result(self) -> u8 {
        ((self.0 & Self::TRAN_RESULT) >> 24) as u8
    }
    /// Get the current status of the TWI state machine (read-only).
    pub const fn twi_status(self) -> u8 {
        ((self.0 & Self::TWI_STA) >> 16) as u8
    }
    /// Set the timeout counter in F_SCL clock cycles.
    pub const fn set_timeout(self, n: u8) -> Self {
        Self((self.0 & !Self::TIMEOUT_N) | ((n as u32) << 8))
    }
    pub const fn timeout(self) -> u8 {
        ((self.0 & Self::TIMEOUT_N) >> 8) as u8
    }
    /// Trigger a software reset. `true`: Reset, `false`: Normal operation.
    pub const fn set_soft_reset(self, reset: bool) -> Self {
        Self(if reset {
            self.0 | Self::SOFT_RESET
        } else {
            self.0 & !Self::SOFT_RESET
        })
    }
    /// Enable the TWI driver module. `true`: Enable, `false`: Disable.
    pub const fn set_drv_enable(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::TWI_DRV_EN
        } else {
            self.0 & !Self::TWI_DRV_EN
        })
    }
    pub const fn is_drv_enabled(self) -> bool {
        self.0 & Self::TWI_DRV_EN != 0
    }
}

/// 0x0204: TWI Driver Transmission Configuration Register (Default: 0x0000_0001)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DrvCfg(u32);

impl DrvCfg {
    const PKT_INTERVAL: u32 = 0xFFFF << 16;
    const PACKET_CNT: u32 = 0xFFFF << 0;

    /// Set the interval between each packet (in FSCL clock cycles).
    pub const fn set_packet_interval(self, interval: u16) -> Self {
        Self((self.0 & !Self::PKT_INTERVAL) | ((interval as u32) << 16))
    }
    pub const fn packet_interval(self) -> u16 {
        ((self.0 & Self::PKT_INTERVAL) >> 16) as u16
    }
    /// Set the number of packets to be transmitted.
    pub const fn set_packet_count(self, count: u16) -> Self {
        Self((self.0 & !Self::PACKET_CNT) | (count as u32))
    }
    pub const fn packet_count(self) -> u16 {
        (self.0 & Self::PACKET_CNT) as u16
    }
}

/// 0x0208: TWI Driver Slave ID Register (Default: 0x0000_0000)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DrvSlv(u32);

impl DrvSlv {
    const SLV_ID: u32 = 0xFFFF << 16;
    const CMD: u32 = 1 << 8;
    const SLV_ID_X: u32 = 0xFF << 0;

    /// Set the slave device ID (address).
    pub const fn set_slave_id(self, id: u16) -> Self {
        Self((self.0 & !Self::SLV_ID) | ((id as u32) << 16))
    }
    pub const fn slave_id(self) -> u16 {
        ((self.0 & Self::SLV_ID) >> 16) as u16
    }
    /// Set the operation command. `false`: Write, `true`: Read.
    pub const fn set_command_read(self, read: bool) -> Self {
        Self(if read {
            self.0 | Self::CMD
        } else {
            self.0 & !Self::CMD
        })
    }
    pub const fn is_command_read(self) -> bool {
        self.0 & Self::CMD != 0
    }
    /// For 10-bit addressing mode, set the lower 8 bits of the slave device ID.
    pub const fn set_slave_id_extended(self, id_ext: u8) -> Self {
        Self((self.0 & !Self::SLV_ID_X) | (id_ext as u32))
    }
    pub const fn slave_id_extended(self) -> u8 {
        (self.0 & Self::SLV_ID_X) as u8
    }
}

/// 0x020C: TWI Driver Packet Format Register (Default: 0x0001_0001)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DrvFmt(u32);

impl DrvFmt {
    const ADDR_BYTE: u32 = 0xFF << 16;
    const DATA_BYTE: u32 = 0xFFFF << 0;

    /// Set the number of bytes for the slave device's register address (0-255).
    pub const fn set_address_bytes(self, bytes: u8) -> Self {
        Self((self.0 & !Self::ADDR_BYTE) | ((bytes as u32) << 16))
    }
    /// Get the number of bytes for the slave device's register address.
    pub const fn address_bytes(self) -> u8 {
        ((self.0 & Self::ADDR_BYTE) >> 16) as u8
    }

    /// Set the number of bytes in the data field for each packet (1-65535).
    pub const fn set_data_bytes(self, bytes: u16) -> Self {
        Self((self.0 & !Self::DATA_BYTE) | (bytes as u32))
    }
    /// Get the number of bytes in the data field for each packet.
    pub const fn data_bytes(self) -> u16 {
        (self.0 & Self::DATA_BYTE) as u16
    }
}

/// 0x0210: TWI Driver Bus Control Register (Default: 0x0000_80C0)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DrvBusCtrl(u32);

impl DrvBusCtrl {
    const CLK_COUNT_MODE: u32 = 1 << 16;
    const CLK_DUTY: u32 = 1 << 15;
    const CLK_N: u32 = 0x7 << 12;
    const CLK_M: u32 = 0xF << 8;
    const SCL_STA: u32 = 1 << 7;
    const SDA_STA: u32 = 1 << 6;
    const SCL_MOV: u32 = 1 << 3;
    const SDA_MOV: u32 = 1 << 2;
    const SCL_MOE: u32 = 1 << 1;
    const SDA_MOE: u32 = 1 << 0;

    /// Set the clock count mode.
    pub const fn set_clock_count_mode(self, on_iscl: bool) -> Self {
        Self(if on_iscl {
            self.0 | Self::CLK_COUNT_MODE
        } else {
            self.0 & !Self::CLK_COUNT_MODE
        })
    }

    /// Set the SCL clock duty cycle.
    /// false: 50%, true: 40%
    pub const fn set_clock_duty_40(self, duty_40: bool) -> Self {
        Self(if duty_40 {
            self.0 | Self::CLK_DUTY
        } else {
            self.0 & !Self::CLK_DUTY
        })
    }

    pub const fn is_clock_duty_40(self) -> bool {
        self.0 & Self::CLK_DUTY != 0
    }

    /// Set the clock division factor N.
    pub const fn set_clock_n(self, n: u8) -> Self {
        Self((self.0 & !Self::CLK_N) | ((n as u32 & 0x7) << 12))
    }

    pub const fn clock_n(self) -> u8 {
        ((self.0 & Self::CLK_N) >> 12) as u8
    }

    /// Set the clock division factor M.
    pub const fn set_clock_m(self, m: u8) -> Self {
        Self((self.0 & !Self::CLK_M) | ((m as u32 & 0xF) << 8))
    }

    pub const fn clock_m(self) -> u8 {
        ((self.0 & Self::CLK_M) >> 8) as u8
    }

    /// Get the current state of the SCL line (read-only).
    pub const fn scl_status(self) -> bool {
        self.0 & Self::SCL_STA != 0
    }

    /// Get the current state of the SDA line (read-only).
    pub const fn sda_status(self) -> bool {
        self.0 & Self::SDA_STA != 0
    }

    /// Set the SCL manual output value.
    pub const fn set_scl_manual_output(self, high: bool) -> Self {
        Self(if high {
            self.0 | Self::SCL_MOV
        } else {
            self.0 & !Self::SCL_MOV
        })
    }

    pub const fn is_scl_manual_output_high(self) -> bool {
        self.0 & Self::SCL_MOV != 0
    }

    /// Set the SDA manual output value.
    pub const fn set_sda_manual_output(self, high: bool) -> Self {
        Self(if high {
            self.0 | Self::SDA_MOV
        } else {
            self.0 & !Self::SDA_MOV
        })
    }

    pub const fn is_sda_manual_output_high(self) -> bool {
        self.0 & Self::SDA_MOV != 0
    }

    /// Enable SCL manual output mode.
    pub const fn set_scl_manual_output_enable(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::SCL_MOE
        } else {
            self.0 & !Self::SCL_MOE
        })
    }

    pub const fn is_scl_manual_output_enabled(self) -> bool {
        self.0 & Self::SCL_MOE != 0
    }

    /// Enable SDA manual output mode.
    pub const fn set_sda_manual_output_enable(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::SDA_MOE
        } else {
            self.0 & !Self::SDA_MOE
        })
    }

    pub const fn is_sda_manual_output_enabled(self) -> bool {
        self.0 & Self::SDA_MOE != 0
    }
}

/// 0x0214: TWI Driver Interrupt Control Register (Default: 0x0000_0000)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DrvIntCtrl(u32);

impl DrvIntCtrl {
    const RX_REQ_INT_EN: u32 = 1 << 19;
    const TX_REQ_INT_EN: u32 = 1 << 18;
    const TRAN_ERR_INT_EN: u32 = 1 << 17;
    const TRAN_COM_INT_EN: u32 = 1 << 16;
    const RX_REQ_PD: u32 = 1 << 3;
    const TX_REQ_PD: u32 = 1 << 2;
    const TRAN_ERR_PD: u32 = 1 << 1;
    const TRAN_COM_PD: u32 = 1 << 0;

    /// Enable/disable the Receive Request Interrupt.
    pub const fn set_rx_request_interrupt(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::RX_REQ_INT_EN
        } else {
            self.0 & !Self::RX_REQ_INT_EN
        })
    }
    /// Enable/disable the Transmit Request Interrupt.
    pub const fn set_tx_request_interrupt(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::TX_REQ_INT_EN
        } else {
            self.0 & !Self::TX_REQ_INT_EN
        })
    }
    /// Enable/disable the Transfer Error Interrupt.
    pub const fn set_transfer_error_interrupt(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::TRAN_ERR_INT_EN
        } else {
            self.0 & !Self::TRAN_ERR_INT_EN
        })
    }
    /// Enable/disable the Transfer Complete Interrupt.
    pub const fn set_transfer_complete_interrupt(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::TRAN_COM_INT_EN
        } else {
            self.0 & !Self::TRAN_COM_INT_EN
        })
    }

    /// Check if the Receive Request Interrupt is enabled.
    pub const fn is_rx_request_enabled(self) -> bool {
        self.0 & Self::RX_REQ_INT_EN != 0
    }

    /// Check if the Transmit Request Interrupt is enabled.
    pub const fn is_tx_request_enabled(self) -> bool {
        self.0 & Self::TX_REQ_INT_EN != 0
    }

    /// Check if the Transfer Error Interrupt is enabled.
    pub const fn is_transfer_error_enabled(self) -> bool {
        self.0 & Self::TRAN_ERR_INT_EN != 0
    }

    /// Check if the Transfer Complete Interrupt is enabled.
    pub const fn is_transfer_complete_enabled(self) -> bool {
        self.0 & Self::TRAN_COM_INT_EN != 0
    }

    /// Check if the Receive Request interrupt is pending (read-only).
    pub const fn is_rx_request_pending(self) -> bool {
        self.0 & Self::RX_REQ_PD != 0
    }
    /// Check if the Transmit Request interrupt is pending (read-only).
    pub const fn is_tx_request_pending(self) -> bool {
        self.0 & Self::TX_REQ_PD != 0
    }
    /// Check if the Transfer Error interrupt is pending (read-only).
    pub const fn is_transfer_error_pending(self) -> bool {
        self.0 & Self::TRAN_ERR_PD != 0
    }
    /// Check if the Transfer Complete interrupt is pending (read-only).
    pub const fn is_transfer_complete_pending(self) -> bool {
        self.0 & Self::TRAN_COM_PD != 0
    }

    /// Clear the Receive Request pending status (write 1 to clear).
    pub const fn clear_rx_request_pending(self) -> Self {
        Self(self.0 | Self::RX_REQ_PD)
    }
    /// Clear the Transmit Request pending status (write 1 to clear).
    pub const fn clear_tx_request_pending(self) -> Self {
        Self(self.0 | Self::TX_REQ_PD)
    }
    /// Clear the Transfer Error pending status (write 1 to clear).
    pub const fn clear_transfer_error_pending(self) -> Self {
        Self(self.0 | Self::TRAN_ERR_PD)
    }
    /// Clear the Transfer Complete pending status (write 1 to clear).
    pub const fn clear_transfer_complete_pending(self) -> Self {
        Self(self.0 | Self::TRAN_COM_PD)
    }
}

/// 0x0218: TWI Driver DMA Configuration Register (Default: 0x0010_0010)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DrvDmaCfg(u32);

impl DrvDmaCfg {
    const DMA_RX_EN: u32 = 1 << 24;
    const RX_TRIG: u32 = 0x3F << 16;
    const DMA_TX_EN: u32 = 1 << 8;
    const TX_TRIG: u32 = 0x3F << 0;

    /// Enable/disable DMA receive.
    pub const fn set_dma_rx_enable(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::DMA_RX_EN
        } else {
            self.0 & !Self::DMA_RX_EN
        })
    }
    /// Check if DMA receive is enabled.
    pub const fn is_dma_rx_enabled(self) -> bool {
        self.0 & Self::DMA_RX_EN != 0
    }

    /// Set the DMA trigger level for the RX FIFO.
    pub const fn set_rx_trigger_level(self, level: u8) -> Self {
        Self((self.0 & !Self::RX_TRIG) | ((level as u32 & 0x3F) << 16))
    }

    /// When DMA_RX_EN is set, send DMA RX Req when the data byte number in RECV_FIFO reaches RX_TRIG, or the read transmission is completed, the data of RECV_FIFO does not reach RX_TRIG but as long as the RECV_FIFO is not empty.
    pub const fn rx_trigger_level(self) -> u8 {
        ((self.0 & Self::RX_TRIG) >> 16) as u8
    }

    /// Enable/disable DMA transmit.
    pub const fn set_dma_tx_enable(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::DMA_TX_EN
        } else {
            self.0 & !Self::DMA_TX_EN
        })
    }
    /// Check if DMA transmit is enabled.
    pub const fn is_dma_tx_enabled(self) -> bool {
        self.0 & Self::DMA_TX_EN != 0
    }

    /// Set the DMA trigger level for the TX FIFO.
    pub const fn set_tx_trigger_level(self, level: u8) -> Self {
        Self((self.0 & !Self::TX_TRIG) | (level as u32 & 0x3F))
    }

    /// When DMA_TX_EN is set, send DMA TX Req when the space of SEND_FIFO (FIFO Level – data volume) reaches TX_TRIG.
    pub const fn tx_trigger_level(self) -> u8 {
        (self.0 & Self::TX_TRIG) as u8
    }
}

/// 0x021C: TWI Driver FIFO Content Register (Default: 0x0000_0000)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DrvFifoCon(u32);

impl DrvFifoCon {
    const RECV_FIFO_CLEAR: u32 = 1 << 22;
    const RECV_FIFO_CONTENT: u32 = 0x3F << 16;
    const SEND_FIFO_CLEAR: u32 = 1 << 6;
    const SEND_FIFO_CONTENT: u32 = 0x3F << 0;

    /// Set this bit to clear the RECV FIFO pointer (clears automatically).
    pub const fn clear_recv_fifo(self) -> Self {
        Self(self.0 | Self::RECV_FIFO_CLEAR)
    }

    /// Get the number of data bytes in the RECV FIFO (read-only).
    pub const fn recv_fifo_content(self) -> u8 {
        ((self.0 & Self::RECV_FIFO_CONTENT) >> 16) as u8
    }

    /// Set this bit to clear the SEND FIFO pointer (clears automatically).
    pub const fn clear_send_fifo(self) -> Self {
        Self(self.0 | Self::SEND_FIFO_CLEAR)
    }

    /// Get the number of data bytes in the SEND FIFO (read-only).
    pub const fn send_fifo_content(self) -> u8 {
        (self.0 & Self::SEND_FIFO_CONTENT) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ClockControl, Control, DrvBusCtrl, DrvCfg, DrvControl, DrvDmaCfg, DrvFifoCon, DrvFmt,
        DrvIntCtrl, DrvSlv, EnhanceFeature, LineControl, RegisterBlock, SoftReset, Status,
    };

    use core::mem;

    #[test]
    fn test_register_offsets() {
        assert_eq!(mem::offset_of!(RegisterBlock, addr), 0x0000);
        assert_eq!(mem::offset_of!(RegisterBlock, xaddr), 0x0004);
        assert_eq!(mem::offset_of!(RegisterBlock, data), 0x0008);
        assert_eq!(mem::offset_of!(RegisterBlock, cntr), 0x000C);
        assert_eq!(mem::offset_of!(RegisterBlock, stat), 0x0010);
        assert_eq!(mem::offset_of!(RegisterBlock, ccr), 0x0014);
        assert_eq!(mem::offset_of!(RegisterBlock, srst), 0x0018);
        assert_eq!(mem::offset_of!(RegisterBlock, efr), 0x001C);
        assert_eq!(mem::offset_of!(RegisterBlock, lcr), 0x0020);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_ctrl), 0x0200);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_cfg), 0x0204);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_slv), 0x0208);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_fmt), 0x020C);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_bus_ctrl), 0x0210);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_int_ctrl), 0x0214);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_dma_cfg), 0x0218);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_fifo_con), 0x021C);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_send_fifo_acc), 0x0300);
        assert_eq!(mem::offset_of!(RegisterBlock, drv_recv_fifo_acc), 0x0304);
    }

    #[test]
    fn test_control() {
        let mut ctrl = Control::default();
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.enable_interrupt();
        assert!(ctrl.is_interrupt_enabled());

        ctrl = ctrl.enable_bus();
        assert!(ctrl.is_bus_enabled());
        assert_eq!(ctrl.0, (1 << 7) | (1 << 6));

        // Note: start_bit and stop_bit are self-clearing, so we can't test state persistence.
        // only test that the setters correctly set the bits.
        assert_eq!(ctrl.set_start_bit().0, (1 << 7) | (1 << 6) | (1 << 5));
        assert_eq!(ctrl.set_stop_bit().0, (1 << 7) | (1 << 6) | (1 << 4));

        ctrl = ctrl.clear_interrupt_flag();
        assert_eq!(ctrl.0, (1 << 7) | (1 << 6) | (1 << 3));

        ctrl = ctrl.set_ack(true);
        assert_eq!(ctrl.0, (1 << 7) | (1 << 6) | (1 << 3) | (1 << 2));

        ctrl = ctrl.set_ack(false);
        assert_eq!(ctrl.0, (1 << 7) | (1 << 6) | (1 << 3));
    }

    #[test]
    fn test_status() {
        // These constants are defined in the manual on pages 27-28
        let stat = Status(Status::IDLE);
        assert_eq!(stat.code(), 0xF8);

        let stat = Status(Status::DATA_READ_ACK);
        assert_eq!(stat.code(), 0x50);
    }

    #[test]
    fn test_clock_control() {
        let mut ccr = ClockControl::default();
        // Manual default is 0x80, but our default() is 0 for testing purposes
        assert_eq!(ccr.0, 0);

        ccr = ccr.set_duty_cycle(true);
        assert_eq!(ccr.0, 1 << 7);

        ccr = ccr.set_m(0xA);
        assert_eq!(ccr.0, (1 << 7) | (0xA << 3));
        assert_eq!(ccr.m(), 0xA);

        ccr = ccr.set_n(0x5);
        assert_eq!(ccr.0, (1 << 7) | (0xA << 3) | 0x5);
        assert_eq!(ccr.n(), 0x5);
    }

    #[test]
    fn test_soft_reset() {
        let mut srst = SoftReset::default();
        // The SOFT_RST bit is self-clearing
        assert_eq!(srst.set_soft_reset().0, 1);
    }

    #[test]
    fn test_enhance_feature() {
        let mut efr = EnhanceFeature::default();
        assert_eq!(efr.0, 0);

        efr = efr.set_data_byte(2);
        assert_eq!(efr.data_byte(), 2);
    }

    #[test]
    fn test_line_control() {
        // 0x3A in binary is 0b0011_1010.
        let lcr = LineControl(0x0000_003A);
        assert!(lcr.scl_state());
        assert!(lcr.sda_state());
        assert!(lcr.scl_control());
        assert!(!lcr.is_scl_control_enabled());
        assert!(lcr.sda_control());
        assert!(!lcr.is_sda_control_enabled());

        //test write
        let mut lcr_mut = LineControl(0x0);
        lcr_mut = lcr_mut.set_scl_control(true);
        assert!(lcr_mut.scl_control());
        lcr_mut = lcr_mut.enable_scl_control();
        assert!(lcr_mut.is_scl_control_enabled());
        lcr_mut = lcr_mut.disable_scl_control();
        assert!(!lcr_mut.is_scl_control_enabled());

        lcr_mut = lcr_mut.set_sda_control(true);
        assert!(lcr_mut.sda_control());
        lcr_mut = lcr_mut.enable_sda_control();
        assert!(lcr_mut.is_sda_control_enabled());
        lcr_mut = lcr_mut.disable_sda_control();
        assert!(!lcr_mut.is_sda_control_enabled());
    }

    #[test]
    fn test_drv_control() {
        let mut reg = DrvControl::default();
        assert_eq!(reg.0, 0);

        reg = reg.start_transmission();
        assert_eq!(reg.0, 1 << 31);

        reg = reg.set_restart_mode(true);
        assert_eq!(reg.0, (1 << 31) | (1 << 29));

        reg = reg.set_timeout(0xAB);
        assert_eq!(reg.0, (1 << 31) | (1 << 29) | (0xAB << 8));
        assert_eq!(reg.timeout(), 0xAB);

        reg = reg.set_soft_reset(true);
        assert_eq!(reg.0, (1 << 31) | (1 << 29) | (0xAB << 8) | (1 << 1));

        reg = reg.set_drv_enable(true);
        assert_eq!(reg.0, (1 << 31) | (1 << 29) | (0xAB << 8) | (1 << 1) | 1);
        assert!(reg.is_drv_enabled());

        // Test read-only fields by creating a new instance with a known value
        let ro_reg = DrvControl(0x0A5F0000);
        assert_eq!(ro_reg.transmission_result(), 0x0A);
        assert_eq!(ro_reg.twi_status(), 0x5F);
    }

    #[test]
    fn test_drv_cfg() {
        let mut reg = DrvCfg::default();
        // We start from a known state (0) for testing.
        assert_eq!(reg.0, 0);

        reg = reg.set_packet_interval(0x1234);
        assert_eq!(reg.packet_interval(), 0x1234);
        assert_eq!(reg.0, 0x1234 << 16);

        reg = reg.set_packet_count(0x5678);
        assert_eq!(reg.packet_count(), 0x5678);
        assert_eq!(reg.0, (0x1234 << 16) | 0x5678);
    }

    #[test]
    fn test_drv_slv() {
        let mut reg = DrvSlv::default();
        assert_eq!(reg.0, 0);

        reg = reg.set_slave_id(0xABCD);
        assert_eq!(reg.slave_id(), 0xABCD);
        assert_eq!(reg.0, 0xABCD << 16);

        reg = reg.set_command_read(true);
        assert!(reg.is_command_read());
        assert_eq!(reg.0, (0xABCD << 16) | (1 << 8));

        reg = reg.set_slave_id_extended(0xEF);
        assert_eq!(reg.slave_id_extended(), 0xEF);
        assert_eq!(reg.0, (0xABCD << 16) | (1 << 8) | 0xEF);
    }

    #[test]
    fn test_drv_fmt() {
        let mut reg = DrvFmt::default();
        assert_eq!(reg.0, 0);

        reg = reg.set_address_bytes(0x12);
        assert_eq!(reg.address_bytes(), 0x12);
        assert_eq!(reg.0, 0x12 << 16);

        reg = reg.set_data_bytes(0x3456);
        assert_eq!(reg.data_bytes(), 0x3456);
        assert_eq!(reg.0, (0x12 << 16) | 0x3456);
    }

    #[test]
    fn test_drv_bus_ctrl() {
        let mut reg = DrvBusCtrl::default();
        assert_eq!(reg.0, 0);

        reg = reg.set_clock_duty_40(true);
        assert!(reg.is_clock_duty_40());
        assert_eq!(reg.0, 1 << 15);

        reg = reg.set_clock_n(0b101); // 5
        assert_eq!(reg.clock_n(), 5);
        assert_eq!(reg.0, (1 << 15) | (5 << 12));

        reg = reg.set_clock_m(0b1101); // 13
        assert_eq!(reg.clock_m(), 13);
        assert_eq!(reg.0, (1 << 15) | (5 << 12) | (13 << 8));

        let default_reg = DrvBusCtrl(0x0000_80C0);
        assert!(default_reg.scl_status());
        assert!(default_reg.sda_status());

        let mut reg = DrvBusCtrl::default();
        reg = reg.set_scl_manual_output(true);
        reg = reg.set_sda_manual_output(false);
        assert!(reg.is_scl_manual_output_high());
        assert!(!reg.is_sda_manual_output_high());

        reg = reg.set_scl_manual_output_enable(true);
        assert!(reg.is_scl_manual_output_enabled());
        assert!(!reg.is_sda_manual_output_enabled());
    }

    #[test]
    fn test_drv_int_ctrl() {
        let mut reg = DrvIntCtrl::default();
        assert_eq!(reg.0, 0);

        reg = reg.set_rx_request_interrupt(true);
        assert_eq!(reg.0, 1 << 19);
        assert!(reg.is_rx_request_enabled());

        reg = reg.set_tx_request_interrupt(true);
        assert_eq!(reg.0, (1 << 19) | (1 << 18));
        assert!(reg.is_tx_request_enabled());

        reg = reg.set_transfer_error_interrupt(true);
        assert_eq!(reg.0, (1 << 19) | (1 << 18) | (1 << 17));
        assert!(reg.is_transfer_error_enabled());

        reg = reg.set_transfer_complete_interrupt(true);
        assert_eq!(reg.0, (1 << 19) | (1 << 18) | (1 << 17) | (1 << 16));
        assert!(reg.is_transfer_complete_enabled());

        // Test write-1-to-clear operation
        reg = reg.clear_transfer_complete_pending();
        assert_eq!(
            reg.0,
            (1 << 19) | (1 << 18) | (1 << 17) | (1 << 16) | (1 << 0)
        );
        assert!(reg.is_transfer_complete_pending());

        reg = reg.clear_rx_request_pending();
        assert!(reg.is_rx_request_pending());
    }

    #[test]
    fn test_drv_dma_cfg() {
        let mut reg = DrvDmaCfg::default();
        assert_eq!(reg.0, 0);

        reg = reg.set_dma_rx_enable(true);
        assert!(reg.is_dma_rx_enabled());
        assert_eq!(reg.0, 1 << 24);

        reg = reg.set_rx_trigger_level(0x2A); // 42
        assert_eq!(reg.rx_trigger_level(), 0x2A);
        assert_eq!(reg.0, (1 << 24) | (0x2A << 16));

        reg = reg.set_dma_tx_enable(true);
        assert!(reg.is_dma_tx_enabled());
        assert_eq!(reg.0, (1 << 24) | (0x2A << 16) | (1 << 8));

        reg = reg.set_tx_trigger_level(0x15); // 21
        assert_eq!(reg.tx_trigger_level(), 0x15);
        assert_eq!(reg.0, (1 << 24) | (0x2A << 16) | (1 << 8) | 0x15);
    }

    #[test]
    fn test_drv_fifo_con() {
        let mut reg = DrvFifoCon::default();
        assert_eq!(reg.0, 0);

        // Test write-auto-clear setters
        reg = reg.clear_recv_fifo();
        assert_eq!(reg.0, 1 << 22);

        reg = DrvFifoCon::default().clear_send_fifo();
        assert_eq!(reg.0, 1 << 6);

        // Test read-only fields
        let ro_reg = DrvFifoCon((0b101010 << 16) | 0b111010);
        assert_eq!(ro_reg.recv_fifo_content(), 0b101010);
        assert_eq!(ro_reg.send_fifo_content(), 0b111010);
    }
}
