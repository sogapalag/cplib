use super::pnode::*;
use crate::algebra::Monoid;
use crate::core::Rng;
use std::ops::Range;

/// Peresistent and lazy treap support flip range, insert/delete element, query range sum.
pub struct TreapPersistent<V, F> {
    pub versions: Vec<Edge<V, F>>,
    rng: Rng,
}

impl<V, F> TreapPersistent<V, F>
where
    V: Monoid<F> + Copy,
    F: Clone,
{
    fn singleton(&self, val: V) -> Edge<V, F> {
        Edge::singleton(val, self.rng.gen())
    }
    pub fn new() -> Self {
        Self {
            versions: vec![None; 1],
            rng: Rng::new(),
        }
    }
    /// Not sigificantly fast
    pub fn with_capacity(n: usize) -> Self {
        let mut versions = Vec::with_capacity(n + 1);
        versions.push(None);
        Self {
            versions,
            rng: Rng::new(),
        }
    }
    fn action(&mut self, ver: usize, op: TreapOp<Edge<V, F>>) -> Result<V> {
        let mut root = self.versions[ver].clone();
        let res = root.action(op);
        self.versions.push(root);
        res
    }
    /// equiv to `a.insert(n, val)`
    pub fn insert_at(&mut self, ver: usize, n: usize, val: V) {
        self.action(ver, TreapOp::Insert(n, self.singleton(val)));
    }
    pub fn remove_at(&mut self, ver: usize, n: usize) {
        self.remove_range(ver, n..n + 1);
    }
    pub fn remove_range(&mut self, ver: usize, range: Range<usize>) {
        self.action(ver, TreapOp::Remove(range));
    }
    /// equiv to `a[l..r].rev()`
    pub fn rev(&mut self, ver: usize, range: Range<usize>) {
        self.action(ver, TreapOp::Rev(range));
    }
    pub fn sum(&mut self, ver: usize, range: Range<usize>) -> V {
        match self.action(ver, TreapOp::Sum(range)) {
            Result::Sum(res) => res,
            _ => unreachable!(),
        }
    }
}
pub use super::pnode::walk;
pub fn to_vec<V, F>(u: &mut Edge<V, F>) -> Vec<V>
where
    V: Monoid<F> + Copy,
    F: Clone,
{
    let mut res = vec![];
    walk(u, &mut |x| {
        res.push(x);
    });
    res
}
