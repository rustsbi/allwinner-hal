//! A733/T736 Clock Control Unit registers.
//!
//! This layout represents the vendor `sun60iw2` platform.

use super::SingleBusGatingReset;
use volatile_register::{RO, RW};

/// A733/T736 main CCU register block.
#[doc(alias = "sun60iw2")]
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - `PLL_REF_CTRL_REG, SUN60IW2_PLL_REF_CTRL_REG, pll_ref_clk`.
    pub pll_ref_ctrl: RW<u32>,
    /// 0x0004 - `PLL_REF_LOCK_CTRL_REG`.
    pub pll_ref_lock_ctrl: RW<u32>,
    _reserved_0008: [u8; 0x008],
    /// 0x0010 - `PLL_REF_BIAS_REG`.
    pub pll_ref_bias: RW<u32>,
    _reserved_0014: [u8; 0x00c],
    /// 0x0020 - `PLL_DDR_CTRL_REG, SUN60IW2_PLL_DDR_CTRL_REG, pll_ddr_clk`.
    pub pll_ddr_ctrl: RW<u32>,
    /// 0x0024 - `PLL_DDR_LOCK_CTRL_REG`.
    pub pll_ddr_lock_ctrl: RW<u32>,
    /// 0x0028 - `PLL_DDR_PAT0_CTRL_REG`.
    pub pll_ddr_pat0_ctrl: RW<u32>,
    /// 0x002c - `PLL_DDR_PAT1_CTRL_REG`.
    pub pll_ddr_pat1_ctrl: RW<u32>,
    /// 0x0030 - `PLL_DDR_BIAS_REG`.
    pub pll_ddr_bias: RW<u32>,
    _reserved_0034: [u8; 0x06c],
    /// 0x00a0 - `PLL_PERI0_CTRL_REG, SUN60IW2_PLL_PERI0_CTRL_REG, pll_peri0_2x_clk, ...`.
    pub pll_peri0_ctrl: RW<u32>,
    /// 0x00a4 - `PLL_PERI0_LOCK_CTRL_REG`.
    pub pll_peri0_lock_ctrl: RW<u32>,
    /// 0x00a8 - `PLL_PERI0_PAT0_CTRL_REG`.
    pub pll_peri0_pat0_ctrl: RW<u32>,
    /// 0x00ac - `PLL_PERI0_PAT1_CTRL_REG`.
    pub pll_peri0_pat1_ctrl: RW<u32>,
    /// 0x00b0 - `PLL_PERI0_BIAS_REG`.
    pub pll_peri0_bias: RW<u32>,
    _reserved_00b4: [u8; 0x00c],
    /// 0x00c0 - `PLL_PERI1_CTRL_REG, SUN60IW2_PLL_PERI1_CTRL_REG, pll_peri1_2x_clk, ...`.
    pub pll_peri1_ctrl: RW<u32>,
    /// 0x00c4 - `PLL_PERI1_LOCK_CTRL_REG`.
    pub pll_peri1_lock_ctrl: RW<u32>,
    /// 0x00c8 - `PLL_PERI1_PAT0_CTRL_REG`.
    pub pll_peri1_pat0_ctrl: RW<u32>,
    /// 0x00cc - `PLL_PERI1_PAT1_CTRL_REG`.
    pub pll_peri1_pat1_ctrl: RW<u32>,
    /// 0x00d0 - `PLL_PERI1_BIAS_REG`.
    pub pll_peri1_bias: RW<u32>,
    _reserved_00d4: [u8; 0x00c],
    /// 0x00e0 - `PLL_GPU0_CTRL_REG, SUN60IW2_PLL_GPU0_CTRL_REG, pll_gpu0_clk`.
    pub pll_gpu0_ctrl: RW<u32>,
    /// 0x00e4 - `PLL_GPU0_LOCK_CTRL_REG`.
    pub pll_gpu0_lock_ctrl: RW<u32>,
    /// 0x00e8 - `PLL_GPU0_PAT0_CTRL_REG`.
    pub pll_gpu0_pat0_ctrl: RW<u32>,
    /// 0x00ec - `PLL_GPU0_PAT1_CTRL_REG`.
    pub pll_gpu0_pat1_ctrl: RW<u32>,
    /// 0x00f0 - `PLL_GPU0_BIAS_REG`.
    pub pll_gpu0_bias: RW<u32>,
    _reserved_00f4: [u8; 0x02c],
    /// 0x0120 - `PLL_VIDEO0_CTRL_REG, SUN60IW2_PLL_VIDEO0_CTRL_REG, pll_video0_3x_clk, ...`.
    pub pll_video0_ctrl: RW<u32>,
    /// 0x0124 - `PLL_VIDEO0_LOCK_CTRL_REG`.
    pub pll_video0_lock_ctrl: RW<u32>,
    /// 0x0128 - `PLL_VIDEO0_PAT0_CTRL_REG`.
    pub pll_video0_pat0_ctrl: RW<u32>,
    /// 0x012c - `PLL_VIDEO0_PAT1_CTRL_REG`.
    pub pll_video0_pat1_ctrl: RW<u32>,
    /// 0x0130 - `PLL_VIDEO0_BIAS_REG`.
    pub pll_video0_bias: RW<u32>,
    _reserved_0134: [u8; 0x00c],
    /// 0x0140 - `PLL_VIDEO1_CTRL_REG, SUN60IW2_PLL_VIDEO1_CTRL_REG, pll_video1_3x_clk, ...`.
    pub pll_video1_ctrl: RW<u32>,
    /// 0x0144 - `PLL_VIDEO1_LOCK_CTRL_REG`.
    pub pll_video1_lock_ctrl: RW<u32>,
    /// 0x0148 - `PLL_VIDEO1_PAT0_CTRL_REG`.
    pub pll_video1_pat0_ctrl: RW<u32>,
    /// 0x014c - `PLL_VIDEO1_PAT1_CTRL_REG`.
    pub pll_video1_pat1_ctrl: RW<u32>,
    /// 0x0150 - `PLL_VIDEO1_BIAS_REG`.
    pub pll_video1_bias: RW<u32>,
    _reserved_0154: [u8; 0x00c],
    /// 0x0160 - `PLL_VIDEO2_CTRL_REG, SUN60IW2_PLL_VIDEO2_CTRL_REG, XO_CONTROL0_REG, ...`.
    pub pll_video2_ctrl: RW<u32>,
    /// 0x0164 - `PLL_VIDEO2_LOCK_CTRL_REG`.
    pub pll_video2_lock_ctrl: RW<u32>,
    /// 0x0168 - `PLL_VIDEO2_PAT0_CTRL_REG`.
    pub pll_video2_pat0_ctrl: RW<u32>,
    /// 0x016c - `PLL_VIDEO2_PAT1_CTRL_REG`.
    pub pll_video2_pat1_ctrl: RW<u32>,
    /// 0x0170 - `PLL_VIDEO2_BIAS_REG`.
    pub pll_video2_bias: RW<u32>,
    _reserved_0174: [u8; 0x0ac],
    /// 0x0220 - `PLL_VE0_CTRL_REG, SUN60IW2_PLL_VE0_CTRL_REG, pll_ve0_clk`.
    pub pll_ve0_ctrl: RW<u32>,
    /// 0x0224 - `PLL_VE0_LOCK_CTRL_REG`.
    pub pll_ve0_lock_ctrl: RW<u32>,
    /// 0x0228 - `PLL_VE0_PAT0_CTRL_REG`.
    pub pll_ve0_pat0_ctrl: RW<u32>,
    /// 0x022c - `PLL_VE0_PAT1_CTRL_REG`.
    pub pll_ve0_pat1_ctrl: RW<u32>,
    /// 0x0230 - `PLL_VE0_BIAS_REG`.
    pub pll_ve0_bias: RW<u32>,
    _reserved_0234: [u8; 0x00c],
    /// 0x0240 - `PLL_VE1_CTRL_REG, SUN60IW2_PLL_VE1_CTRL_REG, pll_ve1_clk`.
    pub pll_ve1_ctrl: RW<u32>,
    /// 0x0244 - `PLL_VE1_LOCK_CTRL_REG`.
    pub pll_ve1_lock_ctrl: RW<u32>,
    /// 0x0248 - `PLL_VE1_PAT0_CTRL_REG`.
    pub pll_ve1_pat0_ctrl: RW<u32>,
    /// 0x024c - `PLL_VE1_PAT1_CTRL_REG`.
    pub pll_ve1_pat1_ctrl: RW<u32>,
    /// 0x0250 - `PLL_VE1_BIAS_REG`.
    pub pll_ve1_bias: RW<u32>,
    _reserved_0254: [u8; 0x00c],
    /// 0x0260 - `PLL_AUDIO0_CTRL_REG, SUN60IW2_PLL_AUDIO0_CTRL_REG, pll_audio0_4x_clk`.
    pub pll_audio0_ctrl: RW<u32>,
    /// 0x0264 - `PLL_AUDIO0_LOCK_CTRL_REG`.
    pub pll_audio0_lock_ctrl: RW<u32>,
    /// 0x0268 - `PLL_AUDIO0_PAT0_CTRL_REG, pll_audio0_sdm_pat0_clk`.
    pub pll_audio0_pat0_ctrl: RW<u32>,
    /// 0x026c - `PLL_AUDIO0_PAT1_CTRL_REG, SUN60IW2_PLL_AUDIO0_PATTERN1_REG, pll_audio0_sdm_pat1_clk`.
    pub pll_audio0_pattern1: RW<u32>,
    /// 0x0270 - `PLL_AUDIO0_BIAS_REG`.
    pub pll_audio0_bias: RW<u32>,
    _reserved_0274: [u8; 0x00c],
    /// 0x0280 - `PLL_AUDIO1_CTRL_REG, SUN60IW2_PLL_AUDIO1_CTRL_REG, pll_audio1_clk, ...`.
    pub pll_audio1_ctrl: RW<u32>,
    /// 0x0284 - `PLL_AUDIO1_LOCK_CTRL_REG`.
    pub pll_audio1_lock_ctrl: RW<u32>,
    /// 0x0288 - `PLL_AUDIO1_PAT0_CTRL_REG, pll_audio1_sdm_pat0_clk`.
    pub pll_audio1_pat0_ctrl: RW<u32>,
    /// 0x028c - `PLL_AUDIO1_PAT1_CTRL_REG`.
    pub pll_audio1_pat1_ctrl: RW<u32>,
    /// 0x0290 - `PLL_AUDIO1_BIAS_REG`.
    pub pll_audio1_bias: RW<u32>,
    _reserved_0294: [u8; 0x00c],
    /// 0x02a0 - `PLL_NPU_CTRL_REG, SUN60IW2_PLL_NPU_CTRL_REG, pll_npu_clk`.
    pub pll_npu_ctrl: RW<u32>,
    /// 0x02a4 - `PLL_NPU_LOCK_CTRL_REG`.
    pub pll_npu_lock_ctrl: RW<u32>,
    /// 0x02a8 - `PLL_NPU_PAT0_CTRL_REG`.
    pub pll_npu_pat0_ctrl: RW<u32>,
    /// 0x02ac - `PLL_NPU_PAT1_CTRL_REG`.
    pub pll_npu_pat1_ctrl: RW<u32>,
    /// 0x02b0 - `PLL_NPU_BIAS_REG`.
    pub pll_npu_bias: RW<u32>,
    _reserved_02b4: [u8; 0x02c],
    /// 0x02e0 - `PLL_DE_CTRL_REG, SUN60IW2_PLL_DE_CTRL_REG, pll_de_3x_clk, ...`.
    pub pll_de_ctrl: RW<u32>,
    /// 0x02e4 - `PLL_DE_LOCK_CTRL_REG`.
    pub pll_de_lock_ctrl: RW<u32>,
    /// 0x02e8 - `PLL_DE_PAT0_CTRL_REG`.
    pub pll_de_pat0_ctrl: RW<u32>,
    /// 0x02ec - `PLL_DE_PAT1_CTRL_REG`.
    pub pll_de_pat1_ctrl: RW<u32>,
    /// 0x02f0 - `PLL_DE_BIAS_REG`.
    pub pll_de_bias: RW<u32>,
    _reserved_02f4: [u8; 0x20c],
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
    _reserved_053c: [u8; 0x004],
    /// 0x0540 - `TRACE_CLK_REG, trace_clk`.
    pub trace_clk: RW<u32>,
    _reserved_0544: [u8; 0x01c],
    /// 0x0560 - `GIC_CLK_REG, gic_clk`.
    pub gic_clk: RW<u32>,
    _reserved_0564: [u8; 0x004],
    /// 0x0568 - `CPU_PERI_CLK_REG, cpu_peri_clk`.
    pub cpu_peri_clk: RW<u32>,
    _reserved_056c: [u8; 0x008],
    /// 0x0574 - `ITS0_BGR_REG, its_pcie0_a_clk, reset map`.
    pub its0_bgr: RW<u32>,
    _reserved_0578: [u8; 0x008],
    /// 0x0580 - `NSI_CLK_REG, nsi_clk, reset map`.
    pub nsi_clk: RW<u32>,
    /// 0x0584 - `NSI_BGR_REG, nsi_cfg_clk, reset map`.
    pub nsi_bgr: RW<SingleBusGatingReset>,
    /// 0x0588 - `MBUS_CLK_REG, mbus_clk`.
    pub mbus_clk: RW<u32>,
    /// 0x058c - `IOMMU0_BGR_REG, iommu0_sys_h_clk, iommu0_sys_mbus_clk, ...`.
    pub iommu0_bgr: RW<SingleBusGatingReset>,
    _reserved_0590: [u8; 0x004],
    /// 0x0594 - `MSI_LITE0_BGR_REG, msi_lite0_clk, reset map`.
    pub msi_lite0_bgr: RW<SingleBusGatingReset>,
    _reserved_0598: [u8; 0x004],
    /// 0x059c - `MSI_LITE1_BGR_REG, msi_lite1_clk, reset map`.
    pub msi_lite1_bgr: RW<SingleBusGatingReset>,
    _reserved_05a0: [u8; 0x004],
    /// 0x05a4 - `MSI_LITE2_BGR_REG, msi_lite2_clk, reset map`.
    pub msi_lite2_bgr: RW<SingleBusGatingReset>,
    _reserved_05a8: [u8; 0x00c],
    /// 0x05b4 - `IOMMU1_BGR_REG, iommu1_sys_h_clk, iommu1_sys_mbus_clk, ...`.
    pub iommu1_bgr: RW<SingleBusGatingReset>,
    _reserved_05b8: [u8; 0x008],
    /// 0x05c0 - `AHB_MAT_CLK_GATING_REG, cpus_hclk_gate_clk, de_ahb_gate_clk, ...`.
    pub ahb_mat_clk_gating: RW<u32>,
    _reserved_05c4: [u8; 0x01c],
    /// 0x05e0 - `MBUS_MAT_CLK_GATING_REG, desys_mbus_gate_clk, gpu0_mbus_gate_clk, ...`.
    pub mbus_mat_clk_gating: RW<u32>,
    /// 0x05e4 - `MBUS_GATE_EN_REG, ce_mbus_clk, csi_mbus_clk, ...`.
    pub mbus_gate_en: RW<u32>,
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
    _reserved_0748: [u8; 0x03c],
    /// 0x0784 - `PWM0_BGR_REG, pwm0_clk, reset map`.
    pub pwm0_bgr: RW<SingleBusGatingReset>,
    _reserved_0788: [u8; 0x004],
    /// 0x078c - `PWM1_BGR_REG, pwm1_clk, reset map`.
    pub pwm1_bgr: RW<SingleBusGatingReset>,
    _reserved_0790: [u8; 0x014],
    /// 0x07a4 - `DBGSYS_BGR_REG, dbgsys_clk, reset map`.
    pub dbgsys_bgr: RW<SingleBusGatingReset>,
    _reserved_07a8: [u8; 0x004],
    /// 0x07ac - `SYSDAP_BGR_REG, reset map, sysdap_clk`.
    pub sysdap_bgr: RW<SingleBusGatingReset>,
    _reserved_07b0: [u8; 0x050],
    /// 0x0800 - `TIMER0_CLK0_CLK_REG, timer0_clk`.
    pub timer0_clk0_clk: RW<u32>,
    /// 0x0804 - `TIMER0_CLK1_CLK_REG, timer1_clk`.
    pub timer0_clk1_clk: RW<u32>,
    /// 0x0808 - `TIMER0_CLK2_CLK_REG, timer2_clk`.
    pub timer0_clk2_clk: RW<u32>,
    /// 0x080c - `TIMER0_CLK3_CLK_REG, timer3_clk`.
    pub timer0_clk3_clk: RW<u32>,
    /// 0x0810 - `TIMER0_CLK4_CLK_REG, timer4_clk`.
    pub timer0_clk4_clk: RW<u32>,
    /// 0x0814 - `TIMER0_CLK5_CLK_REG, timer5_clk`.
    pub timer0_clk5_clk: RW<u32>,
    /// 0x0818 - `TIMER0_CLK6_CLK_REG, timer6_clk`.
    pub timer0_clk6_clk: RW<u32>,
    /// 0x081c - `TIMER0_CLK7_CLK_REG, timer7_clk`.
    pub timer0_clk7_clk: RW<u32>,
    /// 0x0820 - `TIMER0_CLK8_CLK_REG, timer8_clk`.
    pub timer0_clk8_clk: RW<u32>,
    /// 0x0824 - `TIMER0_CLK9_CLK_REG, timer9_clk`.
    pub timer0_clk9_clk: RW<u32>,
    _reserved_0828: [u8; 0x028],
    /// 0x0850 - `TIMER0_BGR_REG, reset map, timer_bus_clk`.
    pub timer0_bgr: RW<SingleBusGatingReset>,
    _reserved_0854: [u8; 0x02c],
    /// 0x0880 - `AVS_CLK_REG, avs_clk`.
    pub avs_clk: RW<u32>,
    _reserved_0884: [u8; 0x17c],
    /// 0x0a00 - `DE0_CLK_REG, de0_clk`.
    pub de0_clk: RW<u32>,
    /// 0x0a04 - `DE0_BGR_REG, de0_gate_clk, reset map`.
    pub de0_bgr: RW<SingleBusGatingReset>,
    _reserved_0a08: [u8; 0x018],
    /// 0x0a20 - `DI_CLK_REG, di_clk`.
    pub di_clk: RW<u32>,
    /// 0x0a24 - `DI_BGR_REG, di_gate_clk, reset map`.
    pub di_bgr: RW<SingleBusGatingReset>,
    _reserved_0a28: [u8; 0x018],
    /// 0x0a40 - `G2D_CLK_REG, g2d_clk`.
    pub g2d_clk: RW<u32>,
    /// 0x0a44 - `G2D_BGR_REG, g2d_gate_clk, reset map`.
    pub g2d_bgr: RW<SingleBusGatingReset>,
    _reserved_0a48: [u8; 0x018],
    /// 0x0a60 - `EINK_CLK_REG, eink_clk`.
    pub eink_clk: RW<u32>,
    /// 0x0a64 - `EINK_PANEL_CLK_REG, eink_panel_clk`.
    pub eink_panel_clk: RW<u32>,
    _reserved_0a68: [u8; 0x004],
    /// 0x0a6c - `EINK_BGR_REG, eink_gate_clk, reset map`.
    pub eink_bgr: RW<SingleBusGatingReset>,
    _reserved_0a70: [u8; 0x004],
    /// 0x0a74 - `DE_SYS_BGR_REG, reset map`.
    pub de_sys_bgr: RW<u32>,
    _reserved_0a78: [u8; 0x008],
    /// 0x0a80 - `VE_ENC0_CLK_REG, ve_enc0_clk`.
    pub ve_enc0_clk: RW<u32>,
    _reserved_0a84: [u8; 0x004],
    /// 0x0a88 - `VE_DEC_CLK_REG, ve_dec_clk`.
    pub ve_dec_clk: RW<u32>,
    /// 0x0a8c - `VE_BGR_REG, reset map, ve_dec_bus_clk, ...`.
    pub ve_bgr: RW<SingleBusGatingReset>,
    _reserved_0a90: [u8; 0x030],
    /// 0x0ac0 - `CE_CLK_REG, ce_clk`.
    pub ce_clk: RW<u32>,
    /// 0x0ac4 - `CE_BGR_REG, ce_bus_clk, ce_sys_clk, ...`.
    pub ce_bgr: RW<SingleBusGatingReset>,
    _reserved_0ac8: [u8; 0x038],
    /// 0x0b00 - `NPU_CLK_REG, npu_clk`.
    pub npu_clk: RW<u32>,
    /// 0x0b04 - `NPU_BGR_REG, npu_bus_clk, reset map`.
    pub npu_bgr: RW<SingleBusGatingReset>,
    _reserved_0b08: [u8; 0x018],
    /// 0x0b20 - `gpu0_clk`.
    pub gpu0: RW<u32>,
    /// 0x0b24 - `gpu0_bus_clk, reset map`.
    pub gpu0_bus: RW<u32>,
    _reserved_0b28: [u8; 0x0d8],
    /// 0x0c00 - `DRAM0_CLK_REG, dram0_clk`.
    pub dram0_clk: RW<u32>,
    _reserved_0c04: [u8; 0x008],
    /// 0x0c0c - `DRAM0_BGR_REG, dram0_bus_clk, reset map`.
    pub dram0_bgr: RW<SingleBusGatingReset>,
    _reserved_0c10: [u8; 0x070],
    /// 0x0c80 - `NAND0_CLK0_CLK_REG, nand0_clk0_clk`.
    pub nand0_clk0_clk: RW<u32>,
    /// 0x0c84 - `NAND0_CLK1_CLK_REG, nand0_clk1_clk`.
    pub nand0_clk1_clk: RW<u32>,
    _reserved_0c88: [u8; 0x004],
    /// 0x0c8c - `NAND0_BGR_REG, nand0_bus_clk, reset map`.
    pub nand0_bgr: RW<SingleBusGatingReset>,
    _reserved_0c90: [u8; 0x070],
    /// 0x0d00 - `SMHC0_CLK_REG, smhc0_clk`.
    pub smhc0_clk: RW<u32>,
    _reserved_0d04: [u8; 0x008],
    /// 0x0d0c - `SMHC0_BGR_REG, reset map, smhc0_gate_clk`.
    pub smhc0_bgr: RW<SingleBusGatingReset>,
    /// 0x0d10 - `SMHC1_CLK_REG, smhc1_clk`.
    pub smhc1_clk: RW<u32>,
    _reserved_0d14: [u8; 0x008],
    /// 0x0d1c - `SMHC1_BGR_REG, reset map, smhc1_gate_clk`.
    pub smhc1_bgr: RW<SingleBusGatingReset>,
    /// 0x0d20 - `SMHC2_CLK_REG, smhc2_clk`.
    pub smhc2_clk: RW<u32>,
    _reserved_0d24: [u8; 0x008],
    /// 0x0d2c - `SMHC2_BGR_REG, reset map, smhc2_gate_clk`.
    pub smhc2_bgr: RW<SingleBusGatingReset>,
    /// 0x0d30 - `SMHC3_CLK_REG, smhc3_clk`.
    pub smhc3_clk: RW<u32>,
    _reserved_0d34: [u8; 0x008],
    /// 0x0d3c - `SMHC3_BGR_REG, reset map, smhc3_bus_clk`.
    pub smhc3_bgr: RW<SingleBusGatingReset>,
    _reserved_0d40: [u8; 0x040],
    /// 0x0d80 - `UFS_AXI_CLK_REG, ufs_axi_clk`.
    pub ufs_axi_clk: RW<u32>,
    /// 0x0d84 - `UFS_CFG_CLK_REG, ufs_cfg_clk`.
    pub ufs_cfg_clk: RW<u32>,
    _reserved_0d88: [u8; 0x004],
    /// 0x0d8c - `UFS_BGR_REG, reset map, ufs_clk`.
    pub ufs_bgr: RW<SingleBusGatingReset>,
    /// 0x0d90 - `UFS_REF_CLK_EN_REG`.
    pub ufs_ref_clk_en: RW<u32>,
    _reserved_0d94: [u8; 0x06c],
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
    _reserved_0e1c: [u8; 0x064],
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
    /// 0x0e9c - `TWI7_BGR_REG, reset map, twi7_clk`.
    pub twi7_bgr: RW<SingleBusGatingReset>,
    /// 0x0ea0 - `TWI8_BGR_REG, reset map, twi8_clk`.
    pub twi8_bgr: RW<SingleBusGatingReset>,
    /// 0x0ea4 - `TWI9_BGR_REG, reset map, twi9_clk`.
    pub twi9_bgr: RW<SingleBusGatingReset>,
    /// 0x0ea8 - `TWI10_BGR_REG, reset map, twi10_clk`.
    pub twi10_bgr: RW<SingleBusGatingReset>,
    /// 0x0eac - `TWI11_BGR_REG, reset map, twi11_clk`.
    pub twi11_bgr: RW<SingleBusGatingReset>,
    /// 0x0eb0 - `TWI12_BGR_REG, reset map, twi12_clk`.
    pub twi12_bgr: RW<SingleBusGatingReset>,
    _reserved_0eb4: [u8; 0x04c],
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
    /// 0x0fc0 - `GPADC0_24M_CLK_REG, gpadc0_24m_clk`.
    pub gpadc0_24m_clk: RW<u32>,
    /// 0x0fc4 - `GPADC0_BGR_REG, gpadc0_clk, reset map`.
    pub gpadc0_bgr: RW<SingleBusGatingReset>,
    _reserved_0fc8: [u8; 0x01c],
    /// 0x0fe4 - `THS0_BGR_REG, reset map, ths0_clk`.
    pub ths0_bgr: RW<SingleBusGatingReset>,
    _reserved_0fe8: [u8; 0x018],
    /// 0x1000 - `IRRX_CLK_REG, irrx_clk`.
    pub irrx_clk: RW<u32>,
    /// 0x1004 - `IRRX_BGR_REG, irrx_gate_clk, reset map`.
    pub irrx_bgr: RW<SingleBusGatingReset>,
    /// 0x1008 - `IRTX_CLK_REG, irtx_clk`.
    pub irtx_clk: RW<u32>,
    /// 0x100c - `IRTX_BGR_REG, irtx_gate_clk, reset map`.
    pub irtx_bgr: RW<SingleBusGatingReset>,
    _reserved_1010: [u8; 0x014],
    /// 0x1024 - `LRADC_BGR_REG, lradc_clk, reset map`.
    pub lradc_bgr: RW<SingleBusGatingReset>,
    _reserved_1028: [u8; 0x038],
    /// 0x1060 - `SGPIO_CLK_REG, sgpio_clk`.
    pub sgpio_clk: RW<u32>,
    /// 0x1064 - `SGPIO_BGR_REG, reset map, sgpio_bus_clk`.
    pub sgpio_bgr: RW<SingleBusGatingReset>,
    _reserved_1068: [u8; 0x018],
    /// 0x1080 - `LPC_CLK_REG, lpc_clk`.
    pub lpc_clk: RW<u32>,
    /// 0x1084 - `LPC_BGR_REG, lpc_bus_clk, reset map`.
    pub lpc_bgr: RW<SingleBusGatingReset>,
    _reserved_1088: [u8; 0x178],
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
    /// 0x1224 - `I2SPCM2_ASRC_CLK_REG, i2spcm2_asrc_clk`.
    pub i2spcm2_asrc_clk: RW<u32>,
    _reserved_1228: [u8; 0x004],
    /// 0x122c - `I2SPCM2_BGR_REG, i2spcm2_bus_clk, reset map`.
    pub i2spcm2_bgr: RW<SingleBusGatingReset>,
    /// 0x1230 - `I2SPCM3_CLK_REG, i2spcm3_clk`.
    pub i2spcm3_clk: RW<u32>,
    _reserved_1234: [u8; 0x008],
    /// 0x123c - `I2SPCM3_BGR_REG, i2spcm3_bus_clk, reset map`.
    pub i2spcm3_bgr: RW<SingleBusGatingReset>,
    /// 0x1240 - `I2SPCM4_CLK_REG, i2spcm4_clk`.
    pub i2spcm4_clk: RW<u32>,
    _reserved_1244: [u8; 0x008],
    /// 0x124c - `I2SPCM4_BGR_REG, i2spcm4_bus_clk, reset map`.
    pub i2spcm4_bgr: RW<SingleBusGatingReset>,
    _reserved_1250: [u8; 0x030],
    /// 0x1280 - `SPDIF_TX_CLK_REG, owa_tx_clk`.
    pub spdif_tx_clk: RW<u32>,
    /// 0x1284 - `SPDIF_RX_CLK_REG, owa_rx_clk`.
    pub spdif_rx_clk: RW<u32>,
    _reserved_1288: [u8; 0x004],
    /// 0x128c - `SPDIF_BGR_REG, owa_bus_clk, reset map`.
    pub spdif_bgr: RW<SingleBusGatingReset>,
    _reserved_1290: [u8; 0x030],
    /// 0x12c0 - `DMIC_CLK_REG, dmic_clk`.
    pub dmic_clk: RW<u32>,
    _reserved_12c4: [u8; 0x008],
    /// 0x12cc - `DMIC_BGR_REG, dmic_bus_clk, reset map`.
    pub dmic_bgr: RW<SingleBusGatingReset>,
    _reserved_12d0: [u8; 0x030],
    /// 0x1300 - `USB0_CLK_REG, reset map, usb_clk`.
    pub usb0_clk: RW<u32>,
    /// 0x1304 - `USB0_BGR_REG, reset map, usb0_device_clk, ...`.
    pub usb0_bgr: RW<SingleBusGatingReset>,
    /// 0x1308 - `USB1_CLK_REG, reset map, usb1_clk`.
    pub usb1_clk: RW<u32>,
    /// 0x130c - `USB1_BGR_REG, reset map, usb1_ehci_clk, ...`.
    pub usb1_bgr: RW<SingleBusGatingReset>,
    _reserved_1310: [u8; 0x030],
    /// 0x1340 - `USB0_USB1_REF_CLK_REG, usb_ref_clk`.
    pub usb0_usb1_ref_clk: RW<u32>,
    _reserved_1344: [u8; 0x004],
    /// 0x1348 - `USB2_U2_REF_CLK_REG, usb2_u2_ref_clk`.
    pub usb2_u2_ref_clk: RW<u32>,
    _reserved_134c: [u8; 0x004],
    /// 0x1350 - `USB2_SUSPEND_CLK_REG, usb2_suspend_clk`.
    pub usb2_suspend_clk: RW<u32>,
    /// 0x1354 - `USB2_MF_CLK_REG, usb2_mf_clk`.
    pub usb2_mf_clk: RW<u32>,
    _reserved_1358: [u8; 0x004],
    /// 0x135c - `USB2_BGR_REG, reset map`.
    pub usb2_bgr: RW<u32>,
    /// 0x1360 - `USB2_U3_UTMI_CLK_REG, usb2_u3_utmi_clk`.
    pub usb2_u3_utmi_clk: RW<u32>,
    /// 0x1364 - `USB2_U2_PIPE_CLK_REG, usb2_u2_pipe_clk`.
    pub usb2_u2_pipe_clk: RW<u32>,
    _reserved_1368: [u8; 0x018],
    /// 0x1380 - `PCIE0_AUX_CLK_REG, pcie0_aux_clk`.
    pub pcie0_aux_clk: RW<u32>,
    /// 0x1384 - `PCIE0_AXI_SLV_CLK_REG, pcie0_axi_slv_clk`.
    pub pcie0_axi_slv_clk: RW<u32>,
    _reserved_1388: [u8; 0x004],
    /// 0x138c - `PCIE0_BGR_REG, reset map`.
    pub pcie0_bgr: RW<u32>,
    _reserved_1390: [u8; 0x030],
    /// 0x13c0 - `SERDES_PHY_CFG_CLK_REG, serdes_phy_cfg_clk`.
    pub serdes_phy_cfg_clk: RW<u32>,
    /// 0x13c4 - `SERDES_BGR_REG, reset map`.
    pub serdes_bgr: RW<u32>,
    _reserved_13c8: [u8; 0x038],
    /// 0x1400 - `GMAC_PTP_CLK_REG, gmac_ptp_clk`.
    pub gmac_ptp_clk: RW<u32>,
    _reserved_1404: [u8; 0x00c],
    /// 0x1410 - `GMAC0_PHY_CLK_REG, gmac0_phy_clk`.
    pub gmac0_phy_clk: RW<u32>,
    _reserved_1414: [u8; 0x008],
    /// 0x141c - `GMAC0_BGR_REG, gmac0_clk, reset map`.
    pub gmac0_bgr: RW<SingleBusGatingReset>,
    /// 0x1420 - `GMAC1_PHY_CLK_REG, gmac1_phy_clk`.
    pub gmac1_phy_clk: RW<u32>,
    _reserved_1424: [u8; 0x008],
    /// 0x142c - `GMAC1_BGR_REG, gmac1_clk, reset map`.
    pub gmac1_bgr: RW<SingleBusGatingReset>,
    _reserved_1430: [u8; 0x0d0],
    /// 0x1500 - `VO0_TCONLCD0_CLK_REG, vo0_tconlcd0_clk`.
    pub vo0_tconlcd0_clk: RW<u32>,
    /// 0x1504 - `VO0_TCONLCD0_BGR_REG, reset map, vo0_tconlcd0_bus_clk`.
    pub vo0_tconlcd0_bgr: RW<SingleBusGatingReset>,
    /// 0x1508 - `VO0_TCONLCD1_CLK_REG, vo0_tconlcd1_clk`.
    pub vo0_tconlcd1_clk: RW<u32>,
    /// 0x150c - `VO0_TCONLCD1_BGR_REG, reset map, vo0_tconlcd1_bus_clk`.
    pub vo0_tconlcd1_bgr: RW<SingleBusGatingReset>,
    /// 0x1510 - `VO0_TCONLCD2_CLK_REG, vo0_tconlcd2_clk`.
    pub vo0_tconlcd2_clk: RW<u32>,
    /// 0x1514 - `VO0_TCONLCD2_BGR_REG, reset map, vo0_tconlcd2_bus_clk`.
    pub vo0_tconlcd2_bgr: RW<SingleBusGatingReset>,
    _reserved_1518: [u8; 0x02c],
    /// 0x1544 - `LVDS0_BGR_REG, reset map`.
    pub lvds0_bgr: RW<u32>,
    _reserved_1548: [u8; 0x004],
    /// 0x154c - `LVDS1_BGR_REG, reset map`.
    pub lvds1_bgr: RW<u32>,
    _reserved_1550: [u8; 0x030],
    /// 0x1580 - `DSI0_CLK_REG, dsi0_clk`.
    pub dsi0_clk: RW<u32>,
    /// 0x1584 - `DSI0_BGR_REG, dsi0_bus_clk, reset map`.
    pub dsi0_bgr: RW<SingleBusGatingReset>,
    /// 0x1588 - `DSI1_CLK_REG, dsi1_clk`.
    pub dsi1_clk: RW<u32>,
    /// 0x158c - `DSI1_BGR_REG, dsi1_bus_clk, reset map`.
    pub dsi1_bgr: RW<SingleBusGatingReset>,
    _reserved_1590: [u8; 0x030],
    /// 0x15c0 - `COMBPHY0_CLK_REG, combphy0_clk`.
    pub combphy0_clk: RW<u32>,
    /// 0x15c4 - `COMBPHY1_CLK_REG, combphy1_clk`.
    pub combphy1_clk: RW<u32>,
    _reserved_15c8: [u8; 0x03c],
    /// 0x1604 - `TCONTV0_BGR_REG, reset map, tcontv0_clk`.
    pub tcontv0_bgr: RW<SingleBusGatingReset>,
    _reserved_1608: [u8; 0x004],
    /// 0x160c - `TCONTV1_BGR_REG, reset map, tcontv1_clk`.
    pub tcontv1_bgr: RW<SingleBusGatingReset>,
    _reserved_1610: [u8; 0x030],
    /// 0x1640 - `EDP_TV_CLK_REG, edp_tv_clk`.
    pub edp_tv_clk: RW<u32>,
    _reserved_1644: [u8; 0x008],
    /// 0x164c - `EDP_BGR_REG, edp_clk, reset map`.
    pub edp_bgr: RW<SingleBusGatingReset>,
    _reserved_1650: [u8; 0x030],
    /// 0x1680 - `HDMI_CEC_CLK_REG, hdmi_ref_clk`.
    pub hdmi_cec_clk: RW<u32>,
    /// 0x1684 - `HDMI_TV_CLK_REG, hdmi_tv_clk`.
    pub hdmi_tv_clk: RW<u32>,
    _reserved_1688: [u8; 0x004],
    /// 0x168c - `HDMI_BGR_REG, hdmi_clk, reset map`.
    pub hdmi_bgr: RW<SingleBusGatingReset>,
    /// 0x1690 - `HDMI_SFR_CLK_REG, hdmi_sfr_clk`.
    pub hdmi_sfr_clk: RW<u32>,
    /// 0x1694 - `HDCP_ESM_CLK_REG, hdcp_esm_clk`.
    pub hdcp_esm_clk: RW<u32>,
    _reserved_1698: [u8; 0x02c],
    /// 0x16c4 - `DPSS_TOP0_BGR_REG, dpss_top0_clk, reset map`.
    pub dpss_top0_bgr: RW<SingleBusGatingReset>,
    _reserved_16c8: [u8; 0x004],
    /// 0x16cc - `DPSS_TOP1_BGR_REG, dpss_top1_clk, reset map`.
    pub dpss_top1_bgr: RW<SingleBusGatingReset>,
    _reserved_16d0: [u8; 0x014],
    /// 0x16e4 - `VIDEO_OUT0_BGR_REG, reset map`.
    pub video_out0_bgr: RW<u32>,
    _reserved_16e8: [u8; 0x004],
    /// 0x16ec - `VIDEO_OUT1_BGR_REG, reset map`.
    pub video_out1_bgr: RW<u32>,
    _reserved_16f0: [u8; 0x010],
    /// 0x1700 - `LEDC_CLK_REG, ledc_clk`.
    pub ledc_clk: RW<u32>,
    /// 0x1704 - `LEDC_BGR_REG, ledc_bus_clk, reset map`.
    pub ledc_bgr: RW<SingleBusGatingReset>,
    _reserved_1708: [u8; 0x03c],
    /// 0x1744 - `DSC_BGR_REG, dsc_clk, reset map`.
    pub dsc_bgr: RW<SingleBusGatingReset>,
    _reserved_1748: [u8; 0x0b8],
    /// 0x1800 - `CSI_MASTER0_CLK_REG, csi_master0_clk`.
    pub csi_master0_clk: RW<u32>,
    /// 0x1804 - `CSI_MASTER1_CLK_REG, csi_master1_clk`.
    pub csi_master1_clk: RW<u32>,
    /// 0x1808 - `CSI_MASTER2_CLK_REG, csi_master2_clk`.
    pub csi_master2_clk: RW<u32>,
    _reserved_180c: [u8; 0x034],
    /// 0x1840 - `CSI_CLK_REG, csi_clk`.
    pub csi_clk: RW<u32>,
    /// 0x1844 - `CSI_BGR_REG, csi_bus_clk, reset map`.
    pub csi_bgr: RW<SingleBusGatingReset>,
    _reserved_1848: [u8; 0x018],
    /// 0x1860 - `ISP_CLK_REG, isp_clk`.
    pub isp_clk: RW<u32>,
    _reserved_1864: [u8; 0x020],
    /// 0x1884 - `VIDEO_IN_BGR_REG, reset map`.
    pub video_in_bgr: RW<u32>,
    _reserved_1888: [u8; 0x07c],
    /// 0x1904 - `DDRPLL_GATE_EN_REG, pll_ddr_auto_clk`.
    pub ddrpll_gate_en: RW<u32>,
    /// 0x1908 - `PERI0PLL_GATE_EN_REG, pll_peri0_150m_auto_clk, pll_peri0_160m_auto_clk, ...`.
    pub peri0pll_gate_en: RW<u32>,
    /// 0x190c - `PERI1PLL_GATE_EN_REG, pll_peri1_150m_auto_clk, pll_peri1_160m_auto_clk, ...`.
    pub peri1pll_gate_en: RW<u32>,
    /// 0x1910 - `VIDEOPLL_GATE_EN_REG, pll_video0_3x_auto_clk, pll_video0_4x_auto_clk, ...`.
    pub videopll_gate_en: RW<u32>,
    /// 0x1914 - `GPUPLL_GATE_EN_REG, pll_gpu0_auto_clk`.
    pub gpupll_gate_en: RW<u32>,
    /// 0x1918 - `VEPLL_GATE_EN_REG, pll_ve0_auto_clk, pll_ve1_auto_clk`.
    pub vepll_gate_en: RW<u32>,
    /// 0x191c - `AUDIOPLL_GATE_EN_REG, pll_audio0_4x_auto_clk, pll_audio1_div2_auto_clk, ...`.
    pub audiopll_gate_en: RW<u32>,
    /// 0x1920 - `NPUPLL_GATE_EN_REG, pll_npu_auto_clk`.
    pub npupll_gate_en: RW<u32>,
    _reserved_1924: [u8; 0x004],
    /// 0x1928 - `DEPLL_GATE_EN_REG, pll_de_3x_auto_clk, pll_de_4x_auto_clk`.
    pub depll_gate_en: RW<u32>,
    _reserved_192c: [u8; 0x058],
    /// 0x1984 - `DDRPLL_GATE_STAT_REG`.
    pub ddrpll_gate_stat: RO<u32>,
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
    /// 0x19a0 - `NPUPLL_GATE_STAT_REG`.
    pub npupll_gate_stat: RO<u32>,
    _reserved_19a4: [u8; 0x004],
    /// 0x19a8 - `DEPLL_GATE_STAT_REG`.
    pub depll_gate_stat: RO<u32>,
    _reserved_19ac: [u8; 0x054],
    /// 0x1a00 - `CLK24M_GATE_EN_REG, res_dcap_24m_clk`.
    pub clk24m_gate_en: RW<u32>,
    _reserved_1a04: [u8; 0x0fc],
    /// 0x1b00 - `CM_VI_CFG_REG`.
    pub cm_vi_cfg: RW<u32>,
    /// 0x1b04 - `CM_DESYS_CFG_REG`.
    pub cm_desys_cfg: RW<u32>,
    _reserved_1b08: [u8; 0x008],
    /// 0x1b10 - `CM_VE_DEC_CFG_REG`.
    pub cm_ve_dec_cfg: RW<u32>,
    /// 0x1b14 - `CM_VE_ENC_CFG_REG`.
    pub cm_ve_enc_cfg: RW<u32>,
    _reserved_1b18: [u8; 0x004],
    /// 0x1b1c - `CM_NPU_CFG_REG`.
    pub cm_npu_cfg: RW<u32>,
    _reserved_1b20: [u8; 0x004],
    /// 0x1b24 - `CM_GPU0_CFG_REG`.
    pub cm_gpu0_cfg: RW<u32>,
    /// 0x1b28 - `CM_PCIE0_CFG_REG`.
    pub cm_pcie0_cfg: RW<u32>,
    _reserved_1b2c: [u8; 0x004],
    /// 0x1b30 - `CM_USB2_CFG_REG`.
    pub cm_usb2_cfg: RW<u32>,
    /// 0x1b34 - `CM_VO_CFG_REG`.
    pub cm_vo_cfg: RW<u32>,
    /// 0x1b38 - `CM_VO1_CFG_REG`.
    pub cm_vo1_cfg: RW<u32>,
    _reserved_1b3c: [u8; 0x0c4],
    /// 0x1c00 - `APB2JTAG_CLK_REG, apb2jtag_clk`.
    pub apb2jtag_clk: RW<u32>,
    /// 0x1c04 - `APB2JTAG_BGR_REG, reset map`.
    pub apb2jtag_bgr: RW<u32>,
    _reserved_1c08: [u8; 0x2f8],
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
    /// 0x1f30 - `CCU_FAN_GATE_REG, fanout_12m_clk, fanout_16m_clk, ...`.
    pub fan_gate: RW<u32>,
    /// 0x1f34 - `CLK27M_FAN_REG, clk27m_fanout_clk`.
    pub clk27m_fan: RW<u32>,
    /// 0x1f38 - `CLK_FAN_REG, clk_fanout_clk`.
    pub clk_fan: RW<u32>,
    /// 0x1f3c - `CCU_FAN_REG, fanout0_clk, fanout1_clk, ...`.
    pub fan: RW<u32>,
    _reserved_1f40: [u8; 0x010],
    /// 0x1f50 - `BUS_CLK_DBG_REG, bus_debug_clk`.
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
        assert_eq!(offset_of!(RegisterBlock, pll_ref_ctrl), 0x000);
        assert_eq!(offset_of!(RegisterBlock, pll_ref_lock_ctrl), 0x004);
        assert_eq!(offset_of!(RegisterBlock, pll_ref_bias), 0x010);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_ctrl), 0x020);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_lock_ctrl), 0x024);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat0_ctrl), 0x028);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_pat1_ctrl), 0x02c);
        assert_eq!(offset_of!(RegisterBlock, pll_ddr_bias), 0x030);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_ctrl), 0x0a0);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_lock_ctrl), 0x0a4);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat0_ctrl), 0x0a8);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_pat1_ctrl), 0x0ac);
        assert_eq!(offset_of!(RegisterBlock, pll_peri0_bias), 0x0b0);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_ctrl), 0x0c0);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_lock_ctrl), 0x0c4);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_pat0_ctrl), 0x0c8);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_pat1_ctrl), 0x0cc);
        assert_eq!(offset_of!(RegisterBlock, pll_peri1_bias), 0x0d0);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu0_ctrl), 0x0e0);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu0_lock_ctrl), 0x0e4);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu0_pat0_ctrl), 0x0e8);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu0_pat1_ctrl), 0x0ec);
        assert_eq!(offset_of!(RegisterBlock, pll_gpu0_bias), 0x0f0);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_ctrl), 0x120);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_lock_ctrl), 0x124);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat0_ctrl), 0x128);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_pat1_ctrl), 0x12c);
        assert_eq!(offset_of!(RegisterBlock, pll_video0_bias), 0x130);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_ctrl), 0x140);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_lock_ctrl), 0x144);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat0_ctrl), 0x148);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_pat1_ctrl), 0x14c);
        assert_eq!(offset_of!(RegisterBlock, pll_video1_bias), 0x150);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_ctrl), 0x160);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_lock_ctrl), 0x164);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_pat0_ctrl), 0x168);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_pat1_ctrl), 0x16c);
        assert_eq!(offset_of!(RegisterBlock, pll_video2_bias), 0x170);
        assert_eq!(offset_of!(RegisterBlock, pll_ve0_ctrl), 0x220);
        assert_eq!(offset_of!(RegisterBlock, pll_ve0_lock_ctrl), 0x224);
        assert_eq!(offset_of!(RegisterBlock, pll_ve0_pat0_ctrl), 0x228);
        assert_eq!(offset_of!(RegisterBlock, pll_ve0_pat1_ctrl), 0x22c);
        assert_eq!(offset_of!(RegisterBlock, pll_ve0_bias), 0x230);
        assert_eq!(offset_of!(RegisterBlock, pll_ve1_ctrl), 0x240);
        assert_eq!(offset_of!(RegisterBlock, pll_ve1_lock_ctrl), 0x244);
        assert_eq!(offset_of!(RegisterBlock, pll_ve1_pat0_ctrl), 0x248);
        assert_eq!(offset_of!(RegisterBlock, pll_ve1_pat1_ctrl), 0x24c);
        assert_eq!(offset_of!(RegisterBlock, pll_ve1_bias), 0x250);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_ctrl), 0x260);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_lock_ctrl), 0x264);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_pat0_ctrl), 0x268);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_pattern1), 0x26c);
        assert_eq!(offset_of!(RegisterBlock, pll_audio0_bias), 0x270);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_ctrl), 0x280);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_lock_ctrl), 0x284);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_pat0_ctrl), 0x288);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_pat1_ctrl), 0x28c);
        assert_eq!(offset_of!(RegisterBlock, pll_audio1_bias), 0x290);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_ctrl), 0x2a0);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_lock_ctrl), 0x2a4);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_pat0_ctrl), 0x2a8);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_pat1_ctrl), 0x2ac);
        assert_eq!(offset_of!(RegisterBlock, pll_npu_bias), 0x2b0);
        assert_eq!(offset_of!(RegisterBlock, pll_de_ctrl), 0x2e0);
        assert_eq!(offset_of!(RegisterBlock, pll_de_lock_ctrl), 0x2e4);
        assert_eq!(offset_of!(RegisterBlock, pll_de_pat0_ctrl), 0x2e8);
        assert_eq!(offset_of!(RegisterBlock, pll_de_pat1_ctrl), 0x2ec);
        assert_eq!(offset_of!(RegisterBlock, pll_de_bias), 0x2f0);
        assert_eq!(offset_of!(RegisterBlock, ahb_clk), 0x500);
        assert_eq!(offset_of!(RegisterBlock, apb0_clk), 0x510);
        assert_eq!(offset_of!(RegisterBlock, apb1_clk), 0x518);
        assert_eq!(offset_of!(RegisterBlock, apb_uart_clk), 0x538);
        assert_eq!(offset_of!(RegisterBlock, trace_clk), 0x540);
        assert_eq!(offset_of!(RegisterBlock, gic_clk), 0x560);
        assert_eq!(offset_of!(RegisterBlock, cpu_peri_clk), 0x568);
        assert_eq!(offset_of!(RegisterBlock, its0_bgr), 0x574);
        assert_eq!(offset_of!(RegisterBlock, nsi_clk), 0x580);
        assert_eq!(offset_of!(RegisterBlock, nsi_bgr), 0x584);
        assert_eq!(offset_of!(RegisterBlock, mbus_clk), 0x588);
        assert_eq!(offset_of!(RegisterBlock, iommu0_bgr), 0x58c);
        assert_eq!(offset_of!(RegisterBlock, msi_lite0_bgr), 0x594);
        assert_eq!(offset_of!(RegisterBlock, msi_lite1_bgr), 0x59c);
        assert_eq!(offset_of!(RegisterBlock, msi_lite2_bgr), 0x5a4);
        assert_eq!(offset_of!(RegisterBlock, iommu1_bgr), 0x5b4);
        assert_eq!(offset_of!(RegisterBlock, ahb_mat_clk_gating), 0x5c0);
        assert_eq!(offset_of!(RegisterBlock, mbus_mat_clk_gating), 0x5e0);
        assert_eq!(offset_of!(RegisterBlock, mbus_gate_en), 0x5e4);
        assert_eq!(offset_of!(RegisterBlock, dma0_bgr), 0x704);
        assert_eq!(offset_of!(RegisterBlock, dma1_bgr), 0x70c);
        assert_eq!(offset_of!(RegisterBlock, spinlock_bgr), 0x724);
        assert_eq!(offset_of!(RegisterBlock, msgbox0_bgr), 0x744);
        assert_eq!(offset_of!(RegisterBlock, pwm0_bgr), 0x784);
        assert_eq!(offset_of!(RegisterBlock, pwm1_bgr), 0x78c);
        assert_eq!(offset_of!(RegisterBlock, dbgsys_bgr), 0x7a4);
        assert_eq!(offset_of!(RegisterBlock, sysdap_bgr), 0x7ac);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk0_clk), 0x800);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk1_clk), 0x804);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk2_clk), 0x808);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk3_clk), 0x80c);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk4_clk), 0x810);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk5_clk), 0x814);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk6_clk), 0x818);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk7_clk), 0x81c);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk8_clk), 0x820);
        assert_eq!(offset_of!(RegisterBlock, timer0_clk9_clk), 0x824);
        assert_eq!(offset_of!(RegisterBlock, timer0_bgr), 0x850);
        assert_eq!(offset_of!(RegisterBlock, avs_clk), 0x880);
        assert_eq!(offset_of!(RegisterBlock, de0_clk), 0xa00);
        assert_eq!(offset_of!(RegisterBlock, de0_bgr), 0xa04);
        assert_eq!(offset_of!(RegisterBlock, di_clk), 0xa20);
        assert_eq!(offset_of!(RegisterBlock, di_bgr), 0xa24);
        assert_eq!(offset_of!(RegisterBlock, g2d_clk), 0xa40);
        assert_eq!(offset_of!(RegisterBlock, g2d_bgr), 0xa44);
        assert_eq!(offset_of!(RegisterBlock, eink_clk), 0xa60);
        assert_eq!(offset_of!(RegisterBlock, eink_panel_clk), 0xa64);
        assert_eq!(offset_of!(RegisterBlock, eink_bgr), 0xa6c);
        assert_eq!(offset_of!(RegisterBlock, de_sys_bgr), 0xa74);
        assert_eq!(offset_of!(RegisterBlock, ve_enc0_clk), 0xa80);
        assert_eq!(offset_of!(RegisterBlock, ve_dec_clk), 0xa88);
        assert_eq!(offset_of!(RegisterBlock, ve_bgr), 0xa8c);
        assert_eq!(offset_of!(RegisterBlock, ce_clk), 0xac0);
        assert_eq!(offset_of!(RegisterBlock, ce_bgr), 0xac4);
        assert_eq!(offset_of!(RegisterBlock, npu_clk), 0xb00);
        assert_eq!(offset_of!(RegisterBlock, npu_bgr), 0xb04);
        assert_eq!(offset_of!(RegisterBlock, gpu0), 0xb20);
        assert_eq!(offset_of!(RegisterBlock, gpu0_bus), 0xb24);
        assert_eq!(offset_of!(RegisterBlock, dram0_clk), 0xc00);
        assert_eq!(offset_of!(RegisterBlock, dram0_bgr), 0xc0c);
        assert_eq!(offset_of!(RegisterBlock, nand0_clk0_clk), 0xc80);
        assert_eq!(offset_of!(RegisterBlock, nand0_clk1_clk), 0xc84);
        assert_eq!(offset_of!(RegisterBlock, nand0_bgr), 0xc8c);
        assert_eq!(offset_of!(RegisterBlock, smhc0_clk), 0xd00);
        assert_eq!(offset_of!(RegisterBlock, smhc0_bgr), 0xd0c);
        assert_eq!(offset_of!(RegisterBlock, smhc1_clk), 0xd10);
        assert_eq!(offset_of!(RegisterBlock, smhc1_bgr), 0xd1c);
        assert_eq!(offset_of!(RegisterBlock, smhc2_clk), 0xd20);
        assert_eq!(offset_of!(RegisterBlock, smhc2_bgr), 0xd2c);
        assert_eq!(offset_of!(RegisterBlock, smhc3_clk), 0xd30);
        assert_eq!(offset_of!(RegisterBlock, smhc3_bgr), 0xd3c);
        assert_eq!(offset_of!(RegisterBlock, ufs_axi_clk), 0xd80);
        assert_eq!(offset_of!(RegisterBlock, ufs_cfg_clk), 0xd84);
        assert_eq!(offset_of!(RegisterBlock, ufs_bgr), 0xd8c);
        assert_eq!(offset_of!(RegisterBlock, ufs_ref_clk_en), 0xd90);
        assert_eq!(offset_of!(RegisterBlock, uart0_bgr), 0xe00);
        assert_eq!(offset_of!(RegisterBlock, uart1_bgr), 0xe04);
        assert_eq!(offset_of!(RegisterBlock, uart2_bgr), 0xe08);
        assert_eq!(offset_of!(RegisterBlock, uart3_bgr), 0xe0c);
        assert_eq!(offset_of!(RegisterBlock, uart4_bgr), 0xe10);
        assert_eq!(offset_of!(RegisterBlock, uart5_bgr), 0xe14);
        assert_eq!(offset_of!(RegisterBlock, uart6_bgr), 0xe18);
        assert_eq!(offset_of!(RegisterBlock, twi0_bgr), 0xe80);
        assert_eq!(offset_of!(RegisterBlock, twi1_bgr), 0xe84);
        assert_eq!(offset_of!(RegisterBlock, twi2_bgr), 0xe88);
        assert_eq!(offset_of!(RegisterBlock, twi3_bgr), 0xe8c);
        assert_eq!(offset_of!(RegisterBlock, twi4_bgr), 0xe90);
        assert_eq!(offset_of!(RegisterBlock, twi5_bgr), 0xe94);
        assert_eq!(offset_of!(RegisterBlock, twi6_bgr), 0xe98);
        assert_eq!(offset_of!(RegisterBlock, twi7_bgr), 0xe9c);
        assert_eq!(offset_of!(RegisterBlock, twi8_bgr), 0xea0);
        assert_eq!(offset_of!(RegisterBlock, twi9_bgr), 0xea4);
        assert_eq!(offset_of!(RegisterBlock, twi10_bgr), 0xea8);
        assert_eq!(offset_of!(RegisterBlock, twi11_bgr), 0xeac);
        assert_eq!(offset_of!(RegisterBlock, twi12_bgr), 0xeb0);
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
        assert_eq!(offset_of!(RegisterBlock, gpadc0_24m_clk), 0xfc0);
        assert_eq!(offset_of!(RegisterBlock, gpadc0_bgr), 0xfc4);
        assert_eq!(offset_of!(RegisterBlock, ths0_bgr), 0xfe4);
        assert_eq!(offset_of!(RegisterBlock, irrx_clk), 0x1000);
        assert_eq!(offset_of!(RegisterBlock, irrx_bgr), 0x1004);
        assert_eq!(offset_of!(RegisterBlock, irtx_clk), 0x1008);
        assert_eq!(offset_of!(RegisterBlock, irtx_bgr), 0x100c);
        assert_eq!(offset_of!(RegisterBlock, lradc_bgr), 0x1024);
        assert_eq!(offset_of!(RegisterBlock, sgpio_clk), 0x1060);
        assert_eq!(offset_of!(RegisterBlock, sgpio_bgr), 0x1064);
        assert_eq!(offset_of!(RegisterBlock, lpc_clk), 0x1080);
        assert_eq!(offset_of!(RegisterBlock, lpc_bgr), 0x1084);
        assert_eq!(offset_of!(RegisterBlock, i2spcm0_clk), 0x1200);
        assert_eq!(offset_of!(RegisterBlock, i2spcm0_bgr), 0x120c);
        assert_eq!(offset_of!(RegisterBlock, i2spcm1_clk), 0x1210);
        assert_eq!(offset_of!(RegisterBlock, i2spcm1_bgr), 0x121c);
        assert_eq!(offset_of!(RegisterBlock, i2spcm2_clk), 0x1220);
        assert_eq!(offset_of!(RegisterBlock, i2spcm2_asrc_clk), 0x1224);
        assert_eq!(offset_of!(RegisterBlock, i2spcm2_bgr), 0x122c);
        assert_eq!(offset_of!(RegisterBlock, i2spcm3_clk), 0x1230);
        assert_eq!(offset_of!(RegisterBlock, i2spcm3_bgr), 0x123c);
        assert_eq!(offset_of!(RegisterBlock, i2spcm4_clk), 0x1240);
        assert_eq!(offset_of!(RegisterBlock, i2spcm4_bgr), 0x124c);
        assert_eq!(offset_of!(RegisterBlock, spdif_tx_clk), 0x1280);
        assert_eq!(offset_of!(RegisterBlock, spdif_rx_clk), 0x1284);
        assert_eq!(offset_of!(RegisterBlock, spdif_bgr), 0x128c);
        assert_eq!(offset_of!(RegisterBlock, dmic_clk), 0x12c0);
        assert_eq!(offset_of!(RegisterBlock, dmic_bgr), 0x12cc);
        assert_eq!(offset_of!(RegisterBlock, usb0_clk), 0x1300);
        assert_eq!(offset_of!(RegisterBlock, usb0_bgr), 0x1304);
        assert_eq!(offset_of!(RegisterBlock, usb1_clk), 0x1308);
        assert_eq!(offset_of!(RegisterBlock, usb1_bgr), 0x130c);
        assert_eq!(offset_of!(RegisterBlock, usb0_usb1_ref_clk), 0x1340);
        assert_eq!(offset_of!(RegisterBlock, usb2_u2_ref_clk), 0x1348);
        assert_eq!(offset_of!(RegisterBlock, usb2_suspend_clk), 0x1350);
        assert_eq!(offset_of!(RegisterBlock, usb2_mf_clk), 0x1354);
        assert_eq!(offset_of!(RegisterBlock, usb2_bgr), 0x135c);
        assert_eq!(offset_of!(RegisterBlock, usb2_u3_utmi_clk), 0x1360);
        assert_eq!(offset_of!(RegisterBlock, usb2_u2_pipe_clk), 0x1364);
        assert_eq!(offset_of!(RegisterBlock, pcie0_aux_clk), 0x1380);
        assert_eq!(offset_of!(RegisterBlock, pcie0_axi_slv_clk), 0x1384);
        assert_eq!(offset_of!(RegisterBlock, pcie0_bgr), 0x138c);
        assert_eq!(offset_of!(RegisterBlock, serdes_phy_cfg_clk), 0x13c0);
        assert_eq!(offset_of!(RegisterBlock, serdes_bgr), 0x13c4);
        assert_eq!(offset_of!(RegisterBlock, gmac_ptp_clk), 0x1400);
        assert_eq!(offset_of!(RegisterBlock, gmac0_phy_clk), 0x1410);
        assert_eq!(offset_of!(RegisterBlock, gmac0_bgr), 0x141c);
        assert_eq!(offset_of!(RegisterBlock, gmac1_phy_clk), 0x1420);
        assert_eq!(offset_of!(RegisterBlock, gmac1_bgr), 0x142c);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd0_clk), 0x1500);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd0_bgr), 0x1504);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd1_clk), 0x1508);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd1_bgr), 0x150c);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd2_clk), 0x1510);
        assert_eq!(offset_of!(RegisterBlock, vo0_tconlcd2_bgr), 0x1514);
        assert_eq!(offset_of!(RegisterBlock, lvds0_bgr), 0x1544);
        assert_eq!(offset_of!(RegisterBlock, lvds1_bgr), 0x154c);
        assert_eq!(offset_of!(RegisterBlock, dsi0_clk), 0x1580);
        assert_eq!(offset_of!(RegisterBlock, dsi0_bgr), 0x1584);
        assert_eq!(offset_of!(RegisterBlock, dsi1_clk), 0x1588);
        assert_eq!(offset_of!(RegisterBlock, dsi1_bgr), 0x158c);
        assert_eq!(offset_of!(RegisterBlock, combphy0_clk), 0x15c0);
        assert_eq!(offset_of!(RegisterBlock, combphy1_clk), 0x15c4);
        assert_eq!(offset_of!(RegisterBlock, tcontv0_bgr), 0x1604);
        assert_eq!(offset_of!(RegisterBlock, tcontv1_bgr), 0x160c);
        assert_eq!(offset_of!(RegisterBlock, edp_tv_clk), 0x1640);
        assert_eq!(offset_of!(RegisterBlock, edp_bgr), 0x164c);
        assert_eq!(offset_of!(RegisterBlock, hdmi_cec_clk), 0x1680);
        assert_eq!(offset_of!(RegisterBlock, hdmi_tv_clk), 0x1684);
        assert_eq!(offset_of!(RegisterBlock, hdmi_bgr), 0x168c);
        assert_eq!(offset_of!(RegisterBlock, hdmi_sfr_clk), 0x1690);
        assert_eq!(offset_of!(RegisterBlock, hdcp_esm_clk), 0x1694);
        assert_eq!(offset_of!(RegisterBlock, dpss_top0_bgr), 0x16c4);
        assert_eq!(offset_of!(RegisterBlock, dpss_top1_bgr), 0x16cc);
        assert_eq!(offset_of!(RegisterBlock, video_out0_bgr), 0x16e4);
        assert_eq!(offset_of!(RegisterBlock, video_out1_bgr), 0x16ec);
        assert_eq!(offset_of!(RegisterBlock, ledc_clk), 0x1700);
        assert_eq!(offset_of!(RegisterBlock, ledc_bgr), 0x1704);
        assert_eq!(offset_of!(RegisterBlock, dsc_bgr), 0x1744);
        assert_eq!(offset_of!(RegisterBlock, csi_master0_clk), 0x1800);
        assert_eq!(offset_of!(RegisterBlock, csi_master1_clk), 0x1804);
        assert_eq!(offset_of!(RegisterBlock, csi_master2_clk), 0x1808);
        assert_eq!(offset_of!(RegisterBlock, csi_clk), 0x1840);
        assert_eq!(offset_of!(RegisterBlock, csi_bgr), 0x1844);
        assert_eq!(offset_of!(RegisterBlock, isp_clk), 0x1860);
        assert_eq!(offset_of!(RegisterBlock, video_in_bgr), 0x1884);
        assert_eq!(offset_of!(RegisterBlock, ddrpll_gate_en), 0x1904);
        assert_eq!(offset_of!(RegisterBlock, peri0pll_gate_en), 0x1908);
        assert_eq!(offset_of!(RegisterBlock, peri1pll_gate_en), 0x190c);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_en), 0x1910);
        assert_eq!(offset_of!(RegisterBlock, gpupll_gate_en), 0x1914);
        assert_eq!(offset_of!(RegisterBlock, vepll_gate_en), 0x1918);
        assert_eq!(offset_of!(RegisterBlock, audiopll_gate_en), 0x191c);
        assert_eq!(offset_of!(RegisterBlock, npupll_gate_en), 0x1920);
        assert_eq!(offset_of!(RegisterBlock, depll_gate_en), 0x1928);
        assert_eq!(offset_of!(RegisterBlock, ddrpll_gate_stat), 0x1984);
        assert_eq!(offset_of!(RegisterBlock, peri0pll_gate_stat), 0x1988);
        assert_eq!(offset_of!(RegisterBlock, peri1pll_gate_stat), 0x198c);
        assert_eq!(offset_of!(RegisterBlock, videopll_gate_stat), 0x1990);
        assert_eq!(offset_of!(RegisterBlock, gpupll_gate_stat), 0x1994);
        assert_eq!(offset_of!(RegisterBlock, vepll_gate_stat), 0x1998);
        assert_eq!(offset_of!(RegisterBlock, audiopll_gate_stat), 0x199c);
        assert_eq!(offset_of!(RegisterBlock, npupll_gate_stat), 0x19a0);
        assert_eq!(offset_of!(RegisterBlock, depll_gate_stat), 0x19a8);
        assert_eq!(offset_of!(RegisterBlock, clk24m_gate_en), 0x1a00);
        assert_eq!(offset_of!(RegisterBlock, cm_vi_cfg), 0x1b00);
        assert_eq!(offset_of!(RegisterBlock, cm_desys_cfg), 0x1b04);
        assert_eq!(offset_of!(RegisterBlock, cm_ve_dec_cfg), 0x1b10);
        assert_eq!(offset_of!(RegisterBlock, cm_ve_enc_cfg), 0x1b14);
        assert_eq!(offset_of!(RegisterBlock, cm_npu_cfg), 0x1b1c);
        assert_eq!(offset_of!(RegisterBlock, cm_gpu0_cfg), 0x1b24);
        assert_eq!(offset_of!(RegisterBlock, cm_pcie0_cfg), 0x1b28);
        assert_eq!(offset_of!(RegisterBlock, cm_usb2_cfg), 0x1b30);
        assert_eq!(offset_of!(RegisterBlock, cm_vo_cfg), 0x1b34);
        assert_eq!(offset_of!(RegisterBlock, cm_vo1_cfg), 0x1b38);
        assert_eq!(offset_of!(RegisterBlock, apb2jtag_clk), 0x1c00);
        assert_eq!(offset_of!(RegisterBlock, apb2jtag_bgr), 0x1c04);
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
