/// Without path compressing, to support roll back.
pub struct DsuRollBack {
    p: Vec<usize>,
    r: Vec<usize>,
    h: Vec<[usize; 2]>,
}

impl DsuRollBack {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            r: vec![1; n],
            h: vec![],
        }
    }
    pub fn find(&self, mut x: usize) -> usize {
        while x != self.p[x] {
            x = self.p[x];
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
        if self.r[x] < self.r[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.h.push([x, y]);
        self.p[y] = x;
        self.r[x] += self.r[y];
        false
    }
    /// one step.
    pub fn roll_back(&mut self) {
        if let Some([x, y]) = self.h.pop() {
            self.r[x] -= self.r[y];
            self.p[y] = y;
        }
    }

    pub fn check(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
