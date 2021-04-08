use super::*;

/// A empty struct for trait generic.
pub struct Xor;
/// A empty struct for trait generic.
pub struct Or;
/// A empty struct for trait generic.
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
    u32, 0xffff_ffff;
    u64, 0xffff_ffff_ffff_ffff; // ::MAX require 1.43, Atcoder now 1.42
    //usize, usize::MAX;
);
