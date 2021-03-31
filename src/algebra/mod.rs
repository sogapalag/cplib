//! Algebraic structural traits.

/// Implicitly imply associativity, but not necessarily be commutative.
///
/// Since in most cases, commutativity is out of consideration.
/// But you have to ensure commutativity when needed.
pub trait Monoid<T> {
    const ID: Self;
    fn binop(x: Self, y: Self) -> Self;
}
/// If with commutativity => Abelian Group
pub trait Group<T>: Monoid<T> {
    fn inv(x: Self) -> Self;
}
pub trait Power<T> {
    fn pow(x: Self, n: usize) -> Self;
}

macro_rules! monoid {
    (impl $a:ident for $t:ty, $e:expr, |$x:ident, $y:ident| $b:expr) => {
        impl Monoid<$a> for $t {
            const ID: Self = $e;
            #[inline]
            fn binop($x: Self, $y: Self) -> Self {
                $b
            }
        }
    };
}
macro_rules! group {
    (impl $a:ident for $t:ty, $e:expr,
    |$x:ident, $y:ident| $b:expr, $c:expr) => {
        monoid!(impl $a for $t, $e, |$x,$y| $b);
        impl Group<$a> for $t {
            #[inline]
            fn inv($x: Self) -> Self {
                $c
            }
        }
    };
}

macro_rules! power {
    (impl $a:ident for $t:ty, |$x:ident, $n:ident| $b:expr) => {
        impl Power<$a> for $t {
            #[inline]
            fn pow($x: Self, $n: usize) -> Self {
                $b
            }
        }
    };
}

mod add;
mod bit;
mod cmp;

pub use self::add::Add;
pub use self::bit::{And, Or, Xor};
pub use self::cmp::{Max, Min};
