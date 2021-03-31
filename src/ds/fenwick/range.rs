use super::Fenwick;
use crate::algebra::{Group, Power};
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct RangeAddPointQuery<T, M> {
    diff: Fenwick<T, M>,
}

impl<T, M> RangeAddPointQuery<T, M>
where
    T: Group<M> + Copy,
{
    pub fn new(n: usize) -> Self {
        Self {
            diff: Fenwick::new(n + 1),
        }
    }
    pub fn add(&mut self, r: Range<usize>, x: T) {
        let Range { start: l, end: r } = r;
        self.diff.add(l, x);
        self.diff.add(r, T::inv(x));
    }
    pub fn get(&self, i: usize) -> T {
        self.diff.prefix(i + 1)
    }
}

#[derive(Clone, Debug)]
pub struct RangeAddRangeQuery<T, M> {
    a: RangeAddPointQuery<T, M>,
    lack: Fenwick<T, M>,
}

impl<T, M> RangeAddRangeQuery<T, M>
where
    T: Group<M> + Copy + Power<M>,
{
    pub fn new(n: usize) -> Self {
        Self {
            a: RangeAddPointQuery::new(n),
            lack: Fenwick::new(n + 1),
        }
    }
    pub fn add(&mut self, r: Range<usize>, x: T) {
        self.a.add(r.clone(), x);
        let Range { start: l, end: r } = r;
        self.lack.add(l, T::pow(x, l));
        self.lack.add(r, T::inv(T::pow(x, r)));
    }
    pub fn prefix(&self, i: usize) -> T {
        if i > 0 {
            T::binop(T::inv(self.lack.prefix(i)), T::pow(self.a.get(i - 1), i))
        } else {
            T::ID
        }
    }
    pub fn sum(&self, r: Range<usize>) -> T {
        let Range { start: l, end: r } = r;
        T::binop(T::inv(self.prefix(l)), self.prefix(r))
    }
}
#[cfg(test)]
mod tests;
