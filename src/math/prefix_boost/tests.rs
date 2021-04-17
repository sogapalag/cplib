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
