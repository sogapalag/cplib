//! Fast zeta, mobius transform on divide poset/lattice.
//!
//! Inplace space transforms. `d|n`.

use std::ops::{AddAssign, SubAssign};
macro_rules! for_up {
    ($d:ident, $n:ident in $f:expr, $b:expr) => {
        let m = $f.len();
        for $d in 1..m {
            for $n in ($d..m).step_by($d) {
                $b
            }
        }
    };
}
macro_rules! for_down {
    ($d:ident, $n:ident in $f:expr, $b:expr) => {
        let m = $f.len();
        for $d in (1..m).rev() {
            for $n in ($d..m).step_by($d) {
                $b
            }
        }
    };
}

/// `F=Z(f)`, `F[n] = sum f[d]`.
pub fn zeta<T: AddAssign + Copy>(f: &mut [T]) {
    for_down!(d, n in f, f[n] += f[d]);
}
/// `F=M(f)`, `F[n] = sum mu(n/d) f[d]`.
pub fn mobius<T: SubAssign + Copy>(f: &mut [T]) {
    for_up!(d, n in f, f[n] -= f[d]);
}
/// `F=Z'(f)`, `F[d] = sum f[n]`, superset zeta.
pub fn zeta_p<T: AddAssign + Copy>(f: &mut [T]) {
    for_up!(d, n in f, f[d] += f[n]);
}
/// `F=M'(f)`, `F[d] = sum mu(n/d) f[n]`, superset mobius.
pub fn mobius_p<T: SubAssign + Copy>(f: &mut [T]) {
    for_down!(d, n in f, f[d] -= f[n]);
}
