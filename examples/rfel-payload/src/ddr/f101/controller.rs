//! F101 PSRAM protocol setup.

use super::{Io, Parameters, modify, read_mode, reset, write_mode};

const MSI: usize = 0x0205_2000;

fn field(io: &mut impl Io, offset: usize, mask: u32, bits: u32) {
    modify(io, MSI + offset, |v| (v & mask) | bits);
}

fn set_command_timing(io: &mut impl Io, value: u32) {
    field(io, 0x150, 0x80ff_80ff, value);
}

fn set_latency(io: &mut impl Io, value: u32) {
    field(io, 0x214, 0xffff_8080, value);
}

fn finish_mode(io: &mut impl Io) {
    field(io, 0x200, 0x00ff_ffff, 0x6000_0000);
}

/// Configure the command protocol and initialize the PSRAM mode registers.
pub fn controller_initial_config(io: &mut impl Io, p: &Parameters) {
    modify(io, 0x0205_308c, |v| v & 0xffff_bbff);
    match p.kind {
        1 => {
            modify(io, 0x0205_308c, |v| (v & 0xff3f_0000) | 255);
            field(io, 0, u32::MAX, 7);
            field(io, 12, 0x03ff_ffff, 0x9000_0000);
            field(io, 0x100, u32::MAX, 2);
            field(io, 0x118, 0x0fff_ffff, 0x8000_0000);
            field(io, 0x11c, !15, 0);
            field(io, 0x124, !4, 0);
            set_command_timing(io, 0x2000_0800);
            field(io, 0x200, 0x00ff_ffff, 0x2000_0000);
            set_latency(io, 0x41c);
            field(io, 0x130, !15, 4);
            reset(io);
            write_mode(io, p.kind, 0, 63, 0);
            write_mode(io, p.kind, 4, 128, 0);
            finish_mode(io);
        }
        4 => {
            modify(io, 0x0205_308c, |v| (v & 0x00ff_0000) | 0x00c0_00ff);
            field(io, 0, u32::MAX, 7);
            field(io, 4, !0x300, 0x200);
            field(io, 12, 0x03ff_ffff, 0x8400_0000);
            field(io, 0x100, u32::MAX, 2);
            field(io, 0x118, 0x0fff_ffff, 0x8000_0000);
            field(io, 0x11c, !15, 0);
            field(io, 0x124, !4, 0);
            let timing = if p.clock <= 204 {
                Some((0x1800_1800, 0x0010_1400, 0x1212_1200, 0x518, 2))
            } else if p.clock <= 252 {
                Some((0x2200_2200, 0x0010_1e00, 0x1212_1a00, 0x522, 4))
            } else if p.clock <= 408 {
                Some((0x2e00_2e00, 0x0010_2a00, 0x1212_2700, 0x52e, 3))
            } else {
                None
            };
            if let Some((command, first, second, latency, divider)) = timing {
                set_command_timing(io, command);
                field(io, 0x200, 255, first);
                field(io, 0x204, 255, second);
                set_latency(io, latency);
                field(io, 0x130, !15, divider);
            }
            reset(io);
            write_mode(io, p.kind, 6, 64, 0);
            read_mode(io, p.kind, 6);
            finish_mode(io);
        }
        2 | 3 => {
            modify(io, 0x0205_308c, |v| (v & 0xff3f_0000) | 0x00c0_0084);
            field(io, 0, u32::MAX, 7);
            field(io, 12, 0x03ff_ffff, 0x8000_0000);
            field(io, 0x100, u32::MAX, 2);
            let _ = io.read(MSI + 0x118);
            io.write(MSI + 0x118, 0x8d95_e4cf);
            field(io, 0x11c, !15, 0);
            field(io, 0x124, !4, 0);
            if p.kind == 2 {
                set_command_timing(io, 0x1e00_1200);
                field(io, 0x204, 0xe0ff_ffff, 0x1200_0000);
                set_latency(io, 0x50e);
                reset(io);
                for (register, value) in [(0, 49), (4, 32), (8, 3)] {
                    write_mode(io, p.kind, register, value, 0);
                    read_mode(io, p.kind, register);
                }
                field(io, 0x200, 0x0fff_ffff, 0x2000_0000);
            } else {
                let high = p.clock > 204;
                if high {
                    field(io, 0x130, !15, 4);
                }
                set_command_timing(io, if high { 0x2800_1600 } else { 0x2000_1200 });
                field(io, 0x200, 0x00ff_ffff, 0x2000_0000);
                set_latency(io, if high { 0x516 } else { 0x512 });
                reset(io);
                write_mode(io, p.kind, 0, if high { 57 } else { 49 }, 0);
                write_mode(io, p.kind, 4, if high { 96 } else { 32 }, 0);
                write_mode(io, p.kind, 8, 67, 0);
                field(io, 12, u32::MAX, 0x8200_0000);
                field(io, 0x100, u32::MAX, 0x100);
                finish_mode(io);
                field(io, 0x17c, u32::MAX, 1);
                io.write(MSI + 0x180, 2000);
            }
        }
        5..=8 => {
            modify(io, 0x0205_308c, |v| (v & 0xffff_0000) | 0x00c0_00ff);
            field(io, 0, u32::MAX, 7);
            field(io, 4, !0x300, 0);
            field(io, 12, 0x03ff_ffff, 0x8800_0000);
            field(io, 12, u32::MAX, 1 << 16);
            field(io, 0x100, u32::MAX, 2);
            field(io, 0x118, 0x0fff_ffff, 0);
            field(io, 0x11c, !15, 8);
            field(io, 0x124, !4, 0);
            if p.kind == 5 {
                if p.clock <= 233 {
                    let high = p.clock > 166;
                    set_command_timing(io, if high { 0x1c00_1c00 } else { 0x1900_1c00 });
                    field(io, 0x200, 0x00ff_ffff, 0x2000_0000);
                    field(io, 0x204, 0x00ff_ffff, 0x1300_0000);
                    set_latency(io, 0x51c);
                    field(io, 0x130, !15, if high { 4 } else { 0 });
                }
                write_mode(io, p.kind, 0, 28, 143);
                finish_mode(io);
            } else {
                let low = if (p.tpr[1] >> 8) & 127 == 0 {
                    0x500
                } else {
                    p.tpr[1] & 0x7f00
                };
                let high = if (p.tpr[1] >> 16) & 127 == 0 {
                    0x0006_0000
                } else {
                    p.tpr[1] & 0x007f_0000
                };
                field(io, 0x138, 0xff80_8080, low | high | 12);
                set_command_timing(io, 0x1e00_2000);
                field(io, 0x200, 0x00ff_ffff, 0x2000_0000);
                field(io, 0x204, 0x00ff_ffff, 0x1300_0000);
                set_latency(io, 0x52c);
                write_mode(io, p.kind, 1, 193, 175);
                write_mode(io, p.kind, 0, 92, if p.kind == 8 { 135 } else { 143 });
                let low = if (p.tpr[0] >> 8) & 127 == 0 {
                    0x2c00
                } else {
                    p.tpr[0] & 0x7f00
                };
                let high = if (p.tpr[0] >> 24) & 127 == 0 {
                    0x2c00_0000
                } else {
                    p.tpr[0] & 0x7f00_0000
                };
                set_command_timing(io, low | high);
                field(io, 0x130, !15, 4);
                finish_mode(io);
            }
        }
        _ => {}
    }
    field(io, 0x12c, !8, 0);
    let timing = if (p.tpr[2] >> 8) & 63 == 0 {
        0xc00
    } else {
        p.tpr[2] & 0x2f00
    };
    field(io, 0x154, 0xff00_0000, timing | 0x008f_0005);
}

