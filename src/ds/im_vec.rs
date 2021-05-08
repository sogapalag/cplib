use std::{
    collections::VecDeque,
    fmt, mem,
    ops::{Index, IndexMut},
    rc::Rc,
};
#[derive(Clone)]
/// Immutable vector.
///
/// A naive 32-ary tree implementation, with capacity upper bound to aligned.
/// Almost every operation is *O*(log *n*), but with cache friendly should be small constant approx to *O*(1).
/// Only basic vector operation, used for other immutable/persistent data structure, e.g. `Dsu`.
pub struct ImVec<T> {
    root: Edge<T>,
    cap: usize,
    len: usize,
    level: u32,
}
impl<T: Clone> ImVec<T> {
    /// Create nonzero `32k >= cap` capacity as bound.
    pub fn with_capacity(cap: usize) -> Self {
        // to aligned
        let cap = std::cmp::max(LEN, cap); // disallow empty;
        let cap = (cap + LEN - 1) / LEN * LEN;
        let mut level = 0;
        let mut full = LEN;
        while full < cap {
            level += 1;
            full <<= BITS;
        }
        let root = alloc(cap);
        Self {
            root,
            cap,
            len: 0,
            level,
        }
    }
    /// panic if not enough capacity.
    pub fn push(&mut self, val: T) {
        let entry = self
            .root
            .entry(self.len, self.level)
            .expect("Capacity not enough!");
        debug_assert!(entry.is_none());
        *entry = Some(val);
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let entry = self
            .root
            .entry(self.len, self.level)
            .expect("Capacity not enough!");
        entry.take()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.root.get(i, self.level)
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.root.get_mut(i, self.level)
    }
    /// exact size used to decide capacity.
    pub fn from_exact_sized_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iter.into_iter();
        let len = iter.len();
        let mut a = Self::with_capacity(len);
        for val in iter {
            a.push(val);
        }
        a
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { a: self, i: 0 }
    }
}

impl<T: fmt::Debug + Clone> fmt::Debug for ImVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "len = {},", self.len)?;
        writeln!(f, "cap = {},", self.cap)?;
        writeln!(f, "level = {},", self.level)?;
        writeln!(f, "[")?;
        for x in self.iter() {
            x.fmt(f)?;
            writeln!(f, ", ")?;
        }
        writeln!(f, "]")
    }
}

// naive
pub struct Iter<'a, T> {
    a: &'a ImVec<T>,
    i: usize,
}
impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i >= self.a.len {
            return None;
        }
        self.i += 1;
        self.a.get(i)
    }
}
impl<T: Clone> Index<usize> for ImVec<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        self.get(i).unwrap()
    }
}
impl<T: Clone> IndexMut<usize> for ImVec<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        self.get_mut(i).unwrap()
    }
}

const BITS: u32 = 5;
const LEN: usize = 1 << BITS;
const MASK: usize = LEN - 1;

type Ref<T> = Rc<T>;
#[derive(Clone)]
struct Leaf<T> {
    items: Vec<Option<T>>,
}
#[derive(Clone)]
enum Edge<T> {
    Internal(Ref<Internal<T>>),
    Leaf(Ref<Leaf<T>>),
}
impl<T> Edge<T> {
    fn internal(edges: Vec<OpE<T>>) -> Self {
        Edge::Internal(Ref::new(Internal { edges }))
    }
    fn leaf(items: Vec<Option<T>>) -> Self {
        Edge::Leaf(Ref::new(Leaf { items }))
    }
}
type OpE<T> = Option<Edge<T>>;
#[derive(Clone)]
struct Internal<T> {
    edges: Vec<OpE<T>>,
}
#[inline(always)]
fn index(i: usize, level: u32) -> usize {
    i >> (level * BITS) & MASK
}
fn alloc<T>(cap: usize) -> Edge<T> {
    let mut q = VecDeque::<Edge<T>>::new();
    let n = (cap + LEN - 1) / LEN;
    for _ in 0..n {
        let mut items = vec![];
        for _ in 0..LEN {
            items.push(None);
        }
        q.push_back(Edge::leaf(items));
    }
    while q.len() > 1 {
        let mut next = VecDeque::new();
        let n = (q.len() + LEN - 1) / LEN;
        for _ in 0..n {
            next.push_back(Edge::internal(
                (0..LEN).into_iter().map(|_| q.pop_front()).collect(),
            ))
        }
        mem::swap(&mut q, &mut next);
    }
    q.pop_front().unwrap()
}

impl<T: Clone> Edge<T> {
    fn get(&self, i: usize, mut level: u32) -> Option<&T> {
        let mut u = self;
        loop {
            match u {
                Edge::Internal(internal) => match &internal.edges[index(i, level)] {
                    Some(v) => u = v,
                    _ => return None,
                },
                Edge::Leaf(leaf) => return leaf.items[index(i, level)].as_ref(),
            }
            level -= 1;
        }
    }
    /// get entry of item of leaf
    fn entry(&mut self, i: usize, mut level: u32) -> Option<&mut Option<T>> {
        let mut u = self;
        loop {
            match u {
                Edge::Internal(internal) => {
                    let new_internal = Ref::make_mut(internal);
                    match &mut new_internal.edges[index(i, level)] {
                        Some(v) => u = v,
                        _ => return None,
                    }
                }
                Edge::Leaf(leaf) => {
                    let new_leaf = Ref::make_mut(leaf);
                    return Some(&mut new_leaf.items[index(i, level)]);
                }
            }
            level -= 1;
        }
    }
    fn get_mut(&mut self, i: usize, level: u32) -> Option<&mut T> {
        self.entry(i, level).map_or(None, |leaf| leaf.as_mut())
    }
}

#[cfg(test)]
mod tests;
