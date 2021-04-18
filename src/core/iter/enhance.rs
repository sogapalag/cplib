use super::{accum, Accum};
/// Enhance iterator methods.
pub trait Enhance: Iterator {
    /// Same with `fold` and `scan` of `std`. But ouput n+1 elements with init as first.
    ///
    /// # Example
    ///
    /// ```
    /// use cplib::core::iter::Enhance;
    ///
    /// let a = &[1, 2, 3];
    /// let b: Vec<i32> = a.iter().accum(0, |sum, x| sum + x).collect();
    /// assert_eq!(b, &[0, 1, 3, 6]);
    /// ```
    fn accum<B, F>(self, init: B, f: F) -> Accum<Self, B, F>
    where
        Self: Sized,
    {
        accum(self, init, f)
    }
}

impl<T: ?Sized> Enhance for T where T: Iterator {}
