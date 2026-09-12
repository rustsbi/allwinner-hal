//! V821 DDR timing calculations.

use super::{Io, PHY, Parameters, modify};

struct Timing {
    ccd: u32,
    faw: u32,
    rrd: u32,
    rcd: u32,
    rc: u32,
    xp: u32,
    wtr: u32,
    rtp: u32,
    wr: u32,
    rp: u32,
    ras: u32,
    rfc: u32,
    refi: u32,
}

impl Timing {
    fn from_parameters(p: &Parameters) -> Self {
        Self {
            ccd: (p.tpr[0] >> 21) & 7,
            faw: (p.tpr[0] >> 15) & 63,
            rrd: (p.tpr[0] >> 11) & 15,
            rcd: (p.tpr[0] >> 6) & 31,
            rc: p.tpr[0] & 63,
            xp: (p.tpr[1] >> 23) & 31,
            wtr: (p.tpr[1] >> 20) & 7,
            rtp: (p.tpr[1] >> 15) & 31,
            wr: (p.tpr[1] >> 11) & 15,
            rp: (p.tpr[1] >> 6) & 31,
            ras: p.tpr[1] & 63,
            rfc: (p.tpr[2] >> 12) & 511,
            refi: p.tpr[2] & 4095,
        }
    }

    fn calculate(p: &Parameters) -> Self {
        let cycles = |ns: u32| {
            let product = ns.wrapping_mul(p.clock >> 1);
            product / 1000 + u32::from(product % 1000 != 0)
        };
        let mut t = Self {
            ccd: 2,
            faw: 16,
            rrd: 3,
            rcd: 6,
            rc: 20,
            xp: 10,
            wtr: 3,
            rtp: 3,
            wr: 8,
            rp: 6,
            ras: 14,
            rfc: 128,
            refi: 98,
        };
        match p.kind {
            2 => {
                t.faw = cycles(50);
                t.rrd = cycles(10);
                t.rcd = cycles(20);
                t.rc = cycles(65);
                t.wtr = cycles(8);
                t.wr = cycles(15);
                t.rp = t.wr;
                t.ras = cycles(45);
                t.refi = cycles(7800).div_ceil(32);
                t.rfc = cycles(328);
                t.xp = 2;
            }
            3 => {
                t.rfc = cycles(350);
                t.refi = cycles(7800).div_ceil(32);
                t.wtr = cycles(8).max(2);
                t.wr = cycles(15).max(2);
                if p.clock <= 800 {
                    t.faw = cycles(50);
                    t.rrd = cycles(10).max(2);
                    t.rcd = cycles(15);
                    t.rc = cycles(53);
                    t.ras = cycles(38);
                    t.xp = t.wtr;
                } else {
                    t.faw = cycles(35);
                    t.rrd = cycles(6).max(2);
                    t.rcd = cycles(14);
                    t.rc = cycles(48);
                    t.ras = cycles(34);
                    t.xp = t.rrd;
                }
                t.rp = t.rcd;
            }
            6 | 7 => {
                t.faw = cycles(50).max(4);
                t.rrd = cycles(10).max(1);
                t.rcd = cycles(24).max(2);
                t.rc = cycles(70);
                t.wtr = cycles(8).max(2);
                t.wr = cycles(15).max(2);
                t.rp = cycles(17);
                t.ras = cycles(42);
                t.refi = cycles(3900).div_ceil(32);
                t.rfc = cycles(210);
                t.xp = if p.kind == 6 { cycles(8).max(1) } else { t.wtr };
            }
            _ => {}
        }
        t.refi /= match ((p.tpr[4] >> 12) & 15, p.kind) {
            (1, 2 | 3) | (2, _) => {
                if matches!(p.kind, 2 | 3) && (p.tpr[4] >> 12) & 15 == 2 {
                    4
                } else {
                    2
                }
            }
            _ => 1,
        };
        t.rtp = t.wtr;
        t
    }

    fn store(&self, p: &mut Parameters) {
        p.tpr[0] =
            (self.ccd << 21) | (self.faw << 15) | (self.rrd << 11) | (self.rcd << 6) | self.rc;
        p.tpr[1] = (self.xp << 23)
            | (self.wtr << 20)
            | (self.rtp << 15)
            | (self.wr << 11)
            | (self.rp << 6)
            | self.ras;
        p.tpr[2] = (self.rfc << 12) | self.refi;
    }
}

