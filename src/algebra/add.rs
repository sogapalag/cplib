use super::*;

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
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {}
}
