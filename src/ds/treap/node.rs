use std::{
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::algebra::Monoid;

pub struct Node<V, F> {
    priority: u64,
    children: [Edge<V, F>; 2],
    parent: Edge<V, F>,
    pub id: u32,
    size: u32,
    pub val: V,
    sum: V,
    flip: u8,
}
impl<V, F> Node<V, F>
where
    V: Monoid<F> + Copy,
{
    fn new(id: u32, val: V, priority: u64) -> Self {
        Self {
            priority,
            children: [Edge::none(); 2],
            parent: Edge::none(),
            id,
            size: 1,
            val,
            sum: val,
            flip: 0,
        }
    }
}
type BoxedNode<V, F> = NonNull<Node<V, F>>;

pub struct NodeRef<V, F> {
    node: BoxedNode<V, F>,
    _f: PhantomData<F>,
}
enum Child {
    Left,
    Right,
    Orphan,
}
impl<V, F> NodeRef<V, F> {
    pub fn into_raw(&self) -> *mut Node<V, F> {
        self.node.as_ptr()
    }

    #[inline]
    fn left(&self) -> &Edge<V, F> {
        &self.children[0]
    }
    #[inline]
    fn right(&self) -> &Edge<V, F> {
        &self.children[1]
    }
    #[inline]
    fn child_kind(&self) -> Child {
        match self.parent.edge {
            None => Child::Orphan,
            Some(p) => match p.left().edge {
                Some(u) if &u == self => Child::Left,
                _ => Child::Right,
            },
        }
    }
    /// Push down lazy tag.
    #[inline]
    fn push(&self) {
        let mut u = *self;
        if u.flip != 0 {
            u.children.swap(0, 1);
            u.flip = 0;
            for v in u.children.iter() {
                v.edge.map(|mut v| v.flip ^= 1);
            }
        }
    }
    #[inline]
    fn detach(&mut self, n: usize) -> Edge<V, F> {
        let res = self.children[n];
        self.children[n] = Edge::none();
        if let Some(mut v) = res.edge {
            v.parent = Edge::none();
        }
        res
    }
    #[inline]
    fn attach(&mut self, n: usize, v: Edge<V, F>) {
        debug_assert!(self.children[n].edge.is_none());
        v.edge.map(|mut v| v.parent = Edge::from(*self));
        self.children[n] = v;
    }
}
impl<V, F> NodeRef<V, F>
where
    V: Monoid<F> + Copy,
{
    pub fn new(id: u32, val: V, priority: u64) -> Self {
        let ptr: *mut _ = Box::into_raw(Box::new(Node::new(id, val, priority)));
        Self {
            node: unsafe { NonNull::new_unchecked(ptr) },
            _f: PhantomData,
        }
    }
    /// Update current node's sum.
    #[inline]
    fn pull(&self) {
        let mut u = *self;
        let [l, r] = u.children;
        u.size = l.len() + r.len() + 1;
        if mem::size_of::<V>() != 0 {
            u.sum = V::binop(V::binop(l.sum(), u.val), r.sum())
        }
    }
    pub fn find_index(&self) -> usize {
        let mut stack = vec![];
        let mut u = *self;
        stack.push(u);
        while let Some(pa) = u.parent.edge {
            stack.push(pa);
            u = pa;
        }
        stack.iter().rev().for_each(|u| u.push());
        (stack[0].left().len()
            + stack.windows(2).fold(0, |sum, k| {
                let u = k[0];
                let p = k[1];
                sum + match u.child_kind() {
                    Child::Right => 1 + p.left().len(),
                    _ => 0,
                }
            })) as usize
    }
}

pub struct Edge<V, F> {
    edge: Option<NodeRef<V, F>>,
}

impl<V, F> Edge<V, F> {
    pub fn none() -> Self {
        Self { edge: None }
    }
    fn len(&self) -> u32 {
        self.edge.map_or(0, |u| u.size)
    }
    fn push(&self) {
        if let Some(u) = self.edge {
            u.push();
        }
    }
    fn is_some(&self) -> bool {
        self.edge.is_some()
    }
    fn is_none(&self) -> bool {
        self.edge.is_none()
    }
}
impl<V, F> Edge<V, F>
where
    V: Monoid<F> + Copy,
{
    fn pull(&self) {
        if let Some(u) = self.edge {
            u.pull()
        }
    }
    fn val(&self) -> V {
        self.edge.map_or(V::ID, |u| u.val)
    }
    fn sum(&self) -> V {
        self.edge.map_or(V::ID, |u| u.sum)
    }
    pub fn split_at(self, n: u32) -> (Self, Self) {
        match self.edge {
            None => (Self::none(), Self::none()),
            Some(mut u) => {
                u.push();
                if u.left().len() < n {
                    // x u lr => xul,r
                    let (l, r) = u.detach(1).split_at(n - u.left().len() - 1);
                    u.attach(1, l);
                    u.pull();
                    (Self::from(u), r)
                } else {
                    // lr u y => l ruy
                    let (l, r) = u.detach(0).split_at(n);
                    u.attach(0, r);
                    u.pull();
                    (l, Self::from(u))
                }
            }
        }
    }
    pub fn merge(self, r: Self) -> Self {
        match (self.edge, r.edge) {
            (None, _) => r,
            (_, None) => self,
            (Some(mut l), Some(mut r)) => {
                l.push();
                r.push();
                let res = if l.priority < r.priority {
                    // l x-r-y => lx -r- y
                    //r.children[0] = Self::from(l).merge(*r.left());
                    let lx = Self::from(l).merge(r.detach(0));
                    r.attach(0, lx);
                    r
                } else {
                    // x-l-y r => x -l- yr
                    //l.children[1] = l.right().merge(Self::from(r));
                    let yr = l.detach(1).merge(Self::from(r));
                    l.attach(1, yr);
                    l
                };
                res.pull();
                Self::from(res)
            }
        }
    }
    /// new single node
    pub fn insert_at(&mut self, n: u32, p: NodeRef<V, F>) {
        let (l, r) = self.split_at(n);
        *self = l.merge(Self::from(p)).merge(r);
    }
    pub fn delete_at(&mut self, n: u32) {
        let (l, r) = self.split_at(n);
        let (_m, r) = r.split_at(1);
        *self = l.merge(r);
    }
    pub fn rev(&mut self, start: u32, end: u32) {
        if start >= end {
            return;
        }
        let (l, r) = self.split_at(end);
        let (l, m) = l.split_at(start);
        m.edge.map(|mut u| u.flip ^= 1);
        *self = l.merge(m).merge(r);
    }
    pub fn summation(&mut self, start: u32, end: u32) -> V {
        if start >= end {
            return V::ID;
        }
        let (l, r) = self.split_at(end);
        let (l, m) = l.split_at(start);
        let res = m.sum();
        *self = l.merge(m).merge(r);
        res
    }
    pub fn walk<G>(&self, g: &mut G)
    where
        G: FnMut(u32, V),
    {
        if let Some(u) = self.edge {
            u.push();
            u.left().walk(g);
            g(u.id, u.val);
            u.right().walk(g);
        }
    }
}
impl<V, F> PartialEq for NodeRef<V, F> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}
impl<V, F> Eq for NodeRef<V, F> {}
impl<V, F> Clone for NodeRef<V, F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<V, F> Copy for NodeRef<V, F> {}
impl<V, F> Deref for NodeRef<V, F> {
    type Target = Node<V, F>;
    fn deref(&self) -> &Node<V, F> {
        unsafe { self.node.as_ref() }
    }
}
impl<V, F> DerefMut for NodeRef<V, F> {
    fn deref_mut(&mut self) -> &mut Node<V, F> {
        unsafe { self.node.as_mut() }
    }
}
impl<V, F> Clone for Edge<V, F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<V, F> Copy for Edge<V, F> {}
impl<V, F> From<NodeRef<V, F>> for Edge<V, F> {
    fn from(u: NodeRef<V, F>) -> Self {
        Self { edge: Some(u) }
    }
}
