//! Fast zeta, mobius transform on subset poset/lattice.
//!
//! Inplace space transforms. `d sub n`.

use std::ops::{AddAssign, SubAssign};
macro_rules! for_bit {
    ($i:ident, $mask:ident in $f:expr, $b:expr) => {
        let m = $f.len();
        assert!(m.is_power_of_two());
        let n = m.trailing_zeros();
        for $i in 0..n {
            for $mask in 0..m {
                if $mask >> $i & 1 != 0 {
                    $b
                }
            }
        }
    };
}

/// `F=Z(f)`, `F[n] = sum f[d]`.
pub fn zeta<T: AddAssign + Copy>(f: &mut [T]) {
    for_bit!(i, mask in f, f[mask] += f[mask ^ 1 << i]);
}
/// `F=M(f)`, `F[n] = sum (-1)^|n\d| f[d]`.
pub fn mobius<T: SubAssign + Copy>(f: &mut [T]) {
    for_bit!(i, mask in f, f[mask] -= f[mask ^ 1 << i]);
}
/// `F=Z'(f)`, `F[d] = sum f[n]`, superset zeta.
pub fn zeta_p<T: AddAssign + Copy>(f: &mut [T]) {
    for_bit!(i, mask in f, f[mask ^ 1 << i] += f[mask]);
}
/// `F=M'(f)`, `F[d] = sum (-1)^|n\d| f[n]`, superset mobius.
pub fn mobius_p<T: SubAssign + Copy>(f: &mut [T]) {
    for_bit!(i, mask in f, f[mask ^ 1 << i] -= f[mask]);
}
