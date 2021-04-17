//! Mod number type support, check macro [`define_mint!`].
#![allow(non_camel_case_types)]
use crate::algebra::Monoid;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

use super::num::identities::{One, Zero};

pub trait Mod<M> {
    const MOD: Self;
}

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
impl<M> Mint<i32, M>
where
    i32: Mod<M>,
{
    pub const fn new(x: i32) -> Self {
        let mut x = x % i32::MOD;
        if x < 0 {
            x += i32::MOD;
        }
        Self::raw(x)
    }
    pub const fn modu() -> i32 {
        i32::MOD
    }
}
impl<M> From<i32> for Mint<i32, M>
where
    i32: Mod<M>,
{
    fn from(x: i32) -> Self {
        Self::new(x)
    }
}
macro_rules! from_larger {
    ($($t:ty)*) => {
        $(
            impl<M> From<$t> for Mint<i32, M>
            where
                i32: Mod<M>,
            {
                fn from(x: $t) -> Self {
                    let x = x.rem_euclid(i32::MOD as $t) as i32;
                    Self::raw(x)
                }
            }
        )*
    };
}
from_larger!(u32 u64 u128 i64 i128 usize isize);
impl<M> From<Mint<i32, M>> for i32 {
    fn from(x: Mint<i32, M>) -> Self {
        x.0
    }
}

impl<M> FromStr for Mint<i32, M>
where
    i32: Mod<M>,
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse::<i32>()?))
    }
}
impl<M> Display for Mint<i32, M>
where
    i32: Mod<M>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<M> AddAssign for Mint<i32, M>
where
    i32: Mod<M>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        if self.0 >= i32::MOD - rhs.0 {
            self.0 -= i32::MOD - rhs.0;
        } else {
            self.0 += rhs.0;
        }
    }
}
impl<M> SubAssign for Mint<i32, M>
where
    i32: Mod<M>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        if self.0 < rhs.0 {
            self.0 += i32::MOD - rhs.0;
        } else {
            self.0 -= rhs.0;
        }
    }
}
impl<M> Neg for Mint<i32, M>
where
    i32: Mod<M>,
{
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::raw(i32::MOD - self.0)
    }
}

impl<M> MulAssign for Mint<i32, M>
where
    i32: Mod<M>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = ((self.0 as i64) * (rhs.0 as i64) % (i32::MOD as i64)) as i32;
    }
}
impl<M: Copy> Mint<i32, M>
where
    i32: Mod<M>,
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
        self.pow(i32::MOD as usize - 2)
    }
}
impl<M: Copy> DivAssign for Mint<i32, M>
where
    i32: Mod<M>,
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
            $t: Mod<M>,
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
forward_binop!(impl Add, add for i32, add_assign);
forward_binop!(impl Sub, sub for i32, sub_assign);
forward_binop!(impl Mul, mul for i32, mul_assign);
forward_binop!(impl Div, div for i32, div_assign);

// Auto traits.
impl<M: Copy> Monoid<crate::algebra::Add> for Mint<i32, M>
where
    i32: Mod<M>,
{
    const ID: Self = Self::new(0);
    fn binop(x: Self, y: Self) -> Self {
        x + y
    }
}

impl<M> One for Mint<i32, M>
where
    Self: Mul<Output = Self>,
{
    const ONE: Self = Self::raw(1);
}
impl<M> Zero for Mint<i32, M>
where
    Self: Add<Output = Self>,
{
    const ZERO: Self = Self::raw(0);
}
///impl<M> Num for Mint<i32, M> {}

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
            impl Mod<$p> for i32 {
                const MOD: i32 = $prime;
            }
            type $name = Mint<i32, $p>;
        };
    }
}
