use crate::prelude::algebra::*;

pub struct Xor;
pub struct Or;
pub struct And;

macro_rules! group_xor {
    ($($t:ty)+) => {$(
group!(impl Xor for $t, 0, |x, y| x^y, 0^x);
    )+};
}
macro_rules! monoid_or {
    ($($t:ty)+) => {$(
monoid!(impl Or for $t, 0, |x, y| x|y);
    )+};
}
macro_rules! monoid_and {
    ($($t:ty, $e:expr;)+) => {$(
monoid!(impl And for $t, $e, |x, y| x&y);
    )+};
}
group_xor!(i32 i64 u32 u64 usize);
monoid_or!(i32 i64 u32 u64 usize);
monoid_and!(
    i32, -1;
    i64, -1;
    u32, u32::MAX;
    u64, u64::MAX;
    usize, usize::MAX;
);
