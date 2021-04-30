use super::node::*;
use crate::algebra::Monoid;
use crate::core::Rng;
use std::{ops::Range, ptr};

/// Lazy treap support flip range, insert/delete element, query range sum.
pub struct TreapLazy<V, F> {
    root: Edge<V, F>,
    buf: Vec<NodeRef<V, F>>,
    length: u32,
    rng: Rng,
}

impl<V, F> TreapLazy<V, F>
where
    V: Monoid<F> + Copy,
{
    pub fn new() -> Self {
        Self {
            root: Edge::none(),
            buf: vec![],
            length: 0,
            rng: Rng::new(),
        }
    }
    /// equiv to `a.insert(n, val)`
    pub fn insert_at(&mut self, n: usize, val: V) {
        let ptr = NodeRef::new(self.length, val, self.rng.gen());
        self.length += 1;
        self.buf.push(ptr);
        self.root.insert_at(n as u32, ptr);
    }
    pub fn delete_at(&mut self, n: usize) {
        self.root.delete_at(n as u32);
    }
    /// equiv to `a[l..r].rev()`
    pub fn rev(&mut self, r: Range<usize>) {
        let Range { start, end } = r;
        self.root.rev(start as u32, end as u32);
    }
    pub fn sum(&mut self, r: Range<usize>) -> V {
        let Range { start, end } = r;
        self.root.summation(start as u32, end as u32)
    }
    /// Find current index of original index.
    pub fn find_index(&self, n: usize) -> usize {
        self.buf[n].find_index()
    }

    /// in-order dfs, call on `(id, val)`.
    pub fn walk<G>(&mut self, mut g: G)
    where
        G: FnMut(u32, V),
    {
        self.root.walk(&mut g);
    }
}
impl<V, F> Drop for TreapLazy<V, F> {
    fn drop(&mut self) {
        while let Some(u) = self.buf.pop() {
            unsafe { ptr::drop_in_place(u.into_raw()) };
        }
    }
}
