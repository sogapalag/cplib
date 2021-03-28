/// Group-Like

// implicitly imply associativity, but not necessarily be commutative.
pub trait Monoid<T> {
    const ID: Self;
    fn mul(x: Self, y: Self) -> Self;
}
// if with commutativity => Abelian Group
pub trait Group<T>: Monoid<T> {
    fn inv(x: Self) -> Self;
}

t macro_rules! monoid {
    (impl $a:ident for $t:ty, $e:expr, |$x:ident, $y:ident| $b:expr) => {
        impl Monoid<$a> for $t {
            const ID: Self = $e;
            #[inline]
            fn mul($x: Self, $y: Self) -> Self {
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
