//! D1/F133 DDR initialization.

use super::{Io, Parameters};

mod timing;
pub use timing::timing_init;

const PLL_DDR: usize = 0x0200_1010;
const MBUS_CLOCK: usize = 0x0200_1540;
const DRAM_CLOCK: usize = 0x0200_1800;
const DRAM_BUS: usize = 0x0200_180c;
const COM: usize = 0x0310_2000;
const PHY: usize = 0x0310_3000;

fn modify(io: &mut impl Io, address: usize, f: impl FnOnce(u32) -> u32) {
    let value = io.read(address);
    io.write(address, f(value));
}

/// Select the DDR2/DDR3 supply voltage.
pub fn voltage_init(io: &mut impl Io, parameters: &Parameters) {
    let voltage = match parameters.kind {
        2 => 47,
        3 => 25,
        _ => 0,
    };
    modify(io, 0x0300_0150, |v| {
        ((v & 0xffff_00ff) | (voltage << 8)) & 0xffdf_ffff
    });
    io.delay_us(1);
}

/// Apply the per-lane DQ/DQS and address delays.
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

/// Program MBUS priorities and PHY pads.
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

/// Configure rank geometry and controller topology.
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

// Keep the index lookup separate: folding the caller's match into a table of
// pointers creates absolute .data relocations in a raw position-independent bin.
#[inline(never)]
fn ac_map(index: usize) -> [u8; 22] {
    const MAPS: [[u8; 22]; 7] = [
        [
            1, 9, 3, 7, 8, 18, 4, 13, 5, 6, 10, 2, 14, 12, 0, 0, 21, 17, 20, 19, 11, 22,
        ],
        [
            4, 9, 3, 7, 8, 18, 1, 13, 2, 6, 10, 5, 14, 12, 0, 0, 21, 17, 20, 19, 11, 22,
        ],
        [
            1, 7, 8, 12, 10, 18, 4, 13, 5, 6, 3, 2, 9, 0, 0, 0, 21, 17, 20, 19, 11, 22,
        ],
        [
            4, 12, 10, 7, 8, 18, 1, 13, 2, 6, 3, 5, 9, 0, 0, 0, 21, 17, 20, 19, 11, 22,
        ],
        [
            13, 2, 7, 9, 12, 19, 5, 1, 6, 3, 4, 8, 10, 0, 0, 0, 21, 22, 18, 17, 11, 20,
        ],
        [
            3, 10, 7, 13, 9, 11, 1, 2, 4, 6, 8, 5, 12, 0, 0, 0, 20, 1, 0, 21, 22, 17,
        ],
        [
            3, 2, 4, 7, 9, 1, 17, 12, 18, 14, 13, 8, 15, 6, 10, 5, 19, 22, 16, 21, 20, 11,
        ],
    ];
    MAPS.get(index).copied().unwrap_or([0; 22])
}

/// Select the package-specific address/command wiring.
pub fn ac_remapping(io: &mut impl Io, parameters: &Parameters) {
    let fuse = (io.read(0x0300_6228) >> 8) & 15;
    let index = if parameters.kind == 2 {
        if !matches!(fuse, 13 | 14) {
            return;
        }
        5
    } else if parameters.kind != 3 {
        return;
    } else if parameters.tpr[13] & (3 << 18) != 0 {
        6
    } else {
        match fuse {
            8 => 1,
            9 => 2,
            10 => 4,
            12 => 0,
            13 | 14 => 7,
            _ => 3,
        }
    };
    let map = ac_map(index);
    let pack = |start: usize, count: usize, first_shift: u32| {
        let mut value = 0;
        for index in 0..count {
            value |= (map[start + index] as u32) << (first_shift + index as u32 * 5);
        }
        value
    };
    let first = pack(0, 5, 5);
    io.write(COM + 0x500, first);
    io.write(COM + 0x504, pack(5, 6, 0));
    io.write(COM + 0x508, pack(11, 5, 0));
    io.write(COM + 0x50c, pack(16, 6, 0));
    io.write(COM + 0x500, first | 1);
}

