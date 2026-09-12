//! Shared F101-S2/S3 PSRAM initialization.

use super::{Io, Parameters};

mod controller;
pub mod training;
pub use controller::{controller_config, controller_initial_config};

const SID: usize = 0x0300_6000;
const PLL_DDR: usize = 0x0200_1010;
const DRAM_CLOCK: usize = 0x0200_1800;
const COM: usize = 0x0310_2000;
const PHY: usize = 0x0310_3000;
const COMMAND: usize = 0x0205_2210;
const DATA: usize = 0x0205_220c;
const CONTROL: usize = 0x0205_2104;
const STATUS: usize = 0x0205_2108;

/// The PSRAM routine extends the common prefix with trained per-bit delays.
#[repr(C)]
pub struct Configuration {
    pub parameters: Parameters,
    pub read_delays: [u8; 32],
    pub write_delays: [u8; 32],
}

impl Configuration {
    pub const fn new(parameters: Parameters) -> Self {
        Self {
            parameters,
            read_delays: [0; 32],
            write_delays: [0; 32],
        }
    }
}

fn modify(io: &mut impl Io, address: usize, f: impl FnOnce(u32) -> u32) {
    let value = io.read(address);
    io.write(address, f(value));
}

/// Power the SID and read one key word.
pub fn read_key(io: &mut impl Io, offset: u32) -> u32 {
    io.write(0x0309_0310, 1 << 31);
    io.write(0x0309_0204, 1);
    io.write(SID + 4, offset >> 2);
    let value = io.read(SID) & 0xfffc;
    io.write(SID, value | 0xadbf_0002);
    while io.read(SID) & 2 != 0 {}
    io.write(SID, value);
    io.read(SID + 12)
}

/// Identify the PSRAM variant from SID fields.
pub fn memory_type(io: &mut impl Io) -> Option<u32> {
    let chip = read_key(io, 0) & 0xffff;
    let package = read_key(io, 28) >> 26;
    match (chip, package) {
        (0x4100, 6) => Some(1),
        (0x4100, 7) => Some(5),
        (0x4300, 4) => Some(7),
        (0x4300, 3 | 5) => Some(3),
        _ => None,
    }
}

/// Constrain the selected chip's clock and capacity.
pub fn merge_parameters(parameters: &mut Parameters) -> bool {
    let (clock_limit, size_limit) = match parameters.kind {
        1 | 5 => (233, 8),
        3 | 7 => (252, 16),
        _ => return false,
    };
    if parameters.clock == 0 || parameters.clock > clock_limit {
        parameters.clock = clock_limit;
    }
    if parameters.zq == 0 {
        parameters.zq = 0x007b_7bfb;
    }
    if parameters.kind == 5 && parameters.clock > 200 {
        parameters.zq = 0x007b_fbfb;
    }
    let mut size = parameters.para1 & 0xffff;
    if size == 0 || size > size_limit {
        size = size_limit;
    }
    parameters.para1 = (parameters.para1 & 0xffff_0000) | size;
    true
}

/// Apply the optional voltage setting and read it back.
pub fn voltage_init(io: &mut impl Io, parameters: &Parameters) {
    let voltage = parameters.tpr[3] & 255;
    if voltage != 0 {
        modify(io, 0x0300_0150, |v| (v & !255) | voltage);
    }
    let _ = io.read(0x0300_0150);
}

/// Configure master access without a delay.
pub fn set_masters(io: &mut impl Io, enable: bool) {
    let masks = if enable {
        [u32::MAX, 255, 65535]
    } else {
        [1, 0, 0]
    };
    for (index, value) in masks.into_iter().enumerate() {
        io.write(COM + 32 + index * 4, value);
    }
}

/// Trigger a reset command, without waiting here.
pub fn reset(io: &mut impl Io) {
    let _ = io.read(COMMAND);
    io.write(COMMAND, 0xffff_0000);
    let _ = io.read(DATA);
    io.write(DATA, 0);
    modify(io, CONTROL, |v| (v & !15) | 13);
}

