use super::*;
#[test]
fn catalan() {
    use crate::define_mint;
    define_mint!(m32, 1_000_000_007, P);
    let c = Comb::<m32>::new(20);
    let cat7 = [1, 7, 27, 75, 165, 297, 429, 429];
    for k in 0..=7 {
        assert_eq!(c.cat(7, k, 1), m32::new(cat7[k]));
    }
}
