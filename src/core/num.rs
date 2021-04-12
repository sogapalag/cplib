use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait Num:
    Sized
    + Copy
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + PartialEq
    + PartialOrd
{
}
macro_rules! impl_num {
    ($($t:ty)*) => {
        $(impl Num for $t {})*
    };
}
impl_num!(u8 i32 u32 i64 u64 isize usize f32 f64);

pub trait Int: Num + Eq + Ord + Rem<Output = Self> + RemAssign {}
macro_rules! impl_int {
    ($($t:ty)*) => {
        $(impl Int for $t {})*
    };
}
impl_int!(u8 i32 u32 i64 u64 isize usize);

pub trait Signed {}
pub trait Unsigned {}
macro_rules! impl_empty_trait {
    ($s:ident, $($t:ty)*) => {
        $(impl $s for $t {})*
    };
}
impl_empty_trait!(Signed, i32 i64 isize);
impl_empty_trait!(Unsigned, u8 u32 u64 usize);
