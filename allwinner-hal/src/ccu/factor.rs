//! Divide factors.

/// Peripheral clock divide factor N.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PeriFactorN {
    /// Don't divide.
    N1 = 0,
    /// Divide frequency by 2.
    N2 = 1,
    /// Divide frequency by 4.
    N4 = 2,
    /// Divide frequency by 8.
    N8 = 3,
}

/// CPU and RISC-V coprocessor AXI clock divide factor N.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AxiFactorN {
    /// Divide frequency by 2.
    N2 = 1,
    /// Divide frequency by 3.
    N3 = 2,
    /// Divide frequency by 4.
    N4 = 3,
}

/// Clock divide factor P.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FactorP {
    /// Don't divide.
    P1,
    /// Divide frequency by 2.
    P2,
    /// Divide frequency by 4.
    P4,
}

/// Calculate the best N-M divide factors from `f_src` and `f_dst` parameters.
#[inline]
pub fn calculate_best_peripheral_factors_nm(f_src: u32, f_dst: u32) -> (PeriFactorN, u8) {
    let mut err = f_src;
    let (mut best_n, mut best_m) = (0, 0);
    for m in 1u8..=16 {
        for n in [1, 2, 4, 8] {
            let actual = f_src / n / m as u32;
            if actual.abs_diff(f_dst) < err {
                err = actual.abs_diff(f_dst);
                (best_n, best_m) = (n, m);
            }
        }
    }
    let factor_n = match best_n {
        1 => PeriFactorN::N1,
        2 => PeriFactorN::N2,
        4 => PeriFactorN::N4,
        8 => PeriFactorN::N8,
        _ => unreachable!(),
    };
    let factor_m = best_m - 1;
    (factor_n, factor_m)
}

// TODO: test module
