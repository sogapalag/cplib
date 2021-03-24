struct Dsu {
    p: Vec<usize>,
    r: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            r: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    // ret: success(disjoint)?
    fn join(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.find(x);
        y = self.find(y);
        if x == y {
            return false;
        }
        if self.r[x] < self.r[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.p[y] = x;
        self.r[x] += self.r[y];
        true
    }
    fn check(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
