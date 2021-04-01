use crate::algebra::Monoid;
use std::marker::PhantomData;
use std::ops::Range;

/// For static RMQ task.
///
/// Warning: must be *Idempotent* monoid.
#[derive(Clone, Debug)]
pub struct SparseTable<T, M> {
    st: Vec<Vec<T>>,
    _m: PhantomData<M>,
}

impl<T, M> SparseTable<T, M>
where
    T: Monoid<M> + Copy,
{
    pub fn new(a: &[T]) -> Self {
        let n = a.len();
        let L = msb(n.next_power_of_two());
        let mut st = vec![Vec::from(a); L + 1];
        for j in 1..=L {
            for i in 0.. {
                if n < i + (1 << j) {
                    break;
                }
                st[j][i] = T::binop(st[j - 1][i], st[j - 1][i + (1 << (j - 1))]);
            }
        }
        Self {
            st,
            _m: PhantomData,
        }
    }
    pub fn query(&self, r: Range<usize>) -> T {
        let Range { start: l, end: r } = r;
        let j = msb(r - l);
        T::binop(self.st[j][l], self.st[j][r - (1 << j)])
    }
}

#[inline]
fn msb(n: usize) -> usize {
    8 * std::mem::size_of::<usize>() - 1 - n.leading_zeros() as usize
}
