use crate::core::Rng;

use super::*;
// Deprecated vanilla sieve
//for p in 2..=n {
//    if p > n / p {
//        break;
//    }
//    if is[p] {
//        for i in (p * p..=n).step_by(p) {
//            is[i] = false;
//        }
//    }
//}

// Deprecated phi, n log n
//let mut phi: Vec<usize> = (0..=n).collect();
//for &p in &self.primes {
//    for i in (p..=n).step_by(p) {
//        phi[i] /= p;
//        phi[i] *= p - 1;
//    }
//}
#[test]
fn consistent() {
    let n = 1000_000;
    let a = Sieve::new(n);
    let b = Sieve::linear(n);
    assert_eq!(a, b);
}

// n log n
fn phi_table(n: usize) -> Vec<usize> {
    let mut phi = vec![0; n + 1];
    phi[1] = 1;
    for p in 2..=n {
        if phi[p] == 0 {
            for i in (p..=n).step_by(p) {
                if phi[i] == 0 {
                    phi[i] = i;
                }
                phi[i] = phi[i] / p * (p - 1);
            }
        }
    }
    phi
}

#[test]
fn consistent_phi() {
    let n = 1000_000;
    let s = Sieve::new(n);
    let a = s.phi_table();
    let b = phi_table(n);
    assert_eq!(a, b);
}

fn mu_table(n: usize) -> Vec<i32> {
    use crate::math::divide::mobius;
    let mut b = vec![0; n + 1];
    b[1] = 1;
    mobius(&mut b);
    b
}
#[test]
fn consistent_mu() {
    let n = 1000_000;
    let s = Sieve::new(n);
    let a = s.mu_table();
    let b = mu_table(n);
    assert_eq!(a, b);
}

#[test]
fn check_prefix_mu_abs() {
    // 367
    let n = 1_000_000;
    // 1144
    //let n = 10_000_000;
    let s = Sieve::new(n);
    let a = s.mu_table();
    let mut mx = 0;
    a.iter().fold(0, |sum, x| {
        mx = std::cmp::max(mx, (sum + x).abs());
        sum + x
    });
    dbg!(mx);
}

#[test]
fn factor_related() {
    let len = 10_000;
    let s = Sieve::new(len);

    assert_eq!(s.factor(1).next(), None);

    let n = 2 * 2 * 2 * 7 * 7 * 31;
    assert!([(2, 3), (7, 2), (31, 1)]
        .iter()
        .map(|pe| *pe)
        .eq(s.factor(n)));

    assert_eq!(s.count_divisors(n), 4 * 3 * 2);

    let d = s.divisors(n);
    assert_eq!(
        d,
        [
            1, 2, 4, 8, 7, 14, 28, 56, 49, 98, 196, 392, 31, 62, 124, 248, 217, 434, 868, 1736,
            1519, 3038, 6076, 12152
        ]
    );

    let n = 9_699_690;
    assert!([2, 3, 5, 7, 11, 13, 17, 19]
        .iter()
        .map(|p| *p)
        .eq(s.prime_factor(n)));

    // consistent mu/phi
    let mu = s.mu_table();
    let phi = s.phi_table();
    for i in 1..=len {
        assert_eq!(mu[i], s.mu(i));
        assert_eq!(phi[i], s.phi(i));
        assert_eq!(phi[i], s.count_coprime(i, i));
    }
}
