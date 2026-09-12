//! V821 DDR initialization.

use super::{Io, Parameters};
mod timing;
pub use timing::timing_init;

const COM: usize = 0x4310_2000;
const PHY: usize = 0x4310_3000;

/// Initialize clocks needed by the microsecond timer.
pub fn timer_init(io: &mut impl Io) -> u32 {
    let fuse = io.read(0x4300_624c);
    let status = io.read(0x4a01_0408);
    let clock = if fuse & (1 << 5) != 0 || status & 2 != 0 {
        if io.read(0x4a01_0404) & (1 << 31) != 0 {
            24
        } else {
            40
        }
    } else {
        io.write(0x4a01_0408, status & !1);
        io.write(0x4a01_0408, status | 1);
        let value = loop {
            let value = io.read(0x4a01_0408);
            if value & 2 != 0 {
                break value;
            }
        };
        if (value >> 4) & 0xfffff < 62_500 {
            24
        } else {
            40
        }
    };
    modify(io, 0x4a01_0404, |v| {
        if clock == 24 {
            v | (1 << 31)
        } else {
            v & !(1 << 31)
        }
    });
    if io.read(0x4200_1010) & (1 << 31) == 0 {
        io.write(0x4200_1010, 1 << 31);
    }
    clock
}

/// Distinguish the 24 MHz and 40 MHz crystals.
pub fn oscillator_mhz(io: &mut impl Io) -> u32 {
    let _ = io.read(0x4a01_0408);
    io.write(0x4a01_0408, 1);
    while io.read(0x4a01_0408) & 2 == 0 {}
    if (io.read(0x4a01_0408) >> 4) & 0xfffff > 62_500 {
        40
    } else {
        24
    }
}

/// Configure DDR PLL N/M divisors and its output clock.
pub fn set_pll(io: &mut impl Io, index: u32, p: &Parameters) -> u32 {
    let clock = if (p.tpr[13] >> 6) & 1 == index {
        p.clock
    } else {
        p.tpr[9]
    };
    let crystal = oscillator_mhz(io);
    let mut n = clock.wrapping_mul(2) / crystal;
    let m = if n <= 11 {
        n = n.wrapping_mul(4);
        2
    } else {
        1
    };
    modify(io, 0x4a01_0080, |v| {
        (v & 0xf7f8_00f8) | ((m - 1) << 1) | (m - 1) | (n.wrapping_sub(1) << 8) | 0xc000_0000
    });
    modify(io, 0x4a01_0080, |v| v & !(1 << 29));
    modify(io, 0x4a01_0080, |v| v | (1 << 29));
    while io.read(0x4a01_0080) & (1 << 28) == 0 {}
    io.delay_us(20);
    modify(io, 0x4a01_0080, |v| v | (1 << 27));
    modify(io, 0x4200_1004, |v| (v & 0xf8fc_ffe0) | 0x8100_0000);
    n.wrapping_mul(crystal) / m / m
}

/// Sequence the V821 bus gates, reset and PHY clock.
pub fn system_init(io: &mut impl Io, p: &mut Parameters) {
    modify(io, 0x4200_1094, |v| v & !(1 << 12));
    modify(io, 0x4200_1090, |v| v & !8);
    modify(io, 0x4200_1084, |v| v & !(1 << 12));
    modify(io, 0x4200_1080, |v| v & !8);
    modify(io, 0x4200_1004, |v| v & !(1 << 31));
    modify(io, 0x4200_1004, |v| v | (1 << 27));
    io.delay_us(10);
    p.clock = set_pll(io, 0, p) >> 1;
    io.delay_us(100);
    set_masters(io, false);
    modify(io, 0x4200_1090, |v| v | 8);
    modify(io, 0x4200_1094, |v| v | (1 << 12));
    modify(io, 0x4200_1080, |v| v | 8);
    modify(io, 0x4200_1084, |v| v | (1 << 12));
    modify(io, 0x4200_1004, |v| v | (1 << 31));
    modify(io, 0x4200_1004, |v| v | (1 << 27));
    io.delay_us(5);
    io.write(PHY + 12, 0x8000);
    io.delay_us(10);
}

