//! 0, 1 and other frenquent identities.
use std::ops::{Add, Mul};

pub trait One: Sized + Mul<Self, Output = Self> {
    const ONE: Self;
    #[inline]
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::ONE
    }
    #[inline]
    fn set_one(&mut self) {
        *self = Self::ONE;
    }
}
pub trait Zero: Sized + Add<Self, Output = Self> {
    const ZERO: Self;
    #[inline]
    fn is_zero(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::ZERO
    }
    #[inline]
    fn set_zero(&mut self) {
        *self = Self::ZERO;
    }
}

// Define other frequent identities.
macro_rules! define_identity {
    ($Id:ident, $ID:ident) => {
        pub trait $Id: Sized {
            const $ID: Self;
        }
    };
}

define_identity!(NegOne, NEG_ONE);
define_identity!(Two, TWO);
define_identity!(Three, THREE);
define_identity!(Four, FOUR);
define_identity!(Five, FIVE);
define_identity!(Six, SIX);
define_identity!(Seven, SEVEN);
define_identity!(Eight, EIGHT);
define_identity!(Nine, NINE);
define_identity!(Ten, TEN);

macro_rules! impl_identity {
    ($($t:ty)*, $e:expr, $Id:ident, $ID:ident) => {
        $(
            impl $Id for $t {
                const $ID: Self = $e;
            }
        )*
    };
}
impl_identity!(i8 i16 i32 i64 i128, -1, NegOne, NEG_ONE);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 0, Zero, ZERO);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 1, One, ONE);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 2, Two, TWO);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 3, Three, THREE);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 4, Four, FOUR);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 5, Five, FIVE);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 6, Six, SIX);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 7, Seven, SEVEN);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 8, Eight, EIGHT);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 9, Nine, NINE);
impl_identity!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize, 10, Ten, TEN);

impl_identity!(f32 f64, -1.0, NegOne, NEG_ONE);
impl_identity!(f32 f64, 0.0, Zero, ZERO);
impl_identity!(f32 f64, 1.0, One, ONE);
impl_identity!(f32 f64, 2.0, Two, TWO);
impl_identity!(f32 f64, 3.0, Three, THREE);
impl_identity!(f32 f64, 4.0, Four, FOUR);
impl_identity!(f32 f64, 5.0, Five, FIVE);
impl_identity!(f32 f64, 6.0, Six, SIX);
impl_identity!(f32 f64, 7.0, Seven, SEVEN);
impl_identity!(f32 f64, 8.0, Eight, EIGHT);
impl_identity!(f32 f64, 9.0, Nine, NINE);
impl_identity!(f32 f64, 10.0, Ten, TEN);
