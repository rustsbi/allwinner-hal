//! T527/A523/A527 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun55iw3` platform.

use super::{BusGatingReset, SingleBusGatingReset};
use volatile_register::{RO, RW};

/// T527/A523/A527 main CCU register block.
#[doc(alias = "sun55iw3")]
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - `PLL_CPU0_CTRL_REG`.
    pub pll_cpu0_ctrl: RW<u32>,
    /// 0x0004 - `PLL_CPU1_CTRL_REG`.
    pub pll_cpu1_ctrl: RW<u32>,
    /// 0x0008 - `PLL_CPU2_CTRL_REG, PLL_CPU3_CTRL_REG`.
    pub pll_cpu2_ctrl: RW<u32>,
    _reserved_000c: [u8; 0x004],
    /// 0x0010 - `PLL_DDR_CTRL_REG, SUN55IW3_PLL_DDR_CTRL_REG, pll_ddr_clk`.
    pub pll_ddr_ctrl: RW<u32>,
    _reserved_0014: [u8; 0x004],
    /// 0x0018 - `CCU_PLL_DDR1_CTRL_REG`.
    pub pll_ddr1_ctrl: RW<u32>,
    _reserved_001c: [u8; 0x004],
    /// 0x0020 - `PLL_PERI0_CTRL_REG, SUN55IW3_PLL_PERI0_CTRL_REG, pll_peri0_2x_clk, ...`.
    pub pll_peri0_ctrl: RW<u32>,
    _reserved_0024: [u8; 0x004],
    /// 0x0028 - `PLL_PERI1_CTRL_REG, SUN55IW3_PLL_PERI1_CTRL_REG, pll_peri1_2x_clk, ...`.
    pub pll_peri1_ctrl: RW<u32>,
    _reserved_002c: [u8; 0x004],
    /// 0x0030 - `PLL_GPU_CTRL_REG, SUN55IW3_PLL_GPU_CTRL_REG, pll_gpu_clk`.
    pub pll_gpu_ctrl: RW<u32>,
    _reserved_0034: [u8; 0x00c],
    /// 0x0040 - `PLL_VIDEO0_CTRL_REG, SUN55IW3_PLL_VIDEO0_CTRL_REG, pll_video0_4x_clk, ...`.
    pub pll_video0_ctrl: RW<u32>,
    _reserved_0044: [u8; 0x004],
    /// 0x0048 - `PLL_VIDEO1_CTRL_REG, SUN55IW3_PLL_VIDEO1_CTRL_REG, pll_video1_4x_clk, ...`.
    pub pll_video1_ctrl: RW<u32>,
    _reserved_004c: [u8; 0x004],
    /// 0x0050 - `PLL_VIDEO2_CTRL_REG, SUN55IW3_PLL_VIDEO2_CTRL_REG, pll_video2_4x_clk, ...`.
    pub pll_video2_ctrl: RW<u32>,
    _reserved_0054: [u8; 0x004],
    /// 0x0058 - `PLL_VE_CTRL_REG, SUN55IW3_PLL_VE_CTRL_REG, pll_ve_clk`.
    pub pll_ve_ctrl: RW<u32>,
    _reserved_005c: [u8; 0x00c],
    /// 0x0068 - `PLL_VIDEO3_CTRL_REG, SUN55IW3_PLL_VIDEO3_CTRL_REG, pll_video3_4x_clk, ...`.
    pub pll_video3_ctrl: RW<u32>,
    _reserved_006c: [u8; 0x004],
    /// 0x0070 - `CCU_PLL_HSIC_CTRL_REG`.
    pub pll_hsic_ctrl: RW<u32>,
    _reserved_0074: [u8; 0x004],
    /// 0x0078 - `PLL_AUDIO_CTRL_REG, SUN55IW3_PLL_AUDIO0_REG, pll_audio0_4x_clk`.
    pub pll_audio_ctrl: RW<u32>,
    _reserved_007c: [u8; 0x004],
    /// 0x0080 - `PLL_NPU_CTRL_REG, SUN55IW3_PLL_NPU_CTRL_REG, pll_npu_4x_clk`.
    pub pll_npu_ctrl: RW<u32>,
    _reserved_0084: [u8; 0x08c],
    /// 0x0110 - `PLL_DDR_PAT0_CTRL_REG`.
    pub pll_ddr_pat0_ctrl: RW<u32>,
    /// 0x0114 - `PLL_DDR_PAT1_CTRL_REG`.
    pub pll_ddr_pat1_ctrl: RW<u32>,
    _reserved_0118: [u8; 0x008],
    /// 0x0120 - `PLL_PERI0_PAT0_CTRL_REG`.
    pub pll_peri0_pat0_ctrl: RW<u32>,
    /// 0x0124 - `PLL_PERI0_PAT1_CTRL_REG`.
    pub pll_peri0_pat1_ctrl: RW<u32>,
    /// 0x0128 - `PLL_PERI1_PAT0_CTRL_REG, SUN55IW3_PLL_PERIPH1_PATTERN0_REG, pll_peri1_clk`.
    pub pll_periph1_pattern0: RW<u32>,
    /// 0x012c - `PLL_PERI1_PAT1_CTRL_REG`.
    pub pll_peri1_pat1_ctrl: RW<u32>,
    /// 0x0130 - `PLL_GPU_PAT0_CTRL_REG`.
    pub pll_gpu_pat0_ctrl: RW<u32>,
    /// 0x0134 - `PLL_GPU_PAT1_CTRL_REG`.
    pub pll_gpu_pat1_ctrl: RW<u32>,
    _reserved_0138: [u8; 0x008],
    /// 0x0140 - `PLL_VIDEO0_PAT0_CTRL_REG`.
    pub pll_video0_pat0_ctrl: RW<u32>,
    /// 0x0144 - `PLL_VIDEO0_PAT1_CTRL_REG`.
    pub pll_video0_pat1_ctrl: RW<u32>,
    /// 0x0148 - `PLL_VIDEO1_PAT0_CTRL_REG`.
    pub pll_video1_pat0_ctrl: RW<u32>,
    /// 0x014c - `PLL_VIDEO1_PAT1_CTRL_REG`.
    pub pll_video1_pat1_ctrl: RW<u32>,
    /// 0x0150 - `PLL_VIDEO2_PAT0_CTRL_REG`.
    pub pll_video2_pat0_ctrl: RW<u32>,
    /// 0x0154 - `PLL_VIDEO2_PAT1_CTRL_REG`.
    pub pll_video2_pat1_ctrl: RW<u32>,
    /// 0x0158 - `PLL_VE_PAT0_CTRL_REG`.
    pub pll_ve_pat0_ctrl: RW<u32>,
    /// 0x015c - `PLL_VE_PAT1_CTRL_REG`.
    pub pll_ve_pat1_ctrl: RW<u32>,
    _reserved_0160: [u8; 0x008],
    /// 0x0168 - `PLL_VIDEO3_PAT0_CTRL_REG`.
    pub pll_video3_pat0_ctrl: RW<u32>,
    /// 0x016c - `PLL_VIDEO3_PAT1_CTRL_REG`.
    pub pll_video3_pat1_ctrl: RW<u32>,
    _reserved_0170: [u8; 0x008],
    /// 0x0178 - `PLL_AUDIO_PAT0_CTRL_REG, pll_audio_sdm_clk`.
    pub pll_audio_pat0_ctrl: RW<u32>,
    /// 0x017c - `PLL_AUDIO_PAT1_CTRL_REG`.
    pub pll_audio_pat1_ctrl: RW<u32>,
    /// 0x0180 - `PLL_NPU_PAT0_CTRL_REG`.
    pub pll_npu_pat0_ctrl: RW<u32>,
    /// 0x0184 - `PLL_NPU_PAT1_CTRL_REG`.
    pub pll_npu_pat1_ctrl: RW<u32>,
    _reserved_0188: [u8; 0x178],
    /// 0x0300 - `PLL_CPU0_BIAS_REG`.
    pub pll_cpu0_bias: RW<u32>,
    _reserved_0304: [u8; 0x004],
    /// 0x0308 - `PLL_CPU1_BIAS_REG`.
    pub pll_cpu1_bias: RW<u32>,
    /// 0x030c - `PLL_CPU2_BIAS_REG`.
    pub pll_cpu2_bias: RW<u32>,
    /// 0x0310 - `PLL_DDR_BIAS_REG`.
    pub pll_ddr_bias: RW<u32>,
    _reserved_0314: [u8; 0x00c],
    /// 0x0320 - `PLL_PERI0_BIAS_REG`.
    pub pll_peri0_bias: RW<u32>,
    _reserved_0324: [u8; 0x004],
    /// 0x0328 - `PLL_PERI1_BIAS_REG`.
    pub pll_peri1_bias: RW<u32>,
    _reserved_032c: [u8; 0x004],
    /// 0x0330 - `PLL_GPU_BIAS_REG`.
    pub pll_gpu_bias: RW<u32>,
    _reserved_0334: [u8; 0x00c],
    /// 0x0340 - `PLL_VIDEO0_BIAS_REG`.
    pub pll_video0_bias: RW<u32>,
    _reserved_0344: [u8; 0x004],
    /// 0x0348 - `PLL_VIDEO1_BIAS_REG`.
    pub pll_video1_bias: RW<u32>,
    _reserved_034c: [u8; 0x004],
    /// 0x0350 - `PLL_VIDEO2_BIAS_REG`.
    pub pll_video2_bias: RW<u32>,
    _reserved_0354: [u8; 0x004],
    /// 0x0358 - `PLL_VE_BIAS_REG`.
    pub pll_ve_bias: RW<u32>,
    _reserved_035c: [u8; 0x00c],
    /// 0x0368 - `PLL_VIDEO3_BIAS_REG`.
    pub pll_video3_bias: RW<u32>,
    _reserved_036c: [u8; 0x00c],
    /// 0x0378 - `PLL_AUDIO_BIAS_REG`.
    pub pll_audio_bias: RW<u32>,
    _reserved_037c: [u8; 0x004],
    /// 0x0380 - `PLL_NPU_BIAS_REG`.
    pub pll_npu_bias: RW<u32>,
    _reserved_0384: [u8; 0x07c],
    /// 0x0400 - `PLL_CPU0_TUN_REG`.
    pub pll_cpu0_tun: RW<u32>,
    _reserved_0404: [u8; 0x004],
    /// 0x0408 - `PLL_CPU1_TUN_REG`.
    pub pll_cpu1_tun: RW<u32>,
    /// 0x040c - `PLL_CPU2_TUN_REG`.
    pub pll_cpu2_tun: RW<u32>,
    _reserved_0410: [u8; 0x0f0],
    /// 0x0500 - `CPU_CLK_REG`.
    pub cpu_clk: RW<u32>,
    /// 0x0504 - `CPU_GATING_REG`.
    pub cpu_gating: RW<u32>,
    /// 0x0508 - `TRACE_CLK_REG, trace_clk`.
    pub trace_clk: RW<u32>,
    /// 0x050c - `DSU_CLK_REG`.
    pub dsu_clk: RW<u32>,
    /// 0x0510 - `AHB_CLK_REG, CCU_AHB0_CFG_REG`.
    pub ahb0_cfg: RW<u32>,
    _reserved_0514: [u8; 0x00c],
    /// 0x0520 - `APB0_CLK_REG, CCU_APB0_CFG_REG`.
    pub apb0_cfg: RW<u32>,
    /// 0x0524 - `APB1_CLK_REG, CCU_APB1_CFG_REG, apb1_clk`.
    pub apb1_cfg: RW<u32>,
    _reserved_0528: [u8; 0x018],
    /// 0x0540 - `CCU_MBUS_CFG_REG, MBUS_CLK_REG, mbus_clk, ...`.
    pub mbus_cfg: RW<u32>,
    _reserved_0544: [u8; 0x008],
    /// 0x054c - `NSI_BGR_REG, nsi_clk, reset map`.
    pub nsi_bgr: RW<SingleBusGatingReset>,
    /// 0x0550 - `GIC_CLK_REG, gic_clk`.
    pub gic_clk: RW<u32>,
    _reserved_0554: [u8; 0x0ac],
    /// 0x0600 - `DE0_CLK_REG, de_clk`.
    pub de0_clk: RW<u32>,
    _reserved_0604: [u8; 0x008],
    /// 0x060c - `DE_BGR_REG, de0_clk, reset map`.
    pub de_bgr: RW<SingleBusGatingReset>,
    _reserved_0610: [u8; 0x010],
    /// 0x0620 - `DI_CLK_REG, di_clk`.
    pub di_clk: RW<u32>,
    _reserved_0624: [u8; 0x008],
    /// 0x062c - `DI_BGR_REG, bus_di_clk, reset map`.
    pub di_bgr: RW<SingleBusGatingReset>,
    /// 0x0630 - `G2D_CLK_REG, g2d_clk`.
    pub g2d_clk: RW<u32>,
    _reserved_0634: [u8; 0x008],
    /// 0x063c - `G2D_BGR_REG, bus_g2d_clk, reset map`.
    pub g2d_bgr: RW<SingleBusGatingReset>,
    _reserved_0640: [u8; 0x030],
    /// 0x0670 - `GPU_CORE_CLK_REG, gpu_clk`.
    pub gpu_core_clk: RW<u32>,
    _reserved_0674: [u8; 0x008],
    /// 0x067c - `GPU_GATING_REG, bus_gpu_clk, reset map`.
    pub gpu_gating: RW<u32>,
    /// 0x0680 - `CCU_CE_CLK_REG, CE_CLK_REG, ce_clk`.
    pub ce_clk: RW<u32>,
    _reserved_0684: [u8; 0x008],
    /// 0x068c - `CCU_CE_BGR_REG, CE_BGR_REG, bus_ce_clk, ...`.
    pub ce_bgr: RW<SingleBusGatingReset>,
    /// 0x0690 - `CCU_VE_CLK_REG, VE_CLK_REG, ve_clk`.
    pub ve_clk: RW<u32>,
    _reserved_0694: [u8; 0x008],
    /// 0x069c - `CCU_VE_BGR_REG, VE_BGR_REG, bus_ve_clk, ...`.
    pub ve_bgr: RW<SingleBusGatingReset>,
    _reserved_06a0: [u8; 0x040],
    /// 0x06e0 - `NPU_CLK_REG, npu_clk`.
    pub npu_clk: RW<u32>,
    _reserved_06e4: [u8; 0x028],
    /// 0x070c - `CCU_DMA_BGR_REG, DMA_BGR_REG, dma_clk, ...`.
    pub dma_bgr: RW<SingleBusGatingReset>,
    _reserved_0710: [u8; 0x00c],
    /// 0x071c - `MSGBOX_BGR_REG, msgbox0_clk, reset map`.
    pub msgbox_bgr: RW<SingleBusGatingReset>,
    _reserved_0720: [u8; 0x00c],
    /// 0x072c - `SPINLOCK_BGR_REG, reset map, spinlock_clk`.
    pub spinlock_bgr: RW<SingleBusGatingReset>,
    /// 0x0730 - `TIMER0_CLK_REG, timer0_clk`.
    pub timer0_clk: RW<u32>,
    /// 0x0734 - `TIMER1_CLK_REG, timer1_clk`.
    pub timer1_clk: RW<u32>,
    /// 0x0738 - `TIMER2_CLK_REG, timer2_clk`.
    pub timer2_clk: RW<u32>,
    /// 0x073c - `TIMER3_CLK_REG, timer3_clk`.
    pub timer3_clk: RW<u32>,
    /// 0x0740 - `TIMER4_CLK_REG, timer4_clk`.
    pub timer4_clk: RW<u32>,
    /// 0x0744 - `TIMER5_CLK_REG, timer5_clk`.
    pub timer5_clk: RW<u32>,
    _reserved_0748: [u8; 0x004],
    /// 0x074c - `CCU_AVS_BGR_REG, TIMER_BGR_REG, reset map, ...`.
    pub timer_bgr: RW<SingleBusGatingReset>,
    /// 0x0750 - `AVS_CLK_REG, CCU_AVS_CLK_REG`.
    pub avs_clk: RW<u32>,
    _reserved_0754: [u8; 0x038],
    /// 0x078c - `DBGSYS_BGR_REG, dbgsys_clk, reset map`.
    pub dbgsys_bgr: RW<SingleBusGatingReset>,
    _reserved_0790: [u8; 0x01c],
    /// 0x07ac - `PWM_BGR_REG, pwm1_clk, pwm_clk, ...`.
    pub pwm_bgr: RW<SingleBusGatingReset>,
    /// 0x07b0 - `iommu_clk`.
    pub iommu: RW<u32>,
    _reserved_07b4: [u8; 0x008],
    /// 0x07bc - `CCU_IOMMU_BGR_REG, IOMMU_BGR_REG, bus_iommu_clk`.
    pub iommu_bgr: RW<u32>,
    _reserved_07c0: [u8; 0x040],
    /// 0x0800 - `CCU_DRAM_CLK_REG, DRAM_CLK_REG, dram_clk`.
    pub dram_clk: RW<u32>,
    /// 0x0804 - `CCU_MBUS_MAT_CLK_GATING_REG, MBUS_MAT_CLK_GATING_REG, ce_mbus_gate_clk, ...`.
    pub mbus_mat_clk_gating: RW<u32>,
    /// 0x0808 - `CCU_PLL_DDR_AUX_REG`.
    pub pll_ddr_aux: RW<u32>,
    /// 0x080c - `CCU_DRAM_BGR_REG, DRAM_BGR_REG, bus_dram_clk, ...`.
    pub dram_bgr: RW<SingleBusGatingReset>,
    /// 0x0810 - `CCU_NAND_CLK_REG, NAND0_CLK0_CLK_REG, nand0_clk0_clk`.
    pub nand0_clk0_clk: RW<u32>,
    /// 0x0814 - `NAND0_CLK1_CLK_REG, nand0_clk1_clk`.
    pub nand0_clk1_clk: RW<u32>,
    _reserved_0818: [u8; 0x014],
    /// 0x082c - `CCU_NAND_BGR_REG, NAND_BGR_REG, nand0_clk, ...`.
    pub nand_bgr: RW<SingleBusGatingReset>,
    /// 0x0830 - `CCU_SMHC0_CLK_REG, SMHC0_CLK_REG, smhc0_clk`.
    pub smhc0_clk: RW<u32>,
    /// 0x0834 - `CCU_SMHC1_CLK_REG, SMHC1_CLK_REG, smhc1_clk`.
    pub smhc1_clk: RW<u32>,
    /// 0x0838 - `CCU_SMHC2_CLK_REG, SMHC2_CLK_REG, smhc2_clk`.
    pub smhc2_clk: RW<u32>,
    _reserved_083c: [u8; 0x010],
    /// 0x084c - `CCU_SMHC_BGR_REG, SMHC_BGR_REG, bus_smhc0_clk, ...`.
    pub smhc_bgr: RW<BusGatingReset<3>>,
    _reserved_0850: [u8; 0x03c],
    /// 0x088c - `SYSDAP_BGR_REG, reset map, sysdap_clk`.
    pub sysdap_bgr: RW<SingleBusGatingReset>,
    _reserved_0890: [u8; 0x07c],
    /// 0x090c - `CCU_UART_BGR_REG, UART_BGR_REG, bus_uart0_clk, ...`.
    pub uart_bgr: RW<BusGatingReset<8>>,
    _reserved_0910: [u8; 0x00c],
    /// 0x091c - `CCU_TWI_BGR_REG, TWI_BGR_REG, reset map, ...`.
    pub twi_bgr: RW<BusGatingReset<6>>,
    _reserved_0920: [u8; 0x00c],
    /// 0x092c - `CAN_BGR_REG`.
    pub can_bgr: RW<SingleBusGatingReset>,
    _reserved_0930: [u8; 0x00c],
    /// 0x093c - `CCU_SCR_BGR_REG`.
    pub scr_bgr: RW<u32>,
    /// 0x0940 - `CCU_SPI0_CLK_REG, spi0_clk`.
    pub spi0_clk: RW<u32>,
    /// 0x0944 - `CCU_SPI1_CLK_REG, SPI1_CLK_REG, spi1_clk`.
    pub spi1_clk: RW<u32>,
    /// 0x0948 - `SPI2_CLK_REG, spi2_clk`.
    pub spi2_clk: RW<u32>,
    _reserved_094c: [u8; 0x004],
    /// 0x0950 - `SPIF_CLK_REG, spif_clk`.
    pub spif_clk: RW<u32>,
    _reserved_0954: [u8; 0x018],
    /// 0x096c - `CCU_SPI_BGR_REG, SPI_BGR_REG, bus_spi0_clk, ...`.
    pub spi_bgr: RW<BusGatingReset<3>>,
    /// 0x0970 - `GMAC0_25M_CLK_REG, gmac0_25m_clk`.
    pub gmac0_25m_clk: RW<u32>,
    /// 0x0974 - `GMAC1_25M_CLK_REG, gmac1_25m_clk`.
    pub gmac1_25m_clk: RW<u32>,
    _reserved_0978: [u8; 0x004],
    /// 0x097c - `GMAC_BGR_REG, gmac0_clk, reset map`.
    pub gmac_bgr: RW<SingleBusGatingReset>,
    _reserved_0980: [u8; 0x00c],
    /// 0x098c - `gmac1_clk`.
    pub gmac1: RW<u32>,
    /// 0x0990 - `IRRX_CLK_REG, irrx_clk`.
    pub irrx_clk: RW<u32>,
    _reserved_0994: [u8; 0x008],
    /// 0x099c - `IRRX_BGR_REG, bus_irrx_clk, reset map`.
    pub irrx_bgr: RW<SingleBusGatingReset>,
    _reserved_09a0: [u8; 0x020],
    /// 0x09c0 - `IRTX_CLK_REG, irtx_clk`.
    pub irtx_clk: RW<u32>,
    _reserved_09c4: [u8; 0x008],
    /// 0x09cc - `IRTX_BGR_REG, bus_irtx_clk, reset map`.
    pub irtx_bgr: RW<SingleBusGatingReset>,
    _reserved_09d0: [u8; 0x010],
    /// 0x09e0 - `CCU_GPADC_CLK_REG, GPADC_24M_CLK_REG, gpadc0_24m_clk`.
    pub gpadc_24m_clk: RW<u32>,
    /// 0x09e4 - `gpadc1_24m_clk`.
    pub gpadc1_24m: RW<u32>,
    _reserved_09e8: [u8; 0x004],
    /// 0x09ec - `CCU_GPADC_BGR_REG, GPADC_BGR_REG, bus_gpadc0_clk, ...`.
    pub gpadc_bgr: RW<SingleBusGatingReset>,
    _reserved_09f0: [u8; 0x00c],
    /// 0x09fc - `THS_BGR_REG, reset map, ths_clk`.
    pub ths_bgr: RW<SingleBusGatingReset>,
    _reserved_0a00: [u8; 0x070],
    /// 0x0a70 - `CCU_USB0_CLK_REG, SUN55IW3_USB0_CTRL_REG, USB0_CLK_REG, ...`.
    pub usb0_ctrl: RW<u32>,
    /// 0x0a74 - `SUN55IW3_USB1_CTRL_REG, USB1_CLK_REG, reset map, ...`.
    pub usb1_ctrl: RW<u32>,
    /// 0x0a78 - `USB2_REF_CLK_REG`.
    pub usb2_ref_clk: RW<u32>,
    /// 0x0a7c - `USB2_SUSPEND_CLK_REG`.
    pub usb2_suspend_clk: RW<u32>,
    /// 0x0a80 - `usb2_ref_clk`.
    pub usb2_ref: RW<u32>,
    /// 0x0a84 - `usb3_ref_clk`.
    pub usb3_ref: RW<u32>,
    /// 0x0a88 - `usb3_suspend_clk`.
    pub usb3_suspend: RW<u32>,
    /// 0x0a8c - `CCU_USB_BGR_REG, USB_BGR_REG, reset map, ...`.
    pub usb_bgr: RW<SingleBusGatingReset>,
    _reserved_0a90: [u8; 0x00c],
    /// 0x0a9c - `CCU_LRADC_BGR_REG, LRADC_BGR_REG, lradc_clk, ...`.
    pub lradc_bgr: RW<SingleBusGatingReset>,
    /// 0x0aa0 - `PCIE_AUX_CLK_REG, pcie_aux_clk`.
    pub pcie_aux_clk: RW<u32>,
    /// 0x0aa4 - `PCIE_REF_CLK_REG`.
    pub pcie_ref_clk: RW<u32>,
    _reserved_0aa8: [u8; 0x004],
    /// 0x0aac - `PCIE_BGR_REG, reset map`.
    pub pcie_bgr: RW<SingleBusGatingReset>,
    _reserved_0ab0: [u8; 0x00c],
    /// 0x0abc - `DPSS_TOP0_BGR_REG, dpss_top0_clk, reset map`.
    pub dpss_top0_bgr: RW<SingleBusGatingReset>,
    _reserved_0ac0: [u8; 0x00c],
    /// 0x0acc - `DPSS_TOP1_BGR_REG, dpss_top1_clk, reset map`.
    pub dpss_top1_bgr: RW<SingleBusGatingReset>,
    _reserved_0ad0: [u8; 0x034],
    /// 0x0b04 - `HDMI_24M_CLK_REG, hdmi_24m_clk`.
    pub hdmi_24m_clk: RW<u32>,
    _reserved_0b08: [u8; 0x008],
    /// 0x0b10 - `HDMI_CEC_CLK_REG, hdmi_cec_clk`.
    pub hdmi_cec_clk: RW<u32>,
    _reserved_0b14: [u8; 0x008],
    /// 0x0b1c - `HDMI_BGR_REG, hdmi_clk, reset map`.
    pub hdmi_bgr: RW<SingleBusGatingReset>,
    _reserved_0b20: [u8; 0x004],
    /// 0x0b24 - `DSI0_CLK_REG, dsi0_clk`.
    pub dsi0_clk: RW<u32>,
    /// 0x0b28 - `DSI1_CLK_REG, dsi1_clk`.
    pub dsi1_clk: RW<u32>,
    _reserved_0b2c: [u8; 0x020],
    /// 0x0b4c - `DSI_BGR_REG, bus_dsi0_clk, bus_dsi1_clk, ...`.
    pub dsi_bgr: RW<SingleBusGatingReset>,
    _reserved_0b50: [u8; 0x010],
    /// 0x0b60 - `VO0_TCONLCD0_CLK_REG, vo0_tconlcd0_clk`.
    pub vo0_tconlcd0_clk: RW<u32>,
    /// 0x0b64 - `VO0_TCONLCD1_CLK_REG, vo0_tconlcd1_clk`.
    pub vo0_tconlcd1_clk: RW<u32>,
    /// 0x0b68 - `VO1_TCONLCD0_CLK_REG, vo1_tconlcd0_clk`.
    pub vo1_tconlcd0_clk: RW<u32>,
    /// 0x0b6c - `COMBPHY0_CLK_REG, combphy0_clk`.
    pub combphy0_clk: RW<u32>,
    /// 0x0b70 - `COMBPHY1_CLK_REG, combphy1_clk`.
    pub combphy1_clk: RW<u32>,
    _reserved_0b74: [u8; 0x008],
    /// 0x0b7c - `TCONLCD_BGR_REG, bus_vo0_tconlcd0_clk, bus_vo0_tconlcd1_clk, ...`.
    pub tconlcd_bgr: RW<SingleBusGatingReset>,
    /// 0x0b80 - `TCONTV_CLK_REG, tcontv_clk`.
    pub tcontv_clk: RW<u32>,
    /// 0x0b84 - `tcontv1_clk`.
    pub tcontv1: RW<u32>,
    _reserved_0b88: [u8; 0x014],
    /// 0x0b9c - `TCONTV_BGR_REG, bus_tcontv1_clk, bus_tcontv_clk, ...`.
    pub tcontv_bgr: RW<SingleBusGatingReset>,
    _reserved_0ba0: [u8; 0x00c],
    /// 0x0bac - `LVDS_BGR_REG, reset map`.
    pub lvds_bgr: RW<u32>,
    /// 0x0bb0 - `edp_clk`.
    pub edp: RW<u32>,
    _reserved_0bb4: [u8; 0x008],
    /// 0x0bbc - `bus_edp_clk, reset map`.
    pub bus_edp: RW<u32>,
    _reserved_0bc0: [u8; 0x030],
    /// 0x0bf0 - `LEDC_CLK_REG, ledc_clk`.
    pub ledc_clk: RW<u32>,
    _reserved_0bf4: [u8; 0x008],
    /// 0x0bfc - `LEDC_BGR_REG, bus_ledc_clk, reset map`.
    pub ledc_bgr: RW<SingleBusGatingReset>,
    _reserved_0c00: [u8; 0x004],
    /// 0x0c04 - `CSI_CLK_REG, csi_clk`.
    pub csi_clk: RW<u32>,
    /// 0x0c08 - `CSI_MASTER0_CLK_REG, csi_master0_clk`.
    pub csi_master0_clk: RW<u32>,
    /// 0x0c0c - `CSI_MASTER1_CLK_REG, csi_master1_clk`.
    pub csi_master1_clk: RW<u32>,
    /// 0x0c10 - `CSI_MASTER2_CLK_REG, csi_master2_clk`.
    pub csi_master2_clk: RW<u32>,
    /// 0x0c14 - `CSI_MASTER3_CLK_REG, csi_master3_clk`.
    pub csi_master3_clk: RW<u32>,
    _reserved_0c18: [u8; 0x004],
    /// 0x0c1c - `CSI_BGR_REG, bus_csi_clk, reset map`.
    pub csi_bgr: RW<SingleBusGatingReset>,
    /// 0x0c20 - `ISP_CLK_REG, isp_clk`.
    pub isp_clk: RW<u32>,
    _reserved_0c24: [u8; 0x008],
    /// 0x0c2c - `ISP_BGR_REG, reset map`.
    pub isp_bgr: RW<u32>,
    _reserved_0c30: [u8; 0x040],
    /// 0x0c70 - `DSP_CLK_REG, dsp_clk`.
    pub dsp_clk: RW<u32>,
    _reserved_0c74: [u8; 0x190],
    /// 0x0e04 - `AHB_GATE_EN_REG, cpus_hclk_gate_clk`.
    pub ahb_gate_en: RW<u32>,
    /// 0x0e08 - `PERI0PLL_GATE_EN_REG`.
    pub peri0pll_gate_en: RW<u32>,
    /// 0x0e0c - `CLK24M_GATE_EN_REG, usb_24m_clk`.
    pub clk24m_gate_en: RW<u32>,
    _reserved_0e10: [u8; 0x0f0],
    /// 0x0f00 - `CCU_SEC_SWITCH_REG`.
    pub sec_switch: RW<u32>,
    /// 0x0f04 - `PLL_LOCK_DBG_CTRL_REG`.
    pub pll_lock_dbg_ctrl: RW<u32>,
    /// 0x0f08 - `SYSDAP_REQ_CTRL_REG`.
    pub sysdap_req_ctrl: RW<u32>,
    _reserved_0f0c: [u8; 0x024],
    /// 0x0f30 - `CCU_FAN_GATE_REG, fanout_12m_clk, fanout_16m_clk, ...`.
    pub fan_gate: RW<u32>,
    /// 0x0f34 - `CLK27M_FAN_REG, clk27m_fanout_clk`.
    pub clk27m_fan: RW<u32>,
    /// 0x0f38 - `CLK_FAN_REG, clk_fanout_clk`.
    pub clk_fan: RW<u32>,
    /// 0x0f3c - `CCU_FAN_REG, fanout0_clk, fanout1_clk, ...`.
    pub fan: RW<u32>,
    /// 0x0f40 - `PLL_CFG0_REG`.
    pub pll_cfg0: RW<u32>,
    /// 0x0f44 - `PLL_CFG1_REG`.
    pub pll_cfg1: RW<u32>,
    /// 0x0f48 - `PLL_CFG2_REG`.
    pub pll_cfg2: RW<u32>,
    _reserved_0f4c: [u8; 0x0a4],
    /// 0x0ff0 - `CCU_VERSION_REG`.
    pub version: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll_cpu0_ctrl), 0x000);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu1_ctrl), 0x004);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu2_ctrl), 0x008);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_ctrl), 0x010);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr1_ctrl), 0x018);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_ctrl), 0x020);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_ctrl), 0x028);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_ctrl), 0x030);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_ctrl), 0x040);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_ctrl), 0x048);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_ctrl), 0x050);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_ctrl), 0x058);
        assert_eq!(offset_of!(RegisterBlock, pll_video3_ctrl), 0x068);
        assert_eq!(offset_of!(RegisterBlock, pll_hsic_ctrl), 0x070);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_ctrl), 0x078);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_ctrl), 0x080);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat0_ctrl), 0x110);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat1_ctrl), 0x114);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat0_ctrl), 0x120);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat1_ctrl), 0x124);
        assert_eq!(offset_of!(RegisterBlock, pll_periph1_pattern0), 0x128);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_pat1_ctrl), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_pat0_ctrl), 0x130);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_pat1_ctrl), 0x134);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat0_ctrl), 0x140);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat1_ctrl), 0x144);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat0_ctrl), 0x148);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat1_ctrl), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_pat0_ctrl), 0x150);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_pat1_ctrl), 0x154);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_pat0_ctrl), 0x158);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_pat1_ctrl), 0x15c);
        assert_eq!(offset_of!(RegisterBlock, pll_video3_pat0_ctrl), 0x168);
        assert_eq!(offset_of!(RegisterBlock, pll_video3_pat1_ctrl), 0x16c);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_pat0_ctrl), 0x178);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_pat1_ctrl), 0x17c);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_pat0_ctrl), 0x180);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_pat1_ctrl), 0x184);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu0_bias), 0x300);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu1_bias), 0x308);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu2_bias), 0x30c);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_bias), 0x310);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_bias), 0x320);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_bias), 0x328);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_bias), 0x330);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_bias), 0x340);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_bias), 0x348);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_bias), 0x350);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_bias), 0x358);
        assert_eq!(offset_of!(RegisterBlock, pll_video3_bias), 0x368);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_bias), 0x378);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_bias), 0x380);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu0_tun), 0x400);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu1_tun), 0x408);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu2_tun), 0x40c);
        assert_eq!(offset_of!(RegisterBlock, cpu_clk), 0x500);
        assert_eq!(offset_of!(RegisterBlock, cpu_gating), 0x504);
        assert_eq!(offset_of!(RegisterBlock, trace_clk), 0x508);
        assert_eq!(offset_of!(RegisterBlock, dsu_clk), 0x50c);
        assert_eq!(offset_of!(RegisterBlock, ahb0_cfg), 0x510);
        assert_eq!(offset_of!(RegisterBlock, apb0_cfg), 0x520);
        assert_eq!(offset_of!(RegisterBlock, apb1_cfg), 0x524);
        assert_eq!(offset_of!(RegisterBlock, mbus_cfg), 0x540);
        assert_eq!(offset_of!(RegisterBlock, nsi_bgr), 0x54c);
        assert_eq!(offset_of!(RegisterBlock, gic_clk), 0x550);
        assert_eq!(offset_of!(RegisterBlock, de0_clk), 0x600);
        assert_eq!(offset_of!(RegisterBlock, de_bgr), 0x60c);
        assert_eq!(offset_of!(RegisterBlock, di_clk), 0x620);
        assert_eq!(offset_of!(RegisterBlock, di_bgr), 0x62c);
        assert_eq!(offset_of!(RegisterBlock, g2d_clk), 0x630);
        assert_eq!(offset_of!(RegisterBlock, g2d_bgr), 0x63c);
        assert_eq!(offset_of!(RegisterBlock, gpu_core_clk), 0x670);
        assert_eq!(offset_of!(RegisterBlock, gpu_gating), 0x67c);
        assert_eq!(offset_of!(RegisterBlock, ce_clk), 0x680);
        assert_eq!(offset_of!(RegisterBlock, ce_bgr), 0x68c);
        assert_eq!(offset_of!(RegisterBlock, ve_clk), 0x690);
        assert_eq!(offset_of!(RegisterBlock, ve_bgr), 0x69c);
        assert_eq!(offset_of!(RegisterBlock, npu_clk), 0x6e0);
        assert_eq!(offset_of!(RegisterBlock, dma_bgr), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, msgbox_bgr), 0x71c);
        assert_eq!(offset_of!(RegisterBlock, spinlock_bgr), 0x72c);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk), 0x730);
        assert_eq!(offset_of!(RegisterBlock, timer1_clk), 0x734);
        assert_eq!(offset_of!(RegisterBlock, timer2_clk), 0x738);
        assert_eq!(offset_of!(RegisterBlock, timer3_clk), 0x73c);
        assert_eq!(offset_of!(RegisterBlock, timer4_clk), 0x740);
        assert_eq!(offset_of!(RegisterBlock, timer5_clk), 0x744);
        assert_eq!(offset_of!(RegisterBlock, timer_bgr), 0x74c);
        assert_eq!(offset_of!(RegisterBlock, avs_clk), 0x750);
        assert_eq!(offset_of!(RegisterBlock, dbgsys_bgr), 0x78c);
        assert_eq!(offset_of!(RegisterBlock, pwm_bgr), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, iommu), 0x7b0);
        assert_eq!(offset_of!(RegisterBlock, iommu_bgr), 0x7bc);
        assert_eq!(offset_of!(RegisterBlock, dram_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gating), 0x804);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_aux), 0x808);
        assert_eq!(offset_of!(RegisterBlock, dram_bgr), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, nand0_clk0_clk), 0x810);
        assert_eq!(offset_of!(RegisterBlock, nand0_clk1_clk), 0x814);
        assert_eq!(offset_of!(RegisterBlock, nand_bgr), 0x82c);
        assert_eq!(offset_of!(RegisterBlock, smhc0_clk), 0x830);
        assert_eq!(offset_of!(RegisterBlock, smhc1_clk), 0x834);
        assert_eq!(offset_of!(RegisterBlock, smhc2_clk), 0x838);
        assert_eq!(offset_of!(RegisterBlock, smhc_bgr), 0x84c);
        assert_eq!(offset_of!(RegisterBlock, sysdap_bgr), 0x88c);
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, twi_bgr), 0x91c);
        assert_eq!(offset_of!(RegisterBlock, can_bgr), 0x92c);
        assert_eq!(offset_of!(RegisterBlock, scr_bgr), 0x93c);
        assert_eq!(offset_of!(RegisterBlock, spi0_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi1_clk), 0x944);
        assert_eq!(offset_of!(RegisterBlock, spi2_clk), 0x948);
        assert_eq!(offset_of!(RegisterBlock, spif_clk), 0x950);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
        assert_eq!(offset_of!(RegisterBlock, gmac0_25m_clk), 0x970);
        assert_eq!(offset_of!(RegisterBlock, gmac1_25m_clk), 0x974);
        assert_eq!(offset_of!(RegisterBlock, gmac_bgr), 0x97c);
        assert_eq!(offset_of!(RegisterBlock, gmac1), 0x98c);
        assert_eq!(offset_of!(RegisterBlock, irrx_clk), 0x990);
        assert_eq!(offset_of!(RegisterBlock, irrx_bgr), 0x99c);
        assert_eq!(offset_of!(RegisterBlock, irtx_clk), 0x9c0);
        assert_eq!(offset_of!(RegisterBlock, irtx_bgr), 0x9cc);
        assert_eq!(offset_of!(RegisterBlock, gpadc_24m_clk), 0x9e0);
        assert_eq!(offset_of!(RegisterBlock, gpadc1_24m), 0x9e4);
        assert_eq!(offset_of!(RegisterBlock, gpadc_bgr), 0x9ec);
        assert_eq!(offset_of!(RegisterBlock, ths_bgr), 0x9fc);
        assert_eq!(offset_of!(RegisterBlock, usb0_ctrl), 0xa70);
        assert_eq!(offset_of!(RegisterBlock, usb1_ctrl), 0xa74);
        assert_eq!(offset_of!(RegisterBlock, usb2_ref_clk), 0xa78);
        assert_eq!(offset_of!(RegisterBlock, usb2_suspend_clk), 0xa7c);
        assert_eq!(offset_of!(RegisterBlock, usb2_ref), 0xa80);
        assert_eq!(offset_of!(RegisterBlock, usb3_ref), 0xa84);
        assert_eq!(offset_of!(RegisterBlock, usb3_suspend), 0xa88);
        assert_eq!(offset_of!(RegisterBlock, usb_bgr), 0xa8c);
        assert_eq!(offset_of!(RegisterBlock, lradc_bgr), 0xa9c);
        assert_eq!(offset_of!(RegisterBlock, pcie_aux_clk), 0xaa0);
        assert_eq!(offset_of!(RegisterBlock, pcie_ref_clk), 0xaa4);
        assert_eq!(offset_of!(RegisterBlock, pcie_bgr), 0xaac);
        assert_eq!(offset_of!(RegisterBlock, dpss_top0_bgr), 0xabc);
        assert_eq!(offset_of!(RegisterBlock, dpss_top1_bgr), 0xacc);
        assert_eq!(offset_of!(RegisterBlock, hdmi_24m_clk), 0xb04);
        assert_eq!(offset_of!(RegisterBlock, hdmi_cec_clk), 0xb10);
        assert_eq!(offset_of!(RegisterBlock, hdmi_bgr), 0xb1c);
        assert_eq!(offset_of!(RegisterBlock, dsi0_clk), 0xb24);
        assert_eq!(offset_of!(RegisterBlock, dsi1_clk), 0xb28);
        assert_eq!(offset_of!(RegisterBlock, dsi_bgr), 0xb4c);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd0_clk), 0xb60);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd1_clk), 0xb64);
        assert_eq!(offset_of!(RegisterBlock, vo1_tconlcd0_clk), 0xb68);
        assert_eq!(offset_of!(RegisterBlock, combphy0_clk), 0xb6c);
        assert_eq!(offset_of!(RegisterBlock, combphy1_clk), 0xb70);
        assert_eq!(offset_of!(RegisterBlock, tconlcd_bgr), 0xb7c);
        assert_eq!(offset_of!(RegisterBlock, tcontv_clk), 0xb80);
        assert_eq!(offset_of!(RegisterBlock, tcontv1), 0xb84);
        assert_eq!(offset_of!(RegisterBlock, tcontv_bgr), 0xb9c);
        assert_eq!(offset_of!(RegisterBlock, lvds_bgr), 0xbac);
        assert_eq!(offset_of!(RegisterBlock, edp), 0xbb0);
        assert_eq!(offset_of!(RegisterBlock, bus_edp), 0xbbc);
        assert_eq!(offset_of!(RegisterBlock, ledc_clk), 0xbf0);
        assert_eq!(offset_of!(RegisterBlock, ledc_bgr), 0xbfc);
        assert_eq!(offset_of!(RegisterBlock, csi_clk), 0xc04);
        assert_eq!(offset_of!(RegisterBlock, csi_master0_clk), 0xc08);
        assert_eq!(offset_of!(RegisterBlock, csi_master1_clk), 0xc0c);
        assert_eq!(offset_of!(RegisterBlock, csi_master2_clk), 0xc10);
        assert_eq!(offset_of!(RegisterBlock, csi_master3_clk), 0xc14);
        assert_eq!(offset_of!(RegisterBlock, csi_bgr), 0xc1c);
        assert_eq!(offset_of!(RegisterBlock, isp_clk), 0xc20);
        assert_eq!(offset_of!(RegisterBlock, isp_bgr), 0xc2c);
        assert_eq!(offset_of!(RegisterBlock, dsp_clk), 0xc70);
        assert_eq!(offset_of!(RegisterBlock, ahb_gate_en), 0xe04);
        assert_eq!(offset_of!(RegisterBlock, peri0pll_gate_en), 0xe08);
        assert_eq!(offset_of!(RegisterBlock, clk24m_gate_en), 0xe0c);
        assert_eq!(offset_of!(RegisterBlock, sec_switch), 0xf00);
        assert_eq!(offset_of!(RegisterBlock, pll_lock_dbg_ctrl), 0xf04);
        assert_eq!(offset_of!(RegisterBlock, sysdap_req_ctrl), 0xf08);
        assert_eq!(offset_of!(RegisterBlock, fan_gate), 0xf30);
        assert_eq!(offset_of!(RegisterBlock, clk27m_fan), 0xf34);
        assert_eq!(offset_of!(RegisterBlock, clk_fan), 0xf38);
        assert_eq!(offset_of!(RegisterBlock, fan), 0xf3c);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg0), 0xf40);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg1), 0xf44);
        assert_eq!(offset_of!(RegisterBlock, pll_cfg2), 0xf48);
        assert_eq!(offset_of!(RegisterBlock, version), 0xff0);
        assert_eq!(size_of::<RegisterBlock>(), 0xff4);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
