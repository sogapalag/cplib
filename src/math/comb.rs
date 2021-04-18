#![allow(non_camel_case_types)]
use crate::core::modular::{Mint, Mod};

type m32<P> = Mint<i32, P>;
type V<P> = Vec<m32<P>>;

/// Combinatorics table related to binomial.
///
/// Build *O*(*n*), each query *O*(1).
pub struct Comb<T> {
    n: usize,
    fac: Vec<T>,
    ifac: Vec<T>,
    inv: Vec<T>,
}

impl<P> Comb<m32<P>>
where
    i32: Mod<P>,
    P: Copy,
{
    /// Create table `..=n`, in *O*(*n*).
    pub fn new(n: usize) -> Self {
        let n = std::cmp::max(2, n + 1);
        let [fac, ifac, inv] = Self::alloc(n);
        Self { n, fac, ifac, inv }
    }

    /// `A(n,k) := n!/(n-k)!`.
    pub fn a(&self, n: usize, k: usize) -> m32<P> {
        if n < k {
            m32::new(0)
        } else {
            self.fac[n] * self.ifac[n - k]
        }
    }
    /// Choose `C(n,k) := n!/[k!(n-k)!]`.
    pub fn c(&self, n: usize, k: usize) -> m32<P> {
        if n < k {
            m32::new(0)
        } else {
            self.fac[n] * self.ifac[n - k] * self.ifac[k]
        }
    }
    pub fn multi_nomial(&self, ks: &[usize]) -> m32<P> {
        let (n, x) = ks
            .iter()
            .map(|&k| (k, self.ifac[k]))
            .fold((0, m32::new(1)), |(n, x), (k, y)| (n + k, x * y));
        self.fac[n] * x
    }
    /// Ways of `k`-len `(x,..)` s.t. `sum x = s` and `x > 0`.
    pub fn p(&self, s: usize, k: usize) -> m32<P> {
        if s == 0 || k == 0 {
            m32::new(0)
        } else {
            self.c(s - 1, k - 1)
        }
    }
    /// Ways of `k`-len `(x,..)` s.t. `sum x = s`, and `x >= 0`.
    pub fn z(&self, s: usize, k: usize) -> m32<P> {
        self.p(s + k, k)
    }
    /// Ways of walk of `n` X-s and `k` Y-s, s.t. exist `Y-X >= m`.
    pub fn hit(&self, n: usize, k: usize, m: usize) -> m32<P> {
        if m + n <= k {
            self.c(n + k, n)
        } else {
            self.c(n + k, n + m)
        }
    }
    /// Ways of walk of `n` X-s and `k` Y-s, s.t. `Y-X < m` at any time.
    /// When `m=1`, indicates catalan triangle.
    /// See [catalan trapezoids](https://en.wikipedia.org/wiki/Catalan%27s_triangle#Generalization)
    pub fn cat(&self, n: usize, k: usize, m: usize) -> m32<P> {
        self.c(n + k, n) - self.hit(n, k, m)
    }
    /// `i!`.
    #[inline]
    pub fn fac(&self, i: usize) -> m32<P> {
        self.fac[i]
    }
    /// `1/i!`.
    #[inline]
    pub fn ifac(&self, i: usize) -> m32<P> {
        self.ifac[i]
    }
    /// `1/i`.
    #[inline]
    pub fn inv(&self, i: usize) -> m32<P> {
        assert!(i > 0);
        self.inv[i]
    }
    fn alloc(n: usize) -> [V<P>; 3] {
        let mut fac = Vec::with_capacity(n);
        let mut ifac = Vec::with_capacity(n);
        let mut inv = Vec::with_capacity(n);
        let one = m32::new(1);
        fac.push(one);
        fac.push(one);
        ifac.push(one);
        ifac.push(one);
        inv.push(one); // unused invalid inv[0]
        inv.push(one);
        for i in 2..n {
            fac.push(fac[i - 1] * m32::new(i as i32));
            inv.push(m32::new(-i32::MOD / i as i32) * inv[i32::MOD as usize % i]);
            ifac.push(ifac[i - 1] * inv[i]);
        }
        [fac, ifac, inv]
    }
}

#[cfg(test)]
mod tests;
