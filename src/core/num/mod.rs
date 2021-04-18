//! Traits wrapper for number generic.
//!
//! Almost same as crate [num](https://docs.rs/num/0.4.0/num/index.html), slightly modified.
pub mod float;
pub mod identities;
pub mod integer;
pub mod modulo;
pub mod number;
mod sign;

pub use self::float::Float;
pub use self::integer::Integer;
pub use self::number::Num;
pub use self::sign::{Signed, Unsigned};

pub fn pow<T>(b: T, mut e: usize) -> T
where
    T: Clone + self::identities::One + std::ops::MulAssign,
{
    let mut res = T::ONE;
    let mut cur = b;
    while e > 0 {
        if e % 2 != 0 {
            res *= cur.clone();
        }
        cur *= cur.clone();
        e /= 2;
    }
    res
}