fn wait_command(io: &mut impl Io) {
    while io.read(STATUS) & 1 != 0 {}
}

/// Encode a mode-register write for the PSRAM protocol.
pub fn write_mode(io: &mut impl Io, kind: u32, register: u32, first: u32, second: u32) {
    let command = match kind {
        1 => Some((0xc000_0000 | (register & 255), first << 24)),
        2 | 3 => Some((
            0xc0c0_0000,
            ((first << 8) & 0xffff) | ((register << 16) & 0x00ff_0000),
        )),
        4 => Some((
            0xc0c0_0000
                | ((register << 24) & 0x0700_0000)
                | ((register << 16) & 0x0007_0000)
                | (first & 255),
            0,
        )),
        5..=8 => Some((
            0x6000_0100,
            ((second << 8) & 0xffff) | (first & 255) | ((register << 16) & 0x00ff_0000),
        )),
        _ => None,
    };
    if let Some((command, data)) = command {
        io.write(COMMAND, command);
        io.write(DATA, data);
    }
    modify(io, CONTROL, |v| (v & !15) | 13);
    wait_command(io);
}

/// Issue a mode-register read; the caller reads DATA.
pub fn read_mode(io: &mut impl Io, kind: u32, register: u32) {
    let command = match kind {
        1 => Some((0x4000_0000 | (register & 255), 0)),
        2 | 3 => Some((0x4040_0000, (register << 16) & 0x00ff_0000)),
        4 => Some((
            0x4040_0000 | ((register << 24) & 0x0700_0000) | ((register << 16) & 0x0007_0000),
            0,
        )),
        5..=8 => Some((0xe000_0100, (register << 16) & 0x00ff_0000)),
        _ => None,
    };
    if let Some((command, data)) = command {
        io.write(COMMAND, command);
        io.write(DATA, data);
    }
    modify(io, CONTROL, |v| v | 15);
    wait_command(io);
}

/// Return the PLL frequency; invalid clocks stop init.
pub fn set_pll(io: &mut impl Io, clock: u32) -> Option<u32> {
    if clock <= 200 {
        let _ = io.read(DRAM_CLOCK);
        io.write(DRAM_CLOCK, u32::MAX);
        // Report invalid clock parameters to the caller.
        return None;
    }
    let multiplier = clock / 24;
    modify(io, PLL_DDR, |v| {
        (v & 0xf7ff_00fc) | ((multiplier - 1) << 8) | 0xc000_0000
    });
    modify(io, PLL_DDR, |v| v & !(1 << 29));
    modify(io, PLL_DDR, |v| v | (1 << 29));
    while io.read(PLL_DDR) & (1 << 28) == 0 {}
    modify(io, PLL_DDR, |v| v | (1 << 27));
    modify(io, DRAM_CLOCK, |v| (v & 0xf8ff_fcfc) | 0x8100_0000);
    Some(multiplier * 24)
}

