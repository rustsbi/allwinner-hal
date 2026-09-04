//! T536/MR536 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun55iw6` platform.

use super::SingleBusGatingReset;
use volatile_register::{RO, RW};

/// T536/MR536 main CCU register block.
#[doc(alias = "sun55iw6")]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0000: [u8; 0x020],
    /// 0x0020 - `PLL_DDR_CTRL_REG, SUN55IW6_PLL_DDR_CTRL_REG, pll_ddr_clk`.
    pub pll_ddr_ctrl: RW<u32>,
    _reserved_0024: [u8; 0x004],
    /// 0x0028 - `PLL_DDR_PAT0_CTRL_REG`.
    pub pll_ddr_pat0_ctrl: RW<u32>,
    /// 0x002c - `PLL_DDR_PAT1_CTRL_REG`.
    pub pll_ddr_pat1_ctrl: RW<u32>,
    /// 0x0030 - `PLL_DDR_BIAS_REG`.
    pub pll_ddr_bias: RW<u32>,
    _reserved_0034: [u8; 0x06c],
    /// 0x00a0 - `PLL_PERI0_CTRL_REG, SUN55IW6_PLL_PERI0_CTRL_REG, pll_peri0_2x_clk, ...`.
    pub pll_peri0_ctrl: RW<u32>,
    _reserved_00a4: [u8; 0x004],
    /// 0x00a8 - `PLL_PERI0_PAT0_CTRL_REG`.
    pub pll_peri0_pat0_ctrl: RW<u32>,
    /// 0x00ac - `PLL_PERI0_PAT1_CTRL_REG`.
    pub pll_peri0_pat1_ctrl: RW<u32>,
    /// 0x00b0 - `PLL_PERI0_BIAS_REG`.
    pub pll_peri0_bias: RW<u32>,
    _reserved_00b4: [u8; 0x00c],
    /// 0x00c0 - `PLL_PERI1_CTRL_REG, SUN55IW6_PLL_PERI1_CTRL_REG, pll_peri1_2x_clk, ...`.
    pub pll_peri1_ctrl: RW<u32>,
    _reserved_00c4: [u8; 0x004],
    /// 0x00c8 - `PLL_PERI1_PAT0_CTRL_REG`.
    pub pll_peri1_pat0_ctrl: RW<u32>,
    /// 0x00cc - `PLL_PERI1_PAT1_CTRL_REG`.
    pub pll_peri1_pat1_ctrl: RW<u32>,
    /// 0x00d0 - `PLL_PERI1_BIAS_REG`.
    pub pll_peri1_bias: RW<u32>,
    _reserved_00d4: [u8; 0x04c],
    /// 0x0120 - `PLL_VIDEO0_CTRL_REG, SUN55IW6_PLL_VIDEO0_CTRL_REG, pll_video0_1x_clk, ...`.
    pub pll_video0_ctrl: RW<u32>,
    _reserved_0124: [u8; 0x004],
    /// 0x0128 - `PLL_VIDEO0_PAT0_CTRL_REG`.
    pub pll_video0_pat0_ctrl: RW<u32>,
    /// 0x012c - `PLL_VIDEO0_PAT1_CTRL_REG`.
    pub pll_video0_pat1_ctrl: RW<u32>,
    /// 0x0130 - `PLL_VIDEO0_BIAS_REG`.
    pub pll_video0_bias: RW<u32>,
    _reserved_0134: [u8; 0x00c],
    /// 0x0140 - `PLL_VIDEO1_CTRL_REG, SUN55IW6_PLL_VIDEO1_CTRL_REG, pll_video1_3x_clk, ...`.
    pub pll_video1_ctrl: RW<u32>,
    _reserved_0144: [u8; 0x004],
    /// 0x0148 - `PLL_VIDEO1_PAT0_CTRL_REG`.
    pub pll_video1_pat0_ctrl: RW<u32>,
    /// 0x014c - `PLL_VIDEO1_PAT1_CTRL_REG`.
    pub pll_video1_pat1_ctrl: RW<u32>,
    /// 0x0150 - `PLL_VIDEO1_BIAS_REG`.
    pub pll_video1_bias: RW<u32>,
    _reserved_0154: [u8; 0x0cc],
    /// 0x0220 - `PLL_VE_CTRL_REG, SUN55IW6_PLL_VE_CTRL_REG, pll_ve_clk`.
    pub pll_ve_ctrl: RW<u32>,
    _reserved_0224: [u8; 0x004],
    /// 0x0228 - `PLL_VE_PAT0_CTRL_REG`.
    pub pll_ve_pat0_ctrl: RW<u32>,
    /// 0x022c - `PLL_VE_PAT1_CTRL_REG`.
    pub pll_ve_pat1_ctrl: RW<u32>,
    /// 0x0230 - `PLL_VE_BIAS_REG`.
    pub pll_ve_bias: RW<u32>,
    _reserved_0234: [u8; 0x02c],
    /// 0x0260 - `PLL_AUDIO0_CTRL_REG, SUN55IW6_PLL_AUDIO0_CTRL_REG, pll_audio0_4x_clk`.
    pub pll_audio0_ctrl: RW<u32>,
    _reserved_0264: [u8; 0x004],
    /// 0x0268 - `PLL_AUDIO0_PAT0_CTRL_REG, pll_audio0_sdm_clk`.
    pub pll_audio0_pat0_ctrl: RW<u32>,
    /// 0x026c - `PLL_AUDIO0_PAT1_CTRL_REG`.
    pub pll_audio0_pat1_ctrl: RW<u32>,
    /// 0x0270 - `PLL_AUDIO0_BIAS_REG`.
    pub pll_audio0_bias: RW<u32>,
    _reserved_0274: [u8; 0x00c],
    /// 0x0280 - `PLL_AUDIO1_CTRL_REG, SUN55IW6_PLL_AUDIO1_CTRL_REG, pll_audio1_4x_clk`.
    pub pll_audio1_ctrl: RW<u32>,
    _reserved_0284: [u8; 0x004],
    /// 0x0288 - `PLL_AUDIO1_PAT0_CTRL_REG, pll_audio1_sdm_clk`.
    pub pll_audio1_pat0_ctrl: RW<u32>,
    /// 0x028c - `PLL_AUDIO1_PAT1_CTRL_REG`.
    pub pll_audio1_pat1_ctrl: RW<u32>,
    /// 0x0290 - `PLL_AUDIO1_BIAS_REG`.
    pub pll_audio1_bias: RW<u32>,
    _reserved_0294: [u8; 0x00c],
    /// 0x02a0 - `PLL_NPU_CTRL_REG, SUN55IW6_PLL_NPU_CTRL_REG, pll_npu_4x_clk`.
    pub pll_npu_ctrl: RW<u32>,
    _reserved_02a4: [u8; 0x004],
    /// 0x02a8 - `PLL_NPU_PAT0_CTRL_REG`.
    pub pll_npu_pat0_ctrl: RW<u32>,
    /// 0x02ac - `PLL_NPU_PAT1_CTRL_REG`.
    pub pll_npu_pat1_ctrl: RW<u32>,
    /// 0x02b0 - `PLL_NPU_BIAS_REG`.
    pub pll_npu_bias: RW<u32>,
    _reserved_02b4: [u8; 0x24c],
    /// 0x0500 - `AHB_CLK_REG`.
    pub ahb_clk: RW<u32>,
    _reserved_0504: [u8; 0x00c],
    /// 0x0510 - `APB0_CLK_REG`.
    pub apb0_clk: RW<u32>,
    _reserved_0514: [u8; 0x004],
    /// 0x0518 - `APB1_CLK_REG, apb1_clk`.
    pub apb1_clk: RW<u32>,
    _reserved_051c: [u8; 0x01c],
    /// 0x0538 - `APB_UART_CLK_REG, apb_uart_clk`.
    pub apb_uart_clk: RW<u32>,
    _reserved_053c: [u8; 0x004],
    /// 0x0540 - `TRACE_CLK_REG, trace_clk`.
    pub trace_clk: RW<u32>,
    _reserved_0544: [u8; 0x01c],
    /// 0x0560 - `GIC_CLK_REG, gic_clk`.
    pub gic_clk: RW<u32>,
    _reserved_0564: [u8; 0x010],
    /// 0x0574 - `ITS0_BGR_REG, its0_aclk, its0_hclk, ...`.
    pub its0_bgr: RW<SingleBusGatingReset>,
    _reserved_0578: [u8; 0x008],
    /// 0x0580 - `NSI_CLK_REG, nsi_clk, reset map`.
    pub nsi_clk: RW<u32>,
    /// 0x0584 - `NSI_BGR_REG, nsi_cfg_clk, reset map`.
    pub nsi_bgr: RW<SingleBusGatingReset>,
    /// 0x0588 - `MBUS_CLK_REG, mbus_clk`.
    pub mbus_clk: RW<u32>,
    /// 0x058c - `IOMMU_BGR_REG, iommu_clk`.
    pub iommu_bgr: RW<u32>,
    _reserved_0590: [u8; 0x030],
    /// 0x05c0 - `AHB_GATE_EN_REG, vid_in_ahb_gate_clk, vid_out_ahb_gate_clk`.
    pub ahb_gate_en: RW<u32>,
    _reserved_05c4: [u8; 0x01c],
    /// 0x05e0 - `MBUS_GATE_EN_REG, ce_mbus_gate_clk, csi_mbus_gate_clk, ...`.
    pub mbus_gate_en: RW<u32>,
    /// 0x05e4 - `MBUS_MAT_CLK_GATING_REG, vid_in_mbus_gate_clk, vo_sys_mbus_gate_clk`.
    pub mbus_mat_clk_gating: RW<u32>,
    _reserved_05e8: [u8; 0x11c],
    /// 0x0704 - `DMA0_BGR_REG, dma0_clk, reset map`.
    pub dma0_bgr: RW<SingleBusGatingReset>,
    _reserved_0708: [u8; 0x004],
    /// 0x070c - `DMA1_BGR_REG, dma1_clk, reset map`.
    pub dma1_bgr: RW<SingleBusGatingReset>,
    _reserved_0710: [u8; 0x014],
    /// 0x0724 - `SPINLOCK_BGR_REG, reset map, spinlock_clk`.
    pub spinlock_bgr: RW<SingleBusGatingReset>,
    _reserved_0728: [u8; 0x01c],
    /// 0x0744 - `MSGBOX0_BGR_REG, msgbox0_clk, reset map`.
    pub msgbox0_bgr: RW<SingleBusGatingReset>,
    _reserved_0748: [u8; 0x004],
    /// 0x074c - `MSGBOX_CORE0_BGR_REG, msgbox_core0_clk, reset map`.
    pub msgbox_core0_bgr: RW<SingleBusGatingReset>,
    _reserved_0750: [u8; 0x004],
    /// 0x0754 - `MSGBOX_CORE1_BGR_REG, msgbox_core1_clk, reset map`.
    pub msgbox_core1_bgr: RW<SingleBusGatingReset>,
    _reserved_0758: [u8; 0x004],
    /// 0x075c - `MSGBOX_CORE2_BGR_REG, msgbox_core2_clk, reset map`.
    pub msgbox_core2_bgr: RW<SingleBusGatingReset>,
    _reserved_0760: [u8; 0x004],
    /// 0x0764 - `MSGBOX_CORE3_BGR_REG, msgbox_core3_clk, reset map`.
    pub msgbox_core3_bgr: RW<SingleBusGatingReset>,
    _reserved_0768: [u8; 0x004],
    /// 0x076c - `MSGBOX_RV_BGR_REG, msgbox_rv_clk, reset map`.
    pub msgbox_rv_bgr: RW<SingleBusGatingReset>,
    _reserved_0770: [u8; 0x014],
    /// 0x0784 - `PWM0_BGR_REG, pwm0_clk, reset map`.
    pub pwm0_bgr: RW<SingleBusGatingReset>,
    _reserved_0788: [u8; 0x004],
    /// 0x078c - `PWM1_BGR_REG, pwm1_clk, reset map`.
    pub pwm1_bgr: RW<SingleBusGatingReset>,
    _reserved_0790: [u8; 0x004],
    /// 0x0794 - `PWM2_BGR_REG, pwm2_clk, reset map`.
    pub pwm2_bgr: RW<SingleBusGatingReset>,
    _reserved_0798: [u8; 0x00c],
    /// 0x07a4 - `DBGSYS_BGR_REG, dbgsys_clk, reset map`.
    pub dbgsys_bgr: RW<SingleBusGatingReset>,
    _reserved_07a8: [u8; 0x004],
    /// 0x07ac - `SYSDAP_BGR_REG, reset map, sysdap_clk`.
    pub sysdap_bgr: RW<SingleBusGatingReset>,
    _reserved_07b0: [u8; 0x050],
    /// 0x0800 - `TIMER0_CLK_REG, timer0_clk`.
    pub timer0_clk: RW<u32>,
    /// 0x0804 - `TIMER1_CLK_REG, timer1_clk`.
    pub timer1_clk: RW<u32>,
    /// 0x0808 - `TIMER2_CLK_REG, timer2_clk`.
    pub timer2_clk: RW<u32>,
    /// 0x080c - `TIMER3_CLK_REG, timer3_clk`.
    pub timer3_clk: RW<u32>,
    /// 0x0810 - `TIMER4_CLK_REG, timer4_clk`.
    pub timer4_clk: RW<u32>,
    /// 0x0814 - `TIMER5_CLK_REG, timer5_clk`.
    pub timer5_clk: RW<u32>,
    /// 0x0818 - `TIMER6_CLK_REG, timer6_clk`.
    pub timer6_clk: RW<u32>,
    /// 0x081c - `TIMER7_CLK_REG, timer7_clk`.
    pub timer7_clk: RW<u32>,
    _reserved_0820: [u8; 0x030],
    /// 0x0850 - `TIMER_BGR_REG, reset map, timer_bus_clk`.
    pub timer_bgr: RW<SingleBusGatingReset>,
    _reserved_0854: [u8; 0x00c],
    /// 0x0860 - `TIMER0_RV_CLK_REG, timer0_rv_clk`.
    pub timer0_rv_clk: RW<u32>,
    /// 0x0864 - `TIMER1_RV_CLK_REG, timer1_rv_clk`.
    pub timer1_rv_clk: RW<u32>,
    /// 0x0868 - `TIMER2_RV_CLK_REG, timer2_rv_clk`.
    pub timer2_rv_clk: RW<u32>,
    /// 0x086c - `TIMER3_RV_CLK_REG, timer3_rv_clk`.
    pub timer3_rv_clk: RW<u32>,
    /// 0x0870 - `TIMER_RV_BGR_REG, reset map, timer_rv_bus_clk`.
    pub timer_rv_bgr: RW<SingleBusGatingReset>,
    _reserved_0874: [u8; 0x18c],
    /// 0x0a00 - `DE0_CLK_REG, de_clk`.
    pub de0_clk: RW<u32>,
    /// 0x0a04 - `DE0_BGR_REG, de0_clk, reset map`.
    pub de0_bgr: RW<SingleBusGatingReset>,
    _reserved_0a08: [u8; 0x038],
    /// 0x0a40 - `G2D_CLK_REG, g2d_clk`.
    pub g2d_clk: RW<u32>,
    /// 0x0a44 - `G2D_BGR_REG, g2d_bus_clk, reset map`.
    pub g2d_bgr: RW<SingleBusGatingReset>,
    _reserved_0a48: [u8; 0x02c],
    /// 0x0a74 - `DE_SYS_BGR_REG, reset map`.
    pub de_sys_bgr: RW<u32>,
    _reserved_0a78: [u8; 0x008],
    /// 0x0a80 - `VE_CLK_REG, ve_clk`.
    pub ve_clk: RW<u32>,
    _reserved_0a84: [u8; 0x008],
    /// 0x0a8c - `VE_BGR_REG, reset map, ve_bus_clk`.
    pub ve_bgr: RW<SingleBusGatingReset>,
    _reserved_0a90: [u8; 0x030],
    /// 0x0ac0 - `CE_CLK_REG, ce_clk`.
    pub ce_clk: RW<u32>,
    /// 0x0ac4 - `CE_BGR_REG, ce_bus_clk, ce_sys_clk, ...`.
    pub ce_bgr: RW<SingleBusGatingReset>,
    _reserved_0ac8: [u8; 0x038],
    /// 0x0b00 - `NPU_CLK_REG, npu_clk`.
    pub npu_clk: RW<u32>,
    /// 0x0b04 - `NPU_BGR_REG, npu_bus_clk, npu_tzma_clk, ...`.
    pub npu_bgr: RW<SingleBusGatingReset>,
    _reserved_0b08: [u8; 0x078],
    /// 0x0b80 - `RV_CORE_CLK_REG, e907_axi_clk, rv_core_clk`.
    pub rv_core_clk: RW<u32>,
    _reserved_0b84: [u8; 0x004],
    /// 0x0b88 - `RV_TS_CLK_REG, rv_ts_clk`.
    pub rv_ts_clk: RW<u32>,
    _reserved_0b8c: [u8; 0x008],
    /// 0x0b94 - `RV_SYS_BGR_REG, reset map`.
    pub rv_sys_bgr: RW<u32>,
    _reserved_0b98: [u8; 0x004],
    /// 0x0b9c - `RV_CFG_BGR_REG, reset map, rv_cfg_clk`.
    pub rv_cfg_bgr: RW<SingleBusGatingReset>,
    _reserved_0ba0: [u8; 0x060],
    /// 0x0c00 - `DRAM_CLK_REG, dram_clk`.
    pub dram_clk: RW<u32>,
    _reserved_0c04: [u8; 0x008],
    /// 0x0c0c - `DRAM_BGR_REG, dram_bus_clk, reset map`.
    pub dram_bgr: RW<SingleBusGatingReset>,
    _reserved_0c10: [u8; 0x070],
    /// 0x0c80 - `NAND0_CLK2X_CLK_REG, nand0_clk2x_clk`.
    pub nand0_clk2x_clk: RW<u32>,
    /// 0x0c84 - `NAND0_CLK1_CLK_REG, nand0_clk`.
    pub nand0_clk1_clk: RW<u32>,
    _reserved_0c88: [u8; 0x004],
    /// 0x0c8c - `NAND0_BGR_REG, nand0_bus_clk, reset map`.
    pub nand0_bgr: RW<SingleBusGatingReset>,
    _reserved_0c90: [u8; 0x070],
    /// 0x0d00 - `SMHC0_CLK_REG, smhc0_clk`.
    pub smhc0_clk: RW<u32>,
    _reserved_0d04: [u8; 0x008],
    /// 0x0d0c - `SMHC0_BGR_REG, reset map, smhc0_bus_clk`.
    pub smhc0_bgr: RW<SingleBusGatingReset>,
    /// 0x0d10 - `SMHC1_CLK_REG, smhc1_clk`.
    pub smhc1_clk: RW<u32>,
    _reserved_0d14: [u8; 0x008],
    /// 0x0d1c - `SMHC1_BGR_REG, reset map, smhc1_bus_clk`.
    pub smhc1_bgr: RW<SingleBusGatingReset>,
    /// 0x0d20 - `SMHC2_CLK_REG, smhc2_clk`.
    pub smhc2_clk: RW<u32>,
    _reserved_0d24: [u8; 0x008],
    /// 0x0d2c - `SMHC2_BGR_REG, reset map, smhc2_bus_clk`.
    pub smhc2_bgr: RW<SingleBusGatingReset>,
    _reserved_0d30: [u8; 0x0d0],
    /// 0x0e00 - `UART0_BGR_REG, reset map, uart0_clk`.
    pub uart0_bgr: RW<SingleBusGatingReset>,
    /// 0x0e04 - `UART1_BGR_REG, reset map, uart1_clk`.
    pub uart1_bgr: RW<SingleBusGatingReset>,
    /// 0x0e08 - `UART2_BGR_REG, reset map, uart2_clk`.
    pub uart2_bgr: RW<SingleBusGatingReset>,
    /// 0x0e0c - `UART3_BGR_REG, reset map, uart3_clk`.
    pub uart3_bgr: RW<SingleBusGatingReset>,
    /// 0x0e10 - `UART4_BGR_REG, reset map, uart4_clk`.
    pub uart4_bgr: RW<SingleBusGatingReset>,
    /// 0x0e14 - `UART5_BGR_REG, reset map, uart5_clk`.
    pub uart5_bgr: RW<SingleBusGatingReset>,
    /// 0x0e18 - `UART6_BGR_REG, reset map, uart6_clk`.
    pub uart6_bgr: RW<SingleBusGatingReset>,
    _reserved_0e1c: [u8; 0x004],
    /// 0x0e20 - `UART7_BGR_REG, reset map, uart7_clk`.
    pub uart7_bgr: RW<SingleBusGatingReset>,
    /// 0x0e24 - `UART8_BGR_REG, reset map, uart8_clk`.
    pub uart8_bgr: RW<SingleBusGatingReset>,
    /// 0x0e28 - `UART9_BGR_REG, reset map, uart9_clk`.
    pub uart9_bgr: RW<SingleBusGatingReset>,
    /// 0x0e2c - `UART10_BGR_REG, reset map, uart10_clk`.
    pub uart10_bgr: RW<SingleBusGatingReset>,
    /// 0x0e30 - `UART11_BGR_REG, reset map, uart11_clk`.
    pub uart11_bgr: RW<SingleBusGatingReset>,
    /// 0x0e34 - `UART12_BGR_REG, reset map, uart12_clk`.
    pub uart12_bgr: RW<SingleBusGatingReset>,
    /// 0x0e38 - `UART13_BGR_REG, reset map, uart13_clk`.
    pub uart13_bgr: RW<SingleBusGatingReset>,
    /// 0x0e3c - `UART14_BGR_REG, reset map, uart14_clk`.
    pub uart14_bgr: RW<SingleBusGatingReset>,
    _reserved_0e40: [u8; 0x040],
    /// 0x0e80 - `TWI0_BGR_REG, reset map, twi0_clk`.
    pub twi0_bgr: RW<SingleBusGatingReset>,
    /// 0x0e84 - `TWI1_BGR_REG, reset map, twi1_clk`.
    pub twi1_bgr: RW<SingleBusGatingReset>,
    /// 0x0e88 - `TWI2_BGR_REG, reset map, twi2_clk`.
    pub twi2_bgr: RW<SingleBusGatingReset>,
    /// 0x0e8c - `TWI3_BGR_REG, reset map, twi3_clk`.
    pub twi3_bgr: RW<SingleBusGatingReset>,
    /// 0x0e90 - `TWI4_BGR_REG, reset map, twi4_clk`.
    pub twi4_bgr: RW<SingleBusGatingReset>,
    /// 0x0e94 - `TWI5_BGR_REG, reset map, twi5_clk`.
    pub twi5_bgr: RW<SingleBusGatingReset>,
    /// 0x0e98 - `TWI6_BGR_REG, reset map, twi6_clk`.
    pub twi6_bgr: RW<SingleBusGatingReset>,
    _reserved_0e9c: [u8; 0x064],
    /// 0x0f00 - `SPI0_CLK_REG, spi0_clk`.
    pub spi0_clk: RW<u32>,
    /// 0x0f04 - `SPI0_BGR_REG, reset map, spi0_bus_clk`.
    pub spi0_bgr: RW<SingleBusGatingReset>,
    /// 0x0f08 - `SPI1_CLK_REG, spi1_clk`.
    pub spi1_clk: RW<u32>,
    /// 0x0f0c - `SPI1_BGR_REG, reset map, spi1_bus_clk`.
    pub spi1_bgr: RW<SingleBusGatingReset>,
    /// 0x0f10 - `SPI2_CLK_REG, spi2_clk`.
    pub spi2_clk: RW<u32>,
    /// 0x0f14 - `SPI2_BGR_REG, reset map, spi2_bus_clk`.
    pub spi2_bgr: RW<SingleBusGatingReset>,
    /// 0x0f18 - `SPIF_CLK_REG, spif_clk`.
    pub spif_clk: RW<u32>,
    /// 0x0f1c - `SPIF_BGR_REG, reset map, spif_bus_clk`.
    pub spif_bgr: RW<SingleBusGatingReset>,
    /// 0x0f20 - `SPI3_CLK_REG, spi3_clk`.
    pub spi3_clk: RW<u32>,
    /// 0x0f24 - `SPI3_BGR_REG, reset map, spi3_bus_clk`.
    pub spi3_bgr: RW<SingleBusGatingReset>,
    /// 0x0f28 - `SPI4_CLK_REG, spi4_clk`.
    pub spi4_clk: RW<u32>,
    /// 0x0f2c - `SPI4_BGR_REG, reset map, spi4_bus_clk`.
    pub spi4_bgr: RW<SingleBusGatingReset>,
    _reserved_0f30: [u8; 0x090],
    /// 0x0fc0 - `GPADC0_CLK_REG, gpadc0_clk`.
    pub gpadc0_clk: RW<u32>,
    /// 0x0fc4 - `GPADC0_BGR_REG, gpadc0_bus_clk, reset map`.
    pub gpadc0_bgr: RW<SingleBusGatingReset>,
    /// 0x0fc8 - `GPADC1_CLK_REG, gpadc1_clk`.
    pub gpadc1_clk: RW<u32>,
    /// 0x0fcc - `GPADC1_BGR_REG, gpadc1_bus_clk, reset map`.
    pub gpadc1_bgr: RW<SingleBusGatingReset>,
    /// 0x0fd0 - `GPADC2_CLK_REG, gpadc2_clk`.
    pub gpadc2_clk: RW<u32>,
    /// 0x0fd4 - `GPADC2_BGR_REG, gpadc2_bus_clk, reset map`.
    pub gpadc2_bgr: RW<SingleBusGatingReset>,
    /// 0x0fd8 - `GPADC3_CLK_REG, gpadc3_clk`.
    pub gpadc3_clk: RW<u32>,
    /// 0x0fdc - `GPADC3_BGR_REG, gpadc3_bus_clk, reset map`.
    pub gpadc3_bgr: RW<SingleBusGatingReset>,
    _reserved_0fe0: [u8; 0x004],
    /// 0x0fe4 - `THS_BGR_REG, reset map, ths_clk`.
    pub ths_bgr: RW<SingleBusGatingReset>,
    _reserved_0fe8: [u8; 0x018],
    /// 0x1000 - `IRRX0_CLK_REG, irrx0_clk`.
    pub irrx0_clk: RW<u32>,
    /// 0x1004 - `IRRX0_BGR_REG, irrx0_bus_clk, reset map`.
    pub irrx0_bgr: RW<SingleBusGatingReset>,
    /// 0x1008 - `IRTX_CLK_REG, irtx_clk`.
    pub irtx_clk: RW<u32>,
    /// 0x100c - `IRTX_BGR_REG, irtx_bus_clk, reset map`.
    pub irtx_bgr: RW<SingleBusGatingReset>,
    _reserved_1010: [u8; 0x014],
    /// 0x1024 - `LRADC_BGR_REG, lradc_clk, reset map`.
    pub lradc_bgr: RW<SingleBusGatingReset>,
    _reserved_1028: [u8; 0x008],
    /// 0x1030 - `TPADC_24M_CLK_REG, tpadc_24m_clk`.
    pub tpadc_24m_clk: RW<u32>,
    /// 0x1034 - `TPADC_BGR_REG, reset map, tpadc_clk`.
    pub tpadc_bgr: RW<SingleBusGatingReset>,
    _reserved_1038: [u8; 0x008],
    /// 0x1040 - `LBC_CLK_REG, lbc_clk`.
    pub lbc_clk: RW<u32>,
    _reserved_1044: [u8; 0x004],
    /// 0x1048 - `LBC_NSI_AHB_CLK_REG, lbc_nsi_ahb_clk`.
    pub lbc_nsi_ahb_clk: RW<u32>,
    /// 0x104c - `LBC_BGR_REG, lbc_bus_clk, reset map`.
    pub lbc_bgr: RW<SingleBusGatingReset>,
    _reserved_1050: [u8; 0x0b0],
    /// 0x1100 - `IRRX1_CLK_REG, irrx1_clk`.
    pub irrx1_clk: RW<u32>,
    /// 0x1104 - `IRRX1_BGR_REG, irrx1_bus_clk, reset map`.
    pub irrx1_bgr: RW<SingleBusGatingReset>,
    /// 0x1108 - `IRRX2_CLK_REG, irrx2_clk`.
    pub irrx2_clk: RW<u32>,
    /// 0x110c - `IRRX2_BGR_REG, irrx2_bus_clk, reset map`.
    pub irrx2_bgr: RW<SingleBusGatingReset>,
    /// 0x1110 - `IRRX3_CLK_REG, irrx3_clk`.
    pub irrx3_clk: RW<u32>,
    /// 0x1114 - `IRRX3_BGR_REG, irrx3_bus_clk, reset map`.
    pub irrx3_bgr: RW<SingleBusGatingReset>,
    _reserved_1118: [u8; 0x0e8],
    /// 0x1200 - `I2SPCM0_CLK_REG, i2spcm0_clk`.
    pub i2spcm0_clk: RW<u32>,
    _reserved_1204: [u8; 0x008],
    /// 0x120c - `I2SPCM0_BGR_REG, i2spcm0_bus_clk, reset map`.
    pub i2spcm0_bgr: RW<SingleBusGatingReset>,
    /// 0x1210 - `I2SPCM1_CLK_REG, i2spcm1_clk`.
    pub i2spcm1_clk: RW<u32>,
    _reserved_1214: [u8; 0x008],
    /// 0x121c - `I2SPCM1_BGR_REG, i2spcm1_bus_clk, reset map`.
    pub i2spcm1_bgr: RW<SingleBusGatingReset>,
    /// 0x1220 - `I2SPCM2_CLK_REG, i2spcm2_clk`.
    pub i2spcm2_clk: RW<u32>,
    _reserved_1224: [u8; 0x008],
    /// 0x122c - `I2SPCM2_BGR_REG, i2spcm2_bus_clk, reset map`.
    pub i2spcm2_bgr: RW<SingleBusGatingReset>,
    /// 0x1230 - `I2SPCM3_CLK_REG, i2spcm3_clk`.
    pub i2spcm3_clk: RW<u32>,
    _reserved_1234: [u8; 0x008],
    /// 0x123c - `I2SPCM3_BGR_REG, i2spcm3_bus_clk, reset map`.
    pub i2spcm3_bgr: RW<SingleBusGatingReset>,
    _reserved_1240: [u8; 0x040],
    /// 0x1280 - `OWA_TX_CLK_REG, owa_tx_clk`.
    pub owa_tx_clk: RW<u32>,
    /// 0x1284 - `OWA_RX_CLK_REG, owa_rx_clk`.
    pub owa_rx_clk: RW<u32>,
    _reserved_1288: [u8; 0x004],
    /// 0x128c - `OWA_BGR_REG, owa_clk, reset map`.
    pub owa_bgr: RW<SingleBusGatingReset>,
    _reserved_1290: [u8; 0x030],
    /// 0x12c0 - `DMIC_CLK_REG, dmic_clk`.
    pub dmic_clk: RW<u32>,
    _reserved_12c4: [u8; 0x008],
    /// 0x12cc - `DMIC_BGR_REG, dmic_bus_clk, reset map`.
    pub dmic_bgr: RW<SingleBusGatingReset>,
    _reserved_12d0: [u8; 0x010],
    /// 0x12e0 - `AUDIO_CODEC_DAC_1X_CLK_REG, audio_codec_dac_1x_clk`.
    pub audio_codec_dac_1x_clk: RW<u32>,
    _reserved_12e4: [u8; 0x008],
    /// 0x12ec - `AUDIO_CODEC_BGR_REG, audio_codec_clk, reset map`.
    pub audio_codec_bgr: RW<SingleBusGatingReset>,
    _reserved_12f0: [u8; 0x010],
    /// 0x1300 - `USB0_CLK_REG, reset map, usb_clk`.
    pub usb0_clk: RW<u32>,
    /// 0x1304 - `USB0_BGR_REG, reset map, usb20_0_device_clk, ...`.
    pub usb0_bgr: RW<SingleBusGatingReset>,
    /// 0x1308 - `USB1_CLK_REG, reset map, usb1_clk`.
    pub usb1_clk: RW<u32>,
    /// 0x130c - `USB1_BGR_REG, reset map, usb20_1_host_ehci_clk, ...`.
    pub usb1_bgr: RW<SingleBusGatingReset>,
    /// 0x1310 - `usb0_usb1_24m_clk`.
    pub usb0_usb1_24m: RW<u32>,
    _reserved_1314: [u8; 0x034],
    /// 0x1348 - `USB2_U2_REF_CLK_REG, usb2_ref_clk`.
    pub usb2_u2_ref_clk: RW<u32>,
    _reserved_134c: [u8; 0x004],
    /// 0x1350 - `USB2_SUSPEND_CLK_REG, usb2_suspend_clk`.
    pub usb2_suspend_clk: RW<u32>,
    /// 0x1354 - `USB2_MF_CLK_REG, usb3_ref_clk`.
    pub usb2_mf_clk: RW<u32>,
    _reserved_1358: [u8; 0x004],
    /// 0x135c - `USB2_BGR_REG, reset map, usb30_clk`.
    pub usb2_bgr: RW<SingleBusGatingReset>,
    _reserved_1360: [u8; 0x020],
    /// 0x1380 - `PCIE_AUX_CLK_REG, pcie_ref_aux_clk`.
    pub pcie_aux_clk: RW<u32>,
    /// 0x1384 - `PCIE_SLV_CLK_REG, pcie_slv_clk`.
    pub pcie_slv_clk: RW<u32>,
    _reserved_1388: [u8; 0x004],
    /// 0x138c - `PCIE_BGR_REG, reset map`.
    pub pcie_bgr: RW<u32>,
    _reserved_1390: [u8; 0x030],
    /// 0x13c0 - `SERDES_PHY_CFG_CLK_REG, serdes_phy_cfg_clk`.
    pub serdes_phy_cfg_clk: RW<u32>,
    /// 0x13c4 - `SERDES_PHY_REF_CLK_REG, serdes_phy_ref_clk`.
    pub serdes_phy_ref_clk: RW<u32>,
    _reserved_13c8: [u8; 0x004],
    /// 0x13cc - `SERDES_BGR_REG, reset map, serdes_ahb_clk, ...`.
    pub serdes_bgr: RW<u32>,
    _reserved_13d0: [u8; 0x010],
    /// 0x13e0 - `SERDES_AXI_CLK_REG, serdes_axi_clk`.
    pub serdes_axi_clk: RW<u32>,
    _reserved_13e4: [u8; 0x01c],
    /// 0x1400 - `GMAC0_PHY_CLK_REG, gmac0_phy_clk`.
    pub gmac0_phy_clk: RW<u32>,
    /// 0x1404 - `GMAC0_PTP_CLK_REG, gmac0_ptp_clk`.
    pub gmac0_ptp_clk: RW<u32>,
    _reserved_1408: [u8; 0x004],
    /// 0x140c - `GMAC0_BGR_REG, gmac0_clk, reset map`.
    pub gmac0_bgr: RW<SingleBusGatingReset>,
    /// 0x1410 - `GMAC1_PHY_CLK_REG, gmac1_phy_clk`.
    pub gmac1_phy_clk: RW<u32>,
    /// 0x1414 - `GMAC1_PTP_CLK_REG, gmac1_ptp_clk`.
    pub gmac1_ptp_clk: RW<u32>,
    _reserved_1418: [u8; 0x004],
    /// 0x141c - `GMAC1_BGR_REG, gmac1_clk, reset map`.
    pub gmac1_bgr: RW<SingleBusGatingReset>,
    /// 0x1420 - `gmac_nsi_clk`.
    pub gmac_nsi: RW<u32>,
    _reserved_1424: [u8; 0x0dc],
    /// 0x1500 - `VO0_TCONLCD0_CLK_REG, vo0_tconlcd0_clk`.
    pub vo0_tconlcd0_clk: RW<u32>,
    /// 0x1504 - `VO0_TCONLCD0_BGR_REG, reset map, vo0_tconlcd0_bus_clk`.
    pub vo0_tconlcd0_bgr: RW<SingleBusGatingReset>,
    _reserved_1508: [u8; 0x03c],
    /// 0x1544 - `LVDS0_BGR_REG, reset map`.
    pub lvds0_bgr: RW<u32>,
    _reserved_1548: [u8; 0x038],
    /// 0x1580 - `DSI0_CLK_REG, dsi0_clk`.
    pub dsi0_clk: RW<u32>,
    /// 0x1584 - `DSI0_BGR_REG, dsi0_bus_clk, reset map`.
    pub dsi0_bgr: RW<SingleBusGatingReset>,
    _reserved_1588: [u8; 0x038],
    /// 0x15c0 - `VO0_COMBPHY0_CLK_REG, vo0_combphy0_clk`.
    pub vo0_combphy0_clk: RW<u32>,
    _reserved_15c4: [u8; 0x100],
    /// 0x16c4 - `DPSS_BGR_REG, dpss_clk, reset map`.
    pub dpss_bgr: RW<SingleBusGatingReset>,
    _reserved_16c8: [u8; 0x01c],
    /// 0x16e4 - `VIDEO_OUT0_BGR_REG, reset map`.
    pub video_out0_bgr: RW<u32>,
    _reserved_16e8: [u8; 0x018],
    /// 0x1700 - `LEDC_CLK_REG, ledc_clk`.
    pub ledc_clk: RW<u32>,
    /// 0x1704 - `LEDC_BGR_REG, ledc_bus_clk, reset map`.
    pub ledc_bgr: RW<SingleBusGatingReset>,
    _reserved_1708: [u8; 0x0f8],
    /// 0x1800 - `CSI_MASTER0_CLK_REG, csi_master0_clk`.
    pub csi_master0_clk: RW<u32>,
    /// 0x1804 - `CSI_MASTER1_CLK_REG, csi_master1_clk`.
    pub csi_master1_clk: RW<u32>,
    /// 0x1808 - `CSI_MASTER2_CLK_REG, csi_master2_clk`.
    pub csi_master2_clk: RW<u32>,
    /// 0x180c - `CSI_MASTER3_CLK_REG, csi_master3_clk`.
    pub csi_master3_clk: RW<u32>,
    _reserved_1810: [u8; 0x030],
    /// 0x1840 - `CSI_CLK_REG, csi_clk`.
    pub csi_clk: RW<u32>,
    /// 0x1844 - `CSI_BGR_REG, csi_bus_clk, reset map`.
    pub csi_bgr: RW<SingleBusGatingReset>,
    _reserved_1848: [u8; 0x018],
    /// 0x1860 - `ISP_CLK_REG, isp_clk`.
    pub isp_clk: RW<u32>,
    /// 0x1864 - `ISP_BGR_REG, isp_bus_clk, reset map`.
    pub isp_bgr: RW<SingleBusGatingReset>,
    _reserved_1868: [u8; 0x0a0],
    /// 0x1908 - `PERI0PLL_GATE_EN_REG`.
    pub peri0pll_gate_en: RW<u32>,
    /// 0x190c - `PERI1PLL_GATE_EN_REG`.
    pub peri1pll_gate_en: RW<u32>,
    /// 0x1910 - `VIDEOPLL_GATE_EN_REG`.
    pub videopll_gate_en: RW<u32>,
    _reserved_1914: [u8; 0x074],
    /// 0x1988 - `PERI0PLL_GATE_STAT_REG`.
    pub peri0pll_gate_stat: RO<u32>,
    /// 0x198c - `PERI1PLL_GATE_STAT_REG`.
    pub peri1pll_gate_stat: RO<u32>,
    _reserved_1990: [u8; 0x008],
    /// 0x1998 - `VIDEOPLL_GATE_STAT_REG`.
    pub videopll_gate_stat: RO<u32>,
    _reserved_199c: [u8; 0x064],
    /// 0x1a00 - `CLK24M_GATE_EN_REG`.
    pub clk24m_gate_en: RW<u32>,
    _reserved_1a04: [u8; 0x00c],
    /// 0x1a10 - `PERI1_FOCPU_EN_REG`.
    pub peri1_focpu_en: RW<u32>,
    _reserved_1a14: [u8; 0x0ec],
    /// 0x1b00 - `CM_VI_CFG_REG`.
    pub cm_vi_cfg: RW<u32>,
    _reserved_1b04: [u8; 0x00c],
    /// 0x1b10 - `CM_VE_CFG_REG`.
    pub cm_ve_cfg: RW<u32>,
    _reserved_1b14: [u8; 0x008],
    /// 0x1b1c - `CM_NPU_CFG_REG`.
    pub cm_npu_cfg: RW<u32>,
    _reserved_1b20: [u8; 0x008],
    /// 0x1b28 - `CM_SERDES_CFG_REG`.
    pub cm_serdes_cfg: RW<u32>,
    _reserved_1b2c: [u8; 0x008],
    /// 0x1b34 - `CM_VO_CFG_REG`.
    pub cm_vo_cfg: RW<u32>,
    _reserved_1b38: [u8; 0x008],
    /// 0x1b40 - `CM_RV_CFG_REG`.
    pub cm_rv_cfg: RW<u32>,
    _reserved_1b44: [u8; 0x3bc],
    /// 0x1f00 - `CCU_SEC_SWITCH_REG`.
    pub sec_switch: RW<u32>,
    _reserved_1f04: [u8; 0x00c],
    /// 0x1f10 - `SYSDAP_REQ_CTRL_REG`.
    pub sysdap_req_ctrl: RW<u32>,
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
    /// 0x1f50 - `BUS_CLK_DBG_REG`.
    pub bus_clk_dbg: RW<u32>,
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
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_ctrl), 0x020);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat0_ctrl), 0x028);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat1_ctrl), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_bias), 0x030);
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
        assert_eq!(offset_of!(RegisterBlock, pll_video1_ctrl), 0x140);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat0_ctrl), 0x148);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat1_ctrl), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_bias), 0x150);
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
        assert_eq!(offset_of!(RegisterBlock, pll_npu_ctrl), 0x2a0);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_pat0_ctrl), 0x2a8);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_pat1_ctrl), 0x2ac);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_bias), 0x2b0);
        assert_eq!(offset_of!(RegisterBlock, ahb_clk), 0x500);
        assert_eq!(offset_of!(RegisterBlock, apb0_clk), 0x510);
        assert_eq!(offset_of!(RegisterBlock, apb1_clk), 0x518);
        assert_eq!(offset_of!(RegisterBlock, apb_uart_clk), 0x538);
        assert_eq!(offset_of!(RegisterBlock, trace_clk), 0x540);
        assert_eq!(offset_of!(RegisterBlock, gic_clk), 0x560);
        assert_eq!(offset_of!(RegisterBlock, its0_bgr), 0x574);
        assert_eq!(offset_of!(RegisterBlock, nsi_clk), 0x580);
        assert_eq!(offset_of!(RegisterBlock, nsi_bgr), 0x584);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk), 0x588);
        assert_eq!(offset_of!(RegisterBlock, iommu_bgr), 0x58c);
        assert_eq!(offset_of!(RegisterBlock, ahb_gate_en), 0x5c0);
        assert_eq!(offset_of!(RegisterBlock, mbus_gate_en), 0x5e0);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gating), 0x5e4);
        assert_eq!(offset_of!(RegisterBlock, dma0_bgr), 0x704);
        assert_eq!(offset_of!(RegisterBlock, dma1_bgr), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, spinlock_bgr), 0x724);
        assert_eq!(offset_of!(RegisterBlock, msgbox0_bgr), 0x744);
        assert_eq!(offset_of!(RegisterBlock, msgbox_core0_bgr), 0x74c);
        assert_eq!(offset_of!(RegisterBlock, msgbox_core1_bgr), 0x754);
        assert_eq!(offset_of!(RegisterBlock, msgbox_core2_bgr), 0x75c);
        assert_eq!(offset_of!(RegisterBlock, msgbox_core3_bgr), 0x764);
        assert_eq!(offset_of!(RegisterBlock, msgbox_rv_bgr), 0x76c);
        assert_eq!(offset_of!(RegisterBlock, pwm0_bgr), 0x784);
        assert_eq!(offset_of!(RegisterBlock, pwm1_bgr), 0x78c);
        assert_eq!(offset_of!(RegisterBlock, pwm2_bgr), 0x794);
        assert_eq!(offset_of!(RegisterBlock, dbgsys_bgr), 0x7a4);
        assert_eq!(offset_of!(RegisterBlock, sysdap_bgr), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, timer1_clk), 0x804);
        assert_eq!(offset_of!(RegisterBlock, timer2_clk), 0x808);
        assert_eq!(offset_of!(RegisterBlock, timer3_clk), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, timer4_clk), 0x810);
        assert_eq!(offset_of!(RegisterBlock, timer5_clk), 0x814);
        assert_eq!(offset_of!(RegisterBlock, timer6_clk), 0x818);
        assert_eq!(offset_of!(RegisterBlock, timer7_clk), 0x81c);
        assert_eq!(offset_of!(RegisterBlock, timer_bgr), 0x850);
        assert_eq!(offset_of!(RegisterBlock, timer0_rv_clk), 0x860);
        assert_eq!(offset_of!(RegisterBlock, timer1_rv_clk), 0x864);
        assert_eq!(offset_of!(RegisterBlock, timer2_rv_clk), 0x868);
        assert_eq!(offset_of!(RegisterBlock, timer3_rv_clk), 0x86c);
        assert_eq!(offset_of!(RegisterBlock, timer_rv_bgr), 0x870);
        assert_eq!(offset_of!(RegisterBlock, de0_clk), 0xa00);
        assert_eq!(offset_of!(RegisterBlock, de0_bgr), 0xa04);
        assert_eq!(offset_of!(RegisterBlock, g2d_clk), 0xa40);
        assert_eq!(offset_of!(RegisterBlock, g2d_bgr), 0xa44);
        assert_eq!(offset_of!(RegisterBlock, de_sys_bgr), 0xa74);
        assert_eq!(offset_of!(RegisterBlock, ve_clk), 0xa80);
        assert_eq!(offset_of!(RegisterBlock, ve_bgr), 0xa8c);
        assert_eq!(offset_of!(RegisterBlock, ce_clk), 0xac0);
        assert_eq!(offset_of!(RegisterBlock, ce_bgr), 0xac4);
        assert_eq!(offset_of!(RegisterBlock, npu_clk), 0xb00);
        assert_eq!(offset_of!(RegisterBlock, npu_bgr), 0xb04);
        assert_eq!(offset_of!(RegisterBlock, rv_core_clk), 0xb80);
        assert_eq!(offset_of!(RegisterBlock, rv_ts_clk), 0xb88);
        assert_eq!(offset_of!(RegisterBlock, rv_sys_bgr), 0xb94);
        assert_eq!(offset_of!(RegisterBlock, rv_cfg_bgr), 0xb9c);
        assert_eq!(offset_of!(RegisterBlock, dram_clk), 0xc00);
        assert_eq!(offset_of!(RegisterBlock, dram_bgr), 0xc0c);
        assert_eq!(offset_of!(RegisterBlock, nand0_clk2x_clk), 0xc80);
        assert_eq!(offset_of!(RegisterBlock, nand0_clk1_clk), 0xc84);
        assert_eq!(offset_of!(RegisterBlock, nand0_bgr), 0xc8c);
        assert_eq!(offset_of!(RegisterBlock, smhc0_clk), 0xd00);
        assert_eq!(offset_of!(RegisterBlock, smhc0_bgr), 0xd0c);
        assert_eq!(offset_of!(RegisterBlock, smhc1_clk), 0xd10);
        assert_eq!(offset_of!(RegisterBlock, smhc1_bgr), 0xd1c);
        assert_eq!(offset_of!(RegisterBlock, smhc2_clk), 0xd20);
        assert_eq!(offset_of!(RegisterBlock, smhc2_bgr), 0xd2c);
        assert_eq!(offset_of!(RegisterBlock, uart0_bgr), 0xe00);
        assert_eq!(offset_of!(RegisterBlock, uart1_bgr), 0xe04);
        assert_eq!(offset_of!(RegisterBlock, uart2_bgr), 0xe08);
        assert_eq!(offset_of!(RegisterBlock, uart3_bgr), 0xe0c);
        assert_eq!(offset_of!(RegisterBlock, uart4_bgr), 0xe10);
        assert_eq!(offset_of!(RegisterBlock, uart5_bgr), 0xe14);
        assert_eq!(offset_of!(RegisterBlock, uart6_bgr), 0xe18);
        assert_eq!(offset_of!(RegisterBlock, uart7_bgr), 0xe20);
        assert_eq!(offset_of!(RegisterBlock, uart8_bgr), 0xe24);
        assert_eq!(offset_of!(RegisterBlock, uart9_bgr), 0xe28);
        assert_eq!(offset_of!(RegisterBlock, uart10_bgr), 0xe2c);
        assert_eq!(offset_of!(RegisterBlock, uart11_bgr), 0xe30);
        assert_eq!(offset_of!(RegisterBlock, uart12_bgr), 0xe34);
        assert_eq!(offset_of!(RegisterBlock, uart13_bgr), 0xe38);
        assert_eq!(offset_of!(RegisterBlock, uart14_bgr), 0xe3c);
        assert_eq!(offset_of!(RegisterBlock, twi0_bgr), 0xe80);
        assert_eq!(offset_of!(RegisterBlock, twi1_bgr), 0xe84);
        assert_eq!(offset_of!(RegisterBlock, twi2_bgr), 0xe88);
        assert_eq!(offset_of!(RegisterBlock, twi3_bgr), 0xe8c);
        assert_eq!(offset_of!(RegisterBlock, twi4_bgr), 0xe90);
        assert_eq!(offset_of!(RegisterBlock, twi5_bgr), 0xe94);
        assert_eq!(offset_of!(RegisterBlock, twi6_bgr), 0xe98);
        assert_eq!(offset_of!(RegisterBlock, spi0_clk), 0xf00);
        assert_eq!(offset_of!(RegisterBlock, spi0_bgr), 0xf04);
        assert_eq!(offset_of!(RegisterBlock, spi1_clk), 0xf08);
        assert_eq!(offset_of!(RegisterBlock, spi1_bgr), 0xf0c);
        assert_eq!(offset_of!(RegisterBlock, spi2_clk), 0xf10);
        assert_eq!(offset_of!(RegisterBlock, spi2_bgr), 0xf14);
        assert_eq!(offset_of!(RegisterBlock, spif_clk), 0xf18);
        assert_eq!(offset_of!(RegisterBlock, spif_bgr), 0xf1c);
        assert_eq!(offset_of!(RegisterBlock, spi3_clk), 0xf20);
        assert_eq!(offset_of!(RegisterBlock, spi3_bgr), 0xf24);
        assert_eq!(offset_of!(RegisterBlock, spi4_clk), 0xf28);
        assert_eq!(offset_of!(RegisterBlock, spi4_bgr), 0xf2c);
        assert_eq!(offset_of!(RegisterBlock, gpadc0_clk), 0xfc0);
        assert_eq!(offset_of!(RegisterBlock, gpadc0_bgr), 0xfc4);
        assert_eq!(offset_of!(RegisterBlock, gpadc1_clk), 0xfc8);
        assert_eq!(offset_of!(RegisterBlock, gpadc1_bgr), 0xfcc);
        assert_eq!(offset_of!(RegisterBlock, gpadc2_clk), 0xfd0);
        assert_eq!(offset_of!(RegisterBlock, gpadc2_bgr), 0xfd4);
        assert_eq!(offset_of!(RegisterBlock, gpadc3_clk), 0xfd8);
        assert_eq!(offset_of!(RegisterBlock, gpadc3_bgr), 0xfdc);
        assert_eq!(offset_of!(RegisterBlock, ths_bgr), 0xfe4);
        assert_eq!(offset_of!(RegisterBlock, irrx0_clk), 0x1000);
        assert_eq!(offset_of!(RegisterBlock, irrx0_bgr), 0x1004);
        assert_eq!(offset_of!(RegisterBlock, irtx_clk), 0x1008);
        assert_eq!(offset_of!(RegisterBlock, irtx_bgr), 0x100c);
        assert_eq!(offset_of!(RegisterBlock, lradc_bgr), 0x1024);
        assert_eq!(offset_of!(RegisterBlock, tpadc_24m_clk), 0x1030);
        assert_eq!(offset_of!(RegisterBlock, tpadc_bgr), 0x1034);
        assert_eq!(offset_of!(RegisterBlock, lbc_clk), 0x1040);
        assert_eq!(offset_of!(RegisterBlock, lbc_nsi_ahb_clk), 0x1048);
        assert_eq!(offset_of!(RegisterBlock, lbc_bgr), 0x104c);
        assert_eq!(offset_of!(RegisterBlock, irrx1_clk), 0x1100);
        assert_eq!(offset_of!(RegisterBlock, irrx1_bgr), 0x1104);
        assert_eq!(offset_of!(RegisterBlock, irrx2_clk), 0x1108);
        assert_eq!(offset_of!(RegisterBlock, irrx2_bgr), 0x110c);
        assert_eq!(offset_of!(RegisterBlock, irrx3_clk), 0x1110);
        assert_eq!(offset_of!(RegisterBlock, irrx3_bgr), 0x1114);
        assert_eq!(offset_of!(RegisterBlock, i2spcm0_clk), 0x1200);
        assert_eq!(offset_of!(RegisterBlock, i2spcm0_bgr), 0x120c);
        assert_eq!(offset_of!(RegisterBlock, i2spcm1_clk), 0x1210);
        assert_eq!(offset_of!(RegisterBlock, i2spcm1_bgr), 0x121c);
        assert_eq!(offset_of!(RegisterBlock, i2spcm2_clk), 0x1220);
        assert_eq!(offset_of!(RegisterBlock, i2spcm2_bgr), 0x122c);
        assert_eq!(offset_of!(RegisterBlock, i2spcm3_clk), 0x1230);
        assert_eq!(offset_of!(RegisterBlock, i2spcm3_bgr), 0x123c);
        assert_eq!(offset_of!(RegisterBlock, owa_tx_clk), 0x1280);
        assert_eq!(offset_of!(RegisterBlock, owa_rx_clk), 0x1284);
        assert_eq!(offset_of!(RegisterBlock, owa_bgr), 0x128c);
        assert_eq!(offset_of!(RegisterBlock, dmic_clk), 0x12c0);
        assert_eq!(offset_of!(RegisterBlock, dmic_bgr), 0x12cc);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_dac_1x_clk), 0x12e0);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_bgr), 0x12ec);
        assert_eq!(offset_of!(RegisterBlock, usb0_clk), 0x1300);
        assert_eq!(offset_of!(RegisterBlock, usb0_bgr), 0x1304);
        assert_eq!(offset_of!(RegisterBlock, usb1_clk), 0x1308);
        assert_eq!(offset_of!(RegisterBlock, usb1_bgr), 0x130c);
        assert_eq!(offset_of!(RegisterBlock, usb0_usb1_24m), 0x1310);
        assert_eq!(offset_of!(RegisterBlock, usb2_u2_ref_clk), 0x1348);
        assert_eq!(offset_of!(RegisterBlock, usb2_suspend_clk), 0x1350);
        assert_eq!(offset_of!(RegisterBlock, usb2_mf_clk), 0x1354);
        assert_eq!(offset_of!(RegisterBlock, usb2_bgr), 0x135c);
        assert_eq!(offset_of!(RegisterBlock, pcie_aux_clk), 0x1380);
        assert_eq!(offset_of!(RegisterBlock, pcie_slv_clk), 0x1384);
        assert_eq!(offset_of!(RegisterBlock, pcie_bgr), 0x138c);
        assert_eq!(offset_of!(RegisterBlock, serdes_phy_cfg_clk), 0x13c0);
        assert_eq!(offset_of!(RegisterBlock, serdes_phy_ref_clk), 0x13c4);
        assert_eq!(offset_of!(RegisterBlock, serdes_bgr), 0x13cc);
        assert_eq!(offset_of!(RegisterBlock, serdes_axi_clk), 0x13e0);
        assert_eq!(offset_of!(RegisterBlock, gmac0_phy_clk), 0x1400);
        assert_eq!(offset_of!(RegisterBlock, gmac0_ptp_clk), 0x1404);
        assert_eq!(offset_of!(RegisterBlock, gmac0_bgr), 0x140c);
        assert_eq!(offset_of!(RegisterBlock, gmac1_phy_clk), 0x1410);
        assert_eq!(offset_of!(RegisterBlock, gmac1_ptp_clk), 0x1414);
        assert_eq!(offset_of!(RegisterBlock, gmac1_bgr), 0x141c);
        assert_eq!(offset_of!(RegisterBlock, gmac_nsi), 0x1420);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd0_clk), 0x1500);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd0_bgr), 0x1504);
        assert_eq!(offset_of!(RegisterBlock, lvds0_bgr), 0x1544);
        assert_eq!(offset_of!(RegisterBlock, dsi0_clk), 0x1580);
        assert_eq!(offset_of!(RegisterBlock, dsi0_bgr), 0x1584);
        assert_eq!(offset_of!(RegisterBlock, vo0_combphy0_clk), 0x15c0);
        assert_eq!(offset_of!(RegisterBlock, dpss_bgr), 0x16c4);
        assert_eq!(offset_of!(RegisterBlock, video_out0_bgr), 0x16e4);
        assert_eq!(offset_of!(RegisterBlock, ledc_clk), 0x1700);
        assert_eq!(offset_of!(RegisterBlock, ledc_bgr), 0x1704);
        assert_eq!(offset_of!(RegisterBlock, csi_master0_clk), 0x1800);
        assert_eq!(offset_of!(RegisterBlock, csi_master1_clk), 0x1804);
        assert_eq!(offset_of!(RegisterBlock, csi_master2_clk), 0x1808);
        assert_eq!(offset_of!(RegisterBlock, csi_master3_clk), 0x180c);
        assert_eq!(offset_of!(RegisterBlock, csi_clk), 0x1840);
        assert_eq!(offset_of!(RegisterBlock, csi_bgr), 0x1844);
        assert_eq!(offset_of!(RegisterBlock, isp_clk), 0x1860);
        assert_eq!(offset_of!(RegisterBlock, isp_bgr), 0x1864);
        assert_eq!(offset_of!(RegisterBlock, peri0pll_gate_en), 0x1908);
        assert_eq!(offset_of!(RegisterBlock, peri1pll_gate_en), 0x190c);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_en), 0x1910);
        assert_eq!(offset_of!(RegisterBlock, peri0pll_gate_stat), 0x1988);
        assert_eq!(offset_of!(RegisterBlock, peri1pll_gate_stat), 0x198c);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_stat), 0x1998);
        assert_eq!(offset_of!(RegisterBlock, clk24m_gate_en), 0x1a00);
        assert_eq!(offset_of!(RegisterBlock, peri1_focpu_en), 0x1a10);
        assert_eq!(offset_of!(RegisterBlock, cm_vi_cfg), 0x1b00);
        assert_eq!(offset_of!(RegisterBlock, cm_ve_cfg), 0x1b10);
        assert_eq!(offset_of!(RegisterBlock, cm_npu_cfg), 0x1b1c);
        assert_eq!(offset_of!(RegisterBlock, cm_serdes_cfg), 0x1b28);
        assert_eq!(offset_of!(RegisterBlock, cm_vo_cfg), 0x1b34);
        assert_eq!(offset_of!(RegisterBlock, cm_rv_cfg), 0x1b40);
        assert_eq!(offset_of!(RegisterBlock, sec_switch), 0x1f00);
        assert_eq!(offset_of!(RegisterBlock, sysdap_req_ctrl), 0x1f10);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg0), 0x1f20);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg1), 0x1f24);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg2), 0x1f28);
        assert_eq!(offset_of!(RegisterBlock, pll_lock_dbg_ctrl), 0x1f2c);
        assert_eq!(offset_of!(RegisterBlock, fan_gate), 0x1f30);
        assert_eq!(offset_of!(RegisterBlock, clk27m_fan), 0x1f34);
        assert_eq!(offset_of!(RegisterBlock, clk_fan), 0x1f38);
        assert_eq!(offset_of!(RegisterBlock, fan), 0x1f3c);
        assert_eq!(offset_of!(RegisterBlock, bus_clk_dbg), 0x1f50);
        assert_eq!(offset_of!(RegisterBlock, version), 0x1ff0);
        assert_eq!(size_of::<RegisterBlock>(), 0x1ff4);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
