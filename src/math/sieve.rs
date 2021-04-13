#[derive(Debug, PartialEq, Eq)]
pub struct Sieve {
    primes: Vec<usize>,
    is: Vec<bool>,
}

impl Sieve {
    /// *O*(*n* log log sqrt(*n*)).
    pub fn new(n: usize) -> Self {
        sieve(n)
    }
    /// *O*(*n*). Do NOT use, even slower.
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
    /// *O*(*n*).
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
    /// *O*(*n*).
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
}

#[cfg(test)]
mod tests;
