//! Allwinner GPIO controller.
mod eint;
mod function;
mod input;
mod mode;
mod output;
mod pad_ext;
mod register;

pub use eint::{EintPad, Event};
pub use function::Function;
pub use input::Input;
pub use output::Output;
pub use pad_ext::PadExt;
pub use register::{Eint, PioPow, Port, RegisterBlock};

// PA to PG: PA => 0, PB => 1, .., PG => 6
// PL:       PL => 0

#[inline]
const fn port_index(p: char) -> usize {
    assert!((p as usize >= b'A' as usize && p as usize <= b'G' as usize) || p == 'L');
    match p {
        'A'..='G' => p as usize - b'A' as usize,
        'L' => 0,
        _ => unreachable!(),
    }
}

#[inline]
const fn port_cfg_index(p: char, n: u8) -> (usize, usize, u8) {
    assert!((p as usize >= b'A' as usize && p as usize <= b'G' as usize) || p == 'L');
    assert!(n <= 31);
    let port_idx = port_index(p);
    let cfg_reg_idx = (n >> 3) as usize;
    let cfg_field_idx = (n & 0b111) << 2;
    (port_idx, cfg_reg_idx, cfg_field_idx)
}

#[cfg(test)]
mod tests {
    use super::{port_cfg_index, port_index};

    #[test]
    fn test_port_index() {
        assert_eq!(port_index('A'), 0);
        assert_eq!(port_index('B'), 1);
        assert_eq!(port_index('C'), 2);
        assert_eq!(port_index('D'), 3);
        assert_eq!(port_index('E'), 4);
        assert_eq!(port_index('F'), 5);
        assert_eq!(port_index('G'), 6);
        assert_eq!(port_index('L'), 0);
    }

    #[test]
    fn test_port_cfg_index() {
        let test_cases = [
            (('A', 0), (0, 0, 0)),
            (('A', 5), (0, 0, 20)),
            (('A', 7), (0, 0, 28)),
            (('A', 8), (0, 1, 0)),
            (('A', 24), (0, 3, 0)),
            (('A', 31), (0, 3, 28)),
            (('B', 11), (1, 1, 12)),
            (('C', 14), (2, 1, 24)),
            (('D', 1), (3, 0, 4)),
            (('F', 17), (5, 2, 4)),
            (('G', 15), (6, 1, 28)),
            (('L', 18), (0, 2, 8)),
        ];
        for (p_n, idx) in test_cases {
            let (p, n) = p_n;
            assert_eq!(port_cfg_index(p, n), idx);
        }
    }
}
