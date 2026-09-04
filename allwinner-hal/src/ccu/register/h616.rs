//! H616/H313/H618 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun50iw9` platform.

use super::{BusGatingReset, SingleBusGatingReset};
use volatile_register::RW;

/// H616/H313/H618 main CCU register block.
#[doc(alias = "sun50iw9")]
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - `CCU_PLL_CPUX_CTRL_REG, SUN50IW9_PLL_CPUX_REG, pll_cpux_clk`.
    pub pll_cpux_ctrl: RW<u32>,
    _reserved_0004: [u8; 0x00c],
    /// 0x0010 - `CCU_PLL_DDR0_CTRL_REG, SUN50IW9_PLL_DDR0_REG, pll_ddr0_clk`.
    pub pll_ddr0_ctrl: RW<u32>,
    _reserved_0014: [u8; 0x004],
    /// 0x0018 - `CCU_PLL_DDR1_CTRL_REG, SUN50IW9_PLL_DDR1_REG, pll_ddr1_clk`.
    pub pll_ddr1_ctrl: RW<u32>,
    _reserved_001c: [u8; 0x004],
    /// 0x0020 - `CCU_PLL_PERI0_CTRL_REG, SUN50IW9_PLL_PERIPH0_REG, pll_periph0_clk`.
    pub pll_peri0_ctrl: RW<u32>,
    _reserved_0024: [u8; 0x004],
    /// 0x0028 - `CCU_PLL_PERI1_CTRL_REG, SUN50IW9_PLL_PERIPH1_REG, pll_periph1_clk`.
    pub pll_peri1_ctrl: RW<u32>,
    _reserved_002c: [u8; 0x004],
    /// 0x0030 - `SUN50IW9_PLL_GPU_REG, pll_gpu_clk`.
    pub pll_gpu: RW<u32>,
    _reserved_0034: [u8; 0x00c],
    /// 0x0040 - `SUN50IW9_PLL_VIDEO0_REG, pll_video0_clk`.
    pub pll_video0: RW<u32>,
    _reserved_0044: [u8; 0x004],
    /// 0x0048 - `SUN50IW9_PLL_VIDEO1_REG, pll_video1_clk`.
    pub pll_video1: RW<u32>,
    _reserved_004c: [u8; 0x004],
    /// 0x0050 - `SUN50IW9_PLL_VIDEO2_REG, pll_video2_clk`.
    pub pll_video2: RW<u32>,
    _reserved_0054: [u8; 0x004],
    /// 0x0058 - `SUN50IW9_PLL_VE_REG, pll_ve_clk`.
    pub pll_ve: RW<u32>,
    _reserved_005c: [u8; 0x004],
    /// 0x0060 - `SUN50IW9_PLL_DE_REG, pll_de_clk`.
    pub pll_de: RW<u32>,
    _reserved_0064: [u8; 0x00c],
    /// 0x0070 - `CCU_PLL_HSIC_CTRL_REG`.
    pub pll_hsic_ctrl: RW<u32>,
    _reserved_0074: [u8; 0x004],
    /// 0x0078 - `SUN50IW9_PLL_AUDIO_REG, pll_audio_4x_clk`.
    pub pll_audio: RW<u32>,
    _reserved_007c: [u8; 0x064],
    /// 0x00e0 - `SUN50IW9_PLL_CSI_REG, pll_csi_clk`.
    pub pll_csi: RW<u32>,
    _reserved_00e4: [u8; 0x044],
    /// 0x0128 - `SUN50IW9_PLL_PERIPH1_PATTERN0_REG, pll_periph1_pattern0_common`.
    pub pll_periph1_pattern0: RW<u32>,
    _reserved_012c: [u8; 0x3d4],
    /// 0x0500 - `CCU_CPUX_AXI_CFG_REG, axi_clk, cpux_apb_clk, ...`.
    pub cpux_axi_cfg: RW<u32>,
    _reserved_0504: [u8; 0x00c],
    /// 0x0510 - `CCU_PSI_AHB1_AHB2_CFG_REG, psi_ahb1_ahb2_clk`.
    pub psi_ahb1_ahb2_cfg: RW<u32>,
    _reserved_0514: [u8; 0x008],
    /// 0x051c - `CCU_AHB3_CFG_GREG, ahb3_clk`.
    pub ahb3_cfg: RW<u32>,
    /// 0x0520 - `CCU_APB1_CFG_GREG, apb1_clk`.
    pub apb1_cfg: RW<u32>,
    /// 0x0524 - `CCU_APB2_CFG_GREG, apb2_clk`.
    pub apb2_cfg: RW<u32>,
    _reserved_0528: [u8; 0x018],
    /// 0x0540 - `CCU_MBUS_CFG_REG, mbus_clk, reset map`.
    pub mbus_cfg: RW<u32>,
    _reserved_0544: [u8; 0x0bc],
    /// 0x0600 - `de_clk`.
    pub de: RW<u32>,
    _reserved_0604: [u8; 0x008],
    /// 0x060c - `bus_de_clk, reset map`.
    pub bus_de: RW<u32>,
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
    _reserved_0640: [u8; 0x030],
    /// 0x0670 - `gpu0_clk`.
    pub gpu0: RW<u32>,
    /// 0x0674 - `gpu1_clk`.
    pub gpu1: RW<u32>,
    _reserved_0678: [u8; 0x004],
    /// 0x067c - `bus_gpu_clk, reset map`.
    pub bus_gpu: RW<u32>,
    /// 0x0680 - `CCU_CE_CLK_REG, ce_clk`.
    pub ce_clk: RW<u32>,
    _reserved_0684: [u8; 0x008],
    /// 0x068c - `CCU_CE_BGR_REG, bus_ce_clk, reset map`.
    pub ce_bgr: RW<u32>,
    /// 0x0690 - `CCU_VE_CLK_REG, ve_clk`.
    pub ve_clk: RW<u32>,
    _reserved_0694: [u8; 0x008],
    /// 0x069c - `CCU_VE_BGR_REG, bus_ve_clk, reset map`.
    pub ve_bgr: RW<u32>,
    _reserved_06a0: [u8; 0x06c],
    /// 0x070c - `CCU_DMA_BGR_REG, bus_dma_clk, reset map`.
    pub dma_bgr: RW<u32>,
    _reserved_0710: [u8; 0x02c],
    /// 0x073c - `bus_hstimer_clk, reset map`.
    pub bus_hstimer: RW<u32>,
    /// 0x0740 - `CCU_AVS_CLK_REG, avs_clk`.
    pub avs_clk: RW<u32>,
    _reserved_0744: [u8; 0x008],
    /// 0x074c - `CCU_AVS_BGR_REG`.
    pub avs_bgr: RW<u32>,
    _reserved_0750: [u8; 0x03c],
    /// 0x078c - `bus_dbg_clk, reset map`.
    pub bus_dbg: RW<u32>,
    _reserved_0790: [u8; 0x00c],
    /// 0x079c - `bus_psi_clk, reset map`.
    pub bus_psi: RW<u32>,
    _reserved_07a0: [u8; 0x00c],
    /// 0x07ac - `bus_pwm_clk, reset map`.
    pub bus_pwm: RW<u32>,
    _reserved_07b0: [u8; 0x00c],
    /// 0x07bc - `CCU_IOMMU_BGR_REG, bus_iommu_clk`.
    pub iommu_bgr: RW<u32>,
    _reserved_07c0: [u8; 0x040],
    /// 0x0800 - `CCU_DRAM_CLK_REG, dram_clk`.
    pub dram_clk: RW<u32>,
    /// 0x0804 - `CCU_MBUS_MAT_CLK_GATING_REG, mbus_ce_clk, mbus_csi_clk, ...`.
    pub mbus_mat_clk_gating: RW<u32>,
    /// 0x0808 - `CCU_PLL_DDR_AUX_REG`.
    pub pll_ddr_aux: RW<u32>,
    /// 0x080c - `CCU_DRAM_BGR_REG, bus_dram_clk, reset map`.
    pub dram_bgr: RW<SingleBusGatingReset>,
    /// 0x0810 - `CCU_NAND_CLK_REG, nand0_clk`.
    pub nand_clk: RW<u32>,
    /// 0x0814 - `nand1_clk`.
    pub nand1: RW<u32>,
    _reserved_0818: [u8; 0x014],
    /// 0x082c - `CCU_NAND_BGR_REG, bus_nand_clk, reset map`.
    pub nand_bgr: RW<u32>,
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
    pub twi_bgr: RW<BusGatingReset<5>>,
    _reserved_0920: [u8; 0x01c],
    /// 0x093c - `CCU_SCR_BGR_REG, bus_scr_clk, reset map`.
    pub scr_bgr: RW<u32>,
    /// 0x0940 - `CCU_SPI0_CLK_REG, spi0_clk`.
    pub spi0_clk: RW<u32>,
    /// 0x0944 - `CCU_SPI1_CLK_REG, spi1_clk`.
    pub spi1_clk: RW<u32>,
    _reserved_0948: [u8; 0x024],
    /// 0x096c - `CCU_SPI_BGR_REG, bus_spi0_clk, bus_spi1_clk, ...`.
    pub spi_bgr: RW<BusGatingReset<2>>,
    /// 0x0970 - `emac_25m_clk`.
    pub emac_25m: RW<u32>,
    _reserved_0974: [u8; 0x008],
    /// 0x097c - `bus_emac0_clk, bus_emac1_clk, reset map`.
    pub bus_emac0: RW<u32>,
    _reserved_0980: [u8; 0x030],
    /// 0x09b0 - `ts_clk`.
    pub ts: RW<u32>,
    _reserved_09b4: [u8; 0x008],
    /// 0x09bc - `bus_ts_clk, reset map`.
    pub bus_ts: RW<u32>,
    _reserved_09c0: [u8; 0x02c],
    /// 0x09ec - `CCU_GPADC_BGR_REG, bus_gpadc_clk, reset map`.
    pub gpadc_bgr: RW<u32>,
    _reserved_09f0: [u8; 0x00c],
    /// 0x09fc - `bus_ths_clk, reset map`.
    pub bus_ths: RW<u32>,
    _reserved_0a00: [u8; 0x020],
    /// 0x0a20 - `owa_clk`.
    pub owa: RW<u32>,
    _reserved_0a24: [u8; 0x008],
    /// 0x0a2c - `bus_owa_clk, reset map`.
    pub bus_owa: RW<u32>,
    _reserved_0a30: [u8; 0x010],
    /// 0x0a40 - `dmic_clk`.
    pub dmic: RW<u32>,
    _reserved_0a44: [u8; 0x008],
    /// 0x0a4c - `bus_dmic_clk, reset map`.
    pub bus_dmic: RW<u32>,
    /// 0x0a50 - `audio_codec_clk`.
    pub audio_codec: RW<u32>,
    /// 0x0a54 - `audio_codec_4x_clk`.
    pub audio_codec_4x: RW<u32>,
    _reserved_0a58: [u8; 0x004],
    /// 0x0a5c - `bus_audio_codec_clk, reset map`.
    pub bus_audio_codec: RW<u32>,
    /// 0x0a60 - `audio_hub_clk`.
    pub audio_hub: RW<u32>,
    _reserved_0a64: [u8; 0x008],
    /// 0x0a6c - `bus_audio_hub_clk, reset map`.
    pub bus_audio_hub: RW<u32>,
    /// 0x0a70 - `CCU_USB0_CLK_REG, SUN50IW9_USB0_CLK_REG, reset map, ...`.
    pub usb0_clk: RW<u32>,
    /// 0x0a74 - `SUN50IW9_USB1_CLK_REG, reset map, usb_ohci1_clk, ...`.
    pub usb1_clk: RW<u32>,
    /// 0x0a78 - `SUN50IW9_USB2_CLK_REG, reset map, usb_ohci2_clk, ...`.
    pub usb2_clk: RW<u32>,
    /// 0x0a7c - `SUN50IW9_USB3_CLK_REG, reset map, usb_ohci3_clk, ...`.
    pub usb3_clk: RW<u32>,
    _reserved_0a80: [u8; 0x00c],
    /// 0x0a8c - `CCU_USB_BGR_REG, bus_ehci0_clk, bus_ehci1_clk, ...`.
    pub usb_bgr: RW<u32>,
    _reserved_0a90: [u8; 0x00c],
    /// 0x0a9c - `bus_lradc_clk, reset map`.
    pub bus_lradc: RW<u32>,
    _reserved_0aa0: [u8; 0x060],
    /// 0x0b00 - `hdmi_clk`.
    pub hdmi: RW<u32>,
    /// 0x0b04 - `hdmi_slow_clk`.
    pub hdmi_slow: RW<u32>,
    _reserved_0b08: [u8; 0x008],
    /// 0x0b10 - `hdmi_cec_clk, pll_periph0_2x_div_clk`.
    pub pll_periph0_2x_div: RW<u32>,
    _reserved_0b14: [u8; 0x008],
    /// 0x0b1c - `bus_hdmi_clk, reset map`.
    pub bus_hdmi: RW<u32>,
    _reserved_0b20: [u8; 0x03c],
    /// 0x0b5c - `bus_display_if_top_clk, reset map`.
    pub bus_display_if_top: RW<u32>,
    /// 0x0b60 - `tcon_lcd0_clk`.
    pub tcon_lcd0: RW<u32>,
    /// 0x0b64 - `tcon_lcd1_clk`.
    pub tcon_lcd1: RW<u32>,
    _reserved_0b68: [u8; 0x014],
    /// 0x0b7c - `bus_tcon_lcd0_clk, bus_tcon_lcd1_clk, reset map`.
    pub bus_tcon_lcd0: RW<u32>,
    /// 0x0b80 - `tcon_tv0_clk`.
    pub tcon_tv0: RW<u32>,
    /// 0x0b84 - `tcon_tv1_clk`.
    pub tcon_tv1: RW<u32>,
    _reserved_0b88: [u8; 0x014],
    /// 0x0b9c - `bus_tcon_tv0_clk, bus_tcon_tv1_clk, reset map`.
    pub bus_tcon_tv0: RW<u32>,
    _reserved_0ba0: [u8; 0x00c],
    /// 0x0bac - `reset map`.
    pub register_0bac: RW<u32>,
    /// 0x0bb0 - `tve_clk`.
    pub tve: RW<u32>,
    _reserved_0bb4: [u8; 0x008],
    /// 0x0bbc - `bus_tve_clk, bus_tve_top_clk, reset map`.
    pub bus_tve_top: RW<u32>,
    _reserved_0bc0: [u8; 0x044],
    /// 0x0c04 - `csi_top_clk`.
    pub csi_top: RW<u32>,
    /// 0x0c08 - `csi0_mclk_clk`.
    pub csi0_mclk: RW<u32>,
    /// 0x0c0c - `csi1_mclk_clk`.
    pub csi1_mclk: RW<u32>,
    _reserved_0c10: [u8; 0x01c],
    /// 0x0c2c - `bus_csi_clk, reset map`.
    pub bus_csi: RW<u32>,
    _reserved_0c30: [u8; 0x010],
    /// 0x0c40 - `hdmi_hdcp_clk`.
    pub hdmi_hdcp: RW<u32>,
    _reserved_0c44: [u8; 0x008],
    /// 0x0c4c - `bus_hdmi_hdcp_clk, reset map`.
    pub bus_hdmi_hdcp: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(offset_of!(RegisterBlock, pll_cpux_ctrl), 0x000);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr0_ctrl), 0x010);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr1_ctrl), 0x018);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_ctrl), 0x020);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_ctrl), 0x028);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu), 0x030);
        assert_eq!(offset_of!(RegisterBlock, pll_video0), 0x040);
        assert_eq!(offset_of!(RegisterBlock, pll_video1), 0x048);
        assert_eq!(offset_of!(RegisterBlock, pll_video2), 0x050);
        assert_eq!(offset_of!(RegisterBlock, pll_ve), 0x058);
        assert_eq!(offset_of!(RegisterBlock, pll_de), 0x060);
        assert_eq!(offset_of!(RegisterBlock, pll_hsic_ctrl), 0x070);
        assert_eq!(offset_of!(RegisterBlock, pll_audio), 0x078);
        assert_eq!(offset_of!(RegisterBlock, pll_csi), 0x0e0);
        assert_eq!(offset_of!(RegisterBlock, pll_periph1_pattern0), 0x128);
        assert_eq!(offset_of!(RegisterBlock, cpux_axi_cfg), 0x500);
        assert_eq!(offset_of!(RegisterBlock, psi_ahb1_ahb2_cfg), 0x510);
        assert_eq!(offset_of!(RegisterBlock, ahb3_cfg), 0x51c);
        assert_eq!(offset_of!(RegisterBlock, apb1_cfg), 0x520);
        assert_eq!(offset_of!(RegisterBlock, apb2_cfg), 0x524);
        assert_eq!(offset_of!(RegisterBlock, mbus_cfg), 0x540);
        assert_eq!(offset_of!(RegisterBlock, de), 0x600);
        assert_eq!(offset_of!(RegisterBlock, bus_de), 0x60c);
        assert_eq!(offset_of!(RegisterBlock, di), 0x620);
        assert_eq!(offset_of!(RegisterBlock, bus_di), 0x62c);
        assert_eq!(offset_of!(RegisterBlock, g2d), 0x630);
        assert_eq!(offset_of!(RegisterBlock, bus_g2d), 0x63c);
        assert_eq!(offset_of!(RegisterBlock, gpu0), 0x670);
        assert_eq!(offset_of!(RegisterBlock, gpu1), 0x674);
        assert_eq!(offset_of!(RegisterBlock, bus_gpu), 0x67c);
        assert_eq!(offset_of!(RegisterBlock, ce_clk), 0x680);
        assert_eq!(offset_of!(RegisterBlock, ce_bgr), 0x68c);
        assert_eq!(offset_of!(RegisterBlock, ve_clk), 0x690);
        assert_eq!(offset_of!(RegisterBlock, ve_bgr), 0x69c);
        assert_eq!(offset_of!(RegisterBlock, dma_bgr), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, bus_hstimer), 0x73c);
        assert_eq!(offset_of!(RegisterBlock, avs_clk), 0x740);
        assert_eq!(offset_of!(RegisterBlock, avs_bgr), 0x74c);
        assert_eq!(offset_of!(RegisterBlock, bus_dbg), 0x78c);
        assert_eq!(offset_of!(RegisterBlock, bus_psi), 0x79c);
        assert_eq!(offset_of!(RegisterBlock, bus_pwm), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, iommu_bgr), 0x7bc);
        assert_eq!(offset_of!(RegisterBlock, dram_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gating), 0x804);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_aux), 0x808);
        assert_eq!(offset_of!(RegisterBlock, dram_bgr), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, nand_clk), 0x810);
        assert_eq!(offset_of!(RegisterBlock, nand1), 0x814);
        assert_eq!(offset_of!(RegisterBlock, nand_bgr), 0x82c);
        assert_eq!(offset_of!(RegisterBlock, smhc0_clk), 0x830);
        assert_eq!(offset_of!(RegisterBlock, smhc1_clk), 0x834);
        assert_eq!(offset_of!(RegisterBlock, smhc2_clk), 0x838);
        assert_eq!(offset_of!(RegisterBlock, smhc_bgr), 0x84c);
        assert_eq!(offset_of!(RegisterBlock, uart_bgr), 0x90c);
        assert_eq!(offset_of!(RegisterBlock, twi_bgr), 0x91c);
        assert_eq!(offset_of!(RegisterBlock, scr_bgr), 0x93c);
        assert_eq!(offset_of!(RegisterBlock, spi0_clk), 0x940);
        assert_eq!(offset_of!(RegisterBlock, spi1_clk), 0x944);
        assert_eq!(offset_of!(RegisterBlock, spi_bgr), 0x96c);
        assert_eq!(offset_of!(RegisterBlock, emac_25m), 0x970);
        assert_eq!(offset_of!(RegisterBlock, bus_emac0), 0x97c);
        assert_eq!(offset_of!(RegisterBlock, ts), 0x9b0);
        assert_eq!(offset_of!(RegisterBlock, bus_ts), 0x9bc);
        assert_eq!(offset_of!(RegisterBlock, gpadc_bgr), 0x9ec);
        assert_eq!(offset_of!(RegisterBlock, bus_ths), 0x9fc);
        assert_eq!(offset_of!(RegisterBlock, owa), 0xa20);
        assert_eq!(offset_of!(RegisterBlock, bus_owa), 0xa2c);
        assert_eq!(offset_of!(RegisterBlock, dmic), 0xa40);
        assert_eq!(offset_of!(RegisterBlock, bus_dmic), 0xa4c);
        assert_eq!(offset_of!(RegisterBlock, audio_codec), 0xa50);
        assert_eq!(offset_of!(RegisterBlock, audio_codec_4x), 0xa54);
        assert_eq!(offset_of!(RegisterBlock, bus_audio_codec), 0xa5c);
        assert_eq!(offset_of!(RegisterBlock, audio_hub), 0xa60);
        assert_eq!(offset_of!(RegisterBlock, bus_audio_hub), 0xa6c);
        assert_eq!(offset_of!(RegisterBlock, usb0_clk), 0xa70);
        assert_eq!(offset_of!(RegisterBlock, usb1_clk), 0xa74);
        assert_eq!(offset_of!(RegisterBlock, usb2_clk), 0xa78);
        assert_eq!(offset_of!(RegisterBlock, usb3_clk), 0xa7c);
        assert_eq!(offset_of!(RegisterBlock, usb_bgr), 0xa8c);
        assert_eq!(offset_of!(RegisterBlock, bus_lradc), 0xa9c);
        assert_eq!(offset_of!(RegisterBlock, hdmi), 0xb00);
        assert_eq!(offset_of!(RegisterBlock, hdmi_slow), 0xb04);
        assert_eq!(offset_of!(RegisterBlock, pll_periph0_2x_div), 0xb10);
        assert_eq!(offset_of!(RegisterBlock, bus_hdmi), 0xb1c);
        assert_eq!(offset_of!(RegisterBlock, bus_display_if_top), 0xb5c);
        assert_eq!(offset_of!(RegisterBlock, tcon_lcd0), 0xb60);
        assert_eq!(offset_of!(RegisterBlock, tcon_lcd1), 0xb64);
        assert_eq!(offset_of!(RegisterBlock, bus_tcon_lcd0), 0xb7c);
        assert_eq!(offset_of!(RegisterBlock, tcon_tv0), 0xb80);
        assert_eq!(offset_of!(RegisterBlock, tcon_tv1), 0xb84);
        assert_eq!(offset_of!(RegisterBlock, bus_tcon_tv0), 0xb9c);
        assert_eq!(offset_of!(RegisterBlock, register_0bac), 0xbac);
        assert_eq!(offset_of!(RegisterBlock, tve), 0xbb0);
        assert_eq!(offset_of!(RegisterBlock, bus_tve_top), 0xbbc);
        assert_eq!(offset_of!(RegisterBlock, csi_top), 0xc04);
        assert_eq!(offset_of!(RegisterBlock, csi0_mclk), 0xc08);
        assert_eq!(offset_of!(RegisterBlock, csi1_mclk), 0xc0c);
        assert_eq!(offset_of!(RegisterBlock, bus_csi), 0xc2c);
        assert_eq!(offset_of!(RegisterBlock, hdmi_hdcp), 0xc40);
        assert_eq!(offset_of!(RegisterBlock, bus_hdmi_hdcp), 0xc4c);
        assert_eq!(size_of::<RegisterBlock>(), 0xc50);
        assert_eq!(align_of::<RegisterBlock>(), 4);
    }
}
