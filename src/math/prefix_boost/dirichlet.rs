//! By apply easily calculated dirichlet convolution, called du's sieve in chinese.
//!
//! # Time comlexity
//!
//! *O*(*n*^2/3) for each query, better prepare table with *n*^2/3, too.
use std::{cell::RefCell, collections::HashMap};

use crate::core::div_block;
use crate::core::num::number::NumAssign;

/// For prefix sum of mobius function.
pub struct PrefixMu<'mu> {
    sum: &'mu [i32],
    // Memory of larger number while recursive.
    // Return i64, since sum(10^10) would overflow i32
    buf: RefCell<HashMap<usize, i64>>,
}

impl<'mu> PrefixMu<'mu> {
    /// Pass the table `sum[n] = mu[1..=n]`.
    pub fn new(sum: &'mu [i32]) -> Self {
        Self {
            sum,
            buf: Default::default(),
        }
    }
    /// Sum of `mu[1..=n]`.
    pub fn prefix(&self, n: usize) -> i64 {
        if n < self.sum.len() {
            return self.sum[n] as i64;
        }
        if self.buf.borrow().contains_key(&n) {
            self.buf.borrow()[&n]
        } else {
            let mut res = 1;
            for (v, k) in div_block(n, 2, n + 1) {
                res -= self.prefix(v) * k as i64;
            }
            self.buf.borrow_mut().insert(n, res);
            res
        }
    }
}

/// For prefix sum of euler function.
///
/// A implementation depents on `PrefixMu`.
pub struct PrefixPhi<'a> {
    pm: &'a PrefixMu<'a>,
}

impl<'a> PrefixPhi<'a> {
    /// Use `PrefixMu` as engine.
    pub fn new(pm: &'a PrefixMu<'a>) -> Self {
        Self { pm }
    }
    /// Sum of `phi[1..=n]`.
    // T = i64 ? m32.
    pub fn prefix<T>(&self, n: usize) -> T
    where
        T: Copy + NumAssign + From<i64>,
    {
        let mut res = T::ZERO;
        let mut l = 1;
        for (v, k) in div_block(n, l, n + 1) {
            let dmu = T::from(self.pm.prefix(l + k - 1) - self.pm.prefix(l - 1));
            let v = T::from(v as i64);
            res += dmu * v * v;
            l += k;
        }
        (res + T::ONE) / T::from(2i64)
    }
}
