use std::marker::PhantomData;
use std::ops::Range;

use crate::algebra::{Group, Monoid};

/// A generalized version Fenwick Tree, also called Binary Indexed Tree(BIT).
///
/// API is 0-based, while inner implementation is 1-based.
/// You can use it as having array `a[0..n]`.
/// Check [design pattern](https://sogapalag.github.io/2021/03/30/Algebraic-Structure-Generic-and-Design-Pattern-of-Rust/) for more detail.
#[derive(Clone, Debug)]
pub struct Fenwick<T, M> {
    n: usize,
    v: Vec<T>,
    _m: PhantomData<M>,
}

impl<T, M> Fenwick<T, M>
where
    T: Copy + Monoid<M>,
{
    /// Creates default view `a[0..n]`, filled with Monoid Identity.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            v: vec![T::ID; n + 1],
            _m: PhantomData,
        }
    }
    /// *O*(*n*) build from initials.
    pub fn from(a: &[T]) -> Self {
        let n = a.len();
        let mut v = Vec::with_capacity(n + 1);
        v.push(T::ID);
        v.extend(a);
        for i in 1..=n {
            let k = i + lsb(i);
            if k <= n {
                v[k] = T::binop(v[i], v[k]);
            }
        }
        Self {
            n,
            v,
            _m: PhantomData,
        }
    }
    /// Behave as `a[i] += x`.
    pub fn add(&mut self, i: usize, x: T) {
        let mut i = i + 1;
        while i <= self.n {
            self.v[i] = T::binop(self.v[i], x);
            i += lsb(i);
        }
    }
    ///  Sum of `a[0..i]`.
    pub fn prefix(&self, i: usize) -> T {
        let mut sum = T::ID;
        let mut i = i;
        while i > 0 {
            sum = T::binop(self.v[i], sum);
            i -= lsb(i);
        }
        sum
    }
    /// Sum of `a[l..r]`.
    pub fn sum(&self, r: Range<usize>) -> T
    where
        T: Group<M>,
    {
        let Range { start: l, end: r } = r;
        T::binop(T::inv(self.prefix(l)), self.prefix(r))
    }
    /// Warning: correct only when convincing pref sorted.
    ///
    /// `res := a[0..res] < x` still. i.e. imply `a[0..=res] >= x` or `res = n`.
    ///
    /// # Time complexity
    ///
    /// *O*(log *n*).
    pub fn binary_search(&self, x: T) -> usize
    where
        T: PartialOrd,
    {
        let mut p = 1;
        while p << 1 <= self.n {
            p <<= 1;
        }
        let mut res = 0;
        let mut sum = T::ID;
        while p > 0 {
            if res + p <= self.n && T::binop(sum, self.v[res + p]) < x {
                sum = T::binop(sum, self.v[res + p]);
                res += p;
            }
            p >>= 1;
        }
        res
    }
}
#[inline]
fn lsb(i: usize) -> usize {
    (i as isize & (-(i as isize))) as usize
}

#[cfg(test)]
mod tests;
