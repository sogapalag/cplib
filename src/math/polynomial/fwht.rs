//! Fast Walsh–Hadamard transform.
use super::Direction;
use crate::core::num::number::NumAssign;

pub fn xor<T>(a: &mut [T], direction: Direction)
where
    T: NumAssign + From<i32> + Copy,
{
    let n = a.len();
    assert!(n.is_power_of_two());
    // put aside 1/sqrt{2} of hadamard away from forward, all put into inverse.
    if let Direction::Inverse = direction {
        let inv = T::ONE / T::from(n as _);
        for x in a.iter_mut() {
            *x *= inv;
        }
    }

    let mut m = 1;
    while m < n {
        for i in (0..n).step_by(m << 1) {
            for j in 0..m {
                let x = a[i + j];
                let y = a[i + j + m];
                a[i + j] = x + y;
                a[i + j + m] = x - y;
            }
        }
        m <<= 1;
    }
}

/// equiv to `zeta` / `mobius`.
pub fn or<T>(a: &mut [T], direction: Direction)
where
    T: NumAssign + Copy,
{
    let n = a.len();
    assert!(n.is_power_of_two());

    let mut m = 1;
    while m < n {
        for i in (0..n).step_by(m << 1) {
            for j in 0..m {
                let x = a[i + j];
                let y = a[i + j + m];
                match direction {
                    // H1 = ( 1 0 \\ 1 1)
                    Direction::Forward => {
                        a[i + j] = x;
                        a[i + j + m] = x + y;
                    }
                    Direction::Inverse => {
                        a[i + j] = x;
                        a[i + j + m] = y - x;
                    }
                }
            }
        }
        m <<= 1;
    }
}

/// equiv to `zeta_p` / `mobius_p`.
pub fn and<T>(a: &mut [T], direction: Direction)
where
    T: NumAssign + Copy,
{
    let n = a.len();
    assert!(n.is_power_of_two());

    let mut m = 1;
    while m < n {
        for i in (0..n).step_by(m << 1) {
            for j in 0..m {
                let x = a[i + j];
                let y = a[i + j + m];
                match direction {
                    // H1 = ( 1 1 \\ 0 1)
                    Direction::Forward => {
                        a[i + j] = x + y;
                        a[i + j + m] = y;
                    }
                    Direction::Inverse => {
                        a[i + j] = x - y;
                        a[i + j + m] = y;
                    }
                }
            }
        }
        m <<= 1;
    }
}
