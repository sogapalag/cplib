//! Query prefix sum of multiplicative functions.
//!
//! By prepare a table to boost query of larger number around ~10^9.
//!
//! # Time comlexity
//!
//! *O*(*n*^2/3) for each query, better prepare table with *n*^2/3, too.
use std::{cell::RefCell, collections::HashMap};

use super::div_block;
/// For prefix sum of mobius function.
pub struct PrefixMu<'mu> {
    sum: &'mu [i32],
    // Memory of larger number while recursive.
    buf: RefCell<HashMap<usize, isize>>,
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
    pub fn prefix(&self, n: usize) -> isize {
        if n < self.sum.len() {
            return self.sum[n] as isize;
        }
        if self.buf.borrow().contains_key(&n) {
            self.buf.borrow()[&n]
        } else {
            let mut res = 1;
            for (v, k) in div_block(n, 2, n + 1) {
                res -= self.prefix(v) * k as isize;
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
    pub fn prefix(&self, n: usize) -> usize {
        let mut res = 0;
        let mut l = 1;
        for (v, k) in div_block(n, l, n + 1) {
            let dmu = self.pm.prefix(l + k - 1) - self.pm.prefix(l - 1);
            let v = v as isize;
            res += dmu * v * v;
            l += k;
        }
        (res as usize + 1) / 2
    }
}

#[cfg(test)]
mod tests;
