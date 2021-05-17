#![cfg(test)]

use cplib::algebra::Add;
use cplib::core::Rng;
use cplib::ds::fenwick::Fenwick;

use test::Bencher;

fn bench_add(n: usize) {
    const M: i64 = 1000_000;
    let mut a = Fenwick::<i64, Add>::new(n);
    let r = Rng::new();
    for _ in 0..n {
        let i = r.gen() as usize % n;
        let x = r.gen() as i64 % M;

        a.add(i, x);
    }
}
#[bench]
fn add_1000_000(b: &mut Bencher) {
    b.iter(|| bench_add(1000_000))
}
