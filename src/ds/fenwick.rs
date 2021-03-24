use crate::prelude::algebra::*;
use std::marker::PhantomData;

struct Fenwick<T, M> {
    n: usize,
    v: Vec<T>,
    _m: PhantomData<M>,
}

// 1-based
impl<T, M> Fenwick<T, M>
where
    T: Copy + Monoid<M>,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            v: vec![T::ID; n + 1],
            _m: PhantomData,
        }
    }
    pub fn add(&mut self, i: usize, val: T) {
        debug_assert!(i != 0);
        let mut i = i;
        while i <= self.n {
            self.v[i] = T::mul(self.v[i], val);
            i += Self::lsb(i);
        }
    }
    // [1..=i]
    pub fn pref(&self, mut i: usize) -> T {
        let mut sum = T::ID;
        while i != 0 {
            sum = T::mul(sum, self.v[i]);
            i -= Self::lsb(i);
        }
        sum
    }
    // (..]
    pub fn range(&self, l: usize, r: usize) -> T
    where
        T: Group<M>,
    {
        T::mul(self.pref(r), T::inv(self.pref(l)))
    }

    #[inline]
    fn lsb(i: usize) -> usize {
        1 << i.trailing_zeros()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::{add::*, algebra::*, bit::*, cmp::*};
    #[test]
    fn test_basic() {
        let mut fen = Fenwick::<i32, Add>::new(100);
        fen.add(3, 10);
        fen.add(5, 20);
        fen.add(9, 33);
        assert_eq!(fen.pref(4), 10);
        assert_eq!(fen.pref(5), 30);
        assert_eq!(fen.pref(10), 63);
        assert_eq!(fen.range(4, 9), 53);
        assert_eq!(fen.range(5, 9), 33); //note(]
    }
    #[test]
    fn test_max() {
        let mut fen = Fenwick::<i64, Max>::new(100);
        fen.add(5, 100);
        fen.add(10, 10);
        assert_eq!(fen.pref(10), 100);
        fen.add(20, 200);
        assert_eq!(fen.pref(20), 200);
        assert_eq!(fen.pref(5), 100);
    }
    #[test]
    #[should_panic]
    fn test_zero() {
        let mut fen = Fenwick::<i32, Add>::new(100);
        fen.add(0, 100);
    }
}