fn modify(io: &mut impl Io, address: usize, f: impl FnOnce(u32) -> u32) {
    let value = io.read(address);
    io.write(address, f(value));
}

pub fn eye_delay_compensation(io: &mut impl Io, parameters: &Parameters) {
    let delay = |shift: u32| -> u32 {
        (((parameters.tpr[11] >> shift) << 9) & 0x1e00)
            | (((parameters.tpr[12] >> shift) << 1) & 0x1e)
    };
    for (start, shift) in [(0x310, 0), (0x390, 4)] {
        for offset in (start..start + 36).step_by(4) {
            modify(io, PHY + offset, |v| v | delay(shift));
        }
    }
    modify(io, PHY + 0x100, |v| v & !(1 << 26));
    for (offset, shift) in [(0x334, 16), (0x338, 16), (0x3b4, 20), (0x3b8, 20)] {
        modify(io, PHY + offset, |v| v | delay(shift));
    }
    for (offset, shift) in [(0x33c, 16), (0x3bc, 20)] {
        modify(io, PHY + offset, |v| {
            v | (((parameters.tpr[11] >> shift) << 25) & 0x1e00_0000)
        });
    }
    modify(io, PHY + 0x100, |v| v | (1 << 26));
    io.delay_us(1);
    for (start, end) in [(0x240, 0x27c), (0x228, 0x240)] {
        for offset in (start..end).step_by(4) {
            modify(io, PHY + offset, |v| {
                v | (((parameters.tpr[10] >> 4) << 8) & 0xf00)
            });
        }
    }
    for (offset, shift) in [(0x218, 0), (0x21c, 8), (0x280, 12)] {
        modify(io, PHY + offset, |v| {
            v | (((parameters.tpr[10] >> shift) << 8) & 0xf00)
        });
    }
}

pub fn set_master_priority(io: &mut impl Io, parameters: &Parameters) {
    modify(io, COM + 0x0c, |v| {
        (v & 0xffff_f000) | (parameters.clock >> 1).wrapping_sub(1)
    });
    for (offset, value) in [
        (0x200, 0x0001_0000),
        (0x210, 0x0100_0009),
        (0x214, 0x0050_0064),
        (0x230, 0x0200_000d),
        (0x234, 0x0060_0100),
        (0x240, 0x0100_0009),
        (0x244, 0x0050_0064),
        (0x260, 0x0064_0209),
        (0x264, 0x0020_0040),
        (0x290, 0x0100_0009),
        (0x294, 0x0040_0080),
        (0x470, 0),
        (0x474, 0),
    ] {
        io.write(COM + offset, value);
    }
    for (offset, value) in [
        (0x1c0, 0x0f80_2f05),
        (0x1c8, 0x0f00_00ff),
        (0x1d0, 0x3f00_005f),
    ] {
        io.write(PHY + offset, value);
    }
}

pub fn set_masters(io: &mut impl Io, enable: bool) {
    let masks = if enable {
        [u32::MAX, 0xff, 0xffff]
    } else {
        [1, 0, 0]
    };
    for (index, mask) in masks.into_iter().enumerate() {
        io.write(COM + 0x20 + index * 4, mask);
    }
    io.delay_us(10);
}

pub fn capacity(io: &mut impl Io) -> u32 {
    fn rank_size(value: u32) -> u32 {
        let shift = ((value >> 4) & 15)
            .wrapping_add((value >> 8) & 15)
            .wrapping_sub(14)
            .wrapping_add((value >> 2) & 3);
        1u32.wrapping_shl(shift)
    }
    let rank0 = io.read(COM);
    let size0 = rank_size(rank0);
    if rank0 & 3 == 0 {
        return size0;
    }
    let rank1 = io.read(COM + 4);
    size0.wrapping_add(if rank1 & 3 == 0 {
        size0
    } else {
        rank_size(rank1)
    })
}

