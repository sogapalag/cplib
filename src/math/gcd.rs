//! Gcd, exgcd, crt and excrt.

use crate::core::num::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exgcd<T> {
    pub gcd: T,
    pub x: T,
    pub y: T,
}
pub trait Gcd: Num {
    fn gcd(self, rhs: Self) -> Self;
    fn exgcd(self, rhs: Self) -> Exgcd<Self>
    where
        Self: Signed;

    fn lcm(self, rhs: Self) -> Self {
        self / self.gcd(rhs) * rhs
    }
    fn gcd_lcm(self, rhs: Self) -> (Self, Self) {
        let g = self.gcd(rhs);
        (g, self / g * rhs)
    }
}

macro_rules! impl_gcd_signed {
    ($($t:ty)*) => {$(
        impl Gcd for $t {
            // Stein's algorithm, iterative shift method.
            #[inline]
            fn gcd(self, rhs: Self) -> Self {
                let mut a = self;
                let mut b = rhs;
                if a == 0 || b == 0 {
                    return (a | b).abs();
                }
                // Panic -MIN
                a = a.abs();
                b = b.abs();
                let shift = (a | b).trailing_zeros();
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                while a != b {
                    if a > b {
                        a -= b;
                        a >>= a.trailing_zeros();
                    } else {
                        b -= a;
                        b >>= b.trailing_zeros();
                    }
                }
                a << shift
            }
            #[inline]
            fn exgcd(self, rhs: Self) -> Exgcd<Self> {
                let mut x = (1, 0);
                let mut y = (0, 1);
                let mut c = (self, rhs);
                while c.1 != 0 {
                    let q = c.0 / c.1;
                    let f = |p: (Self, Self)| {
                        (p.1, p.0 - q * p.1)
                    };
                    x = f(x);
                    y = f(y);
                    c = f(c);
                }
                if c.0 > 0 {
                    Exgcd {
                        gcd: c.0,
                        x: x.0,
                        y: y.0,
                    }
                } else {
                    Exgcd {
                        gcd: -c.0,
                        x: -x.0,
                        y: -y.0,
                    }
                }
            }
        }
    )*};
}
impl_gcd_signed!(i32 i64 isize);

macro_rules! impl_gcd_unsigned {
    ($($t:ty)*) => {$(
        impl Gcd for $t {
            // Stein's algorithm, iterative shift method.
            #[inline]
            fn gcd(self, rhs: Self) -> Self {
                let mut a = self;
                let mut b = rhs;
                if a == 0 || b == 0 {
                    return a | b;
                }
                let shift = (a | b).trailing_zeros();
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                while a != b {
                    if a > b {
                        a -= b;
                        a >>= a.trailing_zeros();
                    } else {
                        b -= a;
                        b >>= b.trailing_zeros();
                    }
                }
                a << shift
            }
            fn exgcd(self, _rhs: Self) -> Exgcd<Self> {
                unimplemented!()
            }
        }
    )*};
}
impl_modulo_unsigned!(u32 u64 usize);

pub fn gcd<T: Gcd>(a: T, b: T) -> T {
    a.gcd(b)
}
pub fn exgcd<T: Gcd + Signed>(a: T, b: T) -> Exgcd<T> {
    a.exgcd(b)
}
pub fn lcm<T: Gcd>(a: T, b: T) -> T {
    a.lcm(b)
}
pub fn gcd_lcm<T: Gcd>(a: T, b: T) -> (T, T) {
    a.gcd_lcm(b)
}
type Ap = (i64, i64); // (a_i, p_i)
/// Only accept coprime modulos.
pub fn crt(s: Ap, t: Ap) -> i64 {
    let (a1, p1) = s;
    let (a2, p2) = t;
    let Exgcd { gcd, x, y } = exgcd(p1, p1);
    assert_eq!(gcd, 1);
    let m = p1 * p2;
    let mut res = ((a1 * y) % m) * p2 % m + ((a2 * x) % m) * p1 % m;
    res %= m;
    if res < 0 {
        res += m;
    }
    res
}
/// Accept non-coprime modulos, but `None` if no solution.
/// Warning: potential mul overflow!
pub fn excrt(aps: &[Ap]) -> Option<Ap> {
    aps.iter().try_fold((0, 1), |(mut r, mut m), &(a, p)| {
        let Exgcd { gcd, x, .. } = exgcd(m, p);
        if (a - r) % gcd != 0 {
            return None;
        }
        let c = (a - r) / gcd * x % (p / gcd);
        r += m * c; // += m * x * da/g
        m *= p / gcd;
        r %= m;
        if r < 0 {
            r += m;
        }
        Some((r, m))
    })
}
/// Replace inner mul cast to `i128`, then cast back to `i64` after modulo.
/// Need to ensure lcm of all modulos under `i64`.
pub fn excrt_cast(aps: &[Ap]) -> Option<Ap> {
    aps.iter().try_fold((0, 1), |(mut r, mut m), &(a, p)| {
        let Exgcd { gcd, x, .. } = exgcd(m, p);
        if (a - r) % gcd != 0 {
            return None;
        }
        let c = ((a - r) / gcd) as i128 * x as i128 % (p / gcd) as i128;
        let c = c as i64;
        r += m * c; // += m * x * da/g
        m *= p / gcd;
        r %= m;
        if r < 0 {
            r += m;
        }
        Some((r, m))
    })
}
#[cfg(test)]
mod tests;
