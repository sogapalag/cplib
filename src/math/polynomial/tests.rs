use super::*;
use crate::define_mint;

#[test]
fn ntt_basic() {
    define_mint!(m32, 998244353, PrimeMod);

    let mut a: Vec<_> = [2, 3, 7].iter().map(|&x| m32::from(x)).collect();
    let mut b: Vec<_> = [1, 11, 5].iter().map(|&x| m32::from(x)).collect();
    let ntt = Ntt::new(8);
    a.resize(8, m32::from(0));
    b.resize(8, m32::from(0));
    ntt.process(&mut a, Direction::Forward);
    ntt.process(&mut b, Direction::Forward);
    a.iter_mut().zip(b.iter()).for_each(|(x, &y)| *x *= y);
    ntt.process(&mut a, Direction::Inverse);
    // (2, 3, 7) x (1, 11, 5) = (2, 3+22=25, 2*5+3*11+7*1=50, 3*5+7*11=92, 7*5=35)
    let c: Vec<_> = [2, 25, 50, 92, 35, 0, 0, 0]
        .iter()
        .map(|&x| m32::from(x))
        .collect();
    assert!(a.iter().eq(c.iter()));
}

use crate::math::subset;
#[test]
fn xor_zeta_consistent() {
    let mut a: Vec<f64> = vec![3.9, 1.09, 32.2, -2.34, 1232.207, 0.0, 1.1, -1.0];
    let mut b = a.clone();

    fwht::or(&mut a, Direction::Forward);
    subset::zeta(&mut b);
    assert_eq!(a, b);

    fwht::or(&mut a, Direction::Inverse);
    subset::mobius(&mut b);
    assert_eq!(a, b);
}

#[test]
fn and_zeta_p_consistent() {
    let mut a: Vec<f64> = vec![3.9, 1.09, 32.2, -2.34, 1232.207, 0.0, 1.1, -1.0];
    let mut b = a.clone();

    fwht::and(&mut a, Direction::Forward);
    subset::zeta_p(&mut b);
    assert_eq!(a, b);

    fwht::and(&mut a, Direction::Inverse);
    subset::mobius_p(&mut b);
    assert_eq!(a, b);
}
