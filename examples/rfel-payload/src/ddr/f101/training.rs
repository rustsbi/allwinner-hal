//! F101 PSRAM training.

use super::{Configuration, Io, PHY, apply_ui_delay, bit_delay_compensation, core_init, modify};

/// Reset the memory test cache path.
pub fn reset_test_cache(io: &mut impl Io) {
    io.write(0x0300_2020, 0);
    modify(io, 0x0300_2020, |v| v | (1 << 16));
    modify(io, 0x0300_2020, |v| v | 1);
    modify(io, 0x0200_1544, |v| v | (1 << 30));
    io.write(0x0300_2020, 0);
}

/// Compare every word, including after a mismatch.
pub fn compare_regions(io: &mut impl Io, first: usize, second: usize, words: u32) -> bool {
    let mut equal = true;
    for i in 0..words as usize {
        let a = io.read(first + i * 4);
        let b = io.read(second + i * 4);
        equal &= a == b;
    }
    equal
}

/// Walk sixteen data bits with alternating complements.
pub fn bitflip_test(io: &mut impl Io, first: usize, second: usize, words: u32) -> bool {
    for bit in 0..16 {
        let mut pattern = 1u32 << bit;
        for _ in 0..8 {
            let previous = pattern;
            pattern = !pattern;
            for i in 0..words as usize {
                let value = if i & 1 == 0 { pattern } else { previous };
                io.write(second + i * 4, value);
                io.write(first + i * 4, value);
            }
            if !compare_regions(io, first, second, words) {
                return false;
            }
        }
    }
    true
}

/// Reinitialize and test a signed UI delay offset.
fn test_point(io: &mut impl Io, config: &mut Configuration, read: i32, write: i32) -> bool {
    let half = (config.parameters.para1 & 0xffff) >> 1;
    if !core_init(io, config) {
        return false;
    }
    apply_ui_delay(
        io,
        &config.parameters,
        read.unsigned_abs(),
        read < 0,
        write.unsigned_abs(),
        write < 0,
    );
    bitflip_test(io, 0x4000_0000, 0x4000_0000 + ((half as usize) << 20), 4096)
}

// Stop at the last passing point; +/-128 are outside the encoded range.
fn boundary(mut test: impl FnMut(i32) -> bool, step: i32) -> i32 {
    let mut offset = step;
    while offset != step * 128 {
        if !test(offset) {
            return offset - step;
        }
        offset += step;
    }
    step * 127
}

/// Center the write window, then the read window.
pub fn scan_ui_window(io: &mut impl Io, config: &mut Configuration) -> u32 {
    scan_ui_window_with(io, config, &mut test_point)
}

pub(crate) fn scan_ui_window_with<I: Io>(
    io: &mut I,
    config: &mut Configuration,
    test: &mut impl FnMut(&mut I, &mut Configuration, i32, i32) -> bool,
) -> u32 {
    if !test(io, config, 0, 0) {
        return 1;
    }
    let left = boundary(|offset| test(io, config, 0, offset), -1);
    let right = boundary(|offset| test(io, config, 0, offset), 1);
    if left >= right {
        return 2;
    }
    let write = (left + right) / 2;
    let left = boundary(|offset| test(io, config, offset, write), -1);
    let right = boundary(|offset| test(io, config, offset, write), 1);
    if left >= right {
        return 3;
    }
    let read = (left + right) / 2;
    if !test(io, config, read, write) {
        return 4;
    }
    config.parameters.tpr[4] = read.unsigned_abs()
        | (u32::from(read < 0) << 7)
        | (write.unsigned_abs() << 8)
        | (u32::from(write < 0) << 15);
    0
}

/// Exercise both full UI axes for diagnostics.
pub fn scan_ui_axes(io: &mut impl Io, config: &mut Configuration) {
    for write in -127..128 {
        let _ = test_point(io, config, 0, write);
    }
    for read in -127..128 {
        let _ = test_point(io, config, read, 0);
    }
}

fn delays(config: &mut Configuration, write: bool) -> &mut [u8; 32] {
    if write {
        &mut config.write_delays
    } else {
        &mut config.read_delays
    }
}

