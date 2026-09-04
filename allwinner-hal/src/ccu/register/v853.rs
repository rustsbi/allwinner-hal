//! V853/V851s Clock Control Unit registers.
//!
//! This layout represents the vendor `sun8iw21` platform.

use super::{BusGatingReset, SingleBusGatingReset};
use volatile_register::RW;

/// V853/V851s main CCU register block.
#[doc(alias = "sun8iw21")]
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - `CCU_PLL_CPU_CTRL_REG, SUN8IW21_PLL_CPU_CTRL_REG, pll_cpu_clk`.
    pub pll_cpu_ctrl: RW<u32>,
    _reserved_0004: [u8; 0x00c],
    /// 0x0010 - `CCU_PLL_DDR_CTRL_REG, SUN8IW21_PLL_DDR_CTRL_REG, pll_ddr_clk`.
    pub pll_ddr_ctrl: RW<u32>,
    _reserved_0014: [u8; 0x00c],
    /// 0x0020 - `CCU_PLL_PERI_CTRL_REG, SUN8IW21_PLL_PERI_CTRL_REG, e907_axi_div_clk, ...`.
    pub pll_peri_ctrl: RW<u32>,
    _reserved_0024: [u8; 0x01c],
    /// 0x0040 - `CCU_PLL_VIDEO_CTRL_REG, SUN8IW21_PLL_VIDEO_CTRL_REG, pll_video_4x_clk`.
    pub pll_video_ctrl: RW<u32>,
    _reserved_0044: [u8; 0x004],
    /// 0x0048 - `CCU_PLL_CSI_CTRL_REG, SUN8IW21_PLL_CSI_CTRL_REG, pll_csi_4x_clk`.
    pub pll_csi_ctrl: RW<u32>,
    _reserved_004c: [u8; 0x02c],
    /// 0x0078 - `CCU_PLL_AUDIO_CTRL_REG, SUN8IW21_PLL_AUDIO_CTRL_REG, pll_audio_div2_clk, ...`.
    pub pll_audio_ctrl: RW<u32>,
    _reserved_007c: [u8; 0x004],
    /// 0x0080 - `CCU_PLL_NPU_CTRL_REG, SUN8IW21_PLL_NPU_CTRL_REG, pll_npu_4x_clk`.
    pub pll_npu_ctrl: RW<u32>,
    _reserved_0084: [u8; 0x08c],
    /// 0x0110 - `CCU_PLL_DDR_PAT0_CTRL_REG`.
    pub pll_ddr_pat0_ctrl: RW<u32>,
    /// 0x0114 - `CCU_PLL_DDR_PAT1_CTRL_REG`.
    pub pll_ddr_pat1_ctrl: RW<u32>,
    _reserved_0118: [u8; 0x008],
    /// 0x0120 - `CCU_PLL_PERI_PAT0_CTRL_REG`.
    pub pll_peri_pat0_ctrl: RW<u32>,
    /// 0x0124 - `CCU_PLL_PERI_PAT1_CTRL_REG`.
    pub pll_peri_pat1_ctrl: RW<u32>,
    _reserved_0128: [u8; 0x018],
    /// 0x0140 - `CCU_PLL_VIDEO_PAT0_CTRL_REG`.
    pub pll_video_pat0_ctrl: RW<u32>,
    /// 0x0144 - `CCU_PLL_VIDEO_PAT1_CTRL_REG`.
    pub pll_video_pat1_ctrl: RW<u32>,
    /// 0x0148 - `CCU_PLL_CSI_PAT0_CTRL_REG`.
    pub pll_csi_pat0_ctrl: RW<u32>,
    /// 0x014c - `CCU_PLL_CSI_PAT1_CTRL_REG`.
    pub pll_csi_pat1_ctrl: RW<u32>,
    _reserved_0150: [u8; 0x028],
    /// 0x0178 - `CCU_PLL_AUDIO_PAT0_CTRL_REG`.
    pub pll_audio_pat0_ctrl: RW<u32>,
    /// 0x017c - `CCU_PLL_AUDIO_PAT1_CTRL_REG`.
    pub pll_audio_pat1_ctrl: RW<u32>,
    /// 0x0180 - `CCU_PLL_NPU_PAT0_CTRL_REG`.
    pub pll_npu_pat0_ctrl: RW<u32>,
    /// 0x0184 - `CCU_PLL_NPU_PAT1_CTRL_REG`.
    pub pll_npu_pat1_ctrl: RW<u32>,
    _reserved_0188: [u8; 0x178],
    /// 0x0300 - `CCU_PLL_CPU_BIAS_REG`.
    pub pll_cpu_bias: RW<u32>,
    _reserved_0304: [u8; 0x00c],
    /// 0x0310 - `CCU_PLL_DDR_BIAS_REG`.
    pub pll_ddr_bias: RW<u32>,
    _reserved_0314: [u8; 0x00c],
    /// 0x0320 - `CCU_PLL_PERI_BIAS_REG`.
    pub pll_peri_bias: RW<u32>,
    _reserved_0324: [u8; 0x01c],
    /// 0x0340 - `CCU_PLL_VIDEO_BIAS_REG`.
    pub pll_video_bias: RW<u32>,
    _reserved_0344: [u8; 0x004],
    /// 0x0348 - `CCU_PLL_CSI_BIAS_REG`.
    pub pll_csi_bias: RW<u32>,
    _reserved_034c: [u8; 0x02c],
    /// 0x0378 - `CCU_PLL_AUDIO_BIAS_REG`.
    pub pll_audio_bias: RW<u32>,
    _reserved_037c: [u8; 0x004],
    /// 0x0380 - `CCU_PLL_NPU_BIAS_REG`.
    pub pll_npu_bias: RW<u32>,
    _reserved_0384: [u8; 0x07c],
    /// 0x0400 - `CCU_PLL_CPU_TUN_REG`.
    pub pll_cpu_tun: RW<u32>,
    _reserved_0404: [u8; 0x0fc],
    /// 0x0500 - `CCU_CPU_CLK_REG, cpu_apb_clk, cpu_axi_clk, ...`.
    pub cpu_clk: RW<u32>,
    /// 0x0504 - `CCU_CPU_GATING_REG, cpu_bus_clk`.
    pub cpu_gating: RW<u32>,
    _reserved_0508: [u8; 0x008],
    /// 0x0510 - `CCU_AHB_CLK_REG, ahb_clk`.
    pub ahb_clk: RW<u32>,
    _reserved_0514: [u8; 0x00c],
    /// 0x0520 - `CCU_APB0_CLK_REG, apb0_clk`.
    pub apb0_clk: RW<u32>,
    /// 0x0524 - `CCU_APB1_CLK_REG, apb1_clk`.
    pub apb1_clk: RW<u32>,
    _reserved_0528: [u8; 0x018],
    /// 0x0540 - `CCU_MBUS_CLK_REG, reset map`.
    pub mbus_clk: RW<u32>,
    _reserved_0544: [u8; 0x0bc],
    /// 0x0600 - `CCU_DE_CLK_REG, de_clk`.
    pub de_clk: RW<u32>,
    _reserved_0604: [u8; 0x008],
    /// 0x060c - `CCU_DE_BGR_REG, de_bus_clk, reset map`.
    pub de_bgr: RW<u32>,
    _reserved_0610: [u8; 0x020],
    /// 0x0630 - `CCU_G2D_CLK_REG, g2d_clk`.
    pub g2d_clk: RW<u32>,
    _reserved_0634: [u8; 0x008],
    /// 0x063c - `CCU_G2D_BGR_REG, g2d_bus_clk, reset map`.
    pub g2d_bgr: RW<u32>,
    _reserved_0640: [u8; 0x040],
    /// 0x0680 - `CCU_CE_CLK_REG, ce_clk`.
    pub ce_clk: RW<u32>,
    _reserved_0684: [u8; 0x008],
    /// 0x068c - `CCU_CE_BGR_REG, ce_bus_clk, ce_sys_clk, ...`.
    pub ce_bgr: RW<u32>,
    /// 0x0690 - `CCU_VE_CLK_REG, ve_clk`.
    pub ve_clk: RW<u32>,
    _reserved_0694: [u8; 0x008],
    /// 0x069c - `CCU_VE_BGR_REG, reset map, ve_bus_clk`.
    pub ve_bgr: RW<u32>,
    _reserved_06a0: [u8; 0x040],
    /// 0x06e0 - `CCU_NPU_CLK_REG, npu_clk`.
    pub npu_clk: RW<u32>,
    _reserved_06e4: [u8; 0x008],
    /// 0x06ec - `CCU_NPU_BGR_REG, npu_bus_clk, reset map`.
    pub npu_bgr: RW<u32>,
    _reserved_06f0: [u8; 0x01c],
    /// 0x070c - `CCU_DMA_BGR_REG, dma_clk, reset map`.
    pub dma_bgr: RW<u32>,
    _reserved_0710: [u8; 0x00c],
    /// 0x071c - `CCU_MSGBOX_BGR_REG, msgbox0_clk, msgbox1_clk, ...`.
    pub msgbox_bgr: RW<u32>,
    _reserved_0720: [u8; 0x00c],
    /// 0x072c - `CCU_SPINLOCK_BGR_REG, reset map, spinlock_clk`.
    pub spinlock_bgr: RW<u32>,
    _reserved_0730: [u8; 0x00c],
    /// 0x073c - `CCU_HSTIMER_BGR_REG, hstimer_clk, reset map`.
    pub hstimer_bgr: RW<u32>,
    /// 0x0740 - `CCU_AVS_CLK_REG, avs_clk`.
    pub avs_clk: RW<u32>,
    _reserved_0744: [u8; 0x048],
    /// 0x078c - `CCU_DBGSYS_BGR_REG, dbgsys_clk, reset map`.
    pub dbgsys_bgr: RW<u32>,
    _reserved_0790: [u8; 0x01c],
    /// 0x07ac - `CCU_PWM_BGR_REG, pwm_clk, reset map`.
    pub pwm_bgr: RW<u32>,
    _reserved_07b0: [u8; 0x00c],
    /// 0x07bc - `CCU_IOMMU_BGR_REG, iommu_clk`.
    pub iommu_bgr: RW<u32>,
    _reserved_07c0: [u8; 0x040],
    /// 0x0800 - `CCU_DRAM_CLK_REG, dram_clk`.
    pub dram_clk: RW<u32>,
    /// 0x0804 - `CCU_MBUS_MAT_CLK_GATING_REG, ce_mbus_clk, csi_mbus_clk, ...`.
    pub mbus_mat_clk_gating: RW<u32>,
    _reserved_0808: [u8; 0x004],
    /// 0x080c - `CCU_DRAM_BGR_REG, dram_bus_clk, reset map`.
    pub dram_bgr: RW<SingleBusGatingReset>,
    _reserved_0810: [u8; 0x020],
    /// 0x0830 - `CCU_SMHC0_CLK_REG, smhc0_clk`.
    pub smhc0_clk: RW<u32>,
    /// 0x0834 - `CCU_SMHC1_CLK_REG, smhc1_clk`.
    pub smhc1_clk: RW<u32>,
    /// 0x0838 - `CCU_SMHC2_CLK_REG, smhc2_clk`.
    pub smhc2_clk: RW<u32>,
    _reserved_083c: [u8; 0x010],
    /// 0x084c - `CCU_SMHC_BGR_REG, reset map, smhc0_bus_clk, ...`.
    pub smhc_bgr: RW<BusGatingReset<3>>,
    _reserved_0850: [u8; 0x0bc],
    /// 0x090c - `CCU_UART_BGR_REG, reset map, uart0_clk, ...`.
    pub uart_bgr: RW<BusGatingReset<4>>,
    _reserved_0910: [u8; 0x00c],
    /// 0x091c - `CCU_TWI_BGR_REG, reset map, twi0_clk, ...`.
    pub twi_bgr: RW<BusGatingReset<5>>,
    _reserved_0920: [u8; 0x020],
    /// 0x0940 - `CCU_SPI0_CLK_REG, spi0_clk`.
    pub spi0_clk: RW<u32>,
    /// 0x0944 - `CCU_SPI1_CLK_REG, spi1_clk`.
    pub spi1_clk: RW<u32>,
    /// 0x0948 - `CCU_SPI2_CLK_REG, spi2_clk`.
    pub spi2_clk: RW<u32>,
    /// 0x094c - `CCU_SPI3_CLK_REG, spi3_clk`.
    pub spi3_clk: RW<u32>,
    /// 0x0950 - `spif_clk`.
    pub spif: RW<u32>,
    _reserved_0954: [u8; 0x018],
    /// 0x096c - `CCU_SPI_BGR_REG, reset map, spi0_bus_clk, ...`.
    pub spi_bgr: RW<BusGatingReset<4>>,
    /// 0x0970 - `CCU_EMAC_25M_CLK_REG, gmac_25m_clk, gmac_25m_clk_src_clk`.
    pub emac_25m_clk: RW<u32>,
    _reserved_0974: [u8; 0x008],
    /// 0x097c - `CCU_EMAC_BGR_REG, gmac_clk, reset map`.
    pub emac_bgr: RW<u32>,
    _reserved_0980: [u8; 0x06c],
    /// 0x09ec - `CCU_GPADC_BGR_REG, gpadc_clk, reset map`.
    pub gpadc_bgr: RW<u32>,
    _reserved_09f0: [u8; 0x00c],
    /// 0x09fc - `CCU_THS_BGR_REG, reset map, ths_clk`.
    pub ths_bgr: RW<u32>,
    _reserved_0a00: [u8; 0x010],
    /// 0x0a10 - `i2s0_clk`.
    pub i2s0: RW<u32>,
    /// 0x0a14 - `CCU_I2S1_CLK_REG, i2s1_clk`.
    pub i2s1_clk: RW<u32>,
    _reserved_0a18: [u8; 0x008],
    /// 0x0a20 - `CCU_I2S_BGR_REG, i2s0_bus_clk, i2s1_bus_clk, ...`.
    pub i2s_bgr: RW<u32>,
    _reserved_0a24: [u8; 0x01c],
    /// 0x0a40 - `CCU_DMIC_CLK_REG, dmic_clk`.
    pub dmic_clk: RW<u32>,
    _reserved_0a44: [u8; 0x008],
    /// 0x0a4c - `CCU_DMIC_BGR_REG, dmic_bus_clk, reset map`.
    pub dmic_bgr: RW<u32>,
    /// 0x0a50 - `CCU_AUDIO_CODEC_DAC_CLK_REG, audio_codec_dac_clk`.
    pub audio_codec_dac_clk: RW<u32>,
    /// 0x0a54 - `CCU_AUDIO_CODEC_ADC_CLK_REG, audio_codec_adc_clk`.
    pub audio_codec_adc_clk: RW<u32>,
    _reserved_0a58: [u8; 0x004],
    /// 0x0a5c - `CCU_AUDIO_CODEC_BGR_REG, audio_codec_clk, reset map`.
    pub audio_codec_bgr: RW<u32>,
    _reserved_0a60: [u8; 0x010],
    /// 0x0a70 - `CCU_USB0_CLK_REG, reset map, usb_clk`.
    pub usb0_clk: RW<u32>,
    _reserved_0a74: [u8; 0x018],
    /// 0x0a8c - `CCU_USB_BGR_REG, reset map, usbehci0_clk, ...`.
    pub usb_bgr: RW<u32>,
    _reserved_0a90: [u8; 0x02c],
    /// 0x0abc - `CCU_DPSS_TOP_BGR_REG, dpss_top_clk, reset map`.
    pub dpss_top_bgr: RW<u32>,
    _reserved_0ac0: [u8; 0x064],
    /// 0x0b24 - `CCU_DSI_CLK_REG, dsi_clk`.
    pub dsi_clk: RW<u32>,
    _reserved_0b28: [u8; 0x024],
    /// 0x0b4c - `CCU_DSI_BGR_REG, dsi_bus_clk, reset map`.
    pub dsi_bgr: RW<u32>,
    _reserved_0b50: [u8; 0x010],
    /// 0x0b60 - `CCU_TCONLCD_CLK_REG, tconlcd_clk`.
    pub tconlcd_clk: RW<u32>,
    _reserved_0b64: [u8; 0x018],
    /// 0x0b7c - `CCU_TCONLCD_BGR_REG, reset map, tconlcd_bus_clk`.
    pub tconlcd_bgr: RW<u32>,
    _reserved_0b80: [u8; 0x084],
    /// 0x0c04 - `CCU_CSI_CLK_REG, csi_clk`.
    pub csi_clk: RW<u32>,
    /// 0x0c08 - `CCU_CSI_MASTER0_CLK_REG, csi_master0_clk`.
    pub csi_master0_clk: RW<u32>,
    /// 0x0c0c - `CCU_CSI_MASTER1_CLK_REG, csi_master1_clk`.
    pub csi_master1_clk: RW<u32>,
    /// 0x0c10 - `CCU_CSI_MASTER2_CLK_REG, csi_master2_clk`.
    pub csi_master2_clk: RW<u32>,
    _reserved_0c14: [u8; 0x018],
    /// 0x0c2c - `CCU_CSI_BGR_REG, csi_bus_clk, reset map`.
    pub csi_bgr: RW<u32>,
    _reserved_0c30: [u8; 0x04c],
    /// 0x0c7c - `CCU_WIEGAND_BGR_REG, reset map, wiegand_clk`.
    pub wiegand_bgr: RW<u32>,
    _reserved_0c80: [u8; 0x080],
    /// 0x0d00 - `e907_core_div_clk`.
    pub e907_core_div: RW<u32>,
    /// 0x0d04 - `e907_core_gate_clk, e907_core_rst_clk, e907_dbg_rst_clk`.
    pub e907_core_gate: RW<u32>,
    _reserved_0d08: [u8; 0x004],
    /// 0x0d0c - `reset map, riscv_cfg_clk`.
    pub riscv_cfg: RW<u32>,
    _reserved_0d10: [u8; 0x0f0],
    /// 0x0e00 - `CCU_PLL_PRE_DIV_REG, pll_audio_1x_clk, pll_audio_4x_clk`.
    pub pll_pre_div: RW<u32>,
    /// 0x0e04 - `CCU_AHB_GATE_EN_REG, cpus_hclk_gate_clk, usb_ahb_gate_clk, ...`.
    pub ahb_gate_en: RW<u32>,
    /// 0x0e08 - `CCU_PERIPLL_GATE_EN_REG`.
    pub peripll_gate_en: RW<u32>,
    /// 0x0e0c - `CCU_CLK24M_GATE_EN_REG, gpadc_24m_clk, res_dcap_24m_clk, ...`.
    pub clk24m_gate_en: RW<u32>,
    _reserved_0e10: [u8; 0x0f0],
    /// 0x0f00 - `CCU_CCMU_SEC_SWITCH_REG`.
    pub ccmu_sec_switch: RW<u32>,
    /// 0x0f04 - `CCU_GPADC_CLK_SEL_REG, gpadc_sel_clk`.
    pub gpadc_clk_sel: RW<u32>,
    /// 0x0f08 - `CCU_FRE_DET_CTRL_REG`.
    pub fre_det_ctrl: RW<u32>,
    /// 0x0f0c - `CCU_FRE_UP_LIM_REG`.
    pub fre_up_lim: RW<u32>,
    /// 0x0f10 - `CCU_FRE_DOWN_LIM_REG`.
    pub fre_down_lim: RW<u32>,
    _reserved_0f14: [u8; 0x01c],
    /// 0x0f30 - `CCU_CCMU_FAN_GATE_REG`.
    pub ccmu_fan_gate: RW<u32>,
    /// 0x0f34 - `CCU_CLK27M_FAN_REG`.
    pub clk27m_fan: RW<u32>,
    /// 0x0f38 - `CCU_CLK_FAN_REG`.
    pub clk_fan: RW<u32>,
    /// 0x0f3c - `CCU_CCMU_FAN_REG`.
    pub ccmu_fan: RW<u32>,
    _reserved_0f40: [u8; 0x4c0],
    /// 0x1400 - `CCU_PLL_CPUX_TUNING_REG`.
    pub pll_cpux_tuning: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_ctrl), 0x000);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_ctrl), 0x010);
        assert_eq!(offset_of!(RegisterBlock, pll_peri_ctrl), 0x020);
        assert_eq!(offset_of!(RegisterBlock, pll_video_ctrl), 0x040);
        assert_eq!(offset_of!(RegisterBlock, pll_csi_ctrl), 0x048);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_ctrl), 0x078);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_ctrl), 0x080);
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
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_bias), 0x300);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_bias), 0x310);
        assert_eq!(offset_of!(RegisterBlock, pll_peri_bias), 0x320);
        assert_eq!(offset_of!(RegisterBlock, pll_video_bias), 0x340);
        assert_eq!(offset_of!(RegisterBlock, pll_csi_bias), 0x348);
        assert_eq!(offset_of!(RegisterBlock, pll_audio_bias), 0x378);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_bias), 0x380);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_tun), 0x400);
        assert_eq!(offset_of!(RegisterBlock, cpu_clk), 0x500);
        assert_eq!(offset_of!(RegisterBlock, cpu_gating), 0x504);
        assert_eq!(offset_of!(RegisterBlock, ahb_clk), 0x510);
        assert_eq!(offset_of!(RegisterBlock, apb0_clk), 0x520);
        assert_eq!(offset_of!(RegisterBlock, apb1_clk), 0x524);
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
        assert_eq!(offset_of!(RegisterBlock, npu_bgr), 0x6ec);
        assert_eq!(offset_of!(RegisterBlock, dma_bgr), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, msgbox_bgr), 0x71c);
        assert_eq!(offset_of!(RegisterBlock, spinlock_bgr), 0x72c);
        assert_eq!(offset_of!(RegisterBlock, hstimer_bgr), 0x73c);
        assert_eq!(offset_of!(RegisterBlock, avs_clk), 0x740);
        assert_eq!(offset_of!(RegisterBlock, dbgsys_bgr), 0x78c);
        assert_eq!(offset_of!(RegisterBlock, pwm_bgr), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, iommu_bgr), 0x7bc);
        assert_eq!(offset_of!(RegisterBlock, dram_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gating), 0x804);
        assert_eq!(offset_of!(RegisterBlock, dram_bgr), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, smhc0_clk), 0x830);
        assert_eq!(offset_of!(RegisterBlock, smhc1_clk), 0x834);
        assert_eq!(offset_of!(RegisterBlock, smhc2_clk), 0x838);
        assert_eq!(offset_of!(RegisterBlock, smhc_bgr), 0x84c);
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, twi_bgr), 0x91c);
        assert_eq!(offset_of!(RegisterBlock, spi0_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi1_clk), 0x944);
        assert_eq!(offset_of!(RegisterBlock, spi2_clk), 0x948);
        assert_eq!(offset_of!(RegisterBlock, spi3_clk), 0x94c);
        assert_eq!(offset_of!(RegisterBlock, spif), 0x950);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
        assert_eq!(offset_of!(RegisterBlock, emac_25m_clk), 0x970);
        assert_eq!(offset_of!(RegisterBlock, emac_bgr), 0x97c);
        assert_eq!(offset_of!(RegisterBlock, gpadc_bgr), 0x9ec);
        assert_eq!(offset_of!(RegisterBlock, ths_bgr), 0x9fc);
        assert_eq!(offset_of!(RegisterBlock, i2s0), 0xa10);
        assert_eq!(offset_of!(RegisterBlock, i2s1_clk), 0xa14);
        assert_eq!(offset_of!(RegisterBlock, i2s_bgr), 0xa20);
        assert_eq!(offset_of!(RegisterBlock, dmic_clk), 0xa40);
        assert_eq!(offset_of!(RegisterBlock, dmic_bgr), 0xa4c);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_dac_clk), 0xa50);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_adc_clk), 0xa54);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_bgr), 0xa5c);
        assert_eq!(offset_of!(RegisterBlock, usb0_clk), 0xa70);
        assert_eq!(offset_of!(RegisterBlock, usb_bgr), 0xa8c);
        assert_eq!(offset_of!(RegisterBlock, dpss_top_bgr), 0xabc);
        assert_eq!(offset_of!(RegisterBlock, dsi_clk), 0xb24);
        assert_eq!(offset_of!(RegisterBlock, dsi_bgr), 0xb4c);
        assert_eq!(offset_of!(RegisterBlock, tconlcd_clk), 0xb60);
        assert_eq!(offset_of!(RegisterBlock, tconlcd_bgr), 0xb7c);
        assert_eq!(offset_of!(RegisterBlock, csi_clk), 0xc04);
        assert_eq!(offset_of!(RegisterBlock, csi_master0_clk), 0xc08);
        assert_eq!(offset_of!(RegisterBlock, csi_master1_clk), 0xc0c);
        assert_eq!(offset_of!(RegisterBlock, csi_master2_clk), 0xc10);
        assert_eq!(offset_of!(RegisterBlock, csi_bgr), 0xc2c);
        assert_eq!(offset_of!(RegisterBlock, wiegand_bgr), 0xc7c);
        assert_eq!(offset_of!(RegisterBlock, e907_core_div), 0xd00);
        assert_eq!(offset_of!(RegisterBlock, e907_core_gate), 0xd04);
        assert_eq!(offset_of!(RegisterBlock, riscv_cfg), 0xd0c);
        assert_eq!(offset_of!(RegisterBlock, pll_pre_div), 0xe00);
        assert_eq!(offset_of!(RegisterBlock, ahb_gate_en), 0xe04);
        assert_eq!(offset_of!(RegisterBlock, peripll_gate_en), 0xe08);
        assert_eq!(offset_of!(RegisterBlock, clk24m_gate_en), 0xe0c);
        assert_eq!(offset_of!(RegisterBlock, ccmu_sec_switch), 0xf00);
        assert_eq!(offset_of!(RegisterBlock, gpadc_clk_sel), 0xf04);
        assert_eq!(offset_of!(RegisterBlock, fre_det_ctrl), 0xf08);
        assert_eq!(offset_of!(RegisterBlock, fre_up_lim), 0xf0c);
        assert_eq!(offset_of!(RegisterBlock, fre_down_lim), 0xf10);
        assert_eq!(offset_of!(RegisterBlock, ccmu_fan_gate), 0xf30);
        assert_eq!(offset_of!(RegisterBlock, clk27m_fan), 0xf34);
        assert_eq!(offset_of!(RegisterBlock, clk_fan), 0xf38);
        assert_eq!(offset_of!(RegisterBlock, ccmu_fan), 0xf3c);
        assert_eq!(offset_of!(RegisterBlock, pll_cpux_tuning), 0x1400);
        assert_eq!(size_of::<RegisterBlock>(), 0x1404);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
