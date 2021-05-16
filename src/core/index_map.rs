//! For index compressing map.
use super::div_block;
pub trait IndexMap {
    type Val;
    fn nth(&self, n: usize) -> Self::Val;
    fn id(&self, v: Self::Val) -> usize;
    fn len(&self) -> usize;
}
/// n/1, n/2, ... sqrt(n), ...3, 2, 1.  <->  id (0, 1, 2, ..., ~2sqrt(n))
pub struct SqrtIndex {
    pub n: usize,
    pub sqrt: usize,
    pub nth: Vec<usize>,
    small: Vec<usize>,
    large: Vec<usize>,
}
impl SqrtIndex {
    pub fn new(n: usize) -> Self {
        let nth = div_block(n, 1, n + 1)
            .map(|(v, _)| v)
            .collect::<Vec<usize>>();
        let sqrt = (n as f64 + 0.25).sqrt() as usize;
        let mut small = vec![0; sqrt];
        let mut large = vec![0; sqrt];
        for (i, &v) in nth.iter().enumerate() {
            if v > sqrt {
                large[n / v - 1] = i;
            } else {
                small[v - 1] = i;
            }
        }

        Self {
            n,
            sqrt,
            nth,
            small,
            large,
        }
    }
}
impl IndexMap for SqrtIndex {
    type Val = usize;

    #[inline]
    fn nth(&self, n: usize) -> Self::Val {
        self.nth[n]
    }
    #[inline]
    fn id(&self, v: Self::Val) -> usize {
        if v > self.sqrt {
            self.large[self.n / v - 1]
        } else {
            self.small[v - 1]
        }
    }
    #[inline]
    fn len(&self) -> usize {
        self.nth.len()
    }
}

pub struct OrdIndex<V> {
    a: Box<[V]>,
}
impl<V: Ord + Clone> OrdIndex<V> {
    pub fn new(raw: &[V]) -> Self {
        let mut a = vec![];
        a.extend_from_slice(raw);
        a.sort();
        a.dedup();
        Self {
            a: a.into_boxed_slice(),
        }
    }
}
impl<V: Ord + Copy> IndexMap for OrdIndex<V> {
    type Val = V;

    fn nth(&self, n: usize) -> V {
        self.a[n]
    }

    fn id(&self, v: V) -> usize {
        self.a.binary_search(&v).expect("Un-contained value")
    }

    fn len(&self) -> usize {
        self.a.len()
    }
}
