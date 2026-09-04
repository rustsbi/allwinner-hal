//! T113 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun8iw20` platform.

use super::{BusGatingReset, SingleBusGatingReset};
use volatile_register::RW;

/// T113 main CCU register block.
#[doc(alias = "sun8iw20")]
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - `CCU_PLL_CPU_CTRL_REG, SUN8IW20_PLL_CPUX_REG, pll_cpux_clk`.
    pub pll_cpu_ctrl: RW<u32>,
    _reserved_0004: [u8; 0x00c],
    /// 0x0010 - `CCU_PLL_DDR_CTRL_REG, SUN8IW20_PLL_DDR0_REG, pll_ddr0_clk`.
    pub pll_ddr_ctrl: RW<u32>,
    _reserved_0014: [u8; 0x00c],
    /// 0x0020 - `CCU_PLL_PERI0_CTRL_REG, SUN8IW20_PLL_PERIPH0_REG, pll_periph0_2x_clk, ...`.
    pub pll_peri0_ctrl: RW<u32>,
    _reserved_0024: [u8; 0x004],
    /// 0x0028 - `CCU_PLL_PERI1_CTRL_REG`.
    pub pll_peri1_ctrl: RW<u32>,
    _reserved_002c: [u8; 0x004],
    /// 0x0030 - `CCU_PLL_GPU_CTRL_REG`.
    pub pll_gpu_ctrl: RW<u32>,
    _reserved_0034: [u8; 0x00c],
    /// 0x0040 - `CCU_PLL_VIDEO0_CTRL_REG, SUN8IW20_PLL_VIDEO0_REG, pll_video0_clk`.
    pub pll_video0_ctrl: RW<u32>,
    _reserved_0044: [u8; 0x004],
    /// 0x0048 - `CCU_PLL_VIDEO1_CTRL_REG, SUN8IW20_PLL_VIDEO1_REG, pll_video1_clk`.
    pub pll_video1_ctrl: RW<u32>,
    _reserved_004c: [u8; 0x00c],
    /// 0x0058 - `SUN8IW20_PLL_VE_REG, pll_ve_clk`.
    pub pll_ve: RW<u32>,
    _reserved_005c: [u8; 0x01c],
    /// 0x0078 - `CCU_PLL_AUDIO0_CTRL_REG, SUN8IW20_PLL_AUDIO0_REG, pll_audio0_4x_clk`.
    pub pll_audio0_ctrl: RW<u32>,
    _reserved_007c: [u8; 0x004],
    /// 0x0080 - `CCU_PLL_AUDIO1_CTRL_REG, SUN8IW20_PLL_AUDIO1_REG, pll_audio1_clk, ...`.
    pub pll_audio1_ctrl: RW<u32>,
    _reserved_0084: [u8; 0x08c],
    /// 0x0110 - `CCU_PLL_DDR_PAT0_CTRL_REG`.
    pub pll_ddr_pat0_ctrl: RW<u32>,
    /// 0x0114 - `CCU_PLL_DDR_PAT1_CTRL_REG`.
    pub pll_ddr_pat1_ctrl: RW<u32>,
    _reserved_0118: [u8; 0x008],
    /// 0x0120 - `CCU_PLL_PERI0_PAT0_CTRL_REG`.
    pub pll_peri0_pat0_ctrl: RW<u32>,
    /// 0x0124 - `CCU_PLL_PERI0_PAT1_CTRL_REG`.
    pub pll_peri0_pat1_ctrl: RW<u32>,
    /// 0x0128 - `CCU_PLL_PERI1_PAT0_CTRL_REG`.
    pub pll_peri1_pat0_ctrl: RW<u32>,
    /// 0x012c - `CCU_PLL_PERI1_PAT1_CTRL_REG`.
    pub pll_peri1_pat1_ctrl: RW<u32>,
    /// 0x0130 - `CCU_PLL_GPU_PAT0_CTRL_REG`.
    pub pll_gpu_pat0_ctrl: RW<u32>,
    /// 0x0134 - `CCU_PLL_GPU_PAT1_CTRL_REG`.
    pub pll_gpu_pat1_ctrl: RW<u32>,
    _reserved_0138: [u8; 0x008],
    /// 0x0140 - `CCU_PLL_VIDEO0_PAT0_CTRL_REG`.
    pub pll_video0_pat0_ctrl: RW<u32>,
    /// 0x0144 - `CCU_PLL_VIDEO0_PAT1_CTRL_REG`.
    pub pll_video0_pat1_ctrl: RW<u32>,
    /// 0x0148 - `CCU_PLL_VIDEO1_PAT0_CTRL_REG`.
    pub pll_video1_pat0_ctrl: RW<u32>,
    /// 0x014c - `CCU_PLL_VIDEO1_PAT1_CTRL_REG`.
    pub pll_video1_pat1_ctrl: RW<u32>,
    _reserved_0150: [u8; 0x008],
    /// 0x0158 - `CCU_PLL_VE_PAT0_CTRL_REG`.
    pub pll_ve_pat0_ctrl: RW<u32>,
    /// 0x015c - `CCU_PLL_VE_PAT1_CTRL_REG`.
    pub pll_ve_pat1_ctrl: RW<u32>,
    /// 0x0160 - `CCU_PLL_DE_PAT0_CTRL_REG`.
    pub pll_de_pat0_ctrl: RW<u32>,
    /// 0x0164 - `CCU_PLL_DE_PAT1_CTRL_REG`.
    pub pll_de_pat1_ctrl: RW<u32>,
    _reserved_0168: [u8; 0x008],
    /// 0x0170 - `CCU_PLL_HSIC_PAT0_CTRL_REG`.
    pub pll_hsic_pat0_ctrl: RW<u32>,
    /// 0x0174 - `CCU_PLL_HSIC_PAT1_CTRL_REG`.
    pub pll_hsic_pat1_ctrl: RW<u32>,
    /// 0x0178 - `CCU_PLL_AUDIO0_PAT0_CTRL_REG`.
    pub pll_audio0_pat0_ctrl: RW<u32>,
    /// 0x017c - `CCU_PLL_AUDIO0_PAT1_CTRL_REG`.
    pub pll_audio0_pat1_ctrl: RW<u32>,
    /// 0x0180 - `CCU_PLL_AUDIO1_PAT0_CTRL_REG`.
    pub pll_audio1_pat0_ctrl: RW<u32>,
    /// 0x0184 - `CCU_PLL_AUDIO1_PAT1_CTRL_REG`.
    pub pll_audio1_pat1_ctrl: RW<u32>,
    _reserved_0188: [u8; 0x178],
    /// 0x0300 - `CCU_PLL_CPU_BIAS_REG`.
    pub pll_cpu_bias: RW<u32>,
    _reserved_0304: [u8; 0x00c],
    /// 0x0310 - `CCU_PLL_DDR_BIAS_REG`.
    pub pll_ddr_bias: RW<u32>,
    _reserved_0314: [u8; 0x00c],
    /// 0x0320 - `CCU_PLL_PERI0_BIAS_REG`.
    pub pll_peri0_bias: RW<u32>,
    _reserved_0324: [u8; 0x004],
    /// 0x0328 - `CCU_PLL_PERI1_BIAS_REG`.
    pub pll_peri1_bias: RW<u32>,
    _reserved_032c: [u8; 0x004],
    /// 0x0330 - `CCU_PLL_GPU_BIAS_REG`.
    pub pll_gpu_bias: RW<u32>,
    _reserved_0334: [u8; 0x00c],
    /// 0x0340 - `CCU_PLL_VIDEO0_BIAS_REG`.
    pub pll_video0_bias: RW<u32>,
    _reserved_0344: [u8; 0x004],
    /// 0x0348 - `CCU_PLL_VIDEO1_BIAS_REG`.
    pub pll_video1_bias: RW<u32>,
    _reserved_034c: [u8; 0x00c],
    /// 0x0358 - `CCU_PLL_VE_BIAS_REG`.
    pub pll_ve_bias: RW<u32>,
    _reserved_035c: [u8; 0x004],
    /// 0x0360 - `CCU_PLL_DE_BIAS_REG`.
    pub pll_de_bias: RW<u32>,
    _reserved_0364: [u8; 0x00c],
    /// 0x0370 - `CCU_PLL_HSIC_BIAS_REG`.
    pub pll_hsic_bias: RW<u32>,
    _reserved_0374: [u8; 0x004],
    /// 0x0378 - `CCU_PLL_AUDIO0_BIAS_REG`.
    pub pll_audio0_bias: RW<u32>,
    _reserved_037c: [u8; 0x004],
    /// 0x0380 - `CCU_PLL_AUDIO1_BIAS_REG`.
    pub pll_audio1_bias: RW<u32>,
    _reserved_0384: [u8; 0x07c],
    /// 0x0400 - `CCU_PLL_CPU_TUN_REG`.
    pub pll_cpu_tun: RW<u32>,
    _reserved_0404: [u8; 0x0fc],
    /// 0x0500 - `CCU_CPU_AXI_CFG_REG, apb_clk, axi_clk, ...`.
    pub cpu_axi_cfg: RW<u32>,
    /// 0x0504 - `CCU_CPU_GATING_REG`.
    pub cpu_gating: RW<u32>,
    _reserved_0508: [u8; 0x008],
    /// 0x0510 - `CCU_PSI_CLK_REG, psi_ahb_clk`.
    pub psi_clk: RW<u32>,
    _reserved_0514: [u8; 0x008],
    /// 0x051c - `CCU_AHB3_CLK_REG`.
    pub ahb3_clk: RW<u32>,
    /// 0x0520 - `CCU_APB0_CLK_REG, apb0_clk`.
    pub apb0_clk: RW<u32>,
    /// 0x0524 - `CCU_APB1_CLK_REG, apb1_clk`.
    pub apb1_clk: RW<u32>,
    _reserved_0528: [u8; 0x018],
    /// 0x0540 - `CCU_MBUS_CLK_REG, reset map`.
    pub mbus_clk: RW<u32>,
    _reserved_0544: [u8; 0x0bc],
    /// 0x0600 - `de0_clk`.
    pub de0: RW<u32>,
    _reserved_0604: [u8; 0x008],
    /// 0x060c - `bus_de0_clk, reset map`.
    pub bus_de0: RW<u32>,
    _reserved_0610: [u8; 0x010],
    /// 0x0620 - `di_clk`.
    pub di: RW<u32>,
    _reserved_0624: [u8; 0x008],
    /// 0x062c - `bus_di_clk, reset map`.
    pub bus_di: RW<u32>,
    /// 0x0630 - `g2d_clk`.
    pub g2d: RW<u32>,
    _reserved_0634: [u8; 0x008],
    /// 0x063c - `bus_g2d_clk, reset map`.
    pub bus_g2d: RW<u32>,
    _reserved_0640: [u8; 0x040],
    /// 0x0680 - `ce_clk`.
    pub ce: RW<u32>,
    _reserved_0684: [u8; 0x008],
    /// 0x068c - `bus_ce_clk, reset map`.
    pub bus_ce: RW<u32>,
    /// 0x0690 - `ve_clk`.
    pub ve: RW<u32>,
    _reserved_0694: [u8; 0x008],
    /// 0x069c - `bus_ve_clk, reset map`.
    pub bus_ve: RW<u32>,
    _reserved_06a0: [u8; 0x06c],
    /// 0x070c - `CCU_DMA_BGR_REG, bus_dma_clk, reset map`.
    pub dma_bgr: RW<u32>,
    _reserved_0710: [u8; 0x00c],
    /// 0x071c - `bus_msgbox0_clk, bus_msgbox1_clk, bus_msgbox2_clk, ...`.
    pub bus_msgbox0: RW<u32>,
    _reserved_0720: [u8; 0x00c],
    /// 0x072c - `bus_spinlock_clk, reset map`.
    pub bus_spinlock: RW<u32>,
    _reserved_0730: [u8; 0x00c],
    /// 0x073c - `bus_hstimer_clk, reset map`.
    pub bus_hstimer: RW<u32>,
    /// 0x0740 - `avs_clk`.
    pub avs: RW<u32>,
    _reserved_0744: [u8; 0x048],
    /// 0x078c - `bus_dbg_clk, reset map`.
    pub bus_dbg: RW<u32>,
    _reserved_0790: [u8; 0x01c],
    /// 0x07ac - `bus_pwm_clk, reset map`.
    pub bus_pwm: RW<u32>,
    _reserved_07b0: [u8; 0x00c],
    /// 0x07bc - `bus_iommu_clk`.
    pub bus_iommu: RW<u32>,
    _reserved_07c0: [u8; 0x040],
    /// 0x0800 - `CCU_DRAM_CLK_REG, dram_clk`.
    pub dram_clk: RW<u32>,
    /// 0x0804 - `CCU_MBUS_MAT_CLK_GATING_REG, mbus_ce_clk, mbus_csi_clk, ...`.
    pub mbus_mat_clk_gating: RW<u32>,
    _reserved_0808: [u8; 0x004],
    /// 0x080c - `CCU_DRAM_BGR_REG, bus_dram_clk, reset map`.
    pub dram_bgr: RW<SingleBusGatingReset>,
    _reserved_0810: [u8; 0x020],
    /// 0x0830 - `CCU_SMHC0_CLK_REG, mmc0_clk`.
    pub smhc0_clk: RW<u32>,
    /// 0x0834 - `CCU_SMHC1_CLK_REG, mmc1_clk`.
    pub smhc1_clk: RW<u32>,
    /// 0x0838 - `CCU_SMHC2_CLK_REG, mmc2_clk`.
    pub smhc2_clk: RW<u32>,
    _reserved_083c: [u8; 0x010],
    /// 0x084c - `CCU_SMHC_BGR_REG, bus_mmc0_clk, bus_mmc1_clk, ...`.
    pub smhc_bgr: RW<BusGatingReset<3>>,
    _reserved_0850: [u8; 0x0bc],
    /// 0x090c - `CCU_UART_BGR_REG, bus_uart0_clk, bus_uart1_clk, ...`.
    pub uart_bgr: RW<BusGatingReset<6>>,
    _reserved_0910: [u8; 0x00c],
    /// 0x091c - `CCU_TWI_BGR_REG, bus_twi0_clk, bus_twi1_clk, ...`.
    pub twi_bgr: RW<BusGatingReset<4>>,
    _reserved_0920: [u8; 0x01c],
    /// 0x093c - `CCU_SCR_BGR_REG`.
    pub scr_bgr: RW<u32>,
    /// 0x0940 - `CCU_SPI0_CLK_REG, spi0_clk`.
    pub spi0_clk: RW<u32>,
    /// 0x0944 - `spi1_clk`.
    pub spi1: RW<u32>,
    _reserved_0948: [u8; 0x024],
    /// 0x096c - `CCU_SPI_BGR_REG, bus_spi0_clk, bus_spi1_clk, ...`.
    pub spi_bgr: RW<BusGatingReset<2>>,
    /// 0x0970 - `emac0_25m_clk`.
    pub emac0_25m: RW<u32>,
    _reserved_0974: [u8; 0x008],
    /// 0x097c - `bus_emac0_clk, reset map`.
    pub bus_emac0: RW<u32>,
    _reserved_0980: [u8; 0x040],
    /// 0x09c0 - `ir_tx_clk`.
    pub ir_tx: RW<u32>,
    _reserved_09c4: [u8; 0x008],
    /// 0x09cc - `bus_ir_tx_clk, reset map`.
    pub bus_ir_tx: RW<u32>,
    _reserved_09d0: [u8; 0x01c],
    /// 0x09ec - `bus_gpadc_clk, reset map`.
    pub bus_gpadc: RW<u32>,
    _reserved_09f0: [u8; 0x00c],
    /// 0x09fc - `bus_ths_clk, reset map`.
    pub bus_ths: RW<u32>,
    _reserved_0a00: [u8; 0x010],
    /// 0x0a10 - `i2s0_clk`.
    pub i2s0: RW<u32>,
    /// 0x0a14 - `i2s1_clk`.
    pub i2s1: RW<u32>,
    /// 0x0a18 - `i2s2_clk`.
    pub i2s2: RW<u32>,
    /// 0x0a1c - `i2s2_asrc_clk`.
    pub i2s2_asrc: RW<u32>,
    /// 0x0a20 - `bus_i2s0_clk, bus_i2s1_clk, bus_i2s2_clk, ...`.
    pub bus_i2s0: RW<u32>,
    /// 0x0a24 - `owa_tx_clk`.
    pub owa_tx: RW<u32>,
    /// 0x0a28 - `owa_rx_clk`.
    pub owa_rx: RW<u32>,
    /// 0x0a2c - `bus_owa_clk, reset map`.
    pub bus_owa: RW<u32>,
    _reserved_0a30: [u8; 0x010],
    /// 0x0a40 - `dmic_clk`.
    pub dmic: RW<u32>,
    _reserved_0a44: [u8; 0x008],
    /// 0x0a4c - `bus_dmic_clk, reset map`.
    pub bus_dmic: RW<u32>,
    /// 0x0a50 - `audio_codec_dac_clk`.
    pub audio_codec_dac: RW<u32>,
    /// 0x0a54 - `audio_codec_adc_clk`.
    pub audio_codec_adc: RW<u32>,
    _reserved_0a58: [u8; 0x004],
    /// 0x0a5c - `bus_audio_codec_clk, reset map`.
    pub bus_audio_codec: RW<u32>,
    _reserved_0a60: [u8; 0x010],
    /// 0x0a70 - `CCU_USB0_CLK_REG, SUN8IW20_USB0_CLK_REG, reset map, ...`.
    pub usb0_clk: RW<u32>,
    /// 0x0a74 - `SUN8IW20_USB1_CLK_REG, reset map, usb_ohci1_clk`.
    pub usb1_clk: RW<u32>,
    _reserved_0a78: [u8; 0x014],
    /// 0x0a8c - `CCU_USB_BGR_REG, bus_ehci0_clk, bus_ehci1_clk, ...`.
    pub usb_bgr: RW<u32>,
    _reserved_0a90: [u8; 0x00c],
    /// 0x0a9c - `bus_lradc_clk, reset map`.
    pub bus_lradc: RW<u32>,
    _reserved_0aa0: [u8; 0x01c],
    /// 0x0abc - `bus_dpss_top0_clk, reset map`.
    pub bus_dpss_top0: RW<u32>,
    _reserved_0ac0: [u8; 0x044],
    /// 0x0b04 - `hdmi_24m_clk`.
    pub hdmi_24m: RW<u32>,
    _reserved_0b08: [u8; 0x008],
    /// 0x0b10 - `hdmi_cec_clk`.
    pub hdmi_cec: RW<u32>,
    _reserved_0b14: [u8; 0x008],
    /// 0x0b1c - `bus_hdmi_clk, reset map`.
    pub bus_hdmi: RW<u32>,
    _reserved_0b20: [u8; 0x004],
    /// 0x0b24 - `mipi_dsi_clk`.
    pub mipi_dsi: RW<u32>,
    _reserved_0b28: [u8; 0x024],
    /// 0x0b4c - `bus_mipi_dsi_clk, reset map`.
    pub bus_mipi_dsi: RW<u32>,
    _reserved_0b50: [u8; 0x010],
    /// 0x0b60 - `tcon_lcd0_clk`.
    pub tcon_lcd0: RW<u32>,
    _reserved_0b64: [u8; 0x018],
    /// 0x0b7c - `bus_tcon_lcd0_clk, reset map`.
    pub bus_tcon_lcd0: RW<u32>,
    /// 0x0b80 - `tcon_tv_clk`.
    pub tcon_tv: RW<u32>,
    _reserved_0b84: [u8; 0x018],
    /// 0x0b9c - `bus_tcon_tv_clk, reset map`.
    pub bus_tcon_tv: RW<u32>,
    _reserved_0ba0: [u8; 0x00c],
    /// 0x0bac - `reset map`.
    pub register_0bac: RW<u32>,
    /// 0x0bb0 - `tve_clk`.
    pub tve: RW<u32>,
    _reserved_0bb4: [u8; 0x008],
    /// 0x0bbc - `bus_tve_clk, bus_tve_top_clk, reset map`.
    pub bus_tve_top: RW<u32>,
    /// 0x0bc0 - `tvd_clk`.
    pub tvd: RW<u32>,
    _reserved_0bc4: [u8; 0x018],
    /// 0x0bdc - `bus_tvd_clk, bus_tvd_top_clk, reset map`.
    pub bus_tvd_top: RW<u32>,
    _reserved_0be0: [u8; 0x010],
    /// 0x0bf0 - `ledc_clk`.
    pub ledc: RW<u32>,
    _reserved_0bf4: [u8; 0x008],
    /// 0x0bfc - `bus_ledc_clk, reset map`.
    pub bus_ledc: RW<u32>,
    _reserved_0c00: [u8; 0x004],
    /// 0x0c04 - `csi_top_clk`.
    pub csi_top: RW<u32>,
    /// 0x0c08 - `csi0_mclk_clk`.
    pub csi0_mclk: RW<u32>,
    _reserved_0c0c: [u8; 0x010],
    /// 0x0c1c - `bus_csi_clk, reset map`.
    pub bus_csi: RW<u32>,
    _reserved_0c20: [u8; 0x030],
    /// 0x0c50 - `tpadc_clk`.
    pub tpadc: RW<u32>,
    _reserved_0c54: [u8; 0x008],
    /// 0x0c5c - `bus_tpadc_clk, reset map`.
    pub bus_tpadc: RW<u32>,
    _reserved_0c60: [u8; 0x00c],
    /// 0x0c6c - `bus_tzma_clk`.
    pub bus_tzma: RW<u32>,
    /// 0x0c70 - `CCU_DSP_CLK_REG, dsp_clk`.
    pub dsp_clk: RW<u32>,
    _reserved_0c74: [u8; 0x008],
    /// 0x0c7c - `CCU_DSP_BGR_REG, bus_dsp_cfg_clk, reset map`.
    pub dsp_bgr: RW<u32>,
    _reserved_0c80: [u8; 0x080],
    /// 0x0d00 - `CCU_RISCV_CLK_REG, riscv_axi_clk, riscv_clk`.
    pub riscv_clk: RW<u32>,
    /// 0x0d04 - `CCU_RISCV_GATING_RST_REG, riscv_bus_clk`.
    pub riscv_gating_rst: RW<u32>,
    _reserved_0d08: [u8; 0x004],
    /// 0x0d0c - `CCU_RISCV_CFG_BGR_REG, bus_riscv_cfg_clk, reset map`.
    pub riscv_cfg_bgr: RW<u32>,
    _reserved_0d10: [u8; 0x210],
    /// 0x0f20 - `CCU_RISCV_RST_REG, riscv_rst_clk`.
    pub riscv_rst: RW<u32>,
    _reserved_0f24: [u8; 0x00c],
    /// 0x0f30 - `fanout_12m_clk, fanout_16m_clk, fanout_24m_clk, ...`.
    pub fanout_12m: RW<u32>,
    /// 0x0f34 - `fanout_27m_clk`.
    pub fanout_27m: RW<u32>,
    /// 0x0f38 - `fanout_pclk`.
    pub fanout_pclk: RW<u32>,
    /// 0x0f3c - `fanout0_out_clk, fanout1_out_clk, fanout2_out_clk`.
    pub fanout0_out: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_ctrl), 0x000);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_ctrl), 0x010);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_ctrl), 0x020);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_ctrl), 0x028);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_ctrl), 0x030);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_ctrl), 0x040);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_ctrl), 0x048);
        assert_eq!(offset_of!(RegisterBlock, pll_ve), 0x058);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_ctrl), 0x078);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_ctrl), 0x080);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat0_ctrl), 0x110);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat1_ctrl), 0x114);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat0_ctrl), 0x120);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat1_ctrl), 0x124);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_pat0_ctrl), 0x128);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_pat1_ctrl), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_pat0_ctrl), 0x130);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_pat1_ctrl), 0x134);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat0_ctrl), 0x140);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat1_ctrl), 0x144);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat0_ctrl), 0x148);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat1_ctrl), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_pat0_ctrl), 0x158);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_pat1_ctrl), 0x15c);
        assert_eq!(offset_of!(RegisterBlock, pll_de_pat0_ctrl), 0x160);
        assert_eq!(offset_of!(RegisterBlock, pll_de_pat1_ctrl), 0x164);
        assert_eq!(offset_of!(RegisterBlock, pll_hsic_pat0_ctrl), 0x170);
        assert_eq!(offset_of!(RegisterBlock, pll_hsic_pat1_ctrl), 0x174);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_pat0_ctrl), 0x178);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_pat1_ctrl), 0x17c);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_pat0_ctrl), 0x180);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_pat1_ctrl), 0x184);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_bias), 0x300);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_bias), 0x310);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_bias), 0x320);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_bias), 0x328);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu_bias), 0x330);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_bias), 0x340);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_bias), 0x348);
        assert_eq!(offset_of!(RegisterBlock, pll_ve_bias), 0x358);
        assert_eq!(offset_of!(RegisterBlock, pll_de_bias), 0x360);
        assert_eq!(offset_of!(RegisterBlock, pll_hsic_bias), 0x370);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_bias), 0x378);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_bias), 0x380);
        assert_eq!(offset_of!(RegisterBlock, pll_cpu_tun), 0x400);
        assert_eq!(offset_of!(RegisterBlock, cpu_axi_cfg), 0x500);
        assert_eq!(offset_of!(RegisterBlock, cpu_gating), 0x504);
        assert_eq!(offset_of!(RegisterBlock, psi_clk), 0x510);
        assert_eq!(offset_of!(RegisterBlock, ahb3_clk), 0x51c);
        assert_eq!(offset_of!(RegisterBlock, apb0_clk), 0x520);
        assert_eq!(offset_of!(RegisterBlock, apb1_clk), 0x524);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk), 0x540);
        assert_eq!(offset_of!(RegisterBlock, de0), 0x600);
        assert_eq!(offset_of!(RegisterBlock, bus_de0), 0x60c);
        assert_eq!(offset_of!(RegisterBlock, di), 0x620);
        assert_eq!(offset_of!(RegisterBlock, bus_di), 0x62c);
        assert_eq!(offset_of!(RegisterBlock, g2d), 0x630);
        assert_eq!(offset_of!(RegisterBlock, bus_g2d), 0x63c);
        assert_eq!(offset_of!(RegisterBlock, ce), 0x680);
        assert_eq!(offset_of!(RegisterBlock, bus_ce), 0x68c);
        assert_eq!(offset_of!(RegisterBlock, ve), 0x690);
        assert_eq!(offset_of!(RegisterBlock, bus_ve), 0x69c);
        assert_eq!(offset_of!(RegisterBlock, dma_bgr), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, bus_msgbox0), 0x71c);
        assert_eq!(offset_of!(RegisterBlock, bus_spinlock), 0x72c);
        assert_eq!(offset_of!(RegisterBlock, bus_hstimer), 0x73c);
        assert_eq!(offset_of!(RegisterBlock, avs), 0x740);
        assert_eq!(offset_of!(RegisterBlock, bus_dbg), 0x78c);
        assert_eq!(offset_of!(RegisterBlock, bus_pwm), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, bus_iommu), 0x7bc);
        assert_eq!(offset_of!(RegisterBlock, dram_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gating), 0x804);
        assert_eq!(offset_of!(RegisterBlock, dram_bgr), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, smhc0_clk), 0x830);
        assert_eq!(offset_of!(RegisterBlock, smhc1_clk), 0x834);
        assert_eq!(offset_of!(RegisterBlock, smhc2_clk), 0x838);
        assert_eq!(offset_of!(RegisterBlock, smhc_bgr), 0x84c);
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, twi_bgr), 0x91c);
        assert_eq!(offset_of!(RegisterBlock, scr_bgr), 0x93c);
        assert_eq!(offset_of!(RegisterBlock, spi0_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi1), 0x944);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
        assert_eq!(offset_of!(RegisterBlock, emac0_25m), 0x970);
        assert_eq!(offset_of!(RegisterBlock, bus_emac0), 0x97c);
        assert_eq!(offset_of!(RegisterBlock, ir_tx), 0x9c0);
        assert_eq!(offset_of!(RegisterBlock, bus_ir_tx), 0x9cc);
        assert_eq!(offset_of!(RegisterBlock, bus_gpadc), 0x9ec);
        assert_eq!(offset_of!(RegisterBlock, bus_ths), 0x9fc);
        assert_eq!(offset_of!(RegisterBlock, i2s0), 0xa10);
        assert_eq!(offset_of!(RegisterBlock, i2s1), 0xa14);
        assert_eq!(offset_of!(RegisterBlock, i2s2), 0xa18);
        assert_eq!(offset_of!(RegisterBlock, i2s2_asrc), 0xa1c);
        assert_eq!(offset_of!(RegisterBlock, bus_i2s0), 0xa20);
        assert_eq!(offset_of!(RegisterBlock, owa_tx), 0xa24);
        assert_eq!(offset_of!(RegisterBlock, owa_rx), 0xa28);
        assert_eq!(offset_of!(RegisterBlock, bus_owa), 0xa2c);
        assert_eq!(offset_of!(RegisterBlock, dmic), 0xa40);
        assert_eq!(offset_of!(RegisterBlock, bus_dmic), 0xa4c);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_dac), 0xa50);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_adc), 0xa54);
        assert_eq!(offset_of!(RegisterBlock, bus_audio_codec), 0xa5c);
        assert_eq!(offset_of!(RegisterBlock, usb0_clk), 0xa70);
        assert_eq!(offset_of!(RegisterBlock, usb1_clk), 0xa74);
        assert_eq!(offset_of!(RegisterBlock, usb_bgr), 0xa8c);
        assert_eq!(offset_of!(RegisterBlock, bus_lradc), 0xa9c);
        assert_eq!(offset_of!(RegisterBlock, bus_dpss_top0), 0xabc);
        assert_eq!(offset_of!(RegisterBlock, hdmi_24m), 0xb04);
        assert_eq!(offset_of!(RegisterBlock, hdmi_cec), 0xb10);
        assert_eq!(offset_of!(RegisterBlock, bus_hdmi), 0xb1c);
        assert_eq!(offset_of!(RegisterBlock, mipi_dsi), 0xb24);
        assert_eq!(offset_of!(RegisterBlock, bus_mipi_dsi), 0xb4c);
        assert_eq!(offset_of!(RegisterBlock, tcon_lcd0), 0xb60);
        assert_eq!(offset_of!(RegisterBlock, bus_tcon_lcd0), 0xb7c);
        assert_eq!(offset_of!(RegisterBlock, tcon_tv), 0xb80);
        assert_eq!(offset_of!(RegisterBlock, bus_tcon_tv), 0xb9c);
        assert_eq!(offset_of!(RegisterBlock, register_0bac), 0xbac);
        assert_eq!(offset_of!(RegisterBlock, tve), 0xbb0);
        assert_eq!(offset_of!(RegisterBlock, bus_tve_top), 0xbbc);
        assert_eq!(offset_of!(RegisterBlock, tvd), 0xbc0);
        assert_eq!(offset_of!(RegisterBlock, bus_tvd_top), 0xbdc);
        assert_eq!(offset_of!(RegisterBlock, ledc), 0xbf0);
        assert_eq!(offset_of!(RegisterBlock, bus_ledc), 0xbfc);
        assert_eq!(offset_of!(RegisterBlock, csi_top), 0xc04);
        assert_eq!(offset_of!(RegisterBlock, csi0_mclk), 0xc08);
        assert_eq!(offset_of!(RegisterBlock, bus_csi), 0xc1c);
        assert_eq!(offset_of!(RegisterBlock, tpadc), 0xc50);
        assert_eq!(offset_of!(RegisterBlock, bus_tpadc), 0xc5c);
        assert_eq!(offset_of!(RegisterBlock, bus_tzma), 0xc6c);
        assert_eq!(offset_of!(RegisterBlock, dsp_clk), 0xc70);
        assert_eq!(offset_of!(RegisterBlock, dsp_bgr), 0xc7c);
        assert_eq!(offset_of!(RegisterBlock, riscv_clk), 0xd00);
        assert_eq!(offset_of!(RegisterBlock, riscv_gating_rst), 0xd04);
        assert_eq!(offset_of!(RegisterBlock, riscv_cfg_bgr), 0xd0c);
        assert_eq!(offset_of!(RegisterBlock, riscv_rst), 0xf20);
        assert_eq!(offset_of!(RegisterBlock, fanout_12m), 0xf30);
        assert_eq!(offset_of!(RegisterBlock, fanout_27m), 0xf34);
        assert_eq!(offset_of!(RegisterBlock, fanout_pclk), 0xf38);
        assert_eq!(offset_of!(RegisterBlock, fanout0_out), 0xf3c);
        assert_eq!(size_of::<RegisterBlock>(), 0xf40);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