pub fn detect_dqs(io: &mut impl Io, parameters: &mut Parameters) -> bool {
    if io.read(PHY + 0x10) & (1 << 22) == 0 {
        parameters.para2 = (parameters.para2 & !15) | 0x1000;
        return true;
    }
    let lane0 = (io.read(PHY + 0x348) >> 24) & 3;
    let lane1 = (io.read(PHY + 0x3c8) >> 24) & 3;
    match lane0 {
        2 => {
            parameters.para2 &= 0xffff_0ff0;
            if lane1 != 2 {
                parameters.para2 |= 1;
            }
        }
        0 => parameters.para2 = (parameters.para2 & !15) | 0x1001,
        _ => return false,
    }
    true
}

pub fn memory_test(io: &mut impl Io, size_mib: u32, words: u32) -> bool {
    let midpoint = (size_mib >> 1).wrapping_shl(20);
    for index in 0..words {
        let address = 0x8000_0000u32.wrapping_add(index.wrapping_mul(4));
        io.write(address as usize, 0x0123_4567u32.wrapping_add(index));
        io.write(
            address.wrapping_add(midpoint) as usize,
            0xfedc_ba98u32.wrapping_add(index),
        );
    }
    // Check the midpoint before checking the first region.
    for index in 0..words {
        let address = 0x8000_0000usize + (index.wrapping_mul(4) as usize);
        if io.read(address + midpoint as usize) != 0xfedc_ba98u32.wrapping_add(index)
            || io.read(address) != 0x0123_4567u32.wrapping_add(index)
        {
            return false;
        }
    }
    true
}

pub fn vref_zq_init(io: &mut impl Io, parameters: &Parameters) {
    if parameters.tpr[13] & (1 << 17) != 0 {
        return;
    }
    let value = io.read(PHY + 0x110);
    io.write(PHY + 0x110, (value & 0x8080_8080) | parameters.tpr[5]);
    if parameters.tpr[13] & (1 << 16) == 0 {
        let value = io.read(PHY + 0x114);
        io.write(PHY + 0x114, (value & !0x7f) | (parameters.tpr[6] & 0x7f));
    }
}

pub fn common_init(io: &mut impl Io, parameters: &Parameters) {
    modify(io, COM + 8, |v| (v & 0xffff_c0ff) | 0x2000);
    modify(io, COM, |v| {
        (v & 0xff00_0fff)
            | ((parameters.kind << 16) & 0x7_0000)
            | (u32::from(parameters.para2 & 1 == 0) << 12)
            | if matches!(parameters.kind, 6 | 7) {
                0x0048_0000
            } else {
                0x0040_0000 | ((parameters.tpr[13] << 14) & 0x0008_0000)
            }
    });
    let ranks = if parameters.para2 & 0x100 != 0 && (parameters.para2 >> 12) & 15 == 1 {
        2
    } else {
        1
    };
    for rank in 0..ranks {
        let geometry = parameters.para1 >> (rank * 16);
        let page = match geometry & 15 {
            1 => 0x700,
            2 => 0x800,
            4 => 0x900,
            8 => 0xa00,
            _ => 0x600,
        };
        modify(io, COM + rank * 4, |v| {
            (v & 0xffff_f000)
                | ((parameters.para2 >> 12) & 3)
                | ((geometry >> 10) & 4)
                | (((geometry >> 4).wrapping_sub(1) << 4) & 0xff)
                | page
        });
    }
    let dual_rank = io.read(COM) & 1 != 0;
    io.write(PHY + 0x120, if dual_rank { 0x303 } else { 0x201 });
    if parameters.para2 & 1 != 0 {
        io.write(PHY + 0x3c4, 0);
    }
    if parameters.tpr[4] != 0 {
        modify(io, COM, |v| v | ((parameters.tpr[4] << 25) & 0x0600_0000));
        modify(io, COM + 4, |v| {
            v | ((parameters.tpr[4] << 10) & 0x001f_f000)
        });
    }
}

