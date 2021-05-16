use super::Direction;
use crate::core::modular::*;
/// Number theoretic transform.
pub struct Ntt<P> {
    n: usize,
    twiddles: Box<[M32<P>]>,
    bit_rev: Box<[usize]>,
}

impl<P: Copy> Ntt<P>
where
    i32: Mod<P>,
{
    /// Create handle for exact sized `n=2^k`.
    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());
        if n == 1 {
            return Self {
                n,
                twiddles: Box::new([M32::new(0)]),
                bit_rev: Box::new([0]),
            };
        }
        assert!((M32::modu() - 1) as usize % n == 0, "p != cn+1");

        let mut b = vec![0; n];
        let h = n.trailing_zeros() - 1;
        for i in 1..n {
            b[i] = b[i >> 1] >> 1 | ((i & 1) << h);
        }

        let mut w = vec![M32::new(0); n];
        let g = generator();

        let mut m = 1;
        while m < n {
            // omega^{2m} = g^{p-1} = 1
            let omega = g.pow((M32::modu() - 1) as usize / (2 * m));
            let mut x = M32::new(1);
            for i in 0..m {
                w[m + i] = x;
                x *= omega;
            }
            m <<= 1;
        }
        Self {
            n,
            twiddles: w.into_boxed_slice(),
            bit_rev: b.into_boxed_slice(),
        }
    }

    pub fn process(&self, a: &mut [M32<P>], direction: Direction) {
        let n = a.len();
        if n <= 1 {
            return;
        }
        assert_eq!(n, self.n);
        if let Direction::Inverse = direction {
            let inv = M32::new(n as _).inv();
            for x in a.iter_mut() {
                *x *= inv;
            }
            a[1..n].reverse();
        }

        for i in 0..n {
            let j = self.bit_rev[i];
            if i < j {
                a.swap(i, j);
            }
        }

        let mut m = 1;
        while m < n {
            for i in (0..n).step_by(m << 1) {
                for j in 0..m {
                    let x = a[i + j];
                    let y = self.twiddles[m + j] * a[i + j + m];
                    a[i + j] = x + y;
                    a[i + j + m] = x - y;
                }
            }
            m <<= 1;
        }
    }
}

// TODO: impl general P, other than 998244353.
// https://cp-algorithms.com/algebra/primitive-root.html
fn generator<P: Copy>() -> M32<P>
where
    i32: Mod<P>,
{
    M32::new(3)
}
