//! T153 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun8iw22` platform.

use super::SingleBusGatingReset;
use volatile_register::{RO, RW};

/// T153 main CCU register block.
#[doc(alias = "sun8iw22")]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0000: [u8; 0x040],
    /// 0x0040 - `GPIO_POW_MODE_REG`.
    pub gpio_pow_mode: RW<u32>,
    _reserved_0044: [u8; 0x05c],
    /// 0x00a0 - `PLL_PERI0_CTRL_REG`.
    pub pll_peri0_ctrl: RW<u32>,
    _reserved_00a4: [u8; 0x004],
    /// 0x00a8 - `PLL_PERI0_PAT0_CTRL_REG`.
    pub pll_peri0_pat0_ctrl: RW<u32>,
    /// 0x00ac - `PLL_PERI0_PAT1_CTRL_REG`.
    pub pll_peri0_pat1_ctrl: RW<u32>,
    /// 0x00b0 - `PLL_PERI0_BIAS_REG`.
    pub pll_peri0_bias: RW<u32>,
    _reserved_00b4: [u8; 0x00c],
    /// 0x00c0 - `PLL_PERI1_CTRL_REG`.
    pub pll_peri1_ctrl: RW<u32>,
    _reserved_00c4: [u8; 0x004],
    /// 0x00c8 - `PLL_PERI1_PAT0_CTRL_REG`.
    pub pll_peri1_pat0_ctrl: RW<u32>,
    /// 0x00cc - `PLL_PERI1_PAT1_CTRL_REG`.
    pub pll_peri1_pat1_ctrl: RW<u32>,
    /// 0x00d0 - `PLL_PERI1_BIAS_REG`.
    pub pll_peri1_bias: RW<u32>,
    _reserved_00d4: [u8; 0x04c],
    /// 0x0120 - `PLL_VIDEO0_CTRL_REG`.
    pub pll_video0_ctrl: RW<u32>,
    _reserved_0124: [u8; 0x004],
    /// 0x0128 - `PLL_VIDEO0_PAT0_CTRL_REG`.
    pub pll_video0_pat0_ctrl: RW<u32>,
    /// 0x012c - `PLL_VIDEO0_PAT1_CTRL_REG`.
    pub pll_video0_pat1_ctrl: RW<u32>,
    /// 0x0130 - `PLL_VIDEO0_BIAS_REG`.
    pub pll_video0_bias: RW<u32>,
    _reserved_0134: [u8; 0x12c],
    /// 0x0260 - `PLL_AUDIO0_CTRL_REG`.
    pub pll_audio0_ctrl: RW<u32>,
    _reserved_0264: [u8; 0x004],
    /// 0x0268 - `PLL_AUDIO0_PAT0_CTRL_REG`.
    pub pll_audio0_pat0_ctrl: RW<u32>,
    /// 0x026c - `PLL_AUDIO0_PAT1_CTRL_REG`.
    pub pll_audio0_pat1_ctrl: RW<u32>,
    /// 0x0270 - `PLL_AUDIO0_BIAS_REG`.
    pub pll_audio0_bias: RW<u32>,
    _reserved_0274: [u8; 0x0cc],
    /// 0x0340 - `PLL_CPU_CTRL_REG`.
    pub pll_cpu_ctrl: RW<u32>,
    /// 0x0344 - `PLL_CPU_PAT0_CTRL_REG`.
    pub pll_cpu_pat0_ctrl: RW<u32>,
    /// 0x0348 - `PLL_CPU_PAT1_CTRL_REG`.
    pub pll_cpu_pat1_ctrl: RW<u32>,
    /// 0x034c - `PLL_CPU_BIAS_REG`.
    pub pll_cpu_bias: RW<u32>,
    /// 0x0350 - `PLL_CPU_TUN1_REG`.
    pub pll_cpu_tun1: RW<u32>,
    /// 0x0354 - `PLL_CPU_SSC_REG`.
    pub pll_cpu_ssc: RW<u32>,
    /// 0x0358 - `PLL_CPU_ECHO_REG`.
    pub pll_cpu_echo: RW<u32>,
    _reserved_035c: [u8; 0x1a4],
    /// 0x0500 - `AHB_CLK_REG`.
    pub ahb_clk: RW<u32>,
    _reserved_0504: [u8; 0x00c],
    /// 0x0510 - `APB0_CLK_REG`.
    pub apb0_clk: RW<u32>,
    _reserved_0514: [u8; 0x004],
    /// 0x0518 - `APB1_CLK_REG`.
    pub apb1_clk: RW<u32>,
    _reserved_051c: [u8; 0x01c],
    /// 0x0538 - `APB_UART_CLK_REG`.
    pub apb_uart_clk: RW<u32>,
    _reserved_053c: [u8; 0x04c],
    /// 0x0588 - `MBUS_CLK_REG`.
    pub mbus_clk: RW<u32>,
    _reserved_058c: [u8; 0x034],
    /// 0x05c0 - `AHB_MAT_CLK_GATE_EN_REG`.
    pub ahb_mat_clk_gate_en: RW<u32>,
    _reserved_05c4: [u8; 0x00c],
    /// 0x05d0 - `PERI_MAT_CLK_GATE_EN_REG`.
    pub peri_mat_clk_gate_en: RW<u32>,
    _reserved_05d4: [u8; 0x00c],
    /// 0x05e0 - `MBUS_CLK_GATE_EN_REG`.
    pub mbus_clk_gate_en: RW<u32>,
    /// 0x05e4 - `MBUS_MAT_CLK_GATE_EN_REG`.
    pub mbus_mat_clk_gate_en: RW<u32>,
    _reserved_05e8: [u8; 0x008],
    /// 0x05f0 - `AHB_MAT_CLK_AUTO_GATE_EN_REG`.
    pub ahb_mat_clk_auto_gate_en: RW<u32>,
    /// 0x05f4 - `MBUS_MAT_CLK_AUTO_GATE_EN_REG`.
    pub mbus_mat_clk_auto_gate_en: RW<u32>,
    /// 0x05f8 - `AHB_MAT_CLK_GATE_STAT_REG`.
    pub ahb_mat_clk_gate_stat: RO<u32>,
    /// 0x05fc - `MBUS_MAT_CLK_GATE_STAT_REG`.
    pub mbus_mat_clk_gate_stat: RO<u32>,
    _reserved_0600: [u8; 0x104],
    /// 0x0704 - `DMA0_GAR_REG`.
    pub dma0_gar: RW<SingleBusGatingReset>,
    _reserved_0708: [u8; 0x004],
    /// 0x070c - `DMA1_GAR_REG`.
    pub dma1_gar: RW<SingleBusGatingReset>,
    _reserved_0710: [u8; 0x014],
    /// 0x0724 - `SPINLOCK_GAR_REG`.
    pub spinlock_gar: RW<SingleBusGatingReset>,
    _reserved_0728: [u8; 0x01c],
    /// 0x0744 - `MSGBOX_CPUX_GAR_REG`.
    pub msgbox_cpux_gar: RW<SingleBusGatingReset>,
    _reserved_0748: [u8; 0x004],
    /// 0x074c - `MSGBOX_CORE0_GAR_REG`.
    pub msgbox_core0_gar: RW<SingleBusGatingReset>,
    _reserved_0750: [u8; 0x004],
    /// 0x0754 - `MSGBOX_CORE1_GAR_REG`.
    pub msgbox_core1_gar: RW<SingleBusGatingReset>,
    _reserved_0758: [u8; 0x004],
    /// 0x075c - `MSGBOX_CORE2_GAR_REG`.
    pub msgbox_core2_gar: RW<SingleBusGatingReset>,
    _reserved_0760: [u8; 0x004],
    /// 0x0764 - `MSGBOX_CORE3_GAR_REG`.
    pub msgbox_core3_gar: RW<SingleBusGatingReset>,
    _reserved_0768: [u8; 0x004],
    /// 0x076c - `MSGBOX_RV_GAR_REG`.
    pub msgbox_rv_gar: RW<SingleBusGatingReset>,
    _reserved_0770: [u8; 0x014],
    /// 0x0784 - `PWM0_GAR_REG`.
    pub pwm0_gar: RW<SingleBusGatingReset>,
    _reserved_0788: [u8; 0x004],
    /// 0x078c - `PWM1_GAR_REG`.
    pub pwm1_gar: RW<SingleBusGatingReset>,
    _reserved_0790: [u8; 0x004],
    /// 0x0794 - `PWM2_GAR_REG`.
    pub pwm2_gar: RW<SingleBusGatingReset>,
    _reserved_0798: [u8; 0x00c],
    /// 0x07a4 - `DCU_GAR_REG`.
    pub dcu_gar: RW<SingleBusGatingReset>,
    _reserved_07a8: [u8; 0x004],
    /// 0x07ac - `DAP_GAR_REG`.
    pub dap_gar: RW<SingleBusGatingReset>,
    _reserved_07b0: [u8; 0x010],
    /// 0x07c0 - `PWMCS0_CLK_REG`.
    pub pwmcs0_clk: RW<u32>,
    /// 0x07c4 - `PWMCS0_GAR_REG`.
    pub pwmcs0_gar: RW<SingleBusGatingReset>,
    /// 0x07c8 - `PWMCS1_CLK_REG`.
    pub pwmcs1_clk: RW<u32>,
    /// 0x07cc - `PWMCS1_GAR_REG`.
    pub pwmcs1_gar: RW<SingleBusGatingReset>,
    _reserved_07d0: [u8; 0x030],
    /// 0x0800 - `TIMER0_0_CLK_REG`.
    pub timer0_0_clk: RW<u32>,
    /// 0x0804 - `TIMER0_1_CLK_REG`.
    pub timer0_1_clk: RW<u32>,
    /// 0x0808 - `TIMER0_2_CLK_REG`.
    pub timer0_2_clk: RW<u32>,
    /// 0x080c - `TIMER0_3_CLK_REG`.
    pub timer0_3_clk: RW<u32>,
    /// 0x0810 - `TIMER0_4_CLK_REG`.
    pub timer0_4_clk: RW<u32>,
    /// 0x0814 - `TIMER0_5_CLK_REG`.
    pub timer0_5_clk: RW<u32>,
    /// 0x0818 - `TIMER0_6_CLK_REG`.
    pub timer0_6_clk: RW<u32>,
    /// 0x081c - `TIMER0_7_CLK_REG`.
    pub timer0_7_clk: RW<u32>,
    _reserved_0820: [u8; 0x030],
    /// 0x0850 - `TIMER0_GAR_REG`.
    pub timer0_gar: RW<SingleBusGatingReset>,
    _reserved_0854: [u8; 0x00c],
    /// 0x0860 - `TIMER0_0_RV_CLK_REG`.
    pub timer0_0_rv_clk: RW<u32>,
    /// 0x0864 - `TIMER0_1_RV_CLK_REG`.
    pub timer0_1_rv_clk: RW<u32>,
    /// 0x0868 - `TIMER0_2_RV_CLK_REG`.
    pub timer0_2_rv_clk: RW<u32>,
    /// 0x086c - `TIMER0_3_RV_CLK_REG`.
    pub timer0_3_rv_clk: RW<u32>,
    /// 0x0870 - `TIMER0_RV_GAR_REG`.
    pub timer0_rv_gar: RW<SingleBusGatingReset>,
    _reserved_0874: [u8; 0x18c],
    /// 0x0a00 - `DE0_CLK_REG`.
    pub de0_clk: RW<u32>,
    /// 0x0a04 - `DE0_GAR_REG`.
    pub de0_gar: RW<SingleBusGatingReset>,
    _reserved_0a08: [u8; 0x038],
    /// 0x0a40 - `G2D_CLK_REG`.
    pub g2d_clk: RW<u32>,
    /// 0x0a44 - `G2D_GAR_REG`.
    pub g2d_gar: RW<SingleBusGatingReset>,
    _reserved_0a48: [u8; 0x078],
    /// 0x0ac0 - `CE_SYS_CLK_REG`.
    pub ce_sys_clk: RW<u32>,
    /// 0x0ac4 - `CE_SYS_GAR_REG`.
    pub ce_sys_gar: RW<u32>,
    _reserved_0ac8: [u8; 0x0b8],
    /// 0x0b80 - `RV_CORE_CLK_REG`.
    pub rv_core_clk: RW<u32>,
    _reserved_0b84: [u8; 0x004],
    /// 0x0b88 - `RV_TS_CLK_REG`.
    pub rv_ts_clk: RW<u32>,
    _reserved_0b8c: [u8; 0x008],
    /// 0x0b94 - `RV_SYS_GAR_REG`.
    pub rv_sys_gar: RW<u32>,
    _reserved_0b98: [u8; 0x004],
    /// 0x0b9c - `RV_CFG_GAR_REG`.
    pub rv_cfg_gar: RW<SingleBusGatingReset>,
    _reserved_0ba0: [u8; 0x06c],
    /// 0x0c0c - `DRAMC_GAR_REG`.
    pub dramc_gar: RW<SingleBusGatingReset>,
    _reserved_0c10: [u8; 0x0f0],
    /// 0x0d00 - `SMHC0_CLK_REG`.
    pub smhc0_clk: RW<u32>,
    _reserved_0d04: [u8; 0x008],
    /// 0x0d0c - `SMHC0_GAR_REG`.
    pub smhc0_gar: RW<SingleBusGatingReset>,
    /// 0x0d10 - `SMHC1_CLK_REG`.
    pub smhc1_clk: RW<u32>,
    _reserved_0d14: [u8; 0x008],
    /// 0x0d1c - `SMHC1_GAR_REG`.
    pub smhc1_gar: RW<SingleBusGatingReset>,
    /// 0x0d20 - `SMHC2_CLK_REG`.
    pub smhc2_clk: RW<u32>,
    _reserved_0d24: [u8; 0x008],
    /// 0x0d2c - `SMHC2_GAR_REG`.
    pub smhc2_gar: RW<SingleBusGatingReset>,
    _reserved_0d30: [u8; 0x0d0],
    /// 0x0e00 - `UART0_GAR_REG`.
    pub uart0_gar: RW<SingleBusGatingReset>,
    /// 0x0e04 - `UART1_GAR_REG`.
    pub uart1_gar: RW<SingleBusGatingReset>,
    /// 0x0e08 - `UART2_GAR_REG`.
    pub uart2_gar: RW<SingleBusGatingReset>,
    /// 0x0e0c - `UART3_GAR_REG`.
    pub uart3_gar: RW<SingleBusGatingReset>,
    /// 0x0e10 - `UART4_GAR_REG`.
    pub uart4_gar: RW<SingleBusGatingReset>,
    /// 0x0e14 - `UART5_GAR_REG`.
    pub uart5_gar: RW<SingleBusGatingReset>,
    /// 0x0e18 - `UART6_GAR_REG`.
    pub uart6_gar: RW<SingleBusGatingReset>,
    _reserved_0e1c: [u8; 0x004],
    /// 0x0e20 - `UART7_GAR_REG`.
    pub uart7_gar: RW<SingleBusGatingReset>,
    /// 0x0e24 - `UART8_GAR_REG`.
    pub uart8_gar: RW<SingleBusGatingReset>,
    /// 0x0e28 - `UART9_GAR_REG`.
    pub uart9_gar: RW<SingleBusGatingReset>,
    _reserved_0e2c: [u8; 0x054],
    /// 0x0e80 - `TWI0_GAR_REG`.
    pub twi0_gar: RW<SingleBusGatingReset>,
    /// 0x0e84 - `TWI1_GAR_REG`.
    pub twi1_gar: RW<SingleBusGatingReset>,
    /// 0x0e88 - `TWI2_GAR_REG`.
    pub twi2_gar: RW<SingleBusGatingReset>,
    /// 0x0e8c - `TWI3_GAR_REG`.
    pub twi3_gar: RW<SingleBusGatingReset>,
    /// 0x0e90 - `TWI4_GAR_REG`.
    pub twi4_gar: RW<SingleBusGatingReset>,
    /// 0x0e94 - `TWI5_GAR_REG`.
    pub twi5_gar: RW<SingleBusGatingReset>,
    _reserved_0e98: [u8; 0x068],
    /// 0x0f00 - `SPI0_CLK_REG`.
    pub spi0_clk: RW<u32>,
    /// 0x0f04 - `SPI0_GAR_REG`.
    pub spi0_gar: RW<SingleBusGatingReset>,
    /// 0x0f08 - `SPI1_CLK_REG`.
    pub spi1_clk: RW<u32>,
    /// 0x0f0c - `SPI1_GAR_REG`.
    pub spi1_gar: RW<SingleBusGatingReset>,
    /// 0x0f10 - `SPI2_CLK_REG`.
    pub spi2_clk: RW<u32>,
    /// 0x0f14 - `SPI2_GAR_REG`.
    pub spi2_gar: RW<SingleBusGatingReset>,
    /// 0x0f18 - `SPIF_CLK_REG`.
    pub spif_clk: RW<u32>,
    /// 0x0f1c - `SPIF_GAR_REG`.
    pub spif_gar: RW<SingleBusGatingReset>,
    /// 0x0f20 - `SPI3_CLK_REG`.
    pub spi3_clk: RW<u32>,
    /// 0x0f24 - `SPI3_GAR_REG`.
    pub spi3_gar: RW<SingleBusGatingReset>,
    _reserved_0f28: [u8; 0x098],
    /// 0x0fc0 - `GPADC0_CLK_REG`.
    pub gpadc0_clk: RW<u32>,
    /// 0x0fc4 - `GPADC0_GAR_REG`.
    pub gpadc0_gar: RW<SingleBusGatingReset>,
    /// 0x0fc8 - `GPADC1_CLK_REG`.
    pub gpadc1_clk: RW<u32>,
    /// 0x0fcc - `GPADC1_GAR_REG`.
    pub gpadc1_gar: RW<SingleBusGatingReset>,
    /// 0x0fd0 - `GPADC2_CLK_REG`.
    pub gpadc2_clk: RW<u32>,
    /// 0x0fd4 - `GPADC2_GAR_REG`.
    pub gpadc2_gar: RW<SingleBusGatingReset>,
    _reserved_0fd8: [u8; 0x00c],
    /// 0x0fe4 - `TSENSOR_GAR_REG`.
    pub tsensor_gar: RW<SingleBusGatingReset>,
    _reserved_0fe8: [u8; 0x018],
    /// 0x1000 - `IR_RX0_CLK_REG`.
    pub ir_rx0_clk: RW<u32>,
    /// 0x1004 - `IR_RX0_GAR_REG`.
    pub ir_rx0_gar: RW<SingleBusGatingReset>,
    /// 0x1008 - `IR_TX_CLK_REG`.
    pub ir_tx_clk: RW<u32>,
    /// 0x100c - `IR_TX_GAR_REG`.
    pub ir_tx_gar: RW<SingleBusGatingReset>,
    _reserved_1010: [u8; 0x020],
    /// 0x1030 - `TPADC_CLK_REG`.
    pub tpadc_clk: RW<u32>,
    /// 0x1034 - `TPADC_GAR_REG`.
    pub tpadc_gar: RW<SingleBusGatingReset>,
    _reserved_1038: [u8; 0x008],
    /// 0x1040 - `LBC_CLK_REG`.
    pub lbc_clk: RW<u32>,
    _reserved_1044: [u8; 0x008],
    /// 0x104c - `LBC_GAR_REG`.
    pub lbc_gar: RW<SingleBusGatingReset>,
    _reserved_1050: [u8; 0x0b0],
    /// 0x1100 - `IR_RX1_CLK_REG`.
    pub ir_rx1_clk: RW<u32>,
    /// 0x1104 - `IR_RX1_GAR_REG`.
    pub ir_rx1_gar: RW<SingleBusGatingReset>,
    /// 0x1108 - `IR_RX2_CLK_REG`.
    pub ir_rx2_clk: RW<u32>,
    /// 0x110c - `IR_RX2_GAR_REG`.
    pub ir_rx2_gar: RW<SingleBusGatingReset>,
    /// 0x1110 - `IR_RX3_CLK_REG`.
    pub ir_rx3_clk: RW<u32>,
    /// 0x1114 - `IR_RX3_GAR_REG`.
    pub ir_rx3_gar: RW<SingleBusGatingReset>,
    _reserved_1118: [u8; 0x0e8],
    /// 0x1200 - `I2S0_CLK_REG`.
    pub i2s0_clk: RW<u32>,
    _reserved_1204: [u8; 0x008],
    /// 0x120c - `I2S0_GAR_REG`.
    pub i2s0_gar: RW<SingleBusGatingReset>,
    /// 0x1210 - `I2S1_CLK_REG`.
    pub i2s1_clk: RW<u32>,
    _reserved_1214: [u8; 0x008],
    /// 0x121c - `I2S1_GAR_REG`.
    pub i2s1_gar: RW<SingleBusGatingReset>,
    /// 0x1220 - `I2S2_CLK_REG`.
    pub i2s2_clk: RW<u32>,
    _reserved_1224: [u8; 0x008],
    /// 0x122c - `I2S2_GAR_REG`.
    pub i2s2_gar: RW<SingleBusGatingReset>,
    _reserved_1230: [u8; 0x050],
    /// 0x1280 - `OWA0_TX_CLK_REG`.
    pub owa0_tx_clk: RW<u32>,
    /// 0x1284 - `OWA0_RX_CLK_REG`.
    pub owa0_rx_clk: RW<u32>,
    _reserved_1288: [u8; 0x004],
    /// 0x128c - `OWA0_GAR_REG`.
    pub owa0_gar: RW<SingleBusGatingReset>,
    _reserved_1290: [u8; 0x030],
    /// 0x12c0 - `DMIC_CLK_REG`.
    pub dmic_clk: RW<u32>,
    _reserved_12c4: [u8; 0x008],
    /// 0x12cc - `DMIC_GAR_REG`.
    pub dmic_gar: RW<SingleBusGatingReset>,
    _reserved_12d0: [u8; 0x010],
    /// 0x12e0 - `AUDIOCODEC0_DAC_CLK_REG`.
    pub audiocodec0_dac_clk: RW<u32>,
    _reserved_12e4: [u8; 0x008],
    /// 0x12ec - `AUDIOCODEC0_GAR_REG`.
    pub audiocodec0_gar: RW<SingleBusGatingReset>,
    _reserved_12f0: [u8; 0x010],
    /// 0x1300 - `USB0_CLK_REG`.
    pub usb0_clk: RW<u32>,
    /// 0x1304 - `USB0_GAR_REG`.
    pub usb0_gar: RW<SingleBusGatingReset>,
    /// 0x1308 - `USB1_CLK_REG`.
    pub usb1_clk: RW<u32>,
    /// 0x130c - `USB1_GAR_REG`.
    pub usb1_gar: RW<SingleBusGatingReset>,
    _reserved_1310: [u8; 0x030],
    /// 0x1340 - `USB2P0_SYS_PHY_REF_CLK_REG`.
    pub usb2p0_sys_phy_ref_clk: RW<u32>,
    /// 0x1344 - `USB2P0_SYS_GAR_REG`.
    pub usb2p0_sys_gar: RW<SingleBusGatingReset>,
    _reserved_1348: [u8; 0x0b8],
    /// 0x1400 - `GMAC0_PHY_CLK_REG`.
    pub gmac0_phy_clk: RW<u32>,
    /// 0x1404 - `GMAC0_PTP_REF_CLK_REG`.
    pub gmac0_ptp_ref_clk: RW<u32>,
    _reserved_1408: [u8; 0x004],
    /// 0x140c - `GMAC0_GAR_REG`.
    pub gmac0_gar: RW<SingleBusGatingReset>,
    /// 0x1410 - `GMAC1_PHY_CLK_REG`.
    pub gmac1_phy_clk: RW<u32>,
    /// 0x1414 - `GMAC1_PTP_REF_CLK_REG`.
    pub gmac1_ptp_ref_clk: RW<u32>,
    _reserved_1418: [u8; 0x004],
    /// 0x141c - `GMAC1_GAR_REG`.
    pub gmac1_gar: RW<SingleBusGatingReset>,
    /// 0x1420 - `GMAC2_PHY_CLK_REG`.
    pub gmac2_phy_clk: RW<u32>,
    /// 0x1424 - `GMAC2_PTP_REF_CLK_REG`.
    pub gmac2_ptp_ref_clk: RW<u32>,
    _reserved_1428: [u8; 0x004],
    /// 0x142c - `GMAC2_GAR_REG`.
    pub gmac2_gar: RW<SingleBusGatingReset>,
    _reserved_1430: [u8; 0x0d0],
    /// 0x1500 - `TCON_LCD0_CLK_REG`.
    pub tcon_lcd0_clk: RW<u32>,
    /// 0x1504 - `TCON_LCD0_GAR_REG`.
    pub tcon_lcd0_gar: RW<SingleBusGatingReset>,
    _reserved_1508: [u8; 0x03c],
    /// 0x1544 - `LVDS0_GAR_REG`.
    pub lvds0_gar: RW<u32>,
    _reserved_1548: [u8; 0x038],
    /// 0x1580 - `MIPI_DSI0_CLK_REG`.
    pub mipi_dsi0_clk: RW<u32>,
    /// 0x1584 - `MIPI_DSI0_GAR_REG`.
    pub mipi_dsi0_gar: RW<SingleBusGatingReset>,
    _reserved_1588: [u8; 0x038],
    /// 0x15c0 - `COMBOPHY0_CLK_REG`.
    pub combophy0_clk: RW<u32>,
    _reserved_15c4: [u8; 0x100],
    /// 0x16c4 - `VO0_REG_GAR_REG`.
    pub vo0_reg_gar: RW<SingleBusGatingReset>,
    _reserved_16c8: [u8; 0x01c],
    /// 0x16e4 - `VIDEO_OUT0_GAR_REG`.
    pub video_out0_gar: RW<u32>,
    _reserved_16e8: [u8; 0x018],
    /// 0x1700 - `LEDC_CLK_REG`.
    pub ledc_clk: RW<u32>,
    /// 0x1704 - `LEDC_GAR_REG`.
    pub ledc_gar: RW<SingleBusGatingReset>,
    _reserved_1708: [u8; 0x0f8],
    /// 0x1800 - `CSI_MASTER0_CLK_REG`.
    pub csi_master0_clk: RW<u32>,
    /// 0x1804 - `CSI_MASTER1_CLK_REG`.
    pub csi_master1_clk: RW<u32>,
    /// 0x1808 - `CSI_MASTER2_CLK_REG`.
    pub csi_master2_clk: RW<u32>,
    _reserved_180c: [u8; 0x034],
    /// 0x1840 - `CSI_CLK_REG`.
    pub csi_clk: RW<u32>,
    _reserved_1844: [u8; 0x01c],
    /// 0x1860 - `ISP_CLK_REG`.
    pub isp_clk: RW<u32>,
    _reserved_1864: [u8; 0x020],
    /// 0x1884 - `VIDEO_IN_GAR_REG`.
    pub video_in_gar: RW<SingleBusGatingReset>,
    _reserved_1888: [u8; 0x080],
    /// 0x1908 - `PERI0PLL_GATE_EN_REG`.
    pub peri0pll_gate_en: RW<u32>,
    /// 0x190c - `PERI1PLL_GATE_EN_REG`.
    pub peri1pll_gate_en: RW<u32>,
    /// 0x1910 - `VIDEOPLL_GATE_EN_REG`.
    pub videopll_gate_en: RW<u32>,
    _reserved_1914: [u8; 0x008],
    /// 0x191c - `AUDIOPLL_GATE_EN_REG`.
    pub audiopll_gate_en: RW<u32>,
    _reserved_1920: [u8; 0x068],
    /// 0x1988 - `PERI0PLL_GATE_STAT_REG`.
    pub peri0pll_gate_stat: RO<u32>,
    /// 0x198c - `PERI1PLL_GATE_STAT_REG`.
    pub peri1pll_gate_stat: RO<u32>,
    /// 0x1990 - `VIDEOPLL_GATE_STAT_REG`.
    pub videopll_gate_stat: RO<u32>,
    _reserved_1994: [u8; 0x008],
    /// 0x199c - `AUDIOPLL_GATE_STAT_REG`.
    pub audiopll_gate_stat: RO<u32>,
    _reserved_19a0: [u8; 0x080],
    /// 0x1a20 - `PLL_OPG_BYPASS_REG`.
    pub pll_opg_bypass: RW<u32>,
    _reserved_1a24: [u8; 0x1dc],
    /// 0x1c00 - `AXI_MON_GAR_REG`.
    pub axi_mon_gar: RW<SingleBusGatingReset>,
    /// 0x1c04 - `AHB_MON_GAR_REG`.
    pub ahb_mon_gar: RW<SingleBusGatingReset>,
    _reserved_1c08: [u8; 0x2f8],
    /// 0x1f00 - `CCU_SEC_SWITCH_REG`.
    pub sec_switch: RW<u32>,
    _reserved_1f04: [u8; 0x00c],
    /// 0x1f10 - `DAP_REQ_CTRL_REG`.
    pub dap_req_ctrl: RW<u32>,
    _reserved_1f14: [u8; 0x00c],
    /// 0x1f20 - `PLL_CFG0_REG`.
    pub pll_cfg0: RW<u32>,
    /// 0x1f24 - `PLL_CFG1_REG`.
    pub pll_cfg1: RW<u32>,
    /// 0x1f28 - `PLL_CFG2_REG`.
    pub pll_cfg2: RW<u32>,
    /// 0x1f2c - `PLL_LOCK_DBG_CTRL_REG`.
    pub pll_lock_dbg_ctrl: RW<u32>,
    /// 0x1f30 - `CCU_FAN_GATE_REG`.
    pub fan_gate: RW<u32>,
    /// 0x1f34 - `CLK27M_FAN_REG`.
    pub clk27m_fan: RW<u32>,
    /// 0x1f38 - `CLK_FAN_REG`.
    pub clk_fan: RW<u32>,
    /// 0x1f3c - `CCU_FAN_REG`.
    pub fan: RW<u32>,
    _reserved_1f40: [u8; 0x010],
    /// 0x1f50 - `CLK_DBG_REG`.
    pub clk_dbg: RW<u32>,
    _reserved_1f54: [u8; 0x00c],
    /// 0x1f60 - `FRE_DET_CTRL_REG`.
    pub fre_det_ctrl: RW<u32>,
    /// 0x1f64 - `FRE_UP_LIM_REG`.
    pub fre_up_lim: RW<u32>,
    /// 0x1f68 - `FRE_DOWN_LIM_REG`.
    pub fre_down_lim: RW<u32>,
    _reserved_1f6c: [u8; 0x084],
    /// 0x1ff0 - `CCU_VERSION_REG`.
    pub version: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, gpio_pow_mode), 0x040);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_ctrl), 0x0a0);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat0_ctrl), 0x0a8);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat1_ctrl), 0x0ac);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_bias), 0x0b0);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_ctrl), 0x0c0);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_pat0_ctrl), 0x0c8);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_pat1_ctrl), 0x0cc);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_bias), 0x0d0);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_ctrl), 0x120);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat0_ctrl), 0x128);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat1_ctrl), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_bias), 0x130);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_ctrl), 0x260);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_pat0_ctrl), 0x268);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_pat1_ctrl), 0x26c);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_bias), 0x270);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_ctrl), 0x340);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_pat0_ctrl), 0x344);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_pat1_ctrl), 0x348);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_bias), 0x34c);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_tun1), 0x350);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_ssc), 0x354);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_echo), 0x358);
        assert_eq!(offset_of!(RegisterBlock, ahb_clk), 0x500);
        assert_eq!(offset_of!(RegisterBlock, apb0_clk), 0x510);
        assert_eq!(offset_of!(RegisterBlock, apb1_clk), 0x518);
        assert_eq!(offset_of!(RegisterBlock, apb_uart_clk), 0x538);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk), 0x588);
        assert_eq!(offset_of!(RegisterBlock, ahb_mat_clk_gate_en), 0x5c0);
        assert_eq!(offset_of!(RegisterBlock, peri_mat_clk_gate_en), 0x5d0);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk_gate_en), 0x5e0);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gate_en), 0x5e4);
        assert_eq!(offset_of!(RegisterBlock, ahb_mat_clk_auto_gate_en), 0x5f0);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_auto_gate_en), 0x5f4);
        assert_eq!(offset_of!(RegisterBlock, ahb_mat_clk_gate_stat), 0x5f8);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gate_stat), 0x5fc);
        assert_eq!(offset_of!(RegisterBlock, dma0_gar), 0x704);
        assert_eq!(offset_of!(RegisterBlock, dma1_gar), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, spinlock_gar), 0x724);
        assert_eq!(offset_of!(RegisterBlock, msgbox_cpux_gar), 0x744);
        assert_eq!(offset_of!(RegisterBlock, msgbox_core0_gar), 0x74c);
        assert_eq!(offset_of!(RegisterBlock, msgbox_core1_gar), 0x754);
        assert_eq!(offset_of!(RegisterBlock, msgbox_core2_gar), 0x75c);
        assert_eq!(offset_of!(RegisterBlock, msgbox_core3_gar), 0x764);
        assert_eq!(offset_of!(RegisterBlock, msgbox_rv_gar), 0x76c);
        assert_eq!(offset_of!(RegisterBlock, pwm0_gar), 0x784);
        assert_eq!(offset_of!(RegisterBlock, pwm1_gar), 0x78c);
        assert_eq!(offset_of!(RegisterBlock, pwm2_gar), 0x794);
        assert_eq!(offset_of!(RegisterBlock, dcu_gar), 0x7a4);
        assert_eq!(offset_of!(RegisterBlock, dap_gar), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, pwmcs0_clk), 0x7c0);
        assert_eq!(offset_of!(RegisterBlock, pwmcs0_gar), 0x7c4);
        assert_eq!(offset_of!(RegisterBlock, pwmcs1_clk), 0x7c8);
        assert_eq!(offset_of!(RegisterBlock, pwmcs1_gar), 0x7cc);
        assert_eq!(offset_of!(RegisterBlock, timer0_0_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, timer0_1_clk), 0x804);
        assert_eq!(offset_of!(RegisterBlock, timer0_2_clk), 0x808);
        assert_eq!(offset_of!(RegisterBlock, timer0_3_clk), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, timer0_4_clk), 0x810);
        assert_eq!(offset_of!(RegisterBlock, timer0_5_clk), 0x814);
        assert_eq!(offset_of!(RegisterBlock, timer0_6_clk), 0x818);
        assert_eq!(offset_of!(RegisterBlock, timer0_7_clk), 0x81c);
        assert_eq!(offset_of!(RegisterBlock, timer0_gar), 0x850);
        assert_eq!(offset_of!(RegisterBlock, timer0_0_rv_clk), 0x860);
        assert_eq!(offset_of!(RegisterBlock, timer0_1_rv_clk), 0x864);
        assert_eq!(offset_of!(RegisterBlock, timer0_2_rv_clk), 0x868);
        assert_eq!(offset_of!(RegisterBlock, timer0_3_rv_clk), 0x86c);
        assert_eq!(offset_of!(RegisterBlock, timer0_rv_gar), 0x870);
        assert_eq!(offset_of!(RegisterBlock, de0_clk), 0xa00);
        assert_eq!(offset_of!(RegisterBlock, de0_gar), 0xa04);
        assert_eq!(offset_of!(RegisterBlock, g2d_clk), 0xa40);
        assert_eq!(offset_of!(RegisterBlock, g2d_gar), 0xa44);
        assert_eq!(offset_of!(RegisterBlock, ce_sys_clk), 0xac0);
        assert_eq!(offset_of!(RegisterBlock, ce_sys_gar), 0xac4);
        assert_eq!(offset_of!(RegisterBlock, rv_core_clk), 0xb80);
        assert_eq!(offset_of!(RegisterBlock, rv_ts_clk), 0xb88);
        assert_eq!(offset_of!(RegisterBlock, rv_sys_gar), 0xb94);
        assert_eq!(offset_of!(RegisterBlock, rv_cfg_gar), 0xb9c);
        assert_eq!(offset_of!(RegisterBlock, dramc_gar), 0xc0c);
        assert_eq!(offset_of!(RegisterBlock, smhc0_clk), 0xd00);
        assert_eq!(offset_of!(RegisterBlock, smhc0_gar), 0xd0c);
        assert_eq!(offset_of!(RegisterBlock, smhc1_clk), 0xd10);
        assert_eq!(offset_of!(RegisterBlock, smhc1_gar), 0xd1c);
        assert_eq!(offset_of!(RegisterBlock, smhc2_clk), 0xd20);
        assert_eq!(offset_of!(RegisterBlock, smhc2_gar), 0xd2c);
        assert_eq!(offset_of!(RegisterBlock, uart0_gar), 0xe00);
        assert_eq!(offset_of!(RegisterBlock, uart1_gar), 0xe04);
        assert_eq!(offset_of!(RegisterBlock, uart2_gar), 0xe08);
        assert_eq!(offset_of!(RegisterBlock, uart3_gar), 0xe0c);
        assert_eq!(offset_of!(RegisterBlock, uart4_gar), 0xe10);
        assert_eq!(offset_of!(RegisterBlock, uart5_gar), 0xe14);
        assert_eq!(offset_of!(RegisterBlock, uart6_gar), 0xe18);
        assert_eq!(offset_of!(RegisterBlock, uart7_gar), 0xe20);
        assert_eq!(offset_of!(RegisterBlock, uart8_gar), 0xe24);
        assert_eq!(offset_of!(RegisterBlock, uart9_gar), 0xe28);
        assert_eq!(offset_of!(RegisterBlock, twi0_gar), 0xe80);
        assert_eq!(offset_of!(RegisterBlock, twi1_gar), 0xe84);
        assert_eq!(offset_of!(RegisterBlock, twi2_gar), 0xe88);
        assert_eq!(offset_of!(RegisterBlock, twi3_gar), 0xe8c);
        assert_eq!(offset_of!(RegisterBlock, twi4_gar), 0xe90);
        assert_eq!(offset_of!(RegisterBlock, twi5_gar), 0xe94);
        assert_eq!(offset_of!(RegisterBlock, spi0_clk), 0xf00);
        assert_eq!(offset_of!(RegisterBlock, spi0_gar), 0xf04);
        assert_eq!(offset_of!(RegisterBlock, spi1_clk), 0xf08);
        assert_eq!(offset_of!(RegisterBlock, spi1_gar), 0xf0c);
        assert_eq!(offset_of!(RegisterBlock, spi2_clk), 0xf10);
        assert_eq!(offset_of!(RegisterBlock, spi2_gar), 0xf14);
        assert_eq!(offset_of!(RegisterBlock, spif_clk), 0xf18);
        assert_eq!(offset_of!(RegisterBlock, spif_gar), 0xf1c);
        assert_eq!(offset_of!(RegisterBlock, spi3_clk), 0xf20);
        assert_eq!(offset_of!(RegisterBlock, spi3_gar), 0xf24);
        assert_eq!(offset_of!(RegisterBlock, gpadc0_clk), 0xfc0);
        assert_eq!(offset_of!(RegisterBlock, gpadc0_gar), 0xfc4);
        assert_eq!(offset_of!(RegisterBlock, gpadc1_clk), 0xfc8);
        assert_eq!(offset_of!(RegisterBlock, gpadc1_gar), 0xfcc);
        assert_eq!(offset_of!(RegisterBlock, gpadc2_clk), 0xfd0);
        assert_eq!(offset_of!(RegisterBlock, gpadc2_gar), 0xfd4);
        assert_eq!(offset_of!(RegisterBlock, tsensor_gar), 0xfe4);
        assert_eq!(offset_of!(RegisterBlock, ir_rx0_clk), 0x1000);
        assert_eq!(offset_of!(RegisterBlock, ir_rx0_gar), 0x1004);
        assert_eq!(offset_of!(RegisterBlock, ir_tx_clk), 0x1008);
        assert_eq!(offset_of!(RegisterBlock, ir_tx_gar), 0x100c);
        assert_eq!(offset_of!(RegisterBlock, tpadc_clk), 0x1030);
        assert_eq!(offset_of!(RegisterBlock, tpadc_gar), 0x1034);
        assert_eq!(offset_of!(RegisterBlock, lbc_clk), 0x1040);
        assert_eq!(offset_of!(RegisterBlock, lbc_gar), 0x104c);
        assert_eq!(offset_of!(RegisterBlock, ir_rx1_clk), 0x1100);
        assert_eq!(offset_of!(RegisterBlock, ir_rx1_gar), 0x1104);
        assert_eq!(offset_of!(RegisterBlock, ir_rx2_clk), 0x1108);
        assert_eq!(offset_of!(RegisterBlock, ir_rx2_gar), 0x110c);
        assert_eq!(offset_of!(RegisterBlock, ir_rx3_clk), 0x1110);
        assert_eq!(offset_of!(RegisterBlock, ir_rx3_gar), 0x1114);
        assert_eq!(offset_of!(RegisterBlock, i2s0_clk), 0x1200);
        assert_eq!(offset_of!(RegisterBlock, i2s0_gar), 0x120c);
        assert_eq!(offset_of!(RegisterBlock, i2s1_clk), 0x1210);
        assert_eq!(offset_of!(RegisterBlock, i2s1_gar), 0x121c);
        assert_eq!(offset_of!(RegisterBlock, i2s2_clk), 0x1220);
        assert_eq!(offset_of!(RegisterBlock, i2s2_gar), 0x122c);
        assert_eq!(offset_of!(RegisterBlock, owa0_tx_clk), 0x1280);
        assert_eq!(offset_of!(RegisterBlock, owa0_rx_clk), 0x1284);
        assert_eq!(offset_of!(RegisterBlock, owa0_gar), 0x128c);
        assert_eq!(offset_of!(RegisterBlock, dmic_clk), 0x12c0);
        assert_eq!(offset_of!(RegisterBlock, dmic_gar), 0x12cc);
        assert_eq!(offset_of!(RegisterBlock, audiocodec0_dac_clk), 0x12e0);
        assert_eq!(offset_of!(RegisterBlock, audiocodec0_gar), 0x12ec);
        assert_eq!(offset_of!(RegisterBlock, usb0_clk), 0x1300);
        assert_eq!(offset_of!(RegisterBlock, usb0_gar), 0x1304);
        assert_eq!(offset_of!(RegisterBlock, usb1_clk), 0x1308);
        assert_eq!(offset_of!(RegisterBlock, usb1_gar), 0x130c);
        assert_eq!(offset_of!(RegisterBlock, usb2p0_sys_phy_ref_clk), 0x1340);
        assert_eq!(offset_of!(RegisterBlock, usb2p0_sys_gar), 0x1344);
        assert_eq!(offset_of!(RegisterBlock, gmac0_phy_clk), 0x1400);
        assert_eq!(offset_of!(RegisterBlock, gmac0_ptp_ref_clk), 0x1404);
        assert_eq!(offset_of!(RegisterBlock, gmac0_gar), 0x140c);
        assert_eq!(offset_of!(RegisterBlock, gmac1_phy_clk), 0x1410);
        assert_eq!(offset_of!(RegisterBlock, gmac1_ptp_ref_clk), 0x1414);
        assert_eq!(offset_of!(RegisterBlock, gmac1_gar), 0x141c);
        assert_eq!(offset_of!(RegisterBlock, gmac2_phy_clk), 0x1420);
        assert_eq!(offset_of!(RegisterBlock, gmac2_ptp_ref_clk), 0x1424);
        assert_eq!(offset_of!(RegisterBlock, gmac2_gar), 0x142c);
        assert_eq!(offset_of!(RegisterBlock, tcon_lcd0_clk), 0x1500);
        assert_eq!(offset_of!(RegisterBlock, tcon_lcd0_gar), 0x1504);
        assert_eq!(offset_of!(RegisterBlock, lvds0_gar), 0x1544);
        assert_eq!(offset_of!(RegisterBlock, mipi_dsi0_clk), 0x1580);
        assert_eq!(offset_of!(RegisterBlock, mipi_dsi0_gar), 0x1584);
        assert_eq!(offset_of!(RegisterBlock, combophy0_clk), 0x15c0);
        assert_eq!(offset_of!(RegisterBlock, vo0_reg_gar), 0x16c4);
        assert_eq!(offset_of!(RegisterBlock, video_out0_gar), 0x16e4);
        assert_eq!(offset_of!(RegisterBlock, ledc_clk), 0x1700);
        assert_eq!(offset_of!(RegisterBlock, ledc_gar), 0x1704);
        assert_eq!(offset_of!(RegisterBlock, csi_master0_clk), 0x1800);
        assert_eq!(offset_of!(RegisterBlock, csi_master1_clk), 0x1804);
        assert_eq!(offset_of!(RegisterBlock, csi_master2_clk), 0x1808);
        assert_eq!(offset_of!(RegisterBlock, csi_clk), 0x1840);
        assert_eq!(offset_of!(RegisterBlock, isp_clk), 0x1860);
        assert_eq!(offset_of!(RegisterBlock, video_in_gar), 0x1884);
        assert_eq!(offset_of!(RegisterBlock, peri0pll_gate_en), 0x1908);
        assert_eq!(offset_of!(RegisterBlock, peri1pll_gate_en), 0x190c);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_en), 0x1910);
        assert_eq!(offset_of!(RegisterBlock, audiopll_gate_en), 0x191c);
        assert_eq!(offset_of!(RegisterBlock, peri0pll_gate_stat), 0x1988);
        assert_eq!(offset_of!(RegisterBlock, peri1pll_gate_stat), 0x198c);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_stat), 0x1990);
        assert_eq!(offset_of!(RegisterBlock, audiopll_gate_stat), 0x199c);
        assert_eq!(offset_of!(RegisterBlock, pll_opg_bypass), 0x1a20);
        assert_eq!(offset_of!(RegisterBlock, axi_mon_gar), 0x1c00);
        assert_eq!(offset_of!(RegisterBlock, ahb_mon_gar), 0x1c04);
        assert_eq!(offset_of!(RegisterBlock, sec_switch), 0x1f00);
        assert_eq!(offset_of!(RegisterBlock, dap_req_ctrl), 0x1f10);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg0), 0x1f20);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg1), 0x1f24);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg2), 0x1f28);
        assert_eq!(offset_of!(RegisterBlock, pll_lock_dbg_ctrl), 0x1f2c);
        assert_eq!(offset_of!(RegisterBlock, fan_gate), 0x1f30);
        assert_eq!(offset_of!(RegisterBlock, clk27m_fan), 0x1f34);
        assert_eq!(offset_of!(RegisterBlock, clk_fan), 0x1f38);
        assert_eq!(offset_of!(RegisterBlock, fan), 0x1f3c);
        assert_eq!(offset_of!(RegisterBlock, clk_dbg), 0x1f50);
        assert_eq!(offset_of!(RegisterBlock, fre_det_ctrl), 0x1f60);
        assert_eq!(offset_of!(RegisterBlock, fre_up_lim), 0x1f64);
        assert_eq!(offset_of!(RegisterBlock, fre_down_lim), 0x1f68);
        assert_eq!(offset_of!(RegisterBlock, version), 0x1ff0);
        assert_eq!(size_of::<RegisterBlock>(), 0x1ff4);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