/// Sequence the PSRAM clock, resets and PHY enable.
pub fn system_init(io: &mut impl Io, parameters: &mut Parameters) -> bool {
    modify(io, 0x0300_01f0, |v| v | 1);
    modify(io, DRAM_CLOCK + 8, |v| v & !(1 << 16));
    modify(io, 0x0200_1544, |v| v & !(1 << 31));
    modify(io, 0x0200_1540, |v| v & !(1 << 30));
    modify(io, DRAM_CLOCK + 12, |v| v & !1);
    modify(io, DRAM_CLOCK + 12, |v| v & !(1 << 16));
    modify(io, DRAM_CLOCK, |v| v & !(1 << 30));
    modify(io, DRAM_CLOCK, |v| v & !(1 << 31));
    modify(io, DRAM_CLOCK, |v| v | (1 << 27));
    let Some(clock) = set_pll(io, parameters.clock.wrapping_mul(2)) else {
        return false;
    };
    parameters.clock = clock >> 1;
    set_masters(io, false);
    // The vendor code ORs the address literal 0x03000001, not just bit 0.
    modify(io, 0x0200_1544, |v| v | 0x0300_0001);
    modify(io, DRAM_CLOCK + 12, |v| v | (1 << 16));
    modify(io, 0x0200_1540, |v| v | (1 << 30));
    modify(io, DRAM_CLOCK, |v| v | (1 << 30));
    modify(io, DRAM_CLOCK + 8, |v| v | (1 << 16));
    modify(io, DRAM_CLOCK + 12, |v| v | 1);
    modify(io, 0x0200_1544, |v| v | (1 << 31));
    modify(io, DRAM_CLOCK, |v| v | (1 << 31));
    modify(io, DRAM_CLOCK, |v| v | (1 << 27));
    modify(io, PHY + 12, |v| (v & !32) | 64);
    modify(io, PHY + 12, |v| v | (1 << 15));
    true
}

/// Program the measured or parameter-specified delays.
pub fn bit_delay_compensation(io: &mut impl Io, config: &Configuration) {
    let p = &config.parameters;
    modify(io, PHY + 0x100, |v| v & !(1 << 26));
    let trained = p.tpr[10] & (1 << 20) != 0;
    for index in 0..16 {
        let shift = (index / 8) * 4;
        let write = if trained {
            config.write_delays[index] as u32
        } else {
            (p.tpr[11] >> shift) & 15
        };
        let read = if trained {
            config.read_delays[index] as u32
        } else {
            (p.tpr[12] >> shift) & 15
        };
        let offset = if index < 8 {
            0x310 + index * 4
        } else {
            0x370 + index * 4
        };
        modify(io, PHY + offset, |v| {
            (v & 0xffff_c0c0) | ((write << 9) & 0x1e00) | ((read << 1) & 30)
        });
    }
    modify(io, PHY + 0x100, |v| v & !(1 << 26));
    for (offset, shift) in [(0x334, 9), (0x338, 9), (0x3b4, 5), (0x3b8, 5)] {
        modify(io, PHY + offset, |v| {
            (v & 0xffff_c0c0) | ((p.tpr[11] << shift) & 0x1e00)
        });
    }
    // Preserve the second set of RMWs, even though its mask clears fields
    // programmed above. This is how the vendor's DQS compensation executes.
    for (offset, shift) in [(0x334, 15), (0x338, 15), (0x3b4, 19), (0x3b8, 19)] {
        modify(io, PHY + offset, |v| {
            (v & 0xffff_c0c0) | ((p.tpr[12] >> shift) & 30)
        });
    }
    if matches!(p.kind, 1 | 3 | 5 | 7 | 8) {
        modify(io, 0x0205_3058, |v| {
            let common =
                (v & 0x8000_007f) | ((p.tpr[11] >> 8) & 0xf00) | ((p.tpr[10] << 12) & 0x00f0_0000);
            if matches!(p.kind, 1 | 5) {
                common | ((p.tpr[11] >> 6) & 0x0003_c000) | ((p.tpr[10] << 14) & 0x3c00_0000)
            } else {
                common | ((p.tpr[11] >> 12) & 0xf00)
            }
        });
    }
    for (offset, shift) in [(0x33c, 9), (0x3bc, 5)] {
        modify(io, PHY + offset, |v| {
            v | ((p.tpr[11] << shift) & 0x1e00_0000)
        });
    }
    modify(io, PHY + 0x100, |v| v | 0x0600_0000);
    for (start, end) in [(0x240, 0x27c), (0x228, 0x240)] {
        for offset in (start..end).step_by(4) {
            modify(io, PHY + offset, |v| v | ((p.tpr[10] << 4) & 0xf00));
        }
    }
    modify(io, PHY + 0x218, |v| v | ((p.tpr[10] << 8) & 0xf00));
    modify(io, PHY + 0x21c, |v| v | (p.tpr[10] & 0xf00));
    modify(io, PHY + 0x280, |v| v | ((p.tpr[10] >> 4) & 0xf00));
}

