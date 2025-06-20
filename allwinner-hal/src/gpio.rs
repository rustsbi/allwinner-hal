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

#[inline]
const fn cfg_index(n: u8) -> (usize, u8) {
    assert!(n <= 31);
    let cfg_reg_idx = (n >> 3) as usize;
    let cfg_field_idx = (n & 0b111) << 2;
    (cfg_reg_idx, cfg_field_idx)
}

#[cfg(test)]
mod tests {
    use super::cfg_index;

    #[test]
    fn test_cfg_index() {
        let test_cases = [
            (0, (0, 0)),
            (5, (0, 20)),
            (7, (0, 28)),
            (8, (1, 0)),
            (24, (3, 0)),
            (31, (3, 28)),
            (11, (1, 12)),
            (14, (1, 24)),
            (1, (0, 4)),
            (17, (2, 4)),
            (15, (1, 28)),
            (18, (2, 8)),
        ];
        for (n, idx) in test_cases {
            assert_eq!(cfg_index(n), idx);
        }
    }
}