// RISC-V DIVU returns all ones for a zero divisor.
fn divu(numerator: u32, denominator: u32) -> u32 {
    numerator.checked_div(denominator).unwrap_or(u32::MAX)
}

/// Search individual DQ windows and compensate DQS.
pub fn scan_dq_eye(io: &mut impl Io, config: &mut Configuration) -> bool {
    scan_dq_eye_with(io, config, &mut bitflip_test)
}

pub(crate) fn scan_dq_eye_with<I: Io>(
    io: &mut I,
    config: &mut Configuration,
    test: &mut impl FnMut(&mut I, usize, usize, u32) -> bool,
) -> bool {
    let clock = config.parameters.clock;
    let required = ((config.parameters.tpr[10] >> 16) & 7) * 40_000;
    let mut margins = [0; 2];
    for (lane, margin) in margins.iter_mut().enumerate() {
        let phase = (io.read(PHY + 0x300 + lane * 0x80) >> 8) & 255;
        *margin = divu(required, divu(500_000_000, phase.wrapping_mul(clock)));
    }
    let original_write = config.parameters.tpr[11];
    let original_read = config.parameters.tpr[12];
    if !core_init(io, config) {
        // Propagate invalid training parameters to the FEL caller.
        return false;
    }
    if !test(io, 0x4000_0000, 0x4040_0000, 4096) {
        return false;
    }
    let size = config.parameters.para1;
    let lanes = 2 - (config.parameters.para2 & 1);
    reset_test_cache(io);
    config.parameters.clock = clock;
    let _ = core_init(io, config);
    let second = 0x4000_0000 + (((size & 0xffff) as usize >> 1) << 20);
    for (write, original) in [(false, original_read), (true, original_write)] {
        for (lane, &margin) in margins.iter().enumerate().take(lanes as usize) {
            if !scan_lane(io, config, write, lane, original, margin, second, test) {
                return false;
            }
        }
    }
    true
}

