/// Binary search `l..r`, find first `i` s.t. `f(i)` is true.
#[macro_export]
macro_rules! bs_first {
    ($l:expr , $r:expr, |$i:ident| $b:expr) => {{
        let mut l = $l;
        let mut r = $r;
        assert!(l < r);
        let f = |$i| $b;
        let mut mid;
        while l < r {
            mid = l + (r - l) / 2;
            if f(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        r
    }};
}
/// Binary search `l..r`, find last `i` s.t. `f(i)` still true.
#[macro_export]
macro_rules! bs_last {
    ($l:expr , $r:expr, |$i:ident| $b:expr) => {{
        let mut l = $l;
        let mut r = $r;
        assert!(l < r);
        let f = |$i| $b;
        let mut mid;
        while l < r {
            mid = l + (r - l) / 2;
            if !f(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        r - 1
    }};
}
#[cfg(test)]
mod tests;
