use crate::core::{iter::Enhance, num::Num};

type V2<T> = Vec<Vec<T>>;

/// (n k): n choose k.
pub fn binomial<T>(n: usize) -> V2<T>
where
    T: Num + Copy,
{
    let mut res = vec![vec![T::ZERO; n + 1]; n + 1];
    res[0][0] = T::ONE;
    for i in 1..=n {
        res[i][0] = T::ONE;
        for j in 1..=i {
            // last choose or not, i.e. take j-1 or j in first i-1.
            res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
        }
    }
    res
}

/// [n k]: n into k cycles.
pub fn stirling_first<T>(n: usize) -> V2<T>
where
    T: Num + Copy + From<i32>,
{
    let mut res = vec![vec![T::ZERO; n + 1]; n + 1];
    res[0][0] = T::ONE;
    for i in 1..=n {
        for j in 1..=i {
            // last self cycle or join other, if join, total i-1 positions choice.
            res[i][j] = res[i - 1][j - 1] + res[i - 1][j] * T::from((i - 1) as _);
        }
    }
    res
}

/// {n k}: n into k sets.
pub fn stirling_second<T>(n: usize) -> V2<T>
where
    T: Num + Copy + From<i32>,
{
    let mut res = vec![vec![T::ZERO; n + 1]; n + 1];
    res[0][0] = T::ONE;
    for i in 1..=n {
        for j in 1..=i {
            // last self set or join other, if join, j sets choice.
            res[i][j] = res[i - 1][j - 1] + res[i - 1][j] * T::from(j as _);
        }
    }
    res
}

pub fn power<T>(x: T, n: usize) -> Vec<T>
where
    T: Num + Copy,
{
    (0..n).accum(T::ONE, |s, _| s * x).collect()
}

pub fn rising<T>(x: T, n: usize) -> Vec<T>
where
    T: Num + Copy + From<i32>,
{
    (0..n as _)
        .accum(T::ONE, |s, i| s * (x + T::from(i)))
        .collect()
}
pub fn falling<T>(x: T, n: usize) -> Vec<T>
where
    T: Num + Copy + From<i32>,
{
    (0..n as _)
        .accum(T::ONE, |s, i| s * (x - T::from(i)))
        .collect()
}
