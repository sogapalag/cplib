use super::*;
use crate::math::Sieve;

// verified by https://www.luogu.com.cn/problem/P4213
#[test]
fn prefix_mu() {
    let n = 10_000;
    let mut mu = Sieve::new(n).mu_table();
    for i in 1..=n {
        mu[i] += mu[i - 1];
    }

    let m = 1000;
    let pm = PrefixMu::new(&mu[..m]);

    for i in m..=n {
        assert_eq!(mu[i] as i64, pm.prefix(i));
    }
}

#[test]
fn prefix_phi() {
    let n = 10_000;
    let s = Sieve::new(n);
    let mut mu = s.mu_table();
    let mut phi = s.phi_table();
    for i in 1..=n {
        mu[i] += mu[i - 1];
        phi[i] += phi[i - 1];
    }
    let m = 1000;
    let pm = PrefixMu::new(&mu[..m]);

    let pp = PrefixPhi::new(&pm);

    for i in m..=n {
        assert_eq!(phi[i], pp.prefix::<i64>(i) as _);
    }
}

// verified by https://judge.yosupo.jp/submission/45208
#[test]
fn pi() {
    let n = 100_000;
    let s = Sieve::new(n);
    let t = Sieve::new(1000);
    let pi_counter = PrimeCounter::new(&t);
    let m = 1387;
    assert_eq!(
        s.is[1..=m].iter().map(|&x| x as usize).sum::<usize>(),
        pi_counter.pi(m)
    );
}

#[test]
fn sum_prime() {
    let n = 100_000;
    let s = Sieve::new(n);
    let t = Sieve::new(1000);
    let pc = PrimeCounter::new(&t);
    let m = 1387;
    assert_eq!(
        s.primes
            .iter()
            .take_while(|&p| *p <= m)
            .map(|&x| x as usize)
            .sum::<usize>(),
        pc.sum::<usize>(m)
    );
}
