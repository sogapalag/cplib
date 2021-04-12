use cplib::algebra::Add;
use cplib::core::Rng;
use cplib::ds::fenwick::Fenwick;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
fn basic(c: &mut Criterion) {
    c.bench_function("add 1000_000", |b| {
        b.iter(|| bench_add(black_box(1000_000)))
    });
}

criterion_group!(benches, basic);
criterion_main!(benches);
