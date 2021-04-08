use super::*;

#[test]
fn test_uniform() {
    let (m, n) = (37, 1000_000);
    let mut a = vec![0; m];
    let rng = Rng::new();
    for _ in 0..n {
        a[rng.gen() as usize % m] += 1;
    }
    dbg!(a);
}
