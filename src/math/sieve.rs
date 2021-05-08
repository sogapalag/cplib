/// Exact sized sieve.
#[derive(Debug, PartialEq, Eq)]
pub struct Sieve {
    pub primes: Vec<usize>,
    pub is: Vec<bool>,
}

impl Sieve {
    /// Time: *O*(*n* log log sqrt(*n*)).
    pub fn new(n: usize) -> Self {
        sieve(n)
    }
    /// Time: *O*(*n*). Do NOT use, even slower.
    pub fn linear(n: usize) -> Self {
        linear_sieve(n)
    }
}

// Well optimized.
fn sieve(n: usize) -> Sieve {
    let mut is = vec![true; n + 1];
    is[0] = false;
    is[1] = false;
    // Take care prime 2.
    for i in (4..=n).step_by(2) {
        is[i] = false;
    }
    // Take care odd primes.
    for p in (3..=n).step_by(2) {
        if p > n / p {
            break;
        }
        if is[p] {
            for i in (p * p..=n).step_by(2 * p) {
                is[i] = false;
            }
        }
    }
    let mut primes = vec![];
    for p in 2..=n {
        if is[p] {
            primes.push(p);
        }
    }
    Sieve { primes, is }
}

// Each composite sieved once by its least prime factor.
// Slow in practice, primes iter costs.
fn linear_sieve(n: usize) -> Sieve {
    let mut primes = vec![];
    let mut is = vec![true; n + 1];
    is[0] = false;
    is[1] = false;
    for i in 2..=n {
        if is[i] {
            primes.push(i);
        }
        for &p in &primes {
            if i > n / p {
                break;
            }
            is[i * p] = false;
            if i % p == 0 {
                break;
            }
        }
    }
    Sieve { primes, is }
}

impl Sieve {
    /// Time: *O*(*n*).
    pub fn phi_table(&self) -> Vec<usize> {
        let n = self.is.len() - 1;
        let mut phi = vec![0; n + 1];
        phi[1] = 1;
        for i in 2..=n {
            if self.is[i] {
                phi[i] = i - 1;
            }
            for &p in &self.primes {
                if p > n / i {
                    break;
                }
                if i % p == 0 {
                    phi[i * p] = phi[i] * p;
                    break;
                }
                phi[i * p] = phi[i] * (p - 1);
            }
        }
        phi
    }
    /// Time: *O*(*n*).
    pub fn mu_table(&self) -> Vec<i32> {
        let n = self.is.len() - 1;
        let mut mu = vec![0; n + 1];
        mu[1] = 1;
        for i in 2..=n {
            if self.is[i] {
                mu[i] = -1;
            }
            for &p in &self.primes {
                if p > n / i {
                    break;
                }
                if i % p == 0 {
                    mu[i * p] = 0;
                    break;
                }
                mu[i * p] = -mu[i];
            }
        }
        mu
    }
    // although actually need contain all p <= sqrt(n), but check is.len() instead.
    fn ensure_valid(&self, n: usize) -> bool {
        let x = self.is.len() - 1;
        return x >= n / x;
    }
    /// Warning: not guarantee correct if not include all primes <=sqrt(n).
    pub fn factor(&self, n: usize) -> Factor<'_> {
        debug_assert!(n > 0);
        debug_assert!(self.ensure_valid(n));
        Factor {
            n,
            primes: &self.primes,
            i: 0,
        }
    }
    pub fn count_divisors(&self, n: usize) -> u32 {
        self.factor(n).fold(1, |prod, (_, e)| prod * (1 + e))
    }
    /// unsorted.
    pub fn divisors(&self, n: usize) -> Vec<usize> {
        let mut res = vec![1];
        for (p, mut e) in self.factor(n) {
            let n = res.len();
            let mut k = 1;
            loop {
                k *= p;
                for i in 0..n {
                    res.push(res[i] * k);
                }
                e -= 1;
                if e == 0 {
                    break;
                }
            }
        }
        res
    }
    pub fn prime_factor(&self, n: usize) -> PrimeFactor<'_> {
        PrimeFactor {
            factor: self.factor(n),
        }
    }
    pub fn is_prime(&self, n: usize) -> bool {
        if n < self.is.len() {
            self.is[n]
        } else {
            self.prime_factor(n).take(1).next().unwrap() == n
        }
    }
    pub fn phi(&self, n: usize) -> usize {
        self.prime_factor(n).fold(n, |phi, p| phi / p * (p - 1))
    }
    pub fn mu(&self, n: usize) -> i32 {
        let mut res = 1;
        for (_, e) in self.factor(n) {
            if e > 1 {
                return 0;
            }
            res *= -1;
        }
        res
    }
    /// count `1..=m` coprime to n.
    /// Note first 8 primes `2*3*...*19 = 9_699_690`. As long as n can be factored validly, m can be arbitrarily large.
    ///
    /// Time: ~8*2^8 = 2048.
    ///
    /// Hint: If many queries, prep mu table, then use formula:
    /// `coprime(m,n) = sum_{d|n} mu[d] * m/d`.
    pub fn count_coprime(&self, m: usize, n: usize) -> usize {
        let ps: Vec<_> = self.prime_factor(n).collect();
        let m = m as _;
        let MSK: u32 = 1 << ps.len();
        // IEP
        let mut res: i64 = m;
        for msk in 1..MSK {
            let mut prod = 1;
            for (i, &p) in ps.iter().enumerate() {
                if msk >> i & 1 == 1 {
                    prod *= p as i64;
                }
            }
            let sign: i64 = if msk.count_ones() % 2 == 0 { 1 } else { -1 };
            res += sign * m / prod;
        }
        res as _
    }
}

pub struct PrimeFactor<'p> {
    factor: Factor<'p>,
}
impl Iterator for PrimeFactor<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.factor.next().map(|(p, _)| p)
    }
}
pub struct Factor<'p> {
    n: usize,
    primes: &'p [usize],
    i: usize,
}
impl Iterator for Factor<'_> {
    type Item = (usize, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 1 {
            return None;
        }
        while self.i < self.primes.len() {
            let p = self.primes[self.i];
            if p > self.n / p {
                let res = Some((self.n, 1));
                self.n = 1;
                return res;
            }
            self.i += 1;
            if self.n % p == 0 {
                let mut e = 0;
                while self.n % p == 0 {
                    self.n /= p;
                    e += 1;
                }
                return Some((p, e));
            }
        }
        None
    }
}
#[cfg(test)]
mod tests;
