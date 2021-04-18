//! Sum on prime position, by the technique called min_25's sieve.
//!
//! # Time complexity
//!
//! *O*(*n*^{3/4} / log *n*).

use crate::core::index_map::*;
use crate::core::iter::Enhance;
use crate::core::num::number::NumAssign;
use crate::math::Sieve;
pub struct PrimeCounter<'s> {
    sieve: &'s Sieve,
}
impl<'s> PrimeCounter<'s> {
    pub fn new(sieve: &'s Sieve) -> Self {
        Self { sieve }
    }

    pub fn pi(&self, n: usize) -> usize {
        if n < self.sieve.is.len() {
            self.sieve.is[1..=n].iter().map(|&x| x as usize).sum()
        } else {
            pi(&self.sieve, n)
        }
    }
    /// Sum of primes up to n.
    pub fn sum<T>(&self, n: usize) -> T
    where
        T: From<usize> + Copy + NumAssign,
    {
        if n < self.sieve.is.len() {
            self.sieve
                .primes
                .iter()
                .filter(|&p| *p <= n)
                .map(|&p| T::from(p))
                .fold(T::ZERO, |sum, x| sum + x)
        } else {
            sum(&self.sieve, n)
        }
    }
}

fn pi(sieve: &Sieve, n: usize) -> usize {
    let Sieve { ref primes, ref is } = sieve;
    let index = SqrtIndex::new(n);
    let sqrt = index.sqrt;
    assert!(is.len() > sqrt);
    let s = match primes.binary_search(&(sqrt + 1)) {
        Ok(s) => s,
        Err(s) => s,
    };

    // init g
    let mut g = index.nth.clone();
    for x in &mut g {
        *x -= 1;
    }
    for (k, &p) in primes[0..s].iter().enumerate() {
        for (i, &v) in index.nth.iter().enumerate() {
            if p > v / p {
                break;
            }
            g[i] -= g[index.id(v / p)] - k;
        }
    }
    g[0]
}

fn sum<T>(sieve: &Sieve, n: usize) -> T
where
    T: From<usize> + Copy + NumAssign,
{
    let Sieve { ref primes, ref is } = sieve;
    let index = SqrtIndex::new(n);
    let sqrt = index.sqrt;
    assert!(is.len() > sqrt);
    let s = match primes.binary_search(&(sqrt + 1)) {
        Ok(s) => s,
        Err(s) => s,
    };

    let f = |n: usize| T::from(n);
    // sum f
    let sum = |n: usize| {
        let n = T::from(n);
        n * (n + T::ONE) / T::from(2usize) - T::ONE
    };
    let buf: Vec<T> = primes[0..s]
        .iter()
        .map(|&p| f(p))
        .accum(T::ZERO, |sum, x| sum + x)
        .collect();
    //let buf = primes[0..s]
    //    .iter()
    //    .map(|&p| f(p))
    //    .scan(T::ZERO, |sum, x| {
    //        *sum += x;
    //        Some(*sum)
    //    })
    //    .collect::<Vec<T>>();
    let mut g = index.nth.iter().map(|&p| sum(p)).collect::<Vec<T>>();

    for (k, &p) in primes[0..s].iter().enumerate() {
        for (i, &v) in index.nth.iter().enumerate() {
            if p > v / p {
                break;
            }
            let delta = g[index.id(v / p)] - buf[k]; //if k > 0 { buf[k - 1] } else { T::ZERO };
            g[i] -= f(p) * delta;
        }
    }
    g[0]
}
