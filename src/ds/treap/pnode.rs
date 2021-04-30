use crate::algebra::Monoid;
use std::{marker::PhantomData, mem, ops::Range, rc::Rc};

pub type NodeRef<V, F> = Rc<Node<V, F>>;
pub type Edge<V, F> = Option<NodeRef<V, F>>;
#[derive(Clone)]
pub struct Node<V, F> {
    priority: u64,
    children: [Edge<V, F>; 2],
    len: usize,
    pub val: V,
    sum: V,
    flip: u8,
    _f: PhantomData<F>,
}
impl<V, F> Node<V, F>
where
    V: Copy,
{
    pub fn new(val: V, priority: u64) -> Self {
        Self {
            priority,
            children: [None, None],
            len: 1,
            val,
            sum: val,
            flip: 0,
            _f: PhantomData,
        }
    }
}

pub trait TreapInner {
    type E;
    fn flip(&mut self);
    fn push(&mut self);
    fn pull(&mut self);
    fn detach(&mut self, n: usize) -> Self::E;
    fn attach(&mut self, n: usize, e: Self::E);
}
impl<V, F> TreapInner for NodeRef<V, F>
where
    V: Monoid<F> + Copy,
    F: Clone,
{
    type E = Edge<V, F>;
    #[inline]
    fn flip(&mut self) {
        Rc::make_mut(self).flip ^= 1;
    }
    #[inline]
    fn push(&mut self) {
        let u = self;
        if u.flip != 0 {
            let u = Rc::make_mut(u);
            u.children.swap(0, 1);
            u.flip = 0;
            for v in u.children.iter_mut() {
                v.as_mut().map(|v| Rc::make_mut(v).flip ^= 1);
            }
        }
    }
    #[inline]
    fn pull(&mut self) {
        let u = Rc::make_mut(self);
        // equiv
        //let u = Rc::get_mut(self).unwrap();
        let [ref l, ref r] = u.children;
        u.len = l.len() + r.len() + 1;
        if mem::size_of::<V>() != 0 {
            u.sum = V::binop(V::binop(l.sum(), u.val), r.sum())
        }
    }
    #[inline]
    fn detach(&mut self, n: usize) -> Self::E {
        Rc::make_mut(self).children[n].take()
    }
    #[inline]
    fn attach(&mut self, n: usize, e: Self::E) {
        Rc::make_mut(self).children[n] = e;
        // equiv
        //Rc::get_mut(self).unwrap().children[n] = e;
    }
}

pub enum TreapOp<E, R = Range<usize>> {
    Insert(usize, E),
    Remove(R),
    Rev(R),
    Sum(R),
}
enum Action<E> {
    Insert(E),
    Remove,
    Rev,
    Sum,
}
pub enum Result<V> {
    None,
    Sum(V),
}

