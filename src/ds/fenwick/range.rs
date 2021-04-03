use super::Fenwick;
use crate::algebra::{Group, Power};
use std::ops::Range;

/// For group add only.
///
/// By the fact `a[i] = sum diff[0..=i]`, where `diff[i] = a[i] - a[i-1]`, `a[-1] = 0` implicitly.
#[derive(Clone, Debug)]
pub struct RangeAddPointGet<T, M> {
    diff: Fenwick<T, M>,
}

impl<T, M> RangeAddPointGet<T, M>
where
    T: Group<M> + Copy,
{
    pub fn new(n: usize) -> Self {
        Self {
            diff: Fenwick::new(n + 1),
        }
    }
    /// Add `x` to each element of `a[l..r]`.
    pub fn add(&mut self, r: Range<usize>, x: T) {
        let Range { start: l, end: r } = r;
        self.diff.add(l, x);
        self.diff.add(r, T::inv(x));
    }
    /// `a[i]`.
    pub fn get(&self, i: usize) -> T {
        self.diff.prefix(i + 1)
    }
}

/// For group add with fast power only.
///
/// By the fact `lack = diff (i*x)`.
#[derive(Clone, Debug)]
pub struct RangeAddRangeSum<T, M> {
    a: RangeAddPointGet<T, M>,
    lack: Fenwick<T, M>,
}

impl<T, M> RangeAddRangeSum<T, M>
where
    T: Group<M> + Copy + Power<M>,
{
    pub fn new(n: usize) -> Self {
        Self {
            a: RangeAddPointGet::new(n),
            lack: Fenwick::new(n + 1),
        }
    }
    /// Add `x` to each element of `a[l..r]`.
    pub fn add(&mut self, r: Range<usize>, x: T) {
        self.a.add(r.clone(), x);
        let Range { start: l, end: r } = r;
        self.lack.add(l, T::pow(x, l));
        self.lack.add(r, T::inv(T::pow(x, r)));
    }
    /// Sum of `a[0..i]`.
    pub fn prefix(&self, i: usize) -> T {
        if i > 0 {
            T::binop(T::inv(self.lack.prefix(i)), T::pow(self.a.get(i - 1), i))
        } else {
            T::ID
        }
    }
    /// Sum of `a[l..r]`.
    pub fn sum(&self, r: Range<usize>) -> T {
        let Range { start: l, end: r } = r;
        T::binop(T::inv(self.prefix(l)), self.prefix(r))
    }
}
#[cfg(test)]
mod tests;
