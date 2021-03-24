struct Fenwick2d<T> {
    e: T,
    n: usize,
    m: usize,
    g: Vec<Vec<T>>,
}

// 1-based
impl<T> Fenwick2d<T>
where
    T: Copy + std::ops::AddAssign,
{
    pub fn new(e: T, n: usize, m: usize) -> Self {
        Self {
            e,
            n,
            m,
            g: vec![vec![e; m + 1]; n + 1],
        }
    }
    fn add(&mut self, x: usize, y: usize, val: T) {
        let mut i = x;
        while i <= self.n {
            let mut j = y;
            while j <= self.m {
                self.g[i][j] += val;
                j += Self::lsb(j);
            }
            i += Self::lsb(i)
        }
    }
    // [(1,1)..=(x,y)]
    fn pref(&self, x: usize, y: usize) -> T {
        let mut sum = self.e;
        let mut i = x;
        while i != 0 {
            let mut j = y;
            while j != 0 {
                sum += self.g[i][j];
                j -= Self::lsb(j);
            }
            i -= Self::lsb(i);
        }
        sum
    }
    // (..]
    fn range(&self, xl: usize, yl: usize, xr: usize, yr: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.pref(xr, yr) - self.pref(xl, yr) - (self.pref(xr, yl) - self.pref(xl, yl))
    }

    #[inline]
    fn lsb(i: usize) -> usize {
        let i = i as isize;
        (i & (-i)) as usize
    }
}
