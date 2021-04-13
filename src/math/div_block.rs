type T = usize;
pub struct DivBlock {
    n: T,
    l: T,
    r: T,
}

impl Iterator for DivBlock {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.l >= self.r {
            return None;
        }
        let val = self.n / self.l;
        let mut k = self.n / val + 1;
        if k > self.r {
            k = self.r;
        }
        k -= self.l;
        self.l += k;
        Some((val, k))
    }
}

/// Create a iterator instance of `DivBlock`.
/// Wanna `n/x` for x in `l..r`, group same value, along with count of it as iterator item.
///
/// # Example
///
/// ```
/// use cplib::math::div_block;
///
/// let mut sum = 0;
/// for (val, len) in div_block(6,1,7) {
///    sum += val * len;
/// }
/// assert_eq!(sum, 6+3+2+1+1+1);
/// ```
pub fn div_block(n: T, l: T, r: T) -> DivBlock {
    let l = std::cmp::max(1, l);
    let r = std::cmp::min(r, n + 1);
    DivBlock { n, l, r }
}
