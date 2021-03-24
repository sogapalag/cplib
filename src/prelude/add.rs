use crate::prelude::algebra::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::algebra::*;

    #[test]
    fn test_add() {}
}