pub fn channel_init(io: &mut impl Io, parameters: &Parameters) -> bool {
    let gating = (parameters.tpr[13] >> 2) & 3;
    modify(io, COM + 0x0c, |v| {
        (v & 0xffff_f000) | (parameters.clock >> 1).wrapping_sub(1)
    });
    modify(io, PHY + 0x108, |v| (v & 0xffff_f0ff) | 0x300);
    let odt = ((!parameters.odt) << 5) & 0x20;
    for offset in [0x344, 0x3c4] {
        modify(io, PHY + offset, |v| {
            let value = (v & !0x30) | odt;
            if parameters.clock > 672 {
                (value & 0xffff_09f1) | 0x400
            } else {
                value & 0xffff_0ff1
            }
        });
    }
    modify(io, PHY + 0x208, |v| v | 2);
    eye_delay_compensation(io, parameters);
    let value = io.read(PHY + 0x108);
    match gating {
        1 => {
            io.write(PHY + 0x108, value & !0xc0);
            modify(io, PHY + 0xbc, |v| v & !0x107);
        }
        2 => {
            io.write(PHY + 0x108, (value & !0xc0) | 0x80);
            let latency = ((io.read(PHY + 0x60) >> 16) & 31).wrapping_sub(2);
            modify(io, PHY + 0xbc, |v| (v & !0x107) | latency | 0x100);
            modify(io, PHY + 0x11c, |v| (v & 0x7fff_ffff) | 0x0800_0000);
        }
        _ => {
            io.write(PHY + 0x108, value & !0x40);
            io.delay_us(10);
            modify(io, PHY + 0x108, |v| v | 0xc0);
        }
    }
    if matches!(parameters.kind, 6 | 7) {
        modify(io, PHY + 0x11c, |v| {
            if gating == 1 {
                (v & 0xf7ff_ff3f) | 0x8000_0000
            } else {
                (v & 0x88ff_ffff) | 0x2200_0000
            }
        });
    }
    modify(io, PHY + 0xc0, |v| {
        (v & 0xf000_0000)
            | if parameters.para2 & (1 << 12) != 0 {
                0x0300_0001
            } else {
                0x0100_3087
            }
    });
    if io.read(0x4a00_01c0) & 15 == 1 {
        modify(io, 0x4a00_0838, |v| v & !2);
        io.delay_us(10);
    }
    modify(io, PHY + 0x140, |v| {
        (v & 0xfc00_0000) | (parameters.zq & 0x00ff_ffff) | 0x0200_0000
    });
    let command = if gating == 1 {
        io.write(PHY, 0x52);
        io.write(PHY, 0x53);
        wait_bits(io, PHY + 0x10, 1, 1);
        io.delay_us(10);
        if parameters.kind == 3 { 0x5a0 } else { 0x520 }
    } else if io.read(0x4a00_01c0) & 15 == 1 {
        0x62
    } else if parameters.kind == 3 {
        0x1f2
    } else {
        0x172
    };
    io.write(PHY, command);
    io.write(PHY, command | 1);
    io.delay_us(10);
    wait_bits(io, PHY + 0x10, 1, 1);
    if io.read(0x4a00_01c0) & 15 == 1 {
        modify(io, PHY + 0x10c, |v| (v & 0xf9ff_ffff) | 0x0400_0000);
        io.delay_us(10);
        modify(io, PHY + 4, |v| v | 1);
        wait_bits(io, PHY + 0x18, 7, 3);
        modify(io, 0x4a00_0838, |v| v & !1);
        io.delay_us(10);
        modify(io, PHY + 4, |v| v & !1);
        wait_bits(io, PHY + 0x18, 7, 1);
        io.delay_us(15);
        if gating == 1 {
            modify(io, PHY + 0x108, |v| v & !0xc0);
            modify(io, PHY + 0x10c, |v| (v & 0xf9ff_ffff) | 0x0200_0000);
            io.delay_us(1);
            io.write(PHY, 0x401);
            wait_bits(io, PHY + 0x10, 1, 1);
        }
    }
    let errors = io.read(PHY + 0x10) & 0x0ff0_0000;
    if errors & 0x0010_0000 != 0 {
        return false;
    }
    wait_bits(io, PHY + 0x18, 1, 1);
    modify(io, PHY + 0x8c, |v| v | (1 << 31));
    io.delay_us(10);
    modify(io, PHY + 0x8c, |v| v & !(1 << 31));
    io.delay_us(10);
    modify(io, COM + 0x14, |v| v | (1 << 31));
    io.delay_us(10);
    modify(io, PHY + 0x10c, |v| v & 0xf9ff_ffff);
    if gating == 1 {
        modify(io, PHY + 0x11c, |v| (v & !0xc0) | 0x40);
    }
    errors == 0
}

