use crate::algebra::Monoid;
use std::marker::PhantomData;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct SegTree<T, M> {
    n: usize,
    a: Vec<T>,
    _m: PhantomData<M>,
}

impl<T, M> SegTree<T, M>
where
    T: Monoid<M> + Copy,
{
    pub fn new(n: usize) -> Self {
        let (n, a) = Self::alloc(n);
        Self {
            n,
            a,
            _m: PhantomData,
        }
    }
    pub fn from(leaf: &[T]) -> Self {
        let (n, mut a) = Self::alloc(leaf.len());
        a[n..n + leaf.len()].copy_from_slice(leaf);
        for i in (1..n).rev() {
            a[i] = T::binop(a[i << 1], a[i << 1 | 1]);
        }
        Self {
            n,
            a,
            _m: PhantomData,
        }
    }

    pub fn add(&mut self, i: usize, x: T) {
        let i = i + self.n;
        self.a[i] = T::binop(self.a[i], x);
        self.refresh(i);
    }
    pub fn set(&mut self, i: usize, x: T) {
        let i = i + self.n;
        self.a[i] = x;
        self.refresh(i);
    }
    pub fn sum(&self, r: Range<usize>) -> T {
        let Range {
            start: mut l,
            end: mut r,
        } = r;
        let [mut x, mut y] = [T::ID; 2];
        l += self.n;
        r += self.n;
        while l < r {
            if (l & 1) != 0 {
                x = T::binop(x, self.a[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                y = T::binop(self.a[r], y);
            }
            l >>= 1;
            r >>= 1;
        }
        T::binop(x, y)
    }
    #[inline]
    fn pull(&mut self, i: usize) {
        self.a[i] = T::binop(self.a[i << 1], self.a[i << 1 | 1]);
    }
    fn refresh(&mut self, i: usize) {
        let mut i = i;
        loop {
            i >>= 1;
            if i == 0 {
                break;
            }
            self.pull(i);
        }
    }
    // 2^lg space
    fn alloc(n: usize) -> (usize, Vec<T>) {
        let mut len = 1;
        while len < n {
            len <<= 1;
        }
        (len, vec![T::ID; len << 1])
    }
}