/// Program the timing registers, retaining the parameter-override convention.
pub fn timing_init(io: &mut impl Io, p: &mut Parameters) {
    let mut t = if p.tpr[13] & 2 != 0 {
        Timing::from_parameters(p)
    } else {
        let timing = Timing::calculate(p);
        timing.store(p);
        timing
    };
    let clock = p.clock;
    let (
        cl,
        cwl,
        rd2wr,
        wr2rd,
        wtpre,
        rasmax,
        cks,
        ckesr,
        cke,
        mod_cycles,
        mrd,
        mrw,
        read_enable,
        write_latency,
        init,
        mode,
    ) = match p.kind {
        2 => {
            let (cl, read_enable, mr0) = if clock <= 409 {
                (3, 1, 0xa63)
            } else {
                (4, 2, 0xe73)
            };
            (
                cl,
                3,
                4,
                t.wtr + 5,
                t.wr + 5,
                clock / 30,
                5,
                4,
                3,
                12,
                2,
                0,
                read_enable,
                1,
                [
                    clock.wrapping_mul(400).wrapping_add(1),
                    clock.wrapping_mul(500) / 1000 + 1,
                    clock.wrapping_mul(200).wrapping_add(1),
                    clock.wrapping_add(1),
                ],
                [mr0, p.mr[1], 0, 0],
            )
        }
        3 => {
            let (cl, read_enable, write_latency, mr0, mr2) = if clock <= 800 {
                (6, 4, 2, 0x1c70, 0x18)
            } else {
                (7, 5, 3, 0x1e14, 0x20)
            };
            let rd2wr = if (p.tpr[13] >> 2) & 3 != 1 && clock > 912 {
                6
            } else {
                5
            };
            (
                cl,
                read_enable,
                rd2wr,
                t.wtr + 2 + read_enable,
                t.wr + 2 + read_enable,
                clock / 30,
                5,
                4,
                3,
                12,
                4,
                0,
                read_enable,
                write_latency,
                [
                    clock.wrapping_mul(500).wrapping_add(1),
                    clock.wrapping_mul(360) / 1000 + 1,
                    clock.wrapping_mul(200).wrapping_add(1),
                    clock.wrapping_add(1),
                ],
                [mr0, p.mr[1], mr2, 0],
            )
        }
        6 => (
            4,
            2,
            10,
            t.wtr + 5,
            t.wr + 5,
            clock / 60,
            5,
            5,
            2,
            5,
            5,
            3,
            3,
            1,
            [
                clock.wrapping_mul(200).wrapping_add(1),
                clock.wrapping_mul(100) / 1000 + 1,
                clock.wrapping_mul(11).wrapping_add(1),
                clock.wrapping_add(1),
            ],
            [0, 0xc3, 6, p.mr[3]],
        ),
        7 => {
            let (cl, cwl, read_enable, write_latency, mr2) = if clock <= 800 {
                (6, 3, 5, 2, 10)
            } else {
                (7, 4, 6, 3, 12)
            };
            (
                cl,
                cwl,
                13,
                t.wtr + 5 + cwl,
                t.wr + 5 + cwl,
                clock / 60,
                5,
                5,
                3,
                12,
                5,
                5,
                read_enable,
                write_latency,
                [
                    clock.wrapping_mul(200).wrapping_add(1),
                    clock.wrapping_mul(100) / 1000 + 1,
                    clock.wrapping_mul(11).wrapping_add(1),
                    clock.wrapping_add(1),
                ],
                [0, 0xc3, mr2, p.mr[3]],
            )
        }
        _ => (3, 3, 4, 8, 12, 27, 4, 3, 2, 6, 2, 0, 1, 1, [0; 4], [0; 4]),
    };
    if t.rtp + t.rp < cl + 2 {
        t.rtp = cl.wrapping_sub(t.rp).wrapping_add(2);
    }
    for (mr, value) in p.mr.iter_mut().zip(mode) {
        if *mr >> 16 == 0 {
            *mr = value;
        }
    }
    for (index, mr) in p.mr.iter().enumerate() {
        io.write(PHY + 0x30 + index * 4, mr & 0xffff);
    }
    io.write(PHY + 0x2c, (p.odt >> 4) & 3);
    for (offset, value) in [
        (0x58, (wtpre << 24) | (t.faw << 16) | (rasmax << 8) | t.ras),
        (0x5c, (t.xp << 16) | (t.rtp << 8) | t.rc),
        (0x60, (cwl << 24) | (cl << 16) | (rd2wr << 8) | wr2rd),
        (0x64, (mrw << 16) | (mrd << 12) | mod_cycles),
        (0x68, (t.rcd << 24) | (t.ccd << 16) | (t.rrd << 8) | t.rp),
        (0x6c, (cks << 24) | (cks << 16) | (ckesr << 8) | cke),
    ] {
        io.write(PHY + offset, value);
    }
    modify(io, PHY + 0x78, |v| {
        (v & 0x0fff_0000)
            | if clock > 800 {
                0xf000_7610
            } else {
                0xf000_6610
            }
    });
    io.write(
        PHY + 0x80,
        (read_enable << 16) | write_latency | 0x0200_0100,
    );
    io.write(PHY + 0x50, (init[1] << 20) | init[0]);
    io.write(PHY + 0x54, (init[3] << 20) | init[2]);
    io.write(PHY + 0x90, (t.refi << 16) | t.rfc);
    io.write(PHY + 0x94, (t.refi << 15) & 0x0fff_0000);
}
