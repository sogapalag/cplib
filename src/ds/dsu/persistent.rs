use std::fmt;

use crate::ds::ImVec;

#[derive(Clone)]
struct Node {
    p: usize,
    r: usize,
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "p = {}, r = {}", self.p, self.r)
    }
}
#[derive(Clone, Debug)]
/// Immutable Dsu with ImVec implementation.
///
/// # Example
///
/// ```
/// use cplib::ds::dsu::DsuPersistent;
///
/// let a = DsuPersistent::new(3);
/// let mut b = a.clone();
/// b.join(0, 2);
/// assert!(b.check(0, 2));
/// assert!(!a.check(0, 2));
/// ```
pub struct DsuPersistent(ImVec<Node>);

impl DsuPersistent {
    pub fn new(n: usize) -> Self {
        Self(ImVec::from_exact_sized_iter(
            (0..n).map(|i| Node { p: i, r: 1 }),
        ))
    }
    pub fn find(&self, mut x: usize) -> usize {
        while x != self.0[x].p {
            x = self.0[x].p;
        }
        x
    }
    /// ret: check(x,y)
    pub fn join(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return true;
        }
        if self.0[x].r < self.0[y].r {
            std::mem::swap(&mut x, &mut y);
        }
        self.0[y].p = self.0[x].p;
        self.0[x].r += self.0[y].r;
        false
    }
    pub fn check(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
