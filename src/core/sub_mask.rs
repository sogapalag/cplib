/// Iterator of subsets of bitmask, in decreasing order.
pub struct SubMask {
    mask: usize,
    sub: Option<usize>,
}
impl Iterator for SubMask {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.sub;
        self.sub = self.sub.filter(|x| *x > 0).map(|x| (x - 1) & self.mask);
        res
    }
}
/// Iter subsets of mask in decreasing order.
pub fn subs(mask: usize) -> SubMask {
    SubMask {
        mask,
        sub: Some(mask),
    }
}

#[cfg(test)]
mod tests;
