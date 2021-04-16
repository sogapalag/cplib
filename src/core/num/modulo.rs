use std::ops::{Rem, RemAssign};

use super::Num;

/// Rem and Euclid.
pub trait Modulo: Num + Rem<Output = Self> + RemAssign {
    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
}

macro_rules! impl_modulo {
    (for $($t:ty)*) => {
        $(
        impl Modulo for $t {
            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                self.div_euclid(rhs)
            }
            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
        }
    )*
    };
}
impl_modulo!(for u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize f32 f64);
