use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::identities::{One, Zero};

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
{
}
impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
{
}

pub trait NumAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
{
}
impl<T, Rhs> NumAssignOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
{
}
pub trait NumAssign: Num + NumAssignOps {}
impl<T> NumAssign for T where T: Num + NumAssignOps {}
pub trait NumAssignRef: NumAssign + for<'r> NumAssignOps<&'r Self> {}
impl<T> NumAssignRef for T where T: NumAssign + for<'r> NumAssignOps<&'r T> {}
/// Field.
pub trait Num: PartialEq + Zero + One + NumOps {}
impl<T> Num for T where T: PartialEq + Zero + One + NumOps {}
//auto_trait!(impl Num for u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize f32 f64);
