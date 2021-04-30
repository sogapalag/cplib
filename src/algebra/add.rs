use super::*;

/// A empty struct for trait generic.
#[derive(Debug, Clone, Copy)]
pub struct Add;

macro_rules! group_add {
    ($($t:ty, $e:expr;)+) => {$(
group!(impl Add for $t, $e, |x, y| x+y, -x);
    )+};
}
group_add!(
    i32, 0;
    i64, 0;
    f32, 0.0;
    f64, 0.0;
);
macro_rules! power_add {
    ($($t:ty)+) => {$(
power!(impl Add for $t, |x, n| x * (n as $t));
    )+};
}
power_add!(i64 f32 f64);
