use crate::prelude::algebra::*;

pub struct Max;
pub struct Min;

macro_rules! monoid_max {
    ($($t:ty, $e:expr;)+) => {$(
monoid!(impl Max for $t, $e, |x, y| if x>y {x} else {y});
    )+};
}
macro_rules! monoid_min {
    ($($t:ty, $e:expr;)+) => {$(
monoid!(impl Min for $t, $e, |x, y| if x<y {x} else {y});
    )+};
}

monoid_max!(
    i32, -1061109568; // 0xc0c0_c0c0
    i64, -4557430888798830400; // 0xc0c0_c0c0_c0c0_c0c0
    f32, -1.0e+35_f32;
    f64, -1.0e+300_f64;
);

monoid_min!(
    i32, 1061109567; // 0x3f3f_3f3f
    i64, 4557430888798830399; // 0x3f3f_3f3f_3f3f_3f3f
    f32, 1.0e+35_f32;
    f64, 1.0e+300_f64;
);