/// Initialize the controller and run PHY training.
pub fn core_init(io: &mut impl Io, parameters: &mut Parameters) -> bool {
    system_init(io, parameters);
    vref_zq_init(io, parameters);
    common_init(io, parameters);
    ac_remapping(io, parameters);
    timing_init(io, parameters);
    channel_init(io, parameters)
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

/// Detect row, bank and page aliases for each rank.
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
            0x4000_0000u32
        } else {
            0x4800_0000
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
            0x4080_0000
        } else {
            pattern_base
        };
        configure_scan(io, register, 0xffff_f003, 0x6a4);
        let banks = u32::from(!matches_pattern(io, bank_base + 0x800, pattern_base));
        parameters.para1 = (parameters.para1 & !(15 << (shift + 12))) | (banks << (shift + 12));
        let page_base = if rank == 1 {
            modify(io, COM, |v| (v & 0xffff_f003) | 0xaa0);
            0x4400_0000
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

/// Train a provisional two-rank, full-width layout.
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

/// Perform the scans requested by the parameter flags.
pub fn scan_config(io: &mut impl Io, parameters: &mut Parameters) -> bool {
    if parameters.tpr[13] & (1 << 14) == 0 && !scan_rank_width(io, parameters) {
        return false;
    }
    if parameters.tpr[13] & 1 == 0 && !scan_size(io, parameters) {
        return false;
    }
    if parameters.tpr[13] & (1 << 15) == 0 {
        parameters.tpr[13] |= 0x6003;
    }
    true
}

/// Initialize DRAM and return its capacity in MiB.
/// A zero result reports initialization or memory-test failure.
pub fn init(io: &mut impl Io, parameters: &mut Parameters) -> u32 {
    if parameters.tpr[13] & (1 << 16) != 0 {
        modify(io, 0x0300_0160, |v| v | 0x100);
        io.write(0x0300_0168, 0);
        io.delay_us(10);
    } else {
        io.write(0x0701_0254, 0);
        modify(io, 0x0300_0160, |v| v & !3);
        io.delay_us(10);
        let value = io.read(0x0300_0160) & !0x108;
        io.write(0x0300_0160, value);
        io.write(0x0300_0160, value | 2);
        io.delay_us(10);
        modify(io, 0x0300_0160, |v| v | 1);
        io.delay_us(20);
        let _ = io.read(0x0300_016c);
    }
    // get_pmu_exist in this payload always returns -1.
    voltage_init(io, parameters);
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
    set_masters(io, true);
    if parameters.tpr[13] & (1 << 28) != 0
        && io.read(0x0700_05d4) & (1 << 16) == 0
        && !memory_test(io, size, 4096)
    {
        return 0;
    }
    size
}

fn wait_bits(io: &mut impl Io, address: usize, mask: u32, expected: u32) {
    while io.read(address) & mask != expected {}
}

/// Start PHY initialization and DQS training.
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
    if io.read(0x0700_05d4) & (1 << 16) != 0 {
        modify(io, 0x0701_0250, |v| v & !2);
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
    } else if io.read(0x0700_05d4) & (1 << 16) != 0 {
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
    if io.read(0x0700_05d4) & (1 << 16) != 0 {
        modify(io, PHY + 0x10c, |v| (v & 0xf9ff_ffff) | 0x0400_0000);
        io.delay_us(10);
        modify(io, PHY + 4, |v| v | 1);
        wait_bits(io, PHY + 0x18, 7, 3);
        modify(io, 0x0701_0250, |v| v & !1);
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

/// Configure all three master masks.
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

/// Return the programmed DDR PLL frequency in MHz.
pub fn set_pll(io: &mut impl Io, index: u32, parameters: &Parameters) -> u32 {
    let clock = if ((parameters.tpr[13] >> 6) & 1) == index {
        parameters.clock
    } else {
        parameters.tpr[9]
    };
    let multiplier = clock.wrapping_mul(2) / 24;
    let value = (io.read(PLL_DDR) & 0xfff8_00fc) | multiplier.wrapping_sub(1).wrapping_shl(8);
    // Write the cached value three times; do not
    // merge these into one RMW or reread between lock-detector transitions.
    io.write(PLL_DDR, value | 0xc000_0000);
    let value = value & 0xdfff_ffff;
    io.write(PLL_DDR, value | 0xc000_0000);
    io.write(PLL_DDR, value | 0xe000_0000);
    while io.read(PLL_DDR) & (1 << 28) == 0 {}
    io.delay_us(20);
    let value = io.read(PLL_DDR);
    io.write(PLL_DDR, value | (1 << 27));
    let value = io.read(DRAM_CLOCK);
    io.write(DRAM_CLOCK, (value & 0xfcff_fcfc) | (1 << 31));
    multiplier.wrapping_mul(24)
}

/// Sequence the DDR PLL, bus reset and PHY clock.
pub fn system_init(io: &mut impl Io, parameters: &mut Parameters) {
    let value = io.read(MBUS_CLOCK);
    io.write(MBUS_CLOCK, value & 0xbfff_ffff);
    let value = io.read(DRAM_BUS);
    io.write(DRAM_BUS, value & !1);
    io.write(DRAM_BUS, value & 0xfffe_fffe);
    let value = io.read(DRAM_CLOCK);
    io.write(DRAM_CLOCK, value & 0xbfff_ffff);
    io.write(DRAM_CLOCK, value & 0x3fff_ffff);
    io.write(DRAM_CLOCK, (value & 0x3fff_ffff) | (1 << 27));
    io.delay_us(10);
    parameters.clock = set_pll(io, 0, parameters) >> 1;
    io.delay_us(100);
    set_masters(io, false);
    let value = io.read(DRAM_BUS);
    io.write(DRAM_BUS, value | (1 << 16));
    let value = io.read(MBUS_CLOCK);
    io.write(MBUS_CLOCK, value | (1 << 30));
    let value = io.read(DRAM_CLOCK);
    io.write(DRAM_CLOCK, value | (1 << 30));
    io.delay_us(5);
    let value = io.read(DRAM_BUS);
    io.write(DRAM_BUS, value | 1);
    let value = io.read(DRAM_CLOCK);
    io.write(DRAM_CLOCK, value | (1 << 31));
    io.write(DRAM_CLOCK, value | 0x8800_0000);
    io.delay_us(5);
    io.write(PHY + 0x0c, 0x8000);
    io.delay_us(10);
}

/// Calculate capacity in MiB from the rank registers.
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

/// Infer rank count and bus width from DQS training.
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

/// Test the start and midpoint, in that write order.
/// Returns false on the first mismatch; the test overwrites both regions.
pub fn memory_test(io: &mut impl Io, size_mib: u32, words: u32) -> bool {
    let midpoint = (size_mib >> 1).wrapping_shl(20);
    for index in 0..words {
        let address = 0x4000_0000u32.wrapping_add(index.wrapping_mul(4));
        io.write(address as usize, 0x0123_4567u32.wrapping_add(index));
        io.write(
            address.wrapping_add(midpoint) as usize,
            0xfedc_ba98u32.wrapping_add(index),
        );
    }
    // Check the midpoint before checking the first region.
    for index in 0..words {
        let address = 0x4000_0000usize + (index.wrapping_mul(4) as usize);
        if io.read(address + midpoint as usize) != 0xfedc_ba98u32.wrapping_add(index)
            || io.read(address) != 0x0123_4567u32.wrapping_add(index)
        {
            return false;
        }
    }
    true
}

/// Program Vref unless the parameter flags skip it.
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
