//! Algebraic structural traits.

/// Implicitly imply associativity, but not necessarily be commutative.
///
/// Since in most cases, commutativity is out of consideration.
/// But you have to ensure commutativity when needed.
pub trait Monoid<M> {
    const ID: Self;
    fn binop(x: Self, y: Self) -> Self;
}
/// If with commutativity => Abelian Group
pub trait Group<M>: Monoid<M> {
    fn inv(x: Self) -> Self;
}
pub trait Power<M> {
    fn pow(x: Self, n: usize) -> Self;
}
pub trait Affine<U, F> {
    fn affine(x: Self, u: U) -> Self;
}

/// Impl monoid trait.
#[macro_export]
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
#[macro_export]
macro_rules! monoid_new {
    (impl $a:ident for $t:ty, $e:expr, |$x:ident, $y:ident| $b:expr) => {
        struct $a;
        impl Monoid<$a> for $t {
            const ID: Self = $e;
            #[inline]
            fn binop($x: Self, $y: Self) -> Self {
                $b
            }
        }
    };
}
/// Impl group trait.
#[macro_export]
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
