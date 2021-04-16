use std::ops::Range;

use super::num::identities::Two;
use super::num::Integer;

/// Binary search `l..r`, find first `i` s.t. `f(i)` is true.
pub fn bs_first<T, F>(r: Range<T>, mut f: F) -> T
where
    T: Integer + Two + Copy,
    F: FnMut(T) -> bool,
{
    let Range {
        start: mut l,
        end: mut r,
    } = r;
    assert!(l < r);
    while l < r {
        let mid = l + (r - l) / T::TWO;
        if f(mid) {
            r = mid;
        } else {
            l = mid + T::ONE;
        }
    }
    r
}
/// Binary search `l..r`, find last `i` s.t. `f(i)` still true.
pub fn bs_last<T, F>(r: Range<T>, mut f: F) -> T
where
    T: Integer + Two + Copy,
    F: FnMut(T) -> bool,
{
    let Range {
        start: mut l,
        end: mut r,
    } = r;
    assert!(l < r);
    while l < r {
        let mid = l + (r - l) / T::TWO;
        if !f(mid) {
            r = mid;
        } else {
            l = mid + T::ONE;
        }
    }
    r - T::ONE
}

// Deprecated
//macro_rules! bs_first {
//    ($l:expr , $r:expr, |$i:ident| $b:expr) => {{
//        let mut l = $l;
//        let mut r = $r;
//        assert!(l < r);
//        let f = |$i| $b;
//        let mut mid;
//        while l < r {
//            mid = l + (r - l) / 2;
//            if f(mid) {
//                r = mid;
//            } else {
//                l = mid + 1;
//            }
//        }
//        r
//    }};
//}
//macro_rules! bs_last {
//    ($l:expr , $r:expr, |$i:ident| $b:expr) => {{
//        let mut l = $l;
//        let mut r = $r;
//        assert!(l < r);
//        let f = |$i| $b;
//        let mut mid;
//        while l < r {
//            mid = l + (r - l) / 2;
//            if !f(mid) {
//                r = mid;
//            } else {
//                l = mid + 1;
//            }
//        }
//        r - 1
//    }};
//}
#[cfg(test)]
mod tests;