fn scan_lane<I: Io>(
    io: &mut I,
    config: &mut Configuration,
    write: bool,
    lane: usize,
    original: u32,
    margin: u32,
    second: usize,
    test: &mut impl FnMut(&mut I, usize, usize, u32) -> bool,
) -> bool {
    let lane = lane & 1;
    let tpr = if write { 11 } else { 12 };
    let shift = lane * 4 + 16;
    let original_dqs = (original >> shift) & 15;
    let mask = 15 << shift;
    let mut adjustments = [0u32; 8];
    let mut saved = [0u32; 8];
    let mut left_sum = 0u32;
    let mut right_sum = 0u32;
    for bit in 0..8 {
        let index = lane * 8 + bit;
        delays(config, write)[index] = 0;
        let mut step_index = 3;
        let mut ascending = false;
        let mut lower = 0u32;
        let mut last_upper = 0u32;
        let upper = loop {
            let step = 1u32 << step_index;
            let current = u32::from(delays(config, write)[index] & 15);
            let trial = if ascending {
                (current + step).min(15)
            } else {
                current.saturating_sub(step)
            };
            delays(config, write)[index] = trial as u8;
            bit_delay_compensation(io, config);
            let passed = test(io, 0x4000_0000, second, 4096);
            if ascending {
                if passed && trial != 15 {
                    last_upper = trial;
                    continue;
                }
                if !passed && step_index != 0 {
                    delays(config, write)[index] = ((trial - step) & 15) as u8;
                    step_index -= 1;
                    continue;
                }
                let sum = trial + lower;
                let threshold = adjustments[bit] + if write { original_dqs } else { 0 };
                let center = if threshold < sum {
                    ((sum.wrapping_sub(adjustments[bit]) >> 1) & 15) as u8
                } else {
                    0
                };
                delays(config, write)[index] = center;
                if !passed {
                    saved[bit] = u32::from(center);
                }
                break trial;
            }
            if passed {
                lower = trial;
                if trial != 0 {
                    continue;
                }
                let dqs = (config.parameters.tpr[tpr] >> shift) & 15;
                if dqs == 15 {
                    config.parameters.tpr[tpr] =
                        (config.parameters.tpr[tpr] & !mask) | (original & mask);
                    for other in 0..8 {
                        if other != bit {
                            delays(config, write)[lane * 8 + other] = 0;
                        }
                    }
                    ascending = true;
                    continue;
                }
                adjustments[bit] = (adjustments[bit] + step).min(15);
                let dqs = (original_dqs + adjustments[bit]).min(15);
                adjustments[bit] = dqs - original_dqs;
                config.parameters.tpr[tpr] = (config.parameters.tpr[tpr] & !mask) | (dqs << shift);
                for other in 0..8 {
                    if other != bit {
                        delays(config, write)[lane * 8 + other] =
                            ((saved[other] + adjustments[bit]) as u8).min(15);
                    }
                }
                continue;
            }
            if step_index == 0 {
                if adjustments[bit] == 0 {
                    delays(config, write)[index] = (((trial + last_upper) / 2) & 15) as u8;
                    lower = trial;
                } else {
                    adjustments[bit] -= 1;
                    config.parameters.tpr[tpr] =
                        (config.parameters.tpr[tpr] & !mask) | (original & mask);
                    for other in 0..8 {
                        if other != bit {
                            delays(config, write)[lane * 8 + other] = saved[other] as u8;
                        }
                    }
                }
                step_index = 3;
                ascending = true;
                continue;
            }
            lower = trial;
            if trial != 0 {
                delays(config, write)[index] = ((trial + step) & 15) as u8;
                step_index -= 1;
                continue;
            }
            if adjustments[bit] != 0 {
                step_index -= 1;
                let smaller = 1u32 << step_index;
                adjustments[bit] = adjustments[bit].wrapping_sub(smaller);
                for other in 0..8 {
                    if other != bit {
                        let delay = &mut delays(config, write)[lane * 8 + other];
                        *delay = delay.wrapping_sub(smaller as u8);
                    }
                }
            } else {
                let dqs = (config.parameters.tpr[tpr] >> shift) & 15;
                adjustments[bit] = step.wrapping_add(dqs).wrapping_sub(original_dqs) & 15;
                for other in 0..8 {
                    if other != bit {
                        delays(config, write)[lane * 8 + other] =
                            ((saved[other] + adjustments[bit]) as u8).min(15);
                    }
                }
            }
            let dqs = adjustments[bit].wrapping_add(original_dqs);
            config.parameters.tpr[tpr] = (config.parameters.tpr[tpr] & !mask) | (dqs << shift);
        };
        let right = upper.wrapping_sub(original_dqs);
        let left = adjustments[bit].wrapping_add(original_dqs);
        right_sum = right_sum.wrapping_add(right);
        left_sum = left_sum.wrapping_add(left);
        if right < left {
            delays(config, write)[index] = 0;
        }
        let width = right.wrapping_sub(lower).wrapping_add(left).wrapping_add(1);
        if width.wrapping_mul(2) < margin {
            return false;
        }
    }
    let high = (left_sum as i32) > (right_sum as i32);
    let difference = if high {
        left_sum.wrapping_sub(right_sum)
    } else {
        right_sum.wrapping_sub(left_sum)
    };
    let rounded = (((difference >> 4) & 15) + ((difference >> 3) & 1)) & 15;
    let shift = lane * 4 + if high { 16 } else { 0 };
    config.parameters.tpr[tpr] = (config.parameters.tpr[tpr] & !(15 << shift)) | (rounded << shift);
    true
}

/// Allow four attempts at UI/DQ calibration.
pub fn train(io: &mut impl Io, config: &mut Configuration) -> bool {
    for _ in 0..4 {
        if config.parameters.tpr[10] & (1 << 24) != 0 {
            let _ = scan_ui_window(io, config);
        }
        if config.parameters.tpr[10] & (1 << 25) != 0 {
            scan_ui_axes(io, config);
        }
        if !scan_dq_eye(io, config) {
            continue;
        }
        let ready = core_init(io, config);
        let passed = bitflip_test(io, 0x4000_0000, 0x4200_0000, 4096);
        if ready && passed {
            return true;
        }
    }
    false
}
