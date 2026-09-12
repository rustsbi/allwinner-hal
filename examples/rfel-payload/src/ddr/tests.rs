use super::{Io, Parameters, d1, f101};

struct Replay<'a> {
    operations: core::str::SplitWhitespace<'a>,
    case: &'a str,
}

fn hex(s: &str) -> u32 {
    u32::from_str_radix(s, 16).unwrap()
}

impl Replay<'_> {
    fn access(&mut self, kind: char, address: usize) -> u32 {
        let operation = self.operations.next().expect(self.case);
        assert!(operation.starts_with(kind), "{}: {operation}", self.case);
        let (expected, value) = operation[1..].split_once(':').unwrap();
        assert_eq!(
            address,
            hex(expected) as usize,
            "{}: {operation}",
            self.case
        );
        hex(value)
    }
}

impl Io for Replay<'_> {
    fn read(&mut self, address: usize) -> u32 {
        self.access('r', address)
    }

    fn write(&mut self, address: usize, value: u32) {
        let expected = self.access('w', address);
        assert_eq!(value, expected, "{}: write {address:#x}", self.case);
    }

    fn delay_us(&mut self, microseconds: u32) {
        let operation = self.operations.next().expect(self.case);
        assert!(operation.starts_with('d'), "{}: {operation}", self.case);
        assert_eq!(microseconds, hex(&operation[1..]), "{}", self.case);
    }
}

fn parameters(line: &str) -> Parameters {
    let words: Vec<_> = line.split(',').map(hex).collect();
    assert_eq!(words.len(), 24);
    Parameters {
        clock: words[0],
        kind: words[1],
        zq: words[2],
        odt: words[3],
        para1: words[4],
        para2: words[5],
        mr: words[6..10].try_into().unwrap(),
        tpr: words[10..24].try_into().unwrap(),
    }
}

/// Replay reference MMIO/delay traces captured under Unicorn. Stack accesses
/// and diagnostic output are excluded; electrical behavior requires board tests.
#[test]
fn d1_reference_access_traces() {
    let (_, traces) = include_str!("d1.trace").split_once('\n').unwrap();
    for case in traces.split("\n---\n") {
        let mut lines = case.lines();
        let name = lines.next().unwrap();
        let (function, variant) = name.split_once(' ').unwrap();
        let mut p = parameters(lines.next().unwrap());
        let mut io = Replay {
            operations: lines.next().unwrap().split_whitespace(),
            case: name,
        };
        let expected = parameters(lines.next().unwrap());
        let result = match function {
            "voltage" => {
                d1::voltage_init(&mut io, &p);
                0
            }
            "eye" => {
                d1::eye_delay_compensation(&mut io, &p);
                0
            }
            "masters_on" => {
                d1::set_masters(&mut io, true);
                0
            }
            "masters_off" => {
                d1::set_masters(&mut io, false);
                0
            }
            "priority" => {
                d1::set_master_priority(&mut io, &p);
                0
            }
            "timing" => {
                d1::timing_init(&mut io, &mut p);
                0
            }
            "pll" => d1::set_pll(&mut io, variant.parse::<u32>().unwrap() & 1, &p),
            "system" => {
                d1::system_init(&mut io, &mut p);
                0
            }
            "common" => {
                d1::common_init(&mut io, &p);
                0
            }
            "channel" => u32::from(d1::channel_init(&mut io, &p)),
            "remap" => {
                d1::ac_remapping(&mut io, &p);
                0
            }
            "core" => u32::from(d1::core_init(&mut io, &mut p)),
            "scan_size" => u32::from(d1::scan_size(&mut io, &mut p)),
            "scan_rank" => u32::from(d1::scan_rank_width(&mut io, &mut p)),
            "scan_config" => u32::from(d1::scan_config(&mut io, &mut p)),
            "init" => d1::init(&mut io, &mut p),
            "capacity" => d1::capacity(&mut io),
            "dqs" => u32::from(d1::detect_dqs(&mut io, &mut p)),
            "memory" => u32::from(!d1::memory_test(&mut io, 256, 4)),
            "vref" => {
                d1::vref_zq_init(&mut io, &p);
                0
            }
            _ => panic!("Unknown fixture {name}"),
        };
        assert_eq!(result, hex(lines.next().unwrap()), "{name}: result");
        assert_eq!(p, expected, "{name}: parameters");
        assert_eq!(io.operations.next(), None, "{name}: missing access");
    }
}