/// Use default Vref values when fields are zero.
pub fn vref_zq_init(io: &mut impl Io, p: &Parameters) {
    if p.tpr[13] & (1 << 17) != 0 {
        return;
    }
    modify(io, PHY + 0x110, |v| {
        (v & 0x8080_8080) | if p.tpr[5] == 0 { 0x4848_4848 } else { p.tpr[5] }
    });
    if p.tpr[13] & (1 << 16) == 0 {
        let value = p.tpr[6] & 127;
        modify(io, PHY + 0x114, |v| {
            (v & !127) | if value == 0 { 72 } else { value }
        });
    }
}

/// Write the enabled data-bit remapping pairs.
pub fn data_remapping(io: &mut impl Io, mask: u32, low: &[u32; 4], high: &[u32; 4]) {
    for index in 0..4 {
        if mask & (1 << index) != 0 {
            io.write(COM + 0x510 + index * 8, low[index]);
            io.write(COM + 0x514 + index * 8, high[index]);
        }
    }
    modify(io, COM + 0x500, |v| (v & !30) | ((mask << 1) & 30));
}

/// Initialize the PHY and wait for calibration.
pub fn phy_initial_config(io: &mut impl Io, config: &Configuration) -> bool {
    let p = &config.parameters;
    modify(io, COM + 8, |v| (v & 0xffff_c0ff) | 0x1000);
    let _ = io.read(COM + 20);
    io.write(COM + 20, 0x2020);
    modify(io, PHY + 12, |v| v | 0xa020);
    modify(io, COM + 12, |v| (v & 0xffff_f000) | 399);
    modify(io, COM, |v| {
        let width = if p.para2 & 16 != 0 {
            if p.para2 & 1 != 0 { 0x2000 } else { 0x3000 }
        } else {
            (p.para2 << 13) & 0x2000
        };
        (v & 0xffff_8fff) | width | (1 << 27)
    });
    modify(io, PHY + 0x44, |v| (v & !63) | 195);
    modify(io, PHY + 0x208, |v| (v & 0xfff8_0037) | 0x10000);
    bit_delay_compensation(io, config);
    modify(io, PHY + 0x108, |v| (v & 0xffff_f03f) | 0x380);
    modify(io, PHY + 0xbc, |v| (v & !7) | 0x104);
    modify(io, PHY + 0x11c, |v| v & 0x00ff_ffff);
    modify(io, PHY + 0x140, |v| {
        let zq = p.zq & 0x00ff_ffff;
        (v & 0xf800_0000) | (if zq == 0 { 0x003b_3bbb } else { zq }) | (1 << 25)
    });
    io.write(PHY + 0x444, 0);
    io.write(PHY + 0x4c4, 0);
    // Package-specific data remapping is installed below before training.
    if p.kind == 7 {
        data_remapping(io, 1, &[0x7564_3210, 0x7654_3210, 0, 0], &[8; 4]);
    } else if p.kind == 8 {
        data_remapping(io, 3, &[0x0167_2345, 0x2640_7351, 0, 0], &[8; 4]);
    }
    modify(io, PHY + 0xc0, |v| (v & 0xf000_0000) | 0x0100_3087);
    let command = io.read(PHY);
    io.write(PHY, command | 0x62);
    io.write(PHY, command | 0x63);
    while io.read(PHY + 16) & 1 == 0 {}
    let errors = io.read(PHY + 16) & 0x0ff0_0000;
    if errors & (1 << 20) != 0 {
        return false;
    }
    while io.read(PHY + 24) & 1 == 0 {}
    modify(io, PHY + 0x8c, |v| v | (1 << 31));
    modify(io, PHY + 0x8c, |v| v & !(1 << 31));
    modify(io, COM + 20, |v| v | (1 << 31));
    modify(io, PHY + 0x10c, |v| v & 0xf9ff_ffff);
    let value = io.read(COM + 0x50);
    let width = (io.read(COM) >> 12) & 2;
    io.write(COM + 0x50, value & !width);
    errors == 0
}

