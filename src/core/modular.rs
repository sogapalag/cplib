//! Modular number type support, check macro [`define_mint!`].
#![allow(non_camel_case_types)]
use crate::algebra::Monoid;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

pub trait Modular<M> {
    const MOD: Self;
}
// TODO change int into macro?? rare case i64 MOD
type int = i32;
// for mul avoid overflow cast
type lar = i64;
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Mint<T, M>(T, PhantomData<M>);

impl<T: Sized, M> Mint<T, M> {
    const fn raw(x: T) -> Self {
        Self {
            0: x,
            1: PhantomData,
        }
    }
}
impl<M> Mint<int, M>
where
    int: Modular<M>,
{
    pub const fn new(x: int) -> Self {
        let mut x = x % int::MOD;
        if x < 0 {
            x += int::MOD;
        }
        Self::raw(x)
    }
    pub const fn modular() -> int {
        int::MOD
    }
}
impl<M> From<int> for Mint<int, M>
where
    int: Modular<M>,
{
    fn from(x: int) -> Self {
        Self::new(x)
    }
}
impl<M> FromStr for Mint<int, M>
where
    int: Modular<M>,
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse::<int>()?))
    }
}
impl<M> Display for Mint<int, M>
where
    int: Modular<M>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<M> AddAssign for Mint<int, M>
where
    int: Modular<M>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        if self.0 >= int::MOD - rhs.0 {
            self.0 -= int::MOD - rhs.0;
        } else {
            self.0 += rhs.0;
        }
    }
}
impl<M> SubAssign for Mint<int, M>
where
    int: Modular<M>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        if self.0 < rhs.0 {
            self.0 += int::MOD - rhs.0;
        } else {
            self.0 -= rhs.0;
        }
    }
}
impl<M> Neg for Mint<int, M>
where
    int: Modular<M>,
{
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::raw(int::MOD - self.0)
    }
}

impl<M> MulAssign for Mint<int, M>
where
    int: Modular<M>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = ((self.0 as lar) * (rhs.0 as lar) % (int::MOD as lar)) as int;
    }
}
impl<M: Copy> Mint<int, M>
where
    int: Modular<M>,
{
    pub fn pow(self, mut e: usize) -> Self {
        let mut res = Self::raw(1);
        let mut cur = self;
        while e > 0 {
            if e % 2 != 0 {
                res *= cur;
            }
            cur *= cur;
            e /= 2;
        }
        res
    }

    pub fn inv(self) -> Self {
        self.pow(int::MOD as usize - 2)
    }
}
impl<M: Copy> DivAssign for Mint<int, M>
where
    int: Modular<M>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.mul_assign(rhs.inv())
    }
}

// forward `T op T` by `T op= T`
macro_rules! forward_binop {
    (impl $Op:ident, $f:ident for $t:ty, $fas:ident) => {
        impl<M: Copy> $Op for Mint<$t, M>
        where
            $t: Modular<M>,
        {
            type Output = Self;
            fn $f(self, rhs: Self) -> Self::Output {
                let mut r = self;
                Self::$fas(&mut r, rhs);
                r
            }
        }
    };
}
forward_binop!(impl Add, add for int, add_assign);
forward_binop!(impl Sub, sub for int, sub_assign);
forward_binop!(impl Mul, mul for int, mul_assign);
forward_binop!(impl Div, div for int, div_assign);

impl<M: Copy> Monoid<crate::algebra::Add> for Mint<int, M>
where
    int: Modular<M>,
{
    const ID: Self = Self::new(0);
    fn binop(x: Self, y: Self) -> Self {
        x + y
    }
}

#[macro_use]
mod def_mint {
    /// Define modular number type with custom name and custom prime.
    ///
    /// # Example
    ///
    /// ```
    /// use cplib::core::modular::*;
    /// use cplib::define_mint;
    ///
    /// define_mint!(m32, 1_000_000_007, Anyname);
    /// let x = m32::new(3);
    /// ```
    #[macro_export]
    macro_rules! define_mint {
        ($name:ident, $prime:expr, $p:ident) => {
            #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
            struct $p {}
            impl Modular<$p> for i32 {
                const MOD: i32 = $prime;
            }
            type $name = Mint<i32, $p>;
        };
    }
}
