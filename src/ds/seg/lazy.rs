use crate::algebra::{Affine, Monoid};
use std::marker::PhantomData;
use std::ops::Range;

pub struct SegLazy<T, U, M, D, F> {
    N: usize,
    L: usize,
    a: Vec<T>,
    d: Vec<U>,
    _m: PhantomData<(M, D, F)>,
}
impl<T, U, M, D, F> SegLazy<T, U, M, D, F>
where
    T: Monoid<M> + Copy + Affine<U, F>,
    U: Monoid<M> + Copy + PartialEq + Eq,
{
    pub fn new(n: usize) -> Self {
        let (N, L, a, d) = Self::alloc(n);
        Self {
            N,
            L,
            a,
            d,
            _m: PhantomData,
        }
    }
    pub fn from(leaf: &[T]) -> Self {
        let (N, L, mut a, d) = Self::alloc(leaf.len());
        a[N..N + leaf.len()].copy_from_slice(leaf);
        for i in (1..N).rev() {
            a[i] = T::binop(a[i << 1], a[i << 1 | 1]);
        }
        Self {
            N,
            L,
            a,
            d,
            _m: PhantomData,
        }
    }

    pub fn update(&mut self, r: Range<usize>, u: U) {
        let Range { start: l, end: r } = r;
        self.__update(l, r, u, 1, 0, self.N);
    }
    pub fn query(&mut self, r: Range<usize>) -> T {
        let Range { start: l, end: r } = r;
        self.__query(l, r, 1, 0, self.N)
    }
    /// = `query(p..p+1)`
    pub fn get(&mut self, p: usize) -> T {
        let i = p + self.N;
        for k in (1..=self.L).rev() {
            self.push(i >> k);
        }
        self.a[i]
    }
    /// = `query(0..n)`
    pub fn all(&self) -> T {
        self.a[1]
    }
    pub fn set(&mut self, p: usize, x: T) {
        let mut i = p + self.N;
        for k in (1..=self.L).rev() {
            self.push(i >> k);
        }
        self.a[i] = x;
        loop {
            i >>= 1;
            if i == 0 {
                break;
            }
            self.pull(i);
        }
    }

    fn __update(&mut self, l: usize, r: usize, u: U, i: usize, sl: usize, sr: usize) {
        if r <= sl || sr <= l {
            return;
        }
        if l <= sl && sr <= r {
            self.apply(i, u);
            return;
        }
        let sm = (sl + sr) >> 1;
        let il = i << 1;
        let ir = i << 1 | 1;
        self.push(i);
        self.__update(l, r, u, il, sl, sm);
        self.__update(l, r, u, ir, sm, sr);
        self.pull(i);
    }

    fn __query(&mut self, l: usize, r: usize, i: usize, sl: usize, sr: usize) -> T {
        if r <= sl || sr <= l {
            return T::ID;
        }
        if l <= sl && sr <= r {
            return self.a[i];
        }
        self.push(i);
        let sm = (sl + sr) >> 1;
        let il = i << 1;
        let ir = i << 1 | 1;
        let x = self.__query(l, r, il, sl, sm);
        let y = self.__query(l, r, ir, sm, sr);
        T::binop(x, y)
    }
    #[inline]
    fn push(&mut self, i: usize) {
        if self.d[i] != U::ID {
            self.apply(i << 1, self.d[i]);
            self.apply(i << 1 | 1, self.d[i]);
            self.d[i] = U::ID;
        }
    }
    #[inline]
    fn apply(&mut self, i: usize, u: U) {
        self.a[i] = T::affine(self.a[i], u);
        if i < self.N {
            self.d[i] = U::binop(self.d[i], u);
        }
    }
    #[inline]
    fn pull(&mut self, i: usize) {
        self.a[i] = T::binop(self.a[i << 1], self.a[i << 1 | 1]);
    }
    // 2^lg space
    fn alloc(n: usize) -> (usize, usize, Vec<T>, Vec<U>) {
        let N = n.next_power_of_two();
        (
            N,
            N.trailing_zeros() as usize,
            vec![T::ID; N << 1],
            vec![U::ID; N],
        )
    }
}