/// Adjust DQS UI delays with byte-wise saturation.
pub fn ui_delay_compensation(io: &mut impl Io, p: &Parameters) {
    apply_ui_delay(
        io,
        p,
        p.tpr[4] & 127,
        p.tpr[4] & 128 != 0,
        (p.tpr[4] >> 8) & 127,
        p.tpr[4] & 0x8000 != 0,
    );
}

fn apply_ui_delay(
    io: &mut impl Io,
    p: &Parameters,
    read: u32,
    subtract_read: bool,
    write: u32,
    subtract_write: bool,
) {
    let read = read * 2;
    let write = write * 2;
    for lane in 0..2 - (p.para2 & 1) {
        modify(io, PHY + 0x308 + lane as usize * 0x80, |v| {
            let rd = v & 255;
            let wr = (v >> 8) & 255;
            let rd = if subtract_read {
                rd.saturating_sub(read)
            } else {
                (rd + read).min(255)
            };
            let wr = if subtract_write {
                wr.saturating_sub(write)
            } else {
                (wr + write).min(255)
            };
            (v & 0xff00_0000) | rd | (wr << 8) | (wr << 16)
        });
    }
}

/// Configure the PSRAM PHY and controller together.
pub fn core_init(io: &mut impl Io, config: &mut Configuration) -> bool {
    if !system_init(io, &mut config.parameters) {
        return false;
    }
    vref_zq_init(io, &config.parameters);
    let phy_ok = phy_initial_config(io, config);
    ui_delay_compensation(io, &config.parameters);
    controller_initial_config(io, &config.parameters);
    // Keep early termination disabled for controller setup.
    let controller_ok = controller_config(io, &config.parameters, false);
    modify(io, COM + 20, |v| v | (1 << 31));
    phy_ok && controller_ok
}

/// Detect the package, train, initialize, and test PSRAM.
pub fn init(io: &mut impl Io, config: &mut Configuration) -> u32 {
    init_with(
        io,
        config,
        &mut training::train,
        &mut super::d1::memory_test,
    )
}

pub(crate) fn init_with<I: Io>(
    io: &mut I,
    config: &mut Configuration,
    train: &mut impl FnMut(&mut I, &mut Configuration) -> bool,
    test: &mut impl FnMut(&mut I, u32, u32) -> bool,
) -> u32 {
    modify(io, 0x0300_0160, |v| v & !2);
    let _ = io.read(0x0300_0174);
    io.write(0x0300_0174, 0x1937_0505);
    let _ = io.read(0x0300_0174);
    voltage_init(io, &config.parameters);
    config.write_delays.fill(0);
    config.read_delays.fill(0);
    if config.parameters.tpr[10] & ((1 << 31) | (1 << 19)) == 1 << 19 {
        if !train(io, config) {
            return 0;
        }
        config.parameters.tpr[10] |= 1 << 31;
    }
    if read_key(io, 0) & 0xffff != 0 && config.parameters.tpr[13] & 1 == 0 {
        config.parameters.kind = memory_type(io).unwrap_or(0);
        if config.parameters.kind == 0 {
            return 0;
        }
        let _ = merge_parameters(&mut config.parameters);
    }
    let size = config.parameters.para1 & 0xffff;
    if !core_init(io, config) {
        return 0;
    }
    set_masters(io, true);
    let _ = io.read(PHY + 0x140);
    if !test(io, size, 4096) {
        return 0;
    }
    size
}