/// Select bus width and apply the final latency settings.
pub fn controller_config(io: &mut impl Io, p: &Parameters, early_terminate: bool) -> bool {
    let width = p.para2 & 17;
    let mode = match width {
        0 => 0x8000_0000,
        16 => 0xc000_0000,
        17 => 0xe000_0000,
        _ => return false,
    };
    field(io, 12, 0x1fff_ffff, mode);
    if width == 17 {
        if p.kind == 3 {
            write_mode(io, p.kind, 8, 3, 0);
        }
        modify(io, 0x0205_308c, |v| (v & 0xffff_5f00) | 0xa029);
        modify(io, 0x0205_3094, |v| (v & !3) | 1);
    }
    if p.kind == 4 && p.clock <= 408 {
        field(
            io,
            0x200,
            0x00ff_ffff,
            if p.clock <= 204 { 1 << 30 } else { 0 },
        );
        if p.clock <= 204 {
            match width {
                0 => field(io, 0x204, 0xff00_00ff, 0x0012_1200),
                16 => {
                    field(io, 0x200, 0xff00_00ff, 0x0008_1400);
                    field(io, 0x204, 0xff00_00ff, 0x000a_1200);
                }
                17 => field(io, 0x204, 0xff00_00ff, 0x0010_1000),
                _ => return false,
            }
        } else {
            let (first, second) = if p.clock <= 252 {
                if width == 16 {
                    (0x0008_1c00, 0x000a_1a00)
                } else {
                    (0x0010_1c00, 0x0012_1a00)
                }
            } else {
                match width {
                    0 => (0x4010_2a00, 0x0012_2700),
                    16 => (0x0008_2a00, 0x000a_2700),
                    _ => (0x0010_2a00, 0x0012_2700),
                }
            };
            field(io, 0x200, 0xff00_00ff, first);
            field(io, 0x204, 0xff00_00ff, second);
        }
    }
    if early_terminate {
        field(io, 0x154, 0x80ff_ffff, 1 << 30);
        field(io, 0x124, 0xffff_80ff, 1 << 14);
    }
    true
}
