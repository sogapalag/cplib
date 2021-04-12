use crate::core::modular::*;

type m32<P> = Mint<i32, P>;
/// Recover integer from two `Mint` of different modulo. Similar to excrt.
pub fn garner<P: Copy, Q: Copy>(a: m32<P>, b: m32<Q>) -> i64
where
    i32: Modular<P> + Modular<Q>,
{
    let p = m32::<P>::modular();
    let q = m32::<Q>::modular();
    let ip: i32 = m32::<Q>::new(p).inv().into();
    let iq: i32 = m32::<P>::new(q).inv().into();
    let res =
        (i32::from(a) as i64 * iq as i64 * q as i64) + (i32::from(b) as i64 * ip as i64 * p as i64);
    res % (p as i64 * q as i64)
}
