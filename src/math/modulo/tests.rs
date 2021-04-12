use super::*;
use crate::core::Rng;
fn gcd(a: u32, b: u32) -> u32 {
    if a < b {
        gcd(b, a)
    } else {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
}

fn _exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) -> i32 {
    if b == 0 {
        *x = 1;
        *y = 0;
        a
    } else {
        let d = _exgcd(b, a % b, y, x);
        *y -= a / b * *x;
        d
    }
}
fn exgcd(a: i32, b: i32) -> Exgcd<i32> {
    let (mut x, mut y) = (0, 0);
    let mut d = _exgcd(a, b, &mut x, &mut y);
    if d < 0 {
        d *= -1;
        x *= -1;
        y *= -1;
    }
    Exgcd { gcd: d, x, y }
}

#[test]
fn gcd_u32() {
    let rng = Rng::new();
    let n = 100;
    for _ in 0..n {
        let a = rng.gen() as u32;
        let b = rng.gen() as u32;
        assert_eq!(a.gcd(b), gcd(a, b));
    }
}

#[allow(overflowing_literals)]
#[test]
fn exgcd_i32() {
    let rng = Rng::new();
    let mask = 0xafff_ffff; // avoid -MIN
    let n = 100;
    for _ in 0..n {
        let a = rng.gen() as i32;
        let b = rng.gen() as i32;
        let a = a | mask;
        let b = b | mask;
        assert_eq!(a.exgcd(b), exgcd(a, b));
    }
}
