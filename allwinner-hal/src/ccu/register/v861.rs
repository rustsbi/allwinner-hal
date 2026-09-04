//! V861 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun252iw1` platform.

use super::{BusGatingReset, SingleBusGatingReset};
use volatile_register::{RO, RW};

/// V861 main CCU register block.
#[doc(alias = "sun252iw1")]
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - `PLL_CPU_CTRL_REG, pll_cpu_clk`.
    pub pll_cpu_ctrl: RW<u32>,
    /// 0x0004 - `PLL_CPU_CTRL1_REG`.
    pub pll_cpu_ctrl1: RW<u32>,
    _reserved_0008: [u8; 0x008],
    /// 0x0010 - `PLL_DDR_CTRL_REG, SUN252IW1_PLL_DDR_CTRL_REG, pll_ddr_clk`.
    pub pll_ddr_ctrl: RW<u32>,
    _reserved_0014: [u8; 0x00c],
    /// 0x0020 - `PLL_PERI_CTRL_REG, SUN252IW1_PLL_PERI_CTRL_REG, pll_peri_2x_clk, ...`.
    pub pll_peri_ctrl: RW<u32>,
    _reserved_0024: [u8; 0x01c],
    /// 0x0040 - `PLL_VIDEO_CTRL_REG, SUN252IW1_PLL_VIDEO_CTRL_REG`.
    pub pll_video_ctrl: RW<u32>,
    _reserved_0044: [u8; 0x004],
    /// 0x0048 - `PLL_CSI_CTRL_REG, SUN252IW1_PLL_CSI_CTRL_REG`.
    pub pll_csi_ctrl: RW<u32>,
    _reserved_004c: [u8; 0x02c],
    /// 0x0078 - `PLL_AUDIO_CTRL_REG, SUN252IW1_PLL_AUDIO_CTRL_REG`.
    pub pll_audio_ctrl: RW<u32>,
    _reserved_007c: [u8; 0x004],
    /// 0x0080 - `PLL_NPU_CTRL_REG`.
    pub pll_npu_ctrl: RW<u32>,
    _reserved_0084: [u8; 0x07c],
    /// 0x0100 - `PLL_CPU_PAT0_CTRL_REG`.
    pub pll_cpu_pat0_ctrl: RW<u32>,
    /// 0x0104 - `PLL_CPU_PAT1_CTRL_REG`.
    pub pll_cpu_pat1_ctrl: RW<u32>,
    _reserved_0108: [u8; 0x008],
    /// 0x0110 - `PLL_DDR_PAT0_CTRL_REG`.
    pub pll_ddr_pat0_ctrl: RW<u32>,
    /// 0x0114 - `PLL_DDR_PAT1_CTRL_REG`.
    pub pll_ddr_pat1_ctrl: RW<u32>,
    _reserved_0118: [u8; 0x008],
    /// 0x0120 - `PLL_PERI_PAT0_CTRL_REG`.
    pub pll_peri_pat0_ctrl: RW<u32>,
    /// 0x0124 - `PLL_PERI_PAT1_CTRL_REG`.
    pub pll_peri_pat1_ctrl: RW<u32>,
    _reserved_0128: [u8; 0x018],
    /// 0x0140 - `PLL_VIDEO_PAT0_CTRL_REG`.
    pub pll_video_pat0_ctrl: RW<u32>,
    /// 0x0144 - `PLL_VIDEO_PAT1_CTRL_REG`.
    pub pll_video_pat1_ctrl: RW<u32>,
    /// 0x0148 - `PLL_CSI_PAT0_CTRL_REG`.
    pub pll_csi_pat0_ctrl: RW<u32>,
    /// 0x014c - `PLL_CSI_PAT1_CTRL_REG`.
    pub pll_csi_pat1_ctrl: RW<u32>,
    _reserved_0150: [u8; 0x028],
    /// 0x0178 - `PLL_AUDIO_PAT0_CTRL_REG`.
    pub pll_audio_pat0_ctrl: RW<u32>,
    /// 0x017c - `PLL_AUDIO_PAT1_CTRL_REG`.
    pub pll_audio_pat1_ctrl: RW<u32>,
    /// 0x0180 - `PLL_NPU_PAT0_CTRL_REG`.
    pub pll_npu_pat0_ctrl: RW<u32>,
    /// 0x0184 - `PLL_NPU_PAT1_CTRL_REG`.
    pub pll_npu_pat1_ctrl: RW<u32>,
    _reserved_0188: [u8; 0x078],
    /// 0x0200 - `PLL_CPU_SSC_REG, reset map`.
    pub pll_cpu_ssc: RW<u32>,
    _reserved_0204: [u8; 0x0fc],
    /// 0x0300 - `PLL_CPU_BIAS_REG`.
    pub pll_cpu_bias: RW<u32>,
    _reserved_0304: [u8; 0x00c],
    /// 0x0310 - `PLL_DDR_BIAS_REG`.
    pub pll_ddr_bias: RW<u32>,
    _reserved_0314: [u8; 0x00c],
    /// 0x0320 - `PLL_PERI_BIAS_REG`.
    pub pll_peri_bias: RW<u32>,
    _reserved_0324: [u8; 0x01c],
    /// 0x0340 - `PLL_VIDEO_BIAS_REG`.
    pub pll_video_bias: RW<u32>,
    _reserved_0344: [u8; 0x004],
    /// 0x0348 - `PLL_CSI_BIAS_REG`.
    pub pll_csi_bias: RW<u32>,
    _reserved_034c: [u8; 0x02c],
    /// 0x0378 - `PLL_AUDIO_BIAS_REG`.
    pub pll_audio_bias: RW<u32>,
    _reserved_037c: [u8; 0x004],
    /// 0x0380 - `PLL_NPU_BIAS_REG`.
    pub pll_npu_bias: RW<u32>,
    _reserved_0384: [u8; 0x07c],
    /// 0x0400 - `PLL_CPU_TUN1_REG`.
    pub pll_cpu_tun1: RW<u32>,
    _reserved_0404: [u8; 0x0f8],
    /// 0x04fc - `IPMC_CLK_REG, ipmc_clk`.
    pub ipmc_clk: RW<u32>,
    /// 0x0500 - `CPU_CLK_REG`.
    pub cpu_clk: RW<u32>,
    /// 0x0504 - `CPU_GATING_REG, cpu_clk, cpu_gatin_clk, ...`.
    pub cpu_gating: RW<u32>,
    /// 0x0508 - `PIC_CLK_REG, pic_clk`.
    pub pic_clk: RW<u32>,
    /// 0x050c - `CPU_CFG_BGR_REG, c907_aximon_clk, cpu_cfg_clk, ...`.
    pub cpu_cfg_bgr: RW<SingleBusGatingReset>,
    /// 0x0510 - `AHB_CLK_REG, ahb_clk`.
    pub ahb_clk: RW<u32>,
    _reserved_0514: [u8; 0x00c],
    /// 0x0520 - `APB0_CLK_REG, apb0_clk`.
    pub apb0_clk: RW<u32>,
    /// 0x0524 - `APB1_CLK_REG, apb1_clk`.
    pub apb1_clk: RW<u32>,
    /// 0x0528 - `APB_UART_CLK_REG, apb_uart_clk`.
    pub apb_uart_clk: RW<u32>,
    _reserved_052c: [u8; 0x014],
    /// 0x0540 - `MBUS_CLK_REG, reset map`.
    pub mbus_clk: RW<u32>,
    _reserved_0544: [u8; 0x0bc],
    /// 0x0600 - `DE_CLK_REG, de_clk`.
    pub de_clk: RW<u32>,
    _reserved_0604: [u8; 0x008],
    /// 0x060c - `DE_BGR_REG, de_bus_clk, reset map`.
    pub de_bgr: RW<SingleBusGatingReset>,
    _reserved_0610: [u8; 0x020],
    /// 0x0630 - `G2D_CLK_REG, g2d_clk`.
    pub g2d_clk: RW<u32>,
    _reserved_0634: [u8; 0x008],
    /// 0x063c - `G2D_BGR_REG, g2d_bus_clk, reset map`.
    pub g2d_bgr: RW<SingleBusGatingReset>,
    _reserved_0640: [u8; 0x040],
    /// 0x0680 - `CE_CLK_REG, ce_clk`.
    pub ce_clk: RW<u32>,
    _reserved_0684: [u8; 0x008],
    /// 0x068c - `CE_BGR_REG, ce_bus_clk, ce_sys_clk, ...`.
    pub ce_bgr: RW<SingleBusGatingReset>,
    /// 0x0690 - `VE_CLK_REG, ve_clk`.
    pub ve_clk: RW<u32>,
    _reserved_0694: [u8; 0x008],
    /// 0x069c - `VE_BGR_REG, reset map, ve_bus_clk`.
    pub ve_bgr: RW<SingleBusGatingReset>,
    _reserved_06a0: [u8; 0x040],
    /// 0x06e0 - `NPU_CLK_REG, npu_clk`.
    pub npu_clk: RW<u32>,
    /// 0x06e4 - `NPU_GATING_REG, reset map`.
    pub npu_gating: RW<u32>,
    _reserved_06e8: [u8; 0x004],
    /// 0x06ec - `NPU_BGR_REG, npu_bus_clk, reset map`.
    pub npu_bgr: RW<SingleBusGatingReset>,
    _reserved_06f0: [u8; 0x01c],
    /// 0x070c - `DMA_BGR_REG, ndma_clk, reset map, ...`.
    pub dma_bgr: RW<SingleBusGatingReset>,
    _reserved_0710: [u8; 0x00c],
    /// 0x071c - `MSGBOX_BGR_REG, msgbox0_clk, msgbox1_clk, ...`.
    pub msgbox_bgr: RW<SingleBusGatingReset>,
    _reserved_0720: [u8; 0x00c],
    /// 0x072c - `SPINLOCK_BGR_REG, reset map, spinlock_clk`.
    pub spinlock_bgr: RW<SingleBusGatingReset>,
    _reserved_0730: [u8; 0x00c],
    /// 0x073c - `HSTIMER_BGR_REG, hstimer_clk, reset map`.
    pub hstimer_bgr: RW<SingleBusGatingReset>,
    /// 0x0740 - `AVS_CLK_REG, avs_clk`.
    pub avs_clk: RW<u32>,
    /// 0x0744 - `TIMER_APB_CLK_REG, timer_apb_clk`.
    pub timer_apb_clk: RW<u32>,
    _reserved_0748: [u8; 0x004],
    /// 0x074c - `TIMER_BGR_REG, reset map`.
    pub timer_bgr: RW<u32>,
    _reserved_0750: [u8; 0x00c],
    /// 0x075c - `CAN_BGR_REG`.
    pub can_bgr: RW<SingleBusGatingReset>,
    _reserved_0760: [u8; 0x02c],
    /// 0x078c - `DBGSYS_BGR_REG, dbgsys_clk, reset map`.
    pub dbgsys_bgr: RW<SingleBusGatingReset>,
    _reserved_0790: [u8; 0x01c],
    /// 0x07ac - `PWM_BGR_REG, pwm_clk, reset map`.
    pub pwm_bgr: RW<SingleBusGatingReset>,
    _reserved_07b0: [u8; 0x00c],
    /// 0x07bc - `iommu_clk`.
    pub iommu: RW<u32>,
    _reserved_07c0: [u8; 0x040],
    /// 0x0800 - `DRAM_CLK_REG, dram_clk`.
    pub dram_clk: RW<u32>,
    /// 0x0804 - `MBUS_MAT_CLK_GATING_REG, ce_mbus_gate_clk, dma_mbus_en_clk, ...`.
    pub mbus_mat_clk_gating: RW<u32>,
    _reserved_0808: [u8; 0x004],
    /// 0x080c - `DRAM_BGR_REG, dram_bus_clk, reset map`.
    pub dram_bgr: RW<SingleBusGatingReset>,
    _reserved_0810: [u8; 0x020],
    /// 0x0830 - `SMHC0_CLK_REG, smhc0_clk`.
    pub smhc0_clk: RW<u32>,
    /// 0x0834 - `SMHC1_CLK_REG, smhc1_clk`.
    pub smhc1_clk: RW<u32>,
    /// 0x0838 - `SMHC2_CLK_REG`.
    pub smhc2_clk: RW<u32>,
    _reserved_083c: [u8; 0x010],
    /// 0x084c - `SMHC_BGR_REG, reset map, smhc0_bus_clk, ...`.
    pub smhc_bgr: RW<BusGatingReset<3>>,
    /// 0x0850 - `PSRAM_CLK_REG, lpsramctrl_opi_clk2x_clk`.
    pub psram_clk: RW<u32>,
    _reserved_0854: [u8; 0x008],
    /// 0x085c - `PSRAM_BGR_REG, psram_ctrl_clk, reset map`.
    pub psram_bgr: RW<SingleBusGatingReset>,
    _reserved_0860: [u8; 0x0ac],
    /// 0x090c - `UART_BGR_REG, reset map, uart0_clk, ...`.
    pub uart_bgr: RW<BusGatingReset<4>>,
    _reserved_0910: [u8; 0x00c],
    /// 0x091c - `TWI_BGR_REG, reset map, twi0_clk, ...`.
    pub twi_bgr: RW<BusGatingReset<5>>,
    _reserved_0920: [u8; 0x020],
    /// 0x0940 - `SPI0_CLK_REG, spi0_clk`.
    pub spi0_clk: RW<u32>,
    /// 0x0944 - `SPI1_CLK_REG, spi1_clk`.
    pub spi1_clk: RW<u32>,
    /// 0x0948 - `SPI2_CLK_REG`.
    pub spi2_clk: RW<u32>,
    _reserved_094c: [u8; 0x004],
    /// 0x0950 - `SPIF_CLK_REG, spif_clk`.
    pub spif_clk: RW<u32>,
    _reserved_0954: [u8; 0x018],
    /// 0x096c - `SPI_BGR_REG, reset map, spi0_bus_clk, ...`.
    pub spi_bgr: RW<BusGatingReset<2>>,
    /// 0x0970 - `GMAC_25M_CLK_REG, gmac_25m_clk, gmac_25m_clk_src_clk`.
    pub gmac_25m_clk: RW<u32>,
    _reserved_0974: [u8; 0x008],
    /// 0x097c - `GMAC_BGR_REG, gmac_clk, reset map`.
    pub gmac_bgr: RW<SingleBusGatingReset>,
    _reserved_0980: [u8; 0x06c],
    /// 0x09ec - `GPADC_BGR_REG, gpadc_clk, reset map`.
    pub gpadc_bgr: RW<SingleBusGatingReset>,
    _reserved_09f0: [u8; 0x00c],
    /// 0x09fc - `THS_BGR_REG, reset map, ths_clk`.
    pub ths_bgr: RW<SingleBusGatingReset>,
    _reserved_0a00: [u8; 0x010],
    /// 0x0a10 - `I2S0_CLK_REG, i2s0_clk`.
    pub i2s0_clk: RW<u32>,
    _reserved_0a14: [u8; 0x018],
    /// 0x0a2c - `I2S_BGR_REG, i2s0_bus_clk, reset map`.
    pub i2s_bgr: RW<SingleBusGatingReset>,
    _reserved_0a30: [u8; 0x020],
    /// 0x0a50 - `AUDIO_CODEC_DAC_CLK_REG, audio_codec_dac_clk`.
    pub audio_codec_dac_clk: RW<u32>,
    /// 0x0a54 - `AUDIO_CODEC_ADC_CLK_REG, audio_codec_adc_clk`.
    pub audio_codec_adc_clk: RW<u32>,
    _reserved_0a58: [u8; 0x004],
    /// 0x0a5c - `AUDIO_CODEC_BGR_REG, audio_codec_clk, reset map`.
    pub audio_codec_bgr: RW<SingleBusGatingReset>,
    _reserved_0a60: [u8; 0x010],
    /// 0x0a70 - `USB0_CLK_REG, reset map, usb_clk`.
    pub usb0_clk: RW<u32>,
    _reserved_0a74: [u8; 0x018],
    /// 0x0a8c - `USB_BGR_REG, reset map, usbehci0_clk, ...`.
    pub usb_bgr: RW<SingleBusGatingReset>,
    _reserved_0a90: [u8; 0x02c],
    /// 0x0abc - `DPSS_TOP_BGR_REG, dpss_top_clk, reset map`.
    pub dpss_top_bgr: RW<SingleBusGatingReset>,
    _reserved_0ac0: [u8; 0x0a0],
    /// 0x0b60 - `TCONLCD_CLK_REG, tconlcd_clk`.
    pub tconlcd_clk: RW<u32>,
    _reserved_0b64: [u8; 0x018],
    /// 0x0b7c - `TCONLCD_BGR_REG, reset map, tconlcd_bus_clk`.
    pub tconlcd_bgr: RW<SingleBusGatingReset>,
    _reserved_0b80: [u8; 0x084],
    /// 0x0c04 - `CSI_CLK_REG, csi_clk`.
    pub csi_clk: RW<u32>,
    /// 0x0c08 - `CSI_MASTER0_CLK_REG, csi_master0_clk`.
    pub csi_master0_clk: RW<u32>,
    /// 0x0c0c - `CSI_MASTER1_CLK_REG, csi_master1_clk`.
    pub csi_master1_clk: RW<u32>,
    /// 0x0c10 - `CSI_MASTER2_CLK_REG, csi_master2_clk`.
    pub csi_master2_clk: RW<u32>,
    _reserved_0c14: [u8; 0x018],
    /// 0x0c2c - `CSI_BGR_REG, csi_bus_clk, reset map`.
    pub csi_bgr: RW<SingleBusGatingReset>,
    _reserved_0c30: [u8; 0x0d0],
    /// 0x0d00 - `E907_CLK_REG`.
    pub e907_clk: RW<u32>,
    /// 0x0d04 - `E907_GATING_RST_REG, e907_clk, e907_gating_rs_clk, ...`.
    pub e907_gating_rst: RW<u32>,
    _reserved_0d08: [u8; 0x004],
    /// 0x0d0c - `RISCV_CFG_BGR_REG, e907_aximon_clk, reset map, ...`.
    pub riscv_cfg_bgr: RW<SingleBusGatingReset>,
    _reserved_0d10: [u8; 0x0f0],
    /// 0x0e00 - `PLL_PRE_DIV_REG`.
    pub pll_pre_div: RW<u32>,
    /// 0x0e04 - `AHB_GATE_EN_REG, cpus_hclk_gate_clk, gmac_ahb_gate_clk, ...`.
    pub ahb_gate_en: RW<u32>,
    /// 0x0e08 - `PERIPLL_GATE_EN_REG`.
    pub peripll_gate_en: RW<u32>,
    /// 0x0e0c - `CLK24M_GATE_EN_REG, gpadc_24m_clk, res_dcap_24m_clk, ...`.
    pub clk24m_gate_en: RW<u32>,
    /// 0x0e10 - `PLL_OPG_BYPASS_REG, pll_output_gate_clk`.
    pub pll_opg_bypass: RW<u32>,
    /// 0x0e14 - `AUDIOPLL_GATE_EN_REG`.
    pub audiopll_gate_en: RW<u32>,
    /// 0x0e18 - `VIDEOPLL_GATE_EN_REG`.
    pub videopll_gate_en: RW<u32>,
    /// 0x0e1c - `CSIPLL_GATE_EN_REG`.
    pub csipll_gate_en: RW<u32>,
    /// 0x0e20 - `DDRPLL_GATE_EN_REG`.
    pub ddrpll_gate_en: RW<u32>,
    /// 0x0e24 - `CPUPLL_GATE_EN_REG`.
    pub cpupll_gate_en: RW<u32>,
    /// 0x0e28 - `PERIPLL_GATE_STAT_REG`.
    pub peripll_gate_stat: RO<u32>,
    /// 0x0e2c - `AUDIOPLL_GATE_STAT_REG`.
    pub audiopll_gate_stat: RO<u32>,
    /// 0x0e30 - `VIDEOPLL_GATE_STAT_REG`.
    pub videopll_gate_stat: RO<u32>,
    /// 0x0e34 - `CSIPLL_GATE_STAT_REG`.
    pub csipll_gate_stat: RO<u32>,
    /// 0x0e38 - `DDRPLL_GATE_STAT_REG`.
    pub ddrpll_gate_stat: RO<u32>,
    /// 0x0e3c - `CPUPLL_GATE_STAT_REG`.
    pub cpupll_gate_stat: RO<u32>,
    /// 0x0e40 - `NPUPLL_GATE_EN_REG`.
    pub npupll_gate_en: RW<u32>,
    /// 0x0e44 - `NPUPLL_GATE_STAT_REG`.
    pub npupll_gate_stat: RO<u32>,
    _reserved_0e48: [u8; 0x0b8],
    /// 0x0f00 - `CCU_SEC_SWITCH_REG`.
    pub sec_switch: RW<u32>,
    /// 0x0f04 - `GPADC_CLK_SEL_REG`.
    pub gpadc_clk_sel: RW<u32>,
    /// 0x0f08 - `FRE_DET_CTRL_REG`.
    pub fre_det_ctrl: RW<u32>,
    /// 0x0f0c - `FRE_UP_LIM_REG`.
    pub fre_up_lim: RW<u32>,
    /// 0x0f10 - `FRE_DOWN_LIM_REG`.
    pub fre_down_lim: RW<u32>,
    _reserved_0f14: [u8; 0x01c],
    /// 0x0f30 - `CCU_FAN_GATE_REG`.
    pub fan_gate: RW<u32>,
    /// 0x0f34 - `CLK27M_FAN_REG`.
    pub clk27m_fan: RW<u32>,
    /// 0x0f38 - `CLK_FAN_REG`.
    pub clk_fan: RW<u32>,
    /// 0x0f3c - `CCU_FAN_REG`.
    pub fan: RW<u32>,
    _reserved_0f40: [u8; 0x0b0],
    /// 0x0ff0 - `CCU_VERSION_REG`.
    pub version: RO<u32>,
    _reserved_0ff4: [u8; 0x40c],
    /// 0x1400 - `PLL_CPUX_TUNING_REG`.
    pub pll_cpux_tuning: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_ctrl), 0x000);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_ctrl1), 0x004);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_ctrl), 0x010);
        assert_eq!(offset_of!(RegisterBlock, pll_peri_ctrl), 0x020);
        assert_eq!(offset_of!(RegisterBlock, pll_video_ctrl), 0x040);
        assert_eq!(offset_of!(RegisterBlock, pll_csi_ctrl), 0x048);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_ctrl), 0x078);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_ctrl), 0x080);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_pat0_ctrl), 0x100);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_pat1_ctrl), 0x104);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat0_ctrl), 0x110);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat1_ctrl), 0x114);
        assert_eq!(offset_of!(RegisterBlock, pll_peri_pat0_ctrl), 0x120);
        assert_eq!(offset_of!(RegisterBlock, pll_peri_pat1_ctrl), 0x124);
        assert_eq!(offset_of!(RegisterBlock, pll_video_pat0_ctrl), 0x140);
        assert_eq!(offset_of!(RegisterBlock, pll_video_pat1_ctrl), 0x144);
        assert_eq!(offset_of!(RegisterBlock, pll_csi_pat0_ctrl), 0x148);
        assert_eq!(offset_of!(RegisterBlock, pll_csi_pat1_ctrl), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_pat0_ctrl), 0x178);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_pat1_ctrl), 0x17c);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_pat0_ctrl), 0x180);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_pat1_ctrl), 0x184);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_ssc), 0x200);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_bias), 0x300);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_bias), 0x310);
        assert_eq!(offset_of!(RegisterBlock, pll_peri_bias), 0x320);
        assert_eq!(offset_of!(RegisterBlock, pll_video_bias), 0x340);
        assert_eq!(offset_of!(RegisterBlock, pll_csi_bias), 0x348);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_bias), 0x378);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_bias), 0x380);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_tun1), 0x400);
        assert_eq!(offset_of!(RegisterBlock, ipmc_clk), 0x4fc);
        assert_eq!(offset_of!(RegisterBlock, cpu_clk), 0x500);
        assert_eq!(offset_of!(RegisterBlock, cpu_gating), 0x504);
        assert_eq!(offset_of!(RegisterBlock, pic_clk), 0x508);
        assert_eq!(offset_of!(RegisterBlock, cpu_cfg_bgr), 0x50c);
        assert_eq!(offset_of!(RegisterBlock, ahb_clk), 0x510);
        assert_eq!(offset_of!(RegisterBlock, apb0_clk), 0x520);
        assert_eq!(offset_of!(RegisterBlock, apb1_clk), 0x524);
        assert_eq!(offset_of!(RegisterBlock, apb_uart_clk), 0x528);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk), 0x540);
        assert_eq!(offset_of!(RegisterBlock, de_clk), 0x600);
        assert_eq!(offset_of!(RegisterBlock, de_bgr), 0x60c);
        assert_eq!(offset_of!(RegisterBlock, g2d_clk), 0x630);
        assert_eq!(offset_of!(RegisterBlock, g2d_bgr), 0x63c);
        assert_eq!(offset_of!(RegisterBlock, ce_clk), 0x680);
        assert_eq!(offset_of!(RegisterBlock, ce_bgr), 0x68c);
        assert_eq!(offset_of!(RegisterBlock, ve_clk), 0x690);
        assert_eq!(offset_of!(RegisterBlock, ve_bgr), 0x69c);
        assert_eq!(offset_of!(RegisterBlock, npu_clk), 0x6e0);
        assert_eq!(offset_of!(RegisterBlock, npu_gating), 0x6e4);
        assert_eq!(offset_of!(RegisterBlock, npu_bgr), 0x6ec);
        assert_eq!(offset_of!(RegisterBlock, dma_bgr), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, msgbox_bgr), 0x71c);
        assert_eq!(offset_of!(RegisterBlock, spinlock_bgr), 0x72c);
        assert_eq!(offset_of!(RegisterBlock, hstimer_bgr), 0x73c);
        assert_eq!(offset_of!(RegisterBlock, avs_clk), 0x740);
        assert_eq!(offset_of!(RegisterBlock, timer_apb_clk), 0x744);
        assert_eq!(offset_of!(RegisterBlock, timer_bgr), 0x74c);
        assert_eq!(offset_of!(RegisterBlock, can_bgr), 0x75c);
        assert_eq!(offset_of!(RegisterBlock, dbgsys_bgr), 0x78c);
        assert_eq!(offset_of!(RegisterBlock, pwm_bgr), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, iommu), 0x7bc);
        assert_eq!(offset_of!(RegisterBlock, dram_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gating), 0x804);
        assert_eq!(offset_of!(RegisterBlock, dram_bgr), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, smhc0_clk), 0x830);
        assert_eq!(offset_of!(RegisterBlock, smhc1_clk), 0x834);
        assert_eq!(offset_of!(RegisterBlock, smhc2_clk), 0x838);
        assert_eq!(offset_of!(RegisterBlock, smhc_bgr), 0x84c);
        assert_eq!(offset_of!(RegisterBlock, psram_clk), 0x850);
        assert_eq!(offset_of!(RegisterBlock, psram_bgr), 0x85c);
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, twi_bgr), 0x91c);
        assert_eq!(offset_of!(RegisterBlock, spi0_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi1_clk), 0x944);
        assert_eq!(offset_of!(RegisterBlock, spi2_clk), 0x948);
        assert_eq!(offset_of!(RegisterBlock, spif_clk), 0x950);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
        assert_eq!(offset_of!(RegisterBlock, gmac_25m_clk), 0x970);
        assert_eq!(offset_of!(RegisterBlock, gmac_bgr), 0x97c);
        assert_eq!(offset_of!(RegisterBlock, gpadc_bgr), 0x9ec);
        assert_eq!(offset_of!(RegisterBlock, ths_bgr), 0x9fc);
        assert_eq!(offset_of!(RegisterBlock, i2s0_clk), 0xa10);
        assert_eq!(offset_of!(RegisterBlock, i2s_bgr), 0xa2c);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_dac_clk), 0xa50);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_adc_clk), 0xa54);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_bgr), 0xa5c);
        assert_eq!(offset_of!(RegisterBlock, usb0_clk), 0xa70);
        assert_eq!(offset_of!(RegisterBlock, usb_bgr), 0xa8c);
        assert_eq!(offset_of!(RegisterBlock, dpss_top_bgr), 0xabc);
        assert_eq!(offset_of!(RegisterBlock, tconlcd_clk), 0xb60);
        assert_eq!(offset_of!(RegisterBlock, tconlcd_bgr), 0xb7c);
        assert_eq!(offset_of!(RegisterBlock, csi_clk), 0xc04);
        assert_eq!(offset_of!(RegisterBlock, csi_master0_clk), 0xc08);
        assert_eq!(offset_of!(RegisterBlock, csi_master1_clk), 0xc0c);
        assert_eq!(offset_of!(RegisterBlock, csi_master2_clk), 0xc10);
        assert_eq!(offset_of!(RegisterBlock, csi_bgr), 0xc2c);
        assert_eq!(offset_of!(RegisterBlock, e907_clk), 0xd00);
        assert_eq!(offset_of!(RegisterBlock, e907_gating_rst), 0xd04);
        assert_eq!(offset_of!(RegisterBlock, riscv_cfg_bgr), 0xd0c);
        assert_eq!(offset_of!(RegisterBlock, pll_pre_div), 0xe00);
        assert_eq!(offset_of!(RegisterBlock, ahb_gate_en), 0xe04);
        assert_eq!(offset_of!(RegisterBlock, peripll_gate_en), 0xe08);
        assert_eq!(offset_of!(RegisterBlock, clk24m_gate_en), 0xe0c);
        assert_eq!(offset_of!(RegisterBlock, pll_opg_bypass), 0xe10);
        assert_eq!(offset_of!(RegisterBlock, audiopll_gate_en), 0xe14);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_en), 0xe18);
        assert_eq!(offset_of!(RegisterBlock, csipll_gate_en), 0xe1c);
        assert_eq!(offset_of!(RegisterBlock, ddrpll_gate_en), 0xe20);
        assert_eq!(offset_of!(RegisterBlock, cpupll_gate_en), 0xe24);
        assert_eq!(offset_of!(RegisterBlock, peripll_gate_stat), 0xe28);
        assert_eq!(offset_of!(RegisterBlock, audiopll_gate_stat), 0xe2c);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_stat), 0xe30);
        assert_eq!(offset_of!(RegisterBlock, csipll_gate_stat), 0xe34);
        assert_eq!(offset_of!(RegisterBlock, ddrpll_gate_stat), 0xe38);
        assert_eq!(offset_of!(RegisterBlock, cpupll_gate_stat), 0xe3c);
        assert_eq!(offset_of!(RegisterBlock, npupll_gate_en), 0xe40);
        assert_eq!(offset_of!(RegisterBlock, npupll_gate_stat), 0xe44);
        assert_eq!(offset_of!(RegisterBlock, sec_switch), 0xf00);
        assert_eq!(offset_of!(RegisterBlock, gpadc_clk_sel), 0xf04);
        assert_eq!(offset_of!(RegisterBlock, fre_det_ctrl), 0xf08);
        assert_eq!(offset_of!(RegisterBlock, fre_up_lim), 0xf0c);
        assert_eq!(offset_of!(RegisterBlock, fre_down_lim), 0xf10);
        assert_eq!(offset_of!(RegisterBlock, fan_gate), 0xf30);
        assert_eq!(offset_of!(RegisterBlock, clk27m_fan), 0xf34);
        assert_eq!(offset_of!(RegisterBlock, clk_fan), 0xf38);
        assert_eq!(offset_of!(RegisterBlock, fan), 0xf3c);
        assert_eq!(offset_of!(RegisterBlock, version), 0xff0);
        assert_eq!(offset_of!(RegisterBlock, pll_cpux_tuning), 0x1400);
        assert_eq!(size_of::<RegisterBlock>(), 0x1404);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
