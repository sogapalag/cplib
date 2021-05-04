use std::{
    mem,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign,
        Shr, ShrAssign,
    },
};

type B = u64;
const WIDTH: usize = mem::size_of::<B>() * 8;
const MASK: usize = WIDTH - 1;
/// Fix sized bit set.
#[derive(Clone)]
pub struct BitVec {
    len: usize,
    buf: Vec<B>,
}

impl BitVec {
    pub fn new(len: usize) -> Self {
        let n = (len + WIDTH - 1) / WIDTH;
        Self {
            len,
            buf: vec![0; n],
        }
    }

    pub fn set(&mut self, i: usize, b: bool) {
        debug_assert!(i < self.len);
        let x = i / WIDTH;
        let y = i & MASK;
        if b {
            self.buf[x] |= 1 << y;
        } else {
            self.buf[x] &= !(1 << y);
        }
    }
    pub fn set_all(&mut self, b: bool) {
        if b {
            self.buf.fill(!0);
            self.clear_unused();
        } else {
            self.buf.fill(0);
        }
    }
    pub fn get(&self, i: usize) -> Option<bool> {
        debug_assert!(i < self.len);
        let x = i / WIDTH;
        let y = i & MASK;
        self.buf.get(x).map(|&x| (x >> y & 1) == 1)
    }
    pub fn count_ones(&self) -> usize {
        self.buf.iter().fold(0, |s, x| s + x.count_ones() as usize)
    }
    pub fn count_zeros(&self) -> usize {
        self.len - self.count_ones()
    }
    pub fn first_one(&self) -> Option<usize> {
        let n = self.buf.len();
        for i in 0..n {
            if self.buf[i] != 0 {
                let res = i * WIDTH + self.buf[i].trailing_zeros() as usize;
                return Some(res);
            }
        }
        None
    }
    fn clear_unused(&mut self) {
        let y = self.len & MASK;
        if y != 0 {
            self.buf.last_mut().map(|x| *x &= (!0) >> (WIDTH - y));
        }
    }
}

impl Index<usize> for BitVec {
    type Output = bool;

    fn index(&self, i: usize) -> &bool {
        if self.get(i).unwrap() {
            &true
        } else {
            &false
        }
    }
}

impl ShlAssign<usize> for BitVec {
    fn shl_assign(&mut self, by: usize) {
        if by == 0 {
            return;
        }
        if by >= self.len {
            return self.set_all(false);
        }
        let x = by / WIDTH;
        let y = by & MASK;
        let n = self.buf.len();
        if y == 0 {
            for i in (x..n).rev() {
                self.buf[i] = self.buf[i - x];
            }
        } else {
            for i in (x + 1..n).rev() {
                self.buf[i] = self.buf[i - x] << y | self.buf[i - x - 1] >> (WIDTH - y);
            }
            self.buf[x] = self.buf[0] << y;
        }
        self.clear_unused();
        self.buf[..x].fill(0);
    }
}
impl Shl<usize> for BitVec {
    type Output = Self;

    fn shl(mut self, by: usize) -> Self::Output {
        self <<= by;
        self
    }
}
impl ShrAssign<usize> for BitVec {
    fn shr_assign(&mut self, by: usize) {
        if by == 0 {
            return;
        }
        if by >= self.len {
            return self.set_all(false);
        }
        let x = by / WIDTH;
        let y = by & MASK;
        let n = self.buf.len();
        if y == 0 {
            for i in x..n {
                self.buf[i - x] = self.buf[i];
            }
        } else {
            for i in x + 1..n {
                self.buf[i - x - 1] = self.buf[i] << (WIDTH - y) | self.buf[i - 1] >> y
            }
            self.buf[n - x - 1] = self.buf[n - 1] >> y;
        }
        self.buf[n - x..].fill(0);
    }
}
impl Shr<usize> for BitVec {
    type Output = Self;

    fn shr(mut self, by: usize) -> Self::Output {
        self >>= by;
        self
    }
}

impl BitOrAssign for BitVec {
    fn bitor_assign(&mut self, rhs: Self) {
        debug_assert!(self.len == rhs.len);
        self.buf
            .iter_mut()
            .zip(rhs.buf.iter())
            .for_each(|(x, y)| *x |= y);
    }
}
impl BitOr for BitVec {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl BitAndAssign for BitVec {
    fn bitand_assign(&mut self, rhs: Self) {
        debug_assert!(self.len == rhs.len);
        self.buf
            .iter_mut()
            .zip(rhs.buf.iter())
            .for_each(|(x, y)| *x &= y);
    }
}
impl BitAnd for BitVec {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl BitXorAssign for BitVec {
    fn bitxor_assign(&mut self, rhs: Self) {
        debug_assert!(self.len == rhs.len);
        self.buf
            .iter_mut()
            .zip(rhs.buf.iter())
            .for_each(|(x, y)| *x ^= y);
    }
}
impl BitXor for BitVec {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl Not for BitVec {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.buf.iter_mut().for_each(|x| *x = !*x);
        self.clear_unused();
        self
    }
}
