use crate::core::num::Num;
/// Combinatorics table related to binomial.
///
/// Build *O*(*n*), each query *O*(1).
pub struct Comb<T> {
    // Vec len
    n: usize,
    /// `i!`.
    pub fac: Vec<T>,
    /// `1/i!`.
    pub ifac: Vec<T>,
    /// `1/i`.
    pub inv: Vec<T>,
}

impl<T> Comb<T>
where
    T: Num + Copy,
{
    /// Create table `..=n`, in *O*(*n*).
    pub fn new(n: usize) -> Self
    where
        T: From<i32>,
    {
        let n = std::cmp::max(2, n + 1);
        let (fac, ifac, inv) = Self::alloc(n);
        Self { n, fac, ifac, inv }
    }

    /// `A(n,k) := n!/(n-k)!`.
    pub fn a(&self, n: usize, k: usize) -> T {
        if n < k {
            T::ZERO
        } else {
            self.fac[n] * self.ifac[n - k]
        }
    }
    /// Binomial/Choose `C(n,k) := n!/[k!(n-k)!]`.
    pub fn c(&self, n: usize, k: usize) -> T {
        if n < k {
            T::ZERO
        } else {
            self.fac[n] * self.ifac[n - k] * self.ifac[k]
        }
    }
    pub fn multi_nomial(&self, ks: &[usize]) -> T {
        let (n, x) = ks
            .iter()
            .map(|&k| (k, self.ifac[k]))
            .fold((0, T::ONE), |(n, x), (k, y)| (n + k, x * y));
        self.fac[n] * x
    }
    /// Ways of `k`-len `(x,..)` s.t. `sum x = s` and `x > 0`.
    pub fn p(&self, s: usize, k: usize) -> T {
        if s == 0 || k == 0 {
            T::ZERO
        } else {
            self.c(s - 1, k - 1)
        }
    }
    /// Ways of `k`-len `(x,..)` s.t. `sum x = s`, and `x >= 0`.
    pub fn z(&self, s: usize, k: usize) -> T {
        self.p(s + k, k)
    }
    /// Ways of walk of `n` X-s and `k` Y-s, s.t. exist `Y-X >= m`.
    pub fn hit(&self, n: usize, k: usize, m: usize) -> T {
        if m + n <= k {
            self.c(n + k, n)
        } else {
            self.c(n + k, n + m)
        }
    }
    /// Ways of walk of `n` X-s and `k` Y-s, s.t. `Y-X < m` at any time.
    /// When `m=1`, indicates catalan triangle.
    /// See [catalan trapezoids](https://en.wikipedia.org/wiki/Catalan%27s_triangle#Generalization)
    pub fn cat(&self, n: usize, k: usize, m: usize) -> T {
        self.c(n + k, n) - self.hit(n, k, m)
    }

    fn alloc(n: usize) -> (Vec<T>, Vec<T>, Vec<T>)
    where
        T: From<i32>,
    {
        let mut fac = vec![T::ONE; n];
        for i in 2..n {
            fac[i] = fac[i - 1] * T::from(i as _)
        }
        let mut ifac = vec![T::ONE; n];
        let mut inv = vec![T::ONE; n];
        ifac[n - 1] = T::ONE / fac[n - 1];
        for i in (1..n - 1).rev() {
            inv[i + 1] = fac[i] * ifac[i + 1];
            ifac[i] = ifac[i + 1] * T::from((i + 1) as _);
        }

        (fac, ifac, inv)
    }
}