fn wait_bits(io: &mut impl Io, address: usize, mask: u32, expected: u32) {
    while io.read(address) & mask != expected {}
}

fn matches_pattern(io: &mut impl Io, address: u32, pattern_base: u32) -> bool {
    for index in 0..64u32 {
        let pattern = pattern_base.wrapping_add(index * 4);
        let pattern = if index & 1 == 0 { !pattern } else { pattern };
        if io.read(address as usize + index as usize * 4) != pattern {
            return false;
        }
    }
    true
}

fn configure_scan(io: &mut impl Io, address: usize, mask: u32, bits: u32) {
    let value = (io.read(address) & mask) | bits;
    io.write(address, value);
    wait_bits(io, address, u32::MAX, value);
}

pub fn scan_size(io: &mut impl Io, parameters: &mut Parameters) -> bool {
    if !core_init(io, parameters) {
        return false;
    }
    let ranks = if (parameters.para2 >> 12) & 15 == 1 {
        2
    } else {
        1
    };
    for rank in 0..ranks {
        let shift = rank * 16;
        let register = COM + rank as usize * 4;
        let pattern_base = if rank == 0 {
            0x8000_0000u32
        } else {
            0x8800_0000
        };
        for index in 0..64u32 {
            let address = pattern_base + index * 4;
            io.write(
                address as usize,
                if index & 1 == 0 { !address } else { address },
            );
        }
        configure_scan(io, register, 0xffff_f0f3, 0x6f0);
        let mut rows = 16;
        for candidate in 11..=16 {
            if matches_pattern(
                io,
                pattern_base.wrapping_add(1 << (candidate + 11)),
                pattern_base,
            ) {
                rows = candidate;
                break;
            }
        }
        parameters.para1 = (parameters.para1 & !(0xff << (shift + 4))) | (rows << (shift + 4));
        let bank_base = if rank == 1 {
            modify(io, COM, |v| (v & 0xffff_f003) | 0x6a4);
            0x8080_0000
        } else {
            pattern_base
        };
        configure_scan(io, register, 0xffff_f003, 0x6a4);
        let banks = u32::from(!matches_pattern(io, bank_base + 0x800, pattern_base));
        parameters.para1 = (parameters.para1 & !(15 << (shift + 12))) | (banks << (shift + 12));
        let page_base = if rank == 1 {
            modify(io, COM, |v| (v & 0xffff_f003) | 0xaa0);
            0x8400_0000
        } else {
            pattern_base
        };
        configure_scan(io, register, 0xffff_f003, 0xaa0);
        let mut columns = 13;
        for candidate in 9..=13 {
            if matches_pattern(io, page_base + (1 << candidate), pattern_base) {
                columns = candidate;
                break;
            }
        }
        let page = if columns == 9 { 0 } else { 1 << (columns - 10) };
        parameters.para1 = (parameters.para1 & !(15 << shift)) | (page << shift);
        if rank + 1 != ranks {
            modify(io, COM, |v| (v & 0xffff_f003) | 0x6f0);
            modify(io, COM + 4, |v| (v & 0xffff_f003) | 0x6f1);
        }
    }
    if ranks == 2 {
        parameters.para2 &= 0xffff_f0ff;
        if parameters.para1 >> 16 != parameters.para1 & 0xffff {
            parameters.para2 |= 0x100;
        }
    }
    true
}