pub trait Treap
where
    Self: Sized + Clone,
{
    type V;
    type Ptr;
    fn empty() -> Self;
    fn singleton(val: Self::V, priority: u64) -> Self;
    // base information
    fn len(&self) -> usize;
    fn val(&self) -> Self::V;
    fn sum(&self) -> Self::V;
    // base operation
    fn flip(&mut self);
    fn push(&mut self);
    fn pull(&mut self);
    fn split_at(self, n: usize) -> (Self, Self);
    fn merge(self, r: Self) -> Self;

    // derivable operation
    fn split_range(self, r: Range<usize>) -> (Self, Self, Self) {
        let Range { start, end } = r;
        debug_assert!(start <= end);
        let (l, r) = self.split_at(end);
        let (l, m) = l.split_at(start);
        (l, m, r)
    }
    fn merge_triple(self, m: Self, r: Self) -> Self {
        self.merge(m).merge(r)
    }
    fn action(&mut self, op: TreapOp<Self, Range<usize>>) -> Result<Self::V> {
        let mut u = Self::empty();
        mem::swap(&mut u, self);
        //let u = self.clone();

        let (range, action) = match op {
            TreapOp::Insert(n, x) => (n..n, Action::Insert(x)),
            TreapOp::Remove(r) => (r, Action::Remove),
            TreapOp::Rev(r) => (r, Action::Rev),
            TreapOp::Sum(r) => (r, Action::Sum),
        };

        let (l, mut m, r) = u.split_range(range);

        let (m, res) = match action {
            Action::Insert(x) => (x, Result::None),
            Action::Remove => (Self::empty(), Result::None),
            Action::Rev => {
                m.flip();
                (m, Result::None)
            }
            Action::Sum => {
                let res = m.sum();
                (m, Result::Sum(res))
            }
        };

        *self = l.merge_triple(m, r);
        res
    }
    //fn insert_at(&mut self, n: usize, single: Self) -> Result<Self::V> {
    //    self.action(TreapOp::Insert(n, single))
    //}
    //fn remove_at(&mut self, n: usize) -> Result<Self::V> {
    //    self.remove_range(n..n + 1)
    //}
    //fn remove_range(&mut self, range: Range<usize>) -> Result<Self::V> {
    //    self.action(TreapOp::Remove(range))
    //}
    //fn rev(&mut self, range: Range<usize>) -> Result<Self::V> {
    //    self.action(TreapOp::Rev(range))
    //}    fn singleton(val: V, priority: u64) -> Self {
}
impl<V, F> Treap for Edge<V, F>
where
    V: Monoid<F> + Copy,
    F: Clone,
{
    type V = V;
    type Ptr = NodeRef<V, F>;
    #[inline]
    fn len(&self) -> usize {
        self.as_ref().map_or(0, |u| u.len)
    }
    #[inline]
    fn empty() -> Self {
        None
    }
    #[inline]
    fn singleton(val: V, priority: u64) -> Self {
        Some(Self::Ptr::new(Node::new(val, priority)))
    }
    #[inline]
    fn val(&self) -> V {
        self.as_ref().map_or(V::ID, |u| u.val)
    }
    #[inline]
    fn sum(&self) -> V {
        self.as_ref().map_or(V::ID, |u| u.sum)
    }
    #[inline]
    fn flip(&mut self) {
        self.as_mut().map(|u| u.flip());
    }
    #[inline]
    fn push(&mut self) {
        self.as_mut().map(|u| u.push());
    }
    #[inline]
    fn pull(&mut self) {
        self.as_mut().map(|u| u.pull());
    }
    #[inline]
    fn split_at(self, n: usize) -> (Self, Self) {
        debug_assert!(self.len() >= n);
        match self {
            None => (None, None),
            Some(mut u) => {
                u.push();
                if u.children[0].len() < n {
                    // x u lr => xul,r
                    let (l, r) = u.detach(1).split_at(n - u.children[0].len() - 1);
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
    #[inline]
    fn merge(self, r: Self) -> Self {
        match (self, r) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                l.push();
                r.push();
                let mut res = if l.priority < r.priority {
                    // l x-r-y => lx -r- y
                    //r.children[0] = Self::from(l).merge(*r.left());
                    let lx = Some(l).merge(r.detach(0));
                    r.attach(0, lx);
                    r
                } else {
                    // x-l-y r => x -l- yr
                    //l.children[1] = l.right().merge(Self::from(r));
                    let yr = l.detach(1).merge(Some(r));
                    l.attach(1, yr);
                    l
                };
                res.pull();
                Some(res)
            }
        }
    }
}

pub fn walk<V, F, G>(u: &mut Edge<V, F>, g: &mut G)
where
    V: Monoid<F> + Copy,
    F: Clone,
    G: FnMut(V),
{
    if let Some(u) = u {
        u.push();
        walk(&mut u.children[0].clone(), g);
        g(u.val);
        walk(&mut u.children[1].clone(), g);
    }
}