#[test]
fn f101_reference_access_traces() {
    let (_, traces) = include_str!("f101.trace").split_once('\n').unwrap();
    for case in traces.split("\n---\n") {
        let mut lines = case.lines();
        let name = lines.next().unwrap();
        let (function, _) = name.split_once(' ').unwrap();
        let mut p = parameters(lines.next().unwrap());
        let mut io = Replay {
            operations: lines.next().unwrap().split_whitespace(),
            case: name,
        };
        let expected = parameters(lines.next().unwrap());
        let result = match function {
            "ui_window" => {
                let mut config = f101::Configuration::new(p);
                let result = f101::training::scan_ui_window_with(
                    &mut io,
                    &mut config,
                    &mut |io, _, read, write| {
                        let operation = io.operations.next().unwrap();
                        let values: std::vec::Vec<_> = operation
                            .strip_prefix('u')
                            .unwrap()
                            .split(':')
                            .map(hex)
                            .collect();
                        assert_eq!(
                            &values[..2],
                            &[read as u32, write as u32],
                            "{name}: UI point"
                        );
                        values[2] == 0
                    },
                );
                p = config.parameters;
                result
            }
            "init" => {
                let mut config = f101::Configuration::new(p);
                let result = f101::init_with(
                    &mut io,
                    &mut config,
                    &mut |io, _| hex(io.operations.next().unwrap().strip_prefix('t').unwrap()) == 0,
                    &mut |io, size, words| {
                        let operation = io.operations.next().unwrap();
                        let values: std::vec::Vec<_> = operation
                            .strip_prefix('m')
                            .unwrap()
                            .split(':')
                            .map(hex)
                            .collect();
                        assert_eq!(&values[..2], &[size, words], "{name}: memory test");
                        values[2] == 0
                    },
                );
                p = config.parameters;
                result
            }
            "eye" => {
                let mut config = f101::Configuration::new(p);
                let passed = f101::training::scan_dq_eye_with(
                    &mut io,
                    &mut config,
                    &mut |io, first, second, words| {
                        let operation = io.operations.next().unwrap();
                        let values: std::vec::Vec<_> = operation
                            .strip_prefix('b')
                            .unwrap()
                            .split(':')
                            .map(hex)
                            .collect();
                        assert_eq!(
                            &values[..3],
                            &[first as u32, second as u32, words],
                            "{name}: memory test"
                        );
                        values[3] == 0
                    },
                );
                let arrays = io.operations.next().unwrap().strip_prefix('a').unwrap();
                let expected: std::vec::Vec<u8> = (0..64)
                    .map(|i| hex(&arrays[i * 2..i * 2 + 2]) as u8)
                    .collect();
                assert_eq!(&config.read_delays, &expected[..32], "{name}: read delays");
                assert_eq!(
                    &config.write_delays,
                    &expected[32..],
                    "{name}: write delays"
                );
                p = config.parameters;
                u32::from(!passed)
            }
            "cache" => {
                f101::training::reset_test_cache(&mut io);
                0
            }
            "compare" => {
                if f101::training::compare_regions(&mut io, 0x4000_0000, 0x4040_0000, p.tpr[0]) {
                    0
                } else {
                    u32::MAX
                }
            }
            "bitflip" => {
                if f101::training::bitflip_test(&mut io, 0x4000_0000, 0x4040_0000, p.tpr[0]) {
                    0
                } else {
                    u32::MAX
                }
            }
            "key" => f101::read_key(&mut io, p.tpr[0]),
            "kind" => f101::memory_type(&mut io).unwrap_or(0),
            "merge" => u32::from(!f101::merge_parameters(&mut p)),
            "voltage" => {
                f101::voltage_init(&mut io, &p);
                0
            }
            "masters_on" => {
                f101::set_masters(&mut io, true);
                0
            }
            "masters_off" => {
                f101::set_masters(&mut io, false);
                0
            }
            "reset" => {
                f101::reset(&mut io);
                0
            }
            "write_mode" => {
                f101::write_mode(&mut io, p.kind, p.tpr[0], p.tpr[1], p.tpr[2]);
                0
            }
            "read_mode" => {
                f101::read_mode(&mut io, p.kind, p.tpr[0]);
                0
            }
            "pll" => f101::set_pll(&mut io, p.clock).unwrap_or(0),
            "system" => {
                assert!(f101::system_init(&mut io, &mut p));
                0
            }
            "bit_delay" => {
                f101::bit_delay_compensation(&mut io, &f101::Configuration::new(p));
                0
            }
            "vref" => {
                f101::vref_zq_init(&mut io, &p);
                0
            }
            "phy" => u32::from(f101::phy_initial_config(
                &mut io,
                &f101::Configuration::new(p),
            )),
            "controller" => {
                f101::controller_initial_config(&mut io, &p);
                0
            }
            "controller_config" => u32::from(f101::controller_config(&mut io, &p, p.tpr[0] != 0)),
            "ui" => {
                f101::ui_delay_compensation(&mut io, &p);
                0
            }
            "core" => {
                let mut config = f101::Configuration::new(p);
                let result = u32::from(f101::core_init(&mut io, &mut config));
                p = config.parameters;
                result
            }
            _ => panic!("Unknown fixture {name}"),
        };
        assert_eq!(result, hex(lines.next().unwrap()), "{name}: result");
        assert_eq!(p, expected, "{name}: parameters");
        assert_eq!(io.operations.next(), None, "{name}: missing access");
    }
}
#[test]
fn v821_reference_access_traces() {
    use super::v821;
    let (_, traces) = include_str!("v821.trace").split_once('\n').unwrap();
    for case in traces.split("\n---\n") {
        let mut lines = case.lines();
        let name = lines.next().unwrap();
        let (function, variant) = name.split_once(' ').unwrap();
        let mut p = parameters(lines.next().unwrap());
        let mut io = Replay {
            operations: lines.next().unwrap().split_whitespace(),
            case: name,
        };
        let expected = parameters(lines.next().unwrap());
        let result = match function {
            "masters_on" => {
                v821::set_masters(&mut io, true);
                0
            }
            "masters_off" => {
                v821::set_masters(&mut io, false);
                0
            }
            "eye" => {
                v821::eye_delay_compensation(&mut io, &p);
                0
            }
            "priority" => {
                v821::set_master_priority(&mut io, &p);
                0
            }
            "oscillator" => v821::oscillator_mhz(&mut io),
            "timer" => v821::timer_init(&mut io),
            "core" => u32::from(v821::core_init(&mut io, &mut p)),
            "scan_rank" => u32::from(v821::scan_rank_width(&mut io, &mut p)),
            "scan_size" => u32::from(v821::scan_size(&mut io, &mut p)),
            "scan_config" => u32::from(v821::scan_config(&mut io, &mut p)),
            "init" => v821::init(&mut io, &mut p),
            "timing" => {
                v821::timing_init(&mut io, &mut p);
                0
            }
            "pll" => v821::set_pll(&mut io, variant.parse::<u32>().unwrap() % 2, &p),
            "system" => {
                v821::system_init(&mut io, &mut p);
                0
            }
            "common" => {
                v821::common_init(&mut io, &p);
                0
            }
            "channel" => u32::from(v821::channel_init(&mut io, &p)),
            "capacity" => v821::capacity(&mut io),
            "dqs" => u32::from(v821::detect_dqs(&mut io, &mut p)),
            "vref" => {
                v821::vref_zq_init(&mut io, &p);
                0
            }
            _ => panic!("Unknown fixture {name}"),
        };
        assert_eq!(result, hex(lines.next().unwrap()), "{name}: result");
        assert_eq!(p, expected, "{name}: parameters");
        assert_eq!(io.operations.next(), None, "{name}: missing access");
    }
}