pub fn scan_rank_width(io: &mut impl Io, parameters: &mut Parameters) -> bool {
    let geometry = parameters.para1;
    let flags = parameters.tpr[13];
    parameters.para1 = 0x00b0_00b0;
    parameters.para2 = (parameters.para2 & !15) | 0x1000;
    parameters.tpr[13] = (flags & !8) | 5;
    // Determine success from PGSR and DQS detection,
    // regardless of core_init's return value.
    core_init(io, parameters);
    if io.read(PHY + 0x10) & (1 << 20) != 0 || !detect_dqs(io, parameters) {
        return false;
    }
    parameters.tpr[13] = flags;
    parameters.para1 = geometry;
    true
}

pub fn scan_config(io: &mut impl Io, parameters: &mut Parameters) -> bool {
    if parameters.tpr[13] & (1 << 14) == 0 && !scan_rank_width(io, parameters) {
        return false;
    }
    if parameters.tpr[13] & 1 == 0 && !scan_size(io, parameters) {
        return false;
    }
    if parameters.tpr[13] & (1 << 15) == 0 {
        parameters.tpr[13] |= 0x6001;
    }
    true
}

pub fn init(io: &mut impl Io, parameters: &mut Parameters) -> u32 {
    if parameters.tpr[13] & (1 << 16) != 0 {
        modify(io, 0x4300_0160, |v| v | 0x100);
        io.write(0x4300_0168, 0);
        io.delay_us(10);
    } else {
        io.write(0x4a00_0838, 0);
        io.delay_us(1000);
        io.write(0x4a00_083c, 0);
        io.delay_us(1000);
        modify(io, 0x4300_0160, |v| v & !3);
        io.delay_us(10);
        modify(io, 0x4300_0160, |v| v & !0x104);
        modify(io, 0x4300_0160, |v| v | 2);
        io.delay_us(10);
        modify(io, 0x4300_0160, |v| v | 1);
        io.delay_us(20);
        let _ = io.read(0x4300_016c);
    }
    if parameters.tpr[13] & 1 == 0 && !scan_config(io, parameters) {
        return 0;
    }
    if !core_init(io, parameters) {
        return 0;
    }
    let size = if parameters.para2 & (1 << 31) != 0 {
        (parameters.para2 >> 16) & 0x7fff
    } else {
        let size = capacity(io);
        parameters.para2 = (parameters.para2 & 0xffff) | (size << 16);
        size
    };
    if parameters.tpr[13] & (1 << 30) != 0 {
        let _ = io.read(PHY + 0xa0);
        let value = if parameters.tpr[8] == 0 {
            0x1000_0200
        } else {
            parameters.tpr[8]
        };
        io.write(PHY + 0xa0, value);
        io.write(PHY + 0x9c, 0x40a);
        modify(io, PHY + 4, |v| v | 1);
    } else {
        modify(io, PHY + 0xa0, |v| v & 0xffff_0000);
        modify(io, PHY + 4, |v| v & !1);
    }
    modify(io, PHY + 0x100, |v| {
        (v & 0xffff_0fff)
            | if parameters.tpr[13] & 0x200 != 0 || parameters.kind == 6 {
                0x5000
            } else {
                0
            }
    });
    modify(io, PHY + 0x140, |v| v | (1 << 31));
    if parameters.tpr[13] & 0x100 != 0 {
        modify(io, PHY + 0xb8, |v| v | 0x300);
    }
    modify(io, PHY + 0x108, |v| {
        if parameters.tpr[13] & (1 << 26) != 0 {
            v & !0x2000
        } else {
            v | 0x2000
        }
    });
    if parameters.kind == 7 {
        modify(io, PHY + 0x7c, |v| (v & 0xfff0_ffff) | 0x10000);
    }
    if parameters.tpr[13] & (1 << 27) == 0 {
        set_master_priority(io, parameters);
    }
    set_masters(io, true);
    if parameters.tpr[13] & (1 << 28) != 0
        && io.read(0x4a00_01c0) & 15 != 1
        && !memory_test(io, size, 4096)
    {
        return 0;
    }
    size
}

pub fn core_init(io: &mut impl Io, parameters: &mut Parameters) -> bool {
    system_init(io, parameters);
    vref_zq_init(io, parameters);
    common_init(io, parameters);
    timing_init(io, parameters);
    channel_init(io, parameters)
}
