//! A537/A333 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun65iw1` platform.

use super::SingleBusGatingReset;
use volatile_register::{RO, RW};

/// A537/A333 main CCU register block.
#[doc(alias = "sun65iw1")]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0000: [u8; 0x0a0],
    /// 0x00a0 - `PLL_PERI0_CTRL_REG, SUN65IW1_PLL_PERI0_CTRL_REG, pll_peri0_2x_clk, ...`.
    pub pll_peri0_ctrl: RW<u32>,
    _reserved_00a4: [u8; 0x004],
    /// 0x00a8 - `PLL_PERI0_PAT0_CTRL_REG`.
    pub pll_peri0_pat0_ctrl: RW<u32>,
    /// 0x00ac - `PLL_PERI0_PAT1_CTRL_REG`.
    pub pll_peri0_pat1_ctrl: RW<u32>,
    /// 0x00b0 - `PLL_PERI0_BIAS_REG`.
    pub pll_peri0_bias: RW<u32>,
    _reserved_00b4: [u8; 0x00c],
    /// 0x00c0 - `PLL_PERI1_CTRL_REG, SUN65IW1_PLL_PERI1_CTRL_REG, pll_peri1_2x_clk, ...`.
    pub pll_peri1_ctrl: RW<u32>,
    _reserved_00c4: [u8; 0x004],
    /// 0x00c8 - `PLL_PERI1_PAT0_CTRL_REG, SUN65IW1_PLL_PERIPH1_PATTERN0_REG, pll_peri1_sdm_pat0_clk`.
    pub pll_periph1_pattern0: RW<u32>,
    /// 0x00cc - `PLL_PERI1_PAT1_CTRL_REG`.
    pub pll_peri1_pat1_ctrl: RW<u32>,
    /// 0x00d0 - `PLL_PERI1_BIAS_REG`.
    pub pll_peri1_bias: RW<u32>,
    _reserved_00d4: [u8; 0x00c],
    /// 0x00e0 - `PLL_GPU_CTRL_REG, SUN65IW1_PLL_GPU_CTRL_REG, pll_gpu_clk`.
    pub pll_gpu_ctrl: RW<u32>,
    _reserved_00e4: [u8; 0x004],
    /// 0x00e8 - `PLL_GPU_PAT0_CTRL_REG`.
    pub pll_gpu_pat0_ctrl: RW<u32>,
    /// 0x00ec - `PLL_GPU_PAT1_CTRL_REG`.
    pub pll_gpu_pat1_ctrl: RW<u32>,
    /// 0x00f0 - `PLL_GPU_BIAS_REG`.
    pub pll_gpu_bias: RW<u32>,
    _reserved_00f4: [u8; 0x02c],
    /// 0x0120 - `PLL_VIDEO0_CTRL_REG, SUN65IW1_PLL_VIDEO0_CTRL_REG, pll_video0_3x_clk, ...`.
    pub pll_video0_ctrl: RW<u32>,
    _reserved_0124: [u8; 0x004],
    /// 0x0128 - `PLL_VIDEO0_PAT0_CTRL_REG`.
    pub pll_video0_pat0_ctrl: RW<u32>,
    /// 0x012c - `PLL_VIDEO0_PAT1_CTRL_REG`.
    pub pll_video0_pat1_ctrl: RW<u32>,
    /// 0x0130 - `PLL_VIDEO0_BIAS_REG`.
    pub pll_video0_bias: RW<u32>,
    _reserved_0134: [u8; 0x00c],
    /// 0x0140 - `PLL_VIDEO1_CTRL_REG, SUN65IW1_PLL_VIDEO1_CTRL_REG, pll_video1_3x_clk, ...`.
    pub pll_video1_ctrl: RW<u32>,
    _reserved_0144: [u8; 0x004],
    /// 0x0148 - `PLL_VIDEO1_PAT0_CTRL_REG`.
    pub pll_video1_pat0_ctrl: RW<u32>,
    /// 0x014c - `PLL_VIDEO1_PAT1_CTRL_REG`.
    pub pll_video1_pat1_ctrl: RW<u32>,
    /// 0x0150 - `PLL_VIDEO1_BIAS_REG`.
    pub pll_video1_bias: RW<u32>,
    _reserved_0154: [u8; 0x00c],
    /// 0x0160 - `PLL_VIDEO2_CTRL_REG, SUN65IW1_PLL_VIDEO2_CTRL_REG, pll_video2_3x_clk, ...`.
    pub pll_video2_ctrl: RW<u32>,
    _reserved_0164: [u8; 0x004],
    /// 0x0168 - `PLL_VIDEO2_PAT0_CTRL_REG`.
    pub pll_video2_pat0_ctrl: RW<u32>,
    /// 0x016c - `PLL_VIDEO2_PAT1_CTRL_REG`.
    pub pll_video2_pat1_ctrl: RW<u32>,
    /// 0x0170 - `PLL_VIDEO2_BIAS_REG`.
    pub pll_video2_bias: RW<u32>,
    _reserved_0174: [u8; 0x0ac],
    /// 0x0220 - `PLL_VE_CTRL_REG, SUN65IW1_PLL_VE_CTRL_REG, pll_ve_clk`.
    pub pll_ve_ctrl: RW<u32>,
    _reserved_0224: [u8; 0x004],
    /// 0x0228 - `PLL_VE_PAT0_CTRL_REG`.
    pub pll_ve_pat0_ctrl: RW<u32>,
    /// 0x022c - `PLL_VE_PAT1_CTRL_REG`.
    pub pll_ve_pat1_ctrl: RW<u32>,
    /// 0x0230 - `PLL_VE_BIAS_REG`.
    pub pll_ve_bias: RW<u32>,
    _reserved_0234: [u8; 0x02c],
    /// 0x0260 - `PLL_AUDIO0_CTRL_REG, SUN65IW1_PLL_AUDIO0_CTRL_REG, pll_audio0_clk`.
    pub pll_audio0_ctrl: RW<u32>,
    _reserved_0264: [u8; 0x004],
    /// 0x0268 - `PLL_AUDIO0_PAT0_CTRL_REG, pll_audio0_sdm_pat0_clk`.
    pub pll_audio0_pat0_ctrl: RW<u32>,
    /// 0x026c - `PLL_AUDIO0_PAT1_CTRL_REG`.
    pub pll_audio0_pat1_ctrl: RW<u32>,
    /// 0x0270 - `PLL_AUDIO0_BIAS_REG`.
    pub pll_audio0_bias: RW<u32>,
    _reserved_0274: [u8; 0x00c],
    /// 0x0280 - `PLL_AUDIO1_CTRL_REG, SUN65IW1_PLL_AUDIO1_CTRL_REG, pll_audio1_2x_clk, ...`.
    pub pll_audio1_ctrl: RW<u32>,
    _reserved_0284: [u8; 0x004],
    /// 0x0288 - `PLL_AUDIO1_PAT0_CTRL_REG, pll_audio1_sdm_pat0_clk`.
    pub pll_audio1_pat0_ctrl: RW<u32>,
    /// 0x028c - `PLL_AUDIO1_PAT1_CTRL_REG`.
    pub pll_audio1_pat1_ctrl: RW<u32>,
    /// 0x0290 - `PLL_AUDIO1_BIAS_REG`.
    pub pll_audio1_bias: RW<u32>,
    _reserved_0294: [u8; 0x26c],
    /// 0x0500 - `AHB_CLK_REG, ahb_clk`.
    pub ahb_clk: RW<u32>,
    _reserved_0504: [u8; 0x00c],
    /// 0x0510 - `APB0_CLK_REG, apb0_clk`.
    pub apb0_clk: RW<u32>,
    _reserved_0514: [u8; 0x004],
    /// 0x0518 - `APB1_CLK_REG, apb1_clk`.
    pub apb1_clk: RW<u32>,
    _reserved_051c: [u8; 0x01c],
    /// 0x0538 - `APB_UART_CLK_REG, apb_uart_clk`.
    pub apb_uart_clk: RW<u32>,
    _reserved_053c: [u8; 0x00c],
    /// 0x0548 - `CPU_SYS_DP_CLK_REG, cpu_sys_dp_clk`.
    pub cpu_sys_dp_clk: RW<u32>,
    _reserved_054c: [u8; 0x014],
    /// 0x0560 - `CPUX_GIC_CLK_REG, cpux_gic_clk`.
    pub cpux_gic_clk: RW<u32>,
    _reserved_0564: [u8; 0x01c],
    /// 0x0580 - `NSI_CLK_REG, nsi_clk`.
    pub nsi_clk: RW<u32>,
    /// 0x0584 - `NSI_GAR_REG, nsi_cfg_bus_clk, reset map`.
    pub nsi_gar: RW<SingleBusGatingReset>,
    /// 0x0588 - `MBUS_CLK_REG, mbus_clk`.
    pub mbus_clk: RW<u32>,
    /// 0x058c - `IOMMU_GAR_REG, iommu_apb_bus_clk`.
    pub iommu_gar: RW<u32>,
    _reserved_0590: [u8; 0x030],
    /// 0x05c0 - `AHB_MAT_CLK_GATE_EN_REG, gpu_ahb_sw_bus_clk, hsi_ahb_sw_bus_clk, ...`.
    pub ahb_mat_clk_gate_en: RW<u32>,
    _reserved_05c4: [u8; 0x01c],
    /// 0x05e0 - `MBUS_MAT_CLK_GATE_EN_REG, ce_sys_axi_bus_clk, de_sys_mbus_clk, ...`.
    pub mbus_mat_clk_gate_en: RW<u32>,
    /// 0x05e4 - `MBUS_CLK_GATE_EN_REG, ce_sys_axi_clk, csi_mbus_clk, ...`.
    pub mbus_clk_gate_en: RW<u32>,
    _reserved_05e8: [u8; 0x11c],
    /// 0x0704 - `DMA0_GAR_REG, dma0_ahb_bus_clk, reset map`.
    pub dma0_gar: RW<SingleBusGatingReset>,
    _reserved_0708: [u8; 0x01c],
    /// 0x0724 - `SPINLOCK_GAR_REG, reset map, spinlock_ahb_bus_clk`.
    pub spinlock_gar: RW<SingleBusGatingReset>,
    _reserved_0728: [u8; 0x01c],
    /// 0x0744 - `MSGBOX_CPUX_GAR_REG, msgbox_cpux_ahb_bus_clk, reset map`.
    pub msgbox_cpux_gar: RW<SingleBusGatingReset>,
    _reserved_0748: [u8; 0x004],
    /// 0x074c - `MSGBOX_CPUS_GAR_REG, msgbox_cpus_ahb_bus_clk, reset map`.
    pub msgbox_cpus_gar: RW<SingleBusGatingReset>,
    _reserved_0750: [u8; 0x034],
    /// 0x0784 - `PWM0_GAR_REG, pwm0_apb_bus_clk, reset map`.
    pub pwm0_gar: RW<SingleBusGatingReset>,
    _reserved_0788: [u8; 0x01c],
    /// 0x07a4 - `DCU_GAR_REG, dcu_bus_clk, reset map`.
    pub dcu_gar: RW<SingleBusGatingReset>,
    _reserved_07a8: [u8; 0x004],
    /// 0x07ac - `DAP_GAR_REG, dap_ahb_bus_clk, reset map`.
    pub dap_gar: RW<SingleBusGatingReset>,
    _reserved_07b0: [u8; 0x050],
    /// 0x0800 - `TIMER0_0_CLK_REG, timer0_0_clk`.
    pub timer0_0_clk: RW<u32>,
    /// 0x0804 - `TIMER0_1_CLK_REG, timer0_1_clk`.
    pub timer0_1_clk: RW<u32>,
    /// 0x0808 - `TIMER0_2_CLK_REG, timer0_2_clk`.
    pub timer0_2_clk: RW<u32>,
    /// 0x080c - `TIMER0_3_CLK_REG, timer0_3_clk`.
    pub timer0_3_clk: RW<u32>,
    _reserved_0810: [u8; 0x040],
    /// 0x0850 - `TIMER0_GAR_REG, reset map, timer0_ahb_bus_clk`.
    pub timer0_gar: RW<SingleBusGatingReset>,
    _reserved_0854: [u8; 0x1ac],
    /// 0x0a00 - `DE0_CLK_REG, de_clk`.
    pub de0_clk: RW<u32>,
    /// 0x0a04 - `DE0_GAR_REG, de0_ahb_bus_clk, reset map`.
    pub de0_gar: RW<SingleBusGatingReset>,
    _reserved_0a08: [u8; 0x038],
    /// 0x0a40 - `G2D_CLK_REG, g2d_clk`.
    pub g2d_clk: RW<u32>,
    /// 0x0a44 - `G2D_GAR_REG, g2d_ahb_bus_clk, reset map`.
    pub g2d_gar: RW<SingleBusGatingReset>,
    _reserved_0a48: [u8; 0x01c],
    /// 0x0a64 - `EINK_PANEL_CLK_REG, eink_panel_clk`.
    pub eink_panel_clk: RW<u32>,
    _reserved_0a68: [u8; 0x004],
    /// 0x0a6c - `EINK_GAR_REG, eink_ahb_bus_clk, reset map`.
    pub eink_gar: RW<SingleBusGatingReset>,
    _reserved_0a70: [u8; 0x010],
    /// 0x0a80 - `VE0_CLK_REG, ve0_clk`.
    pub ve0_clk: RW<u32>,
    _reserved_0a84: [u8; 0x008],
    /// 0x0a8c - `VE0_GAR_REG, reset map, ve0_ahb_bus_clk`.
    pub ve0_gar: RW<SingleBusGatingReset>,
    _reserved_0a90: [u8; 0x030],
    /// 0x0ac0 - `CE_SYS_CLK_REG, ce_sys_clk`.
    pub ce_sys_clk: RW<u32>,
    /// 0x0ac4 - `CE_SYS_GAR_REG, ce_sys_ip_ahb_bus_clk, reset map`.
    pub ce_sys_gar: RW<SingleBusGatingReset>,
    _reserved_0ac8: [u8; 0x058],
    /// 0x0b20 - `GPU_CLK_REG, gpu_clk`.
    pub gpu_clk: RW<u32>,
    /// 0x0b24 - `GPU_GAR_REG, gpu_ahb_bus_clk, reset map`.
    pub gpu_gar: RW<SingleBusGatingReset>,
    _reserved_0b28: [u8; 0x0e4],
    /// 0x0c0c - `DRAMC_GAR_REG, dramc_ahb_bus_clk, reset map`.
    pub dramc_gar: RW<SingleBusGatingReset>,
    _reserved_0c10: [u8; 0x0f0],
    /// 0x0d00 - `SMHC0_CLK_REG, smhc0_clk`.
    pub smhc0_clk: RW<u32>,
    _reserved_0d04: [u8; 0x008],
    /// 0x0d0c - `SMHC0_GAR_REG, reset map, smhc0_ahb_bus_clk`.
    pub smhc0_gar: RW<SingleBusGatingReset>,
    /// 0x0d10 - `SMHC1_CLK_REG, smhc1_clk`.
    pub smhc1_clk: RW<u32>,
    _reserved_0d14: [u8; 0x008],
    /// 0x0d1c - `SMHC1_GAR_REG, reset map, smhc1_ahb_bus_clk`.
    pub smhc1_gar: RW<SingleBusGatingReset>,
    /// 0x0d20 - `SMHC2_CLK_REG, smhc2_clk`.
    pub smhc2_clk: RW<u32>,
    _reserved_0d24: [u8; 0x008],
    /// 0x0d2c - `SMHC2_GAR_REG, reset map, smhc2_ahb_bus_clk`.
    pub smhc2_gar: RW<SingleBusGatingReset>,
    _reserved_0d30: [u8; 0x0d0],
    /// 0x0e00 - `UART0_GAR_REG, reset map, uart0_apb_bus_clk`.
    pub uart0_gar: RW<SingleBusGatingReset>,
    /// 0x0e04 - `UART1_GAR_REG, reset map, uart1_apb_bus_clk`.
    pub uart1_gar: RW<SingleBusGatingReset>,
    /// 0x0e08 - `UART2_GAR_REG, reset map, uart2_apb_bus_clk`.
    pub uart2_gar: RW<SingleBusGatingReset>,
    /// 0x0e0c - `UART3_GAR_REG, reset map, uart3_apb_bus_clk`.
    pub uart3_gar: RW<SingleBusGatingReset>,
    /// 0x0e10 - `UART4_GAR_REG, reset map, uart4_apb_bus_clk`.
    pub uart4_gar: RW<SingleBusGatingReset>,
    /// 0x0e14 - `UART5_GAR_REG, reset map, uart5_apb_bus_clk`.
    pub uart5_gar: RW<SingleBusGatingReset>,
    /// 0x0e18 - `UART6_GAR_REG, reset map, uart6_apb_bus_clk`.
    pub uart6_gar: RW<SingleBusGatingReset>,
    /// 0x0e1c - `UART7_GAR_REG, reset map, uart7_apb_bus_clk`.
    pub uart7_gar: RW<SingleBusGatingReset>,
    _reserved_0e20: [u8; 0x060],
    /// 0x0e80 - `TWI0_GAR_REG, reset map, twi0_apb_bus_clk`.
    pub twi0_gar: RW<SingleBusGatingReset>,
    /// 0x0e84 - `TWI1_GAR_REG, reset map, twi1_apb_bus_clk`.
    pub twi1_gar: RW<SingleBusGatingReset>,
    /// 0x0e88 - `TWI2_GAR_REG, reset map, twi2_apb_bus_clk`.
    pub twi2_gar: RW<SingleBusGatingReset>,
    /// 0x0e8c - `TWI3_GAR_REG, reset map, twi3_apb_bus_clk`.
    pub twi3_gar: RW<SingleBusGatingReset>,
    /// 0x0e90 - `TWI4_GAR_REG, reset map, twi4_apb_bus_clk`.
    pub twi4_gar: RW<SingleBusGatingReset>,
    /// 0x0e94 - `TWI5_GAR_REG, reset map, twi5_apb_bus_clk`.
    pub twi5_gar: RW<SingleBusGatingReset>,
    _reserved_0e98: [u8; 0x068],
    /// 0x0f00 - `SPI0_CLK_REG, spi0_clk`.
    pub spi0_clk: RW<u32>,
    /// 0x0f04 - `SPI0_GAR_REG, reset map, spi0_ahb_bus_clk`.
    pub spi0_gar: RW<SingleBusGatingReset>,
    /// 0x0f08 - `SPI1_CLK_REG, spi1_clk`.
    pub spi1_clk: RW<u32>,
    /// 0x0f0c - `SPI1_GAR_REG, reset map, spi1_ahb_bus_clk`.
    pub spi1_gar: RW<SingleBusGatingReset>,
    /// 0x0f10 - `SPI2_CLK_REG, spi2_clk`.
    pub spi2_clk: RW<u32>,
    /// 0x0f14 - `SPI2_GAR_REG, reset map, spi2_ahb_bus_clk`.
    pub spi2_gar: RW<SingleBusGatingReset>,
    _reserved_0f18: [u8; 0x0a8],
    /// 0x0fc0 - `GPADC0_CLK_REG, gpadc0_clk`.
    pub gpadc0_clk: RW<u32>,
    /// 0x0fc4 - `GPADC0_GAR_REG, gpadc0_apb_bus_clk, reset map`.
    pub gpadc0_gar: RW<SingleBusGatingReset>,
    _reserved_0fc8: [u8; 0x01c],
    /// 0x0fe4 - `TSENSOR_GAR_REG, reset map, tsensor_apb_bus_clk`.
    pub tsensor_gar: RW<SingleBusGatingReset>,
    _reserved_0fe8: [u8; 0x018],
    /// 0x1000 - `IR_RX0_CLK_REG, ir_rx0_clk`.
    pub ir_rx0_clk: RW<u32>,
    /// 0x1004 - `IR_RX0_GAR_REG, ir_rx0_apb_bus_clk, reset map`.
    pub ir_rx0_gar: RW<SingleBusGatingReset>,
    /// 0x1008 - `IR_TX_CLK_REG, ir_tx_clk`.
    pub ir_tx_clk: RW<u32>,
    /// 0x100c - `IR_TX_GAR_REG, ir_tx_apb_bus_clk, reset map`.
    pub ir_tx_gar: RW<SingleBusGatingReset>,
    _reserved_1010: [u8; 0x1f0],
    /// 0x1200 - `I2S0_CLK_REG, i2s0_clk`.
    pub i2s0_clk: RW<u32>,
    _reserved_1204: [u8; 0x008],
    /// 0x120c - `I2S0_GAR_REG, i2s0_apb_bus_clk, reset map`.
    pub i2s0_gar: RW<SingleBusGatingReset>,
    /// 0x1210 - `I2S1_CLK_REG, i2s1_clk`.
    pub i2s1_clk: RW<u32>,
    _reserved_1214: [u8; 0x008],
    /// 0x121c - `I2S1_GAR_REG, i2s1_apb_bus_clk, reset map`.
    pub i2s1_gar: RW<SingleBusGatingReset>,
    /// 0x1220 - `I2S2_CLK_REG, i2s2_clk`.
    pub i2s2_clk: RW<u32>,
    _reserved_1224: [u8; 0x008],
    /// 0x122c - `I2S2_GAR_REG, i2s2_apb_bus_clk, reset map`.
    pub i2s2_gar: RW<SingleBusGatingReset>,
    /// 0x1230 - `I2S3_CLK_REG, i2s3_clk`.
    pub i2s3_clk: RW<u32>,
    _reserved_1234: [u8; 0x008],
    /// 0x123c - `I2S3_GAR_REG, i2s3_apb_bus_clk, reset map`.
    pub i2s3_gar: RW<SingleBusGatingReset>,
    _reserved_1240: [u8; 0x040],
    /// 0x1280 - `OWA0_TX_CLK_REG, owa0_tx_clk`.
    pub owa0_tx_clk: RW<u32>,
    /// 0x1284 - `OWA0_RX_CLK_REG, owa0_rx_clk`.
    pub owa0_rx_clk: RW<u32>,
    _reserved_1288: [u8; 0x004],
    /// 0x128c - `OWA0_GAR_REG, owa0_apb_bus_clk, reset map`.
    pub owa0_gar: RW<SingleBusGatingReset>,
    _reserved_1290: [u8; 0x030],
    /// 0x12c0 - `DMIC_CLK_REG, dmic_clk`.
    pub dmic_clk: RW<u32>,
    _reserved_12c4: [u8; 0x008],
    /// 0x12cc - `DMIC_GAR_REG, dmic_apb_bus_clk, reset map`.
    pub dmic_gar: RW<SingleBusGatingReset>,
    _reserved_12d0: [u8; 0x010],
    /// 0x12e0 - `AUDIOCODEC0_DAC_CLK_REG, audiocodec0_dac_clk`.
    pub audiocodec0_dac_clk: RW<u32>,
    _reserved_12e4: [u8; 0x004],
    /// 0x12e8 - `AUDIOCODEC0_ADC_CLK_REG, audiocodec0_adc_clk`.
    pub audiocodec0_adc_clk: RW<u32>,
    /// 0x12ec - `AUDIOCODEC0_GAR_REG, audiocodec0_apb_bus_clk, reset map`.
    pub audiocodec0_gar: RW<SingleBusGatingReset>,
    /// 0x12f0 - `AUDIOCODEC1_DAC_CLK_REG, audiocodec1_dac_clk`.
    pub audiocodec1_dac_clk: RW<u32>,
    _reserved_12f4: [u8; 0x008],
    /// 0x12fc - `AUDIOCODEC1_GAR_REG, audiocodec1_apb_bus_clk, reset map`.
    pub audiocodec1_gar: RW<SingleBusGatingReset>,
    /// 0x1300 - `USB0_CLK_REG, usb0_bus_clk`.
    pub usb0_clk: RW<u32>,
    /// 0x1304 - `USB0_GAR_REG, reset map, usb0_dev_ahb_bus_clk, ...`.
    pub usb0_gar: RW<SingleBusGatingReset>,
    /// 0x1308 - `USB1_CLK_REG, usb1_bus_clk`.
    pub usb1_clk: RW<u32>,
    /// 0x130c - `USB1_GAR_REG, reset map, usb1_ehci_ahb_bus_clk, ...`.
    pub usb1_gar: RW<SingleBusGatingReset>,
    _reserved_1310: [u8; 0x030],
    /// 0x1340 - `USB2P0_SYS_PHY_REF_CLK_REG, usb2p0_sys_phy_ref_bus_clk`.
    pub usb2p0_sys_phy_ref_clk: RW<u32>,
    /// 0x1344 - `USB2P0_SYS_GAR_REG, reset map, usb2p0_sys_ahb_bus_clk`.
    pub usb2p0_sys_gar: RW<SingleBusGatingReset>,
    /// 0x1348 - `USB2_U2_PHY_REF_CLK_REG, usb2_u2_phy_ref_bus_clk`.
    pub usb2_u2_phy_ref_clk: RW<u32>,
    _reserved_134c: [u8; 0x004],
    /// 0x1350 - `USB2_SUSPEND_CLK_REG, usb2_suspend_clk`.
    pub usb2_suspend_clk: RW<u32>,
    /// 0x1354 - `USB2_MF_CLK_REG, usb2_ref_clk`.
    pub usb2_mf_clk: RW<u32>,
    _reserved_1358: [u8; 0x004],
    /// 0x135c - `USB2_GAR_REG, reset map`.
    pub usb2_gar: RW<u32>,
    /// 0x1360 - `USB2_U3_ONLY_UTMI_CLK_REG, usb2_u3_only_utmi_clk`.
    pub usb2_u3_only_utmi_clk: RW<u32>,
    /// 0x1364 - `USB2_U2_ONLY_PIPE_CLK_REG, usb2_u2_only_pipe_clk`.
    pub usb2_u2_only_pipe_clk: RW<u32>,
    _reserved_1368: [u8; 0x018],
    /// 0x1380 - `pcie0_aux_clk`.
    pub pcie0_aux: RW<u32>,
    /// 0x1384 - `pcie0_axi_s_clk`.
    pub pcie0_axi_s: RW<u32>,
    _reserved_1388: [u8; 0x004],
    /// 0x138c - `reset map`.
    pub register_138c: RW<u32>,
    _reserved_1390: [u8; 0x030],
    /// 0x13c0 - `HSI_COMB0_PHY_CFG_CLK_REG, hsi_comb0_phy_cfg_clk`.
    pub hsi_comb0_phy_cfg_clk: RW<u32>,
    /// 0x13c4 - `HSI_COMB0_PHY_REF_CLK_REG, hsi_comb0_phy_ref_clk`.
    pub hsi_comb0_phy_ref_clk: RW<u32>,
    _reserved_13c8: [u8; 0x004],
    /// 0x13cc - `HSI_SYS_GAR_REG, hsi_ahb_bus_clk, hsi_axi_bus_clk, ...`.
    pub hsi_sys_gar: RW<SingleBusGatingReset>,
    _reserved_13d0: [u8; 0x010],
    /// 0x13e0 - `HSI_AXI_CLK_REG, hsi_axi_clk`.
    pub hsi_axi_clk: RW<u32>,
    _reserved_13e4: [u8; 0x01c],
    /// 0x1400 - `GMAC0_PHY_CLK_REG, gmac0_phy_clk`.
    pub gmac0_phy_clk: RW<u32>,
    _reserved_1404: [u8; 0x008],
    /// 0x140c - `GMAC0_GAR_REG, gmac0_ahb_bus_clk, reset map`.
    pub gmac0_gar: RW<SingleBusGatingReset>,
    _reserved_1410: [u8; 0x0f0],
    /// 0x1500 - `TCON_LCD0_CLK_REG, tcon_lcd0_clk`.
    pub tcon_lcd0_clk: RW<u32>,
    /// 0x1504 - `TCON_LCD0_GAR_REG, reset map, tcon_lcd0_ahb_bus_clk`.
    pub tcon_lcd0_gar: RW<SingleBusGatingReset>,
    _reserved_1508: [u8; 0x03c],
    /// 0x1544 - `LVDS0_GAR_REG, reset map`.
    pub lvds0_gar: RW<u32>,
    _reserved_1548: [u8; 0x038],
    /// 0x1580 - `MIPI_DSI00_CLK_REG, mipi_dsi0_clk`.
    pub mipi_dsi00_clk: RW<u32>,
    /// 0x1584 - `MIPI_DSI00_GAR_REG, mipi_dsi0_ahb_bus_clk, reset map`.
    pub mipi_dsi00_gar: RW<SingleBusGatingReset>,
    _reserved_1588: [u8; 0x038],
    /// 0x15c0 - `COMBOPHY0_CLK_REG, combophy0_clk`.
    pub combophy0_clk: RW<u32>,
    _reserved_15c4: [u8; 0x03c],
    /// 0x1600 - `tcon_tv0_edp_clk`.
    pub tcon_tv0_edp: RW<u32>,
    /// 0x1604 - `TCON_TV0_GAR_REG, reset map, tcon_tv0_ahb_bus_clk`.
    pub tcon_tv0_gar: RW<SingleBusGatingReset>,
    _reserved_1608: [u8; 0x044],
    /// 0x164c - `edp_ahb_bus_clk, reset map`.
    pub edp_ahb_bus: RW<u32>,
    _reserved_1650: [u8; 0x074],
    /// 0x16c4 - `VO0_REG_GAR_REG, reset map, vo0_ahb_bus_clk`.
    pub vo0_reg_gar: RW<SingleBusGatingReset>,
    _reserved_16c8: [u8; 0x004],
    /// 0x16cc - `VO1_REG_GAR_REG, reset map, vo1_ahb_bus_clk`.
    pub vo1_reg_gar: RW<SingleBusGatingReset>,
    _reserved_16d0: [u8; 0x014],
    /// 0x16e4 - `VIDEO_OUT0_GAR_REG, reset map`.
    pub video_out0_gar: RW<u32>,
    _reserved_16e8: [u8; 0x018],
    /// 0x1700 - `LEDC_CLK_REG, ledc_clk`.
    pub ledc_clk: RW<u32>,
    /// 0x1704 - `LEDC_GAR_REG, ledc_apb_bus_clk, reset map`.
    pub ledc_gar: RW<SingleBusGatingReset>,
    _reserved_1708: [u8; 0x0f8],
    /// 0x1800 - `CSI_MASTER0_CLK_REG, csi_master0_clk`.
    pub csi_master0_clk: RW<u32>,
    /// 0x1804 - `CSI_MASTER1_CLK_REG, csi_master1_clk`.
    pub csi_master1_clk: RW<u32>,
    /// 0x1808 - `CSI_MASTER2_CLK_REG, csi_master2_clk`.
    pub csi_master2_clk: RW<u32>,
    _reserved_180c: [u8; 0x034],
    /// 0x1840 - `CSI_CLK_REG, csi_clk`.
    pub csi_clk: RW<u32>,
    _reserved_1844: [u8; 0x01c],
    /// 0x1860 - `ISP_CLK_REG, isp_clk`.
    pub isp_clk: RW<u32>,
    _reserved_1864: [u8; 0x020],
    /// 0x1884 - `VIDEO_IN_GAR_REG, reset map, video_in_ahb_bus_clk`.
    pub video_in_gar: RW<SingleBusGatingReset>,
    _reserved_1888: [u8; 0x080],
    /// 0x1908 - `PERI0PLL_GATE_EN_REG, SUN65IW1_PLL_PERI0_EN_REG`.
    pub pll_peri0_en: RW<u32>,
    /// 0x190c - `PERI1PLL_GATE_EN_REG, SUN65IW1_PLL_PERI1_EN_REG`.
    pub pll_peri1_en: RW<u32>,
    /// 0x1910 - `SUN65IW1_PLL_VIDEO_EN_REG, VIDEOPLL_GATE_EN_REG`.
    pub pll_video_en: RW<u32>,
    /// 0x1914 - `GPUPLL_GATE_EN_REG, SUN65IW1_PLL_GPU_EN_REG`.
    pub pll_gpu_en: RW<u32>,
    /// 0x1918 - `SUN65IW1_PLL_VE_EN_REG, VEPLL_GATE_EN_REG`.
    pub pll_ve_en: RW<u32>,
    /// 0x191c - `AUDIOPLL_GATE_EN_REG, SUN65IW1_PLL_AUDIO_EN_REG`.
    pub pll_audio_en: RW<u32>,
    _reserved_1920: [u8; 0x068],
    /// 0x1988 - `PERI0PLL_GATE_STAT_REG`.
    pub peri0pll_gate_stat: RO<u32>,
    /// 0x198c - `PERI1PLL_GATE_STAT_REG`.
    pub peri1pll_gate_stat: RO<u32>,
    /// 0x1990 - `VIDEOPLL_GATE_STAT_REG`.
    pub videopll_gate_stat: RO<u32>,
    /// 0x1994 - `GPUPLL_GATE_STAT_REG`.
    pub gpupll_gate_stat: RO<u32>,
    /// 0x1998 - `VEPLL_GATE_STAT_REG`.
    pub vepll_gate_stat: RO<u32>,
    /// 0x199c - `AUDIOPLL_GATE_STAT_REG`.
    pub audiopll_gate_stat: RO<u32>,
    _reserved_19a0: [u8; 0x060],
    /// 0x1a00 - `RES24M_GATE_EN_REG`.
    pub res24m_gate_en: RW<u32>,
    _reserved_1a04: [u8; 0x00c],
    /// 0x1a10 - `PLL_FO0_EN_REG`.
    pub pll_fo0_en: RW<u32>,
    _reserved_1a14: [u8; 0x00c],
    /// 0x1a20 - `PLL_OPG_BYPASS_REG, pll_output_bypass_bus_clk`.
    pub pll_opg_bypass: RW<u32>,
    _reserved_1a24: [u8; 0x0dc],
    /// 0x1b00 - `CM_VIDEO_IN_CFG_REG`.
    pub cm_video_in_cfg: RW<u32>,
    _reserved_1b04: [u8; 0x00c],
    /// 0x1b10 - `CM_VE_CFG_REG`.
    pub cm_ve_cfg: RW<u32>,
    _reserved_1b14: [u8; 0x014],
    /// 0x1b28 - `CM_HSI_CFG_REG`.
    pub cm_hsi_cfg: RW<u32>,
    _reserved_1b2c: [u8; 0x008],
    /// 0x1b34 - `CM_VIDEO_OUT0_CFG_REG`.
    pub cm_video_out0_cfg: RW<u32>,
    _reserved_1b38: [u8; 0x0c8],
    /// 0x1c00 - `AXI_MON_GAR_REG, ce_sys_aximon_bus_clk, gmac_aximon_bus_clk, ...`.
    pub axi_mon_gar: RW<SingleBusGatingReset>,
    /// 0x1c04 - `AHB_MON_GAR_REG, cpu_sys_aximon_bus_clk, dcu_aximon_bus_clk, ...`.
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
    /// 0x1f30 - `CCU_FAN_GATE_REG, clk12m_bus_clk, clk16m_bus_clk, ...`.
    pub fan_gate: RW<u32>,
    /// 0x1f34 - `CLK27M_FAN_REG, clk27m_clk`.
    pub clk27m_fan: RW<u32>,
    /// 0x1f38 - `CLK_FAN_REG, pclk_clk`.
    pub clk_fan: RW<u32>,
    /// 0x1f3c - `CCU_FAN_REG, fanout0_clk, fanout1_clk, ...`.
    pub fan: RW<u32>,
    _reserved_1f40: [u8; 0x010],
    /// 0x1f50 - `CLK_DBG_REG`.
    pub clk_dbg: RW<u32>,
    _reserved_1f54: [u8; 0x09c],
    /// 0x1ff0 - `CCU_VERSION_REG`.
    pub version: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_ctrl), 0x0a0);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat0_ctrl), 0x0a8);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat1_ctrl), 0x0ac);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_bias), 0x0b0);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_ctrl), 0x0c0);
        assert_eq!(offset_of!(RegisterBlock, pll_periph1_pattern0), 0x0c8);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_pat1_ctrl), 0x0cc);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_bias), 0x0d0);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_ctrl), 0x0e0);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_pat0_ctrl), 0x0e8);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_pat1_ctrl), 0x0ec);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_bias), 0x0f0);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_ctrl), 0x120);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat0_ctrl), 0x128);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat1_ctrl), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_bias), 0x130);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_ctrl), 0x140);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat0_ctrl), 0x148);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat1_ctrl), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_bias), 0x150);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_ctrl), 0x160);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_pat0_ctrl), 0x168);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_pat1_ctrl), 0x16c);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_bias), 0x170);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_ctrl), 0x220);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_pat0_ctrl), 0x228);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_pat1_ctrl), 0x22c);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_bias), 0x230);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_ctrl), 0x260);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_pat0_ctrl), 0x268);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_pat1_ctrl), 0x26c);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_bias), 0x270);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_ctrl), 0x280);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_pat0_ctrl), 0x288);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_pat1_ctrl), 0x28c);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_bias), 0x290);
        assert_eq!(offset_of!(RegisterBlock, ahb_clk), 0x500);
        assert_eq!(offset_of!(RegisterBlock, apb0_clk), 0x510);
        assert_eq!(offset_of!(RegisterBlock, apb1_clk), 0x518);
        assert_eq!(offset_of!(RegisterBlock, apb_uart_clk), 0x538);
        assert_eq!(offset_of!(RegisterBlock, cpu_sys_dp_clk), 0x548);
        assert_eq!(offset_of!(RegisterBlock, cpux_gic_clk), 0x560);
        assert_eq!(offset_of!(RegisterBlock, nsi_clk), 0x580);
        assert_eq!(offset_of!(RegisterBlock, nsi_gar), 0x584);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk), 0x588);
        assert_eq!(offset_of!(RegisterBlock, iommu_gar), 0x58c);
        assert_eq!(offset_of!(RegisterBlock, ahb_mat_clk_gate_en), 0x5c0);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gate_en), 0x5e0);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk_gate_en), 0x5e4);
        assert_eq!(offset_of!(RegisterBlock, dma0_gar), 0x704);
        assert_eq!(offset_of!(RegisterBlock, spinlock_gar), 0x724);
        assert_eq!(offset_of!(RegisterBlock, msgbox_cpux_gar), 0x744);
        assert_eq!(offset_of!(RegisterBlock, msgbox_cpus_gar), 0x74c);
        assert_eq!(offset_of!(RegisterBlock, pwm0_gar), 0x784);
        assert_eq!(offset_of!(RegisterBlock, dcu_gar), 0x7a4);
        assert_eq!(offset_of!(RegisterBlock, dap_gar), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, timer0_0_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, timer0_1_clk), 0x804);
        assert_eq!(offset_of!(RegisterBlock, timer0_2_clk), 0x808);
        assert_eq!(offset_of!(RegisterBlock, timer0_3_clk), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, timer0_gar), 0x850);
        assert_eq!(offset_of!(RegisterBlock, de0_clk), 0xa00);
        assert_eq!(offset_of!(RegisterBlock, de0_gar), 0xa04);
        assert_eq!(offset_of!(RegisterBlock, g2d_clk), 0xa40);
        assert_eq!(offset_of!(RegisterBlock, g2d_gar), 0xa44);
        assert_eq!(offset_of!(RegisterBlock, eink_panel_clk), 0xa64);
        assert_eq!(offset_of!(RegisterBlock, eink_gar), 0xa6c);
        assert_eq!(offset_of!(RegisterBlock, ve0_clk), 0xa80);
        assert_eq!(offset_of!(RegisterBlock, ve0_gar), 0xa8c);
        assert_eq!(offset_of!(RegisterBlock, ce_sys_clk), 0xac0);
        assert_eq!(offset_of!(RegisterBlock, ce_sys_gar), 0xac4);
        assert_eq!(offset_of!(RegisterBlock, gpu_clk), 0xb20);
        assert_eq!(offset_of!(RegisterBlock, gpu_gar), 0xb24);
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
        assert_eq!(offset_of!(RegisterBlock, uart7_gar), 0xe1c);
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
        assert_eq!(offset_of!(RegisterBlock, gpadc0_clk), 0xfc0);
        assert_eq!(offset_of!(RegisterBlock, gpadc0_gar), 0xfc4);
        assert_eq!(offset_of!(RegisterBlock, tsensor_gar), 0xfe4);
        assert_eq!(offset_of!(RegisterBlock, ir_rx0_clk), 0x1000);
        assert_eq!(offset_of!(RegisterBlock, ir_rx0_gar), 0x1004);
        assert_eq!(offset_of!(RegisterBlock, ir_tx_clk), 0x1008);
        assert_eq!(offset_of!(RegisterBlock, ir_tx_gar), 0x100c);
        assert_eq!(offset_of!(RegisterBlock, i2s0_clk), 0x1200);
        assert_eq!(offset_of!(RegisterBlock, i2s0_gar), 0x120c);
        assert_eq!(offset_of!(RegisterBlock, i2s1_clk), 0x1210);
        assert_eq!(offset_of!(RegisterBlock, i2s1_gar), 0x121c);
        assert_eq!(offset_of!(RegisterBlock, i2s2_clk), 0x1220);
        assert_eq!(offset_of!(RegisterBlock, i2s2_gar), 0x122c);
        assert_eq!(offset_of!(RegisterBlock, i2s3_clk), 0x1230);
        assert_eq!(offset_of!(RegisterBlock, i2s3_gar), 0x123c);
        assert_eq!(offset_of!(RegisterBlock, owa0_tx_clk), 0x1280);
        assert_eq!(offset_of!(RegisterBlock, owa0_rx_clk), 0x1284);
        assert_eq!(offset_of!(RegisterBlock, owa0_gar), 0x128c);
        assert_eq!(offset_of!(RegisterBlock, dmic_clk), 0x12c0);
        assert_eq!(offset_of!(RegisterBlock, dmic_gar), 0x12cc);
        assert_eq!(offset_of!(RegisterBlock, audiocodec0_dac_clk), 0x12e0);
        assert_eq!(offset_of!(RegisterBlock, audiocodec0_adc_clk), 0x12e8);
        assert_eq!(offset_of!(RegisterBlock, audiocodec0_gar), 0x12ec);
        assert_eq!(offset_of!(RegisterBlock, audiocodec1_dac_clk), 0x12f0);
        assert_eq!(offset_of!(RegisterBlock, audiocodec1_gar), 0x12fc);
        assert_eq!(offset_of!(RegisterBlock, usb0_clk), 0x1300);
        assert_eq!(offset_of!(RegisterBlock, usb0_gar), 0x1304);
        assert_eq!(offset_of!(RegisterBlock, usb1_clk), 0x1308);
        assert_eq!(offset_of!(RegisterBlock, usb1_gar), 0x130c);
        assert_eq!(offset_of!(RegisterBlock, usb2p0_sys_phy_ref_clk), 0x1340);
        assert_eq!(offset_of!(RegisterBlock, usb2p0_sys_gar), 0x1344);
        assert_eq!(offset_of!(RegisterBlock, usb2_u2_phy_ref_clk), 0x1348);
        assert_eq!(offset_of!(RegisterBlock, usb2_suspend_clk), 0x1350);
        assert_eq!(offset_of!(RegisterBlock, usb2_mf_clk), 0x1354);
        assert_eq!(offset_of!(RegisterBlock, usb2_gar), 0x135c);
        assert_eq!(offset_of!(RegisterBlock, usb2_u3_only_utmi_clk), 0x1360);
        assert_eq!(offset_of!(RegisterBlock, usb2_u2_only_pipe_clk), 0x1364);
        assert_eq!(offset_of!(RegisterBlock, pcie0_aux), 0x1380);
        assert_eq!(offset_of!(RegisterBlock, pcie0_axi_s), 0x1384);
        assert_eq!(offset_of!(RegisterBlock, register_138c), 0x138c);
        assert_eq!(offset_of!(RegisterBlock, hsi_comb0_phy_cfg_clk), 0x13c0);
        assert_eq!(offset_of!(RegisterBlock, hsi_comb0_phy_ref_clk), 0x13c4);
        assert_eq!(offset_of!(RegisterBlock, hsi_sys_gar), 0x13cc);
        assert_eq!(offset_of!(RegisterBlock, hsi_axi_clk), 0x13e0);
        assert_eq!(offset_of!(RegisterBlock, gmac0_phy_clk), 0x1400);
        assert_eq!(offset_of!(RegisterBlock, gmac0_gar), 0x140c);
        assert_eq!(offset_of!(RegisterBlock, tcon_lcd0_clk), 0x1500);
        assert_eq!(offset_of!(RegisterBlock, tcon_lcd0_gar), 0x1504);
        assert_eq!(offset_of!(RegisterBlock, lvds0_gar), 0x1544);
        assert_eq!(offset_of!(RegisterBlock, mipi_dsi00_clk), 0x1580);
        assert_eq!(offset_of!(RegisterBlock, mipi_dsi00_gar), 0x1584);
        assert_eq!(offset_of!(RegisterBlock, combophy0_clk), 0x15c0);
        assert_eq!(offset_of!(RegisterBlock, tcon_tv0_edp), 0x1600);
        assert_eq!(offset_of!(RegisterBlock, tcon_tv0_gar), 0x1604);
        assert_eq!(offset_of!(RegisterBlock, edp_ahb_bus), 0x164c);
        assert_eq!(offset_of!(RegisterBlock, vo0_reg_gar), 0x16c4);
        assert_eq!(offset_of!(RegisterBlock, vo1_reg_gar), 0x16cc);
        assert_eq!(offset_of!(RegisterBlock, video_out0_gar), 0x16e4);
        assert_eq!(offset_of!(RegisterBlock, ledc_clk), 0x1700);
        assert_eq!(offset_of!(RegisterBlock, ledc_gar), 0x1704);
        assert_eq!(offset_of!(RegisterBlock, csi_master0_clk), 0x1800);
        assert_eq!(offset_of!(RegisterBlock, csi_master1_clk), 0x1804);
        assert_eq!(offset_of!(RegisterBlock, csi_master2_clk), 0x1808);
        assert_eq!(offset_of!(RegisterBlock, csi_clk), 0x1840);
        assert_eq!(offset_of!(RegisterBlock, isp_clk), 0x1860);
        assert_eq!(offset_of!(RegisterBlock, video_in_gar), 0x1884);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_en), 0x1908);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_en), 0x190c);
        assert_eq!(offset_of!(RegisterBlock, pll_video_en), 0x1910);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_en), 0x1914);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_en), 0x1918);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_en), 0x191c);
        assert_eq!(offset_of!(RegisterBlock, peri0pll_gate_stat), 0x1988);
        assert_eq!(offset_of!(RegisterBlock, peri1pll_gate_stat), 0x198c);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_stat), 0x1990);
        assert_eq!(offset_of!(RegisterBlock, gpupll_gate_stat), 0x1994);
        assert_eq!(offset_of!(RegisterBlock, vepll_gate_stat), 0x1998);
        assert_eq!(offset_of!(RegisterBlock, audiopll_gate_stat), 0x199c);
        assert_eq!(offset_of!(RegisterBlock, res24m_gate_en), 0x1a00);
        assert_eq!(offset_of!(RegisterBlock, pll_fo0_en), 0x1a10);
        assert_eq!(offset_of!(RegisterBlock, pll_opg_bypass), 0x1a20);
        assert_eq!(offset_of!(RegisterBlock, cm_video_in_cfg), 0x1b00);
        assert_eq!(offset_of!(RegisterBlock, cm_ve_cfg), 0x1b10);
        assert_eq!(offset_of!(RegisterBlock, cm_hsi_cfg), 0x1b28);
        assert_eq!(offset_of!(RegisterBlock, cm_video_out0_cfg), 0x1b34);
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
        assert_eq!(offset_of!(RegisterBlock, version), 0x1ff0);
        assert_eq!(size_of::<RegisterBlock>(), 0x1ff4);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
