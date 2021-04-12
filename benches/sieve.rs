use cplib::math::{sieve, Sieve};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn phi_sieve(n: usize) {
    let s = Sieve::new(n);
    s.phi_table();
}
fn phi_table(n: usize) -> Vec<usize> {
    let mut phi = vec![0; n + 1];
    phi[1] = 1;
    for p in 2..=n {
        if phi[p] == 0 {
            for i in (p..=n).step_by(p) {
                if phi[i] == 0 {
                    phi[i] = i;
                }
                phi[i] = phi[i] / p * (p - 1);
            }
        }
    }
    phi
}

fn mu_sieve(n: usize) {
    let s = Sieve::new(n);
    s.mu_table();
}
fn mu_table(n: usize) -> Vec<i32> {
    use cplib::math::divide::mobius;
    let mut b = vec![0; n + 1];
    b[1] = 1;
    mobius(&mut b);
    b[0] = 1;
    b
}
fn run(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sieve");
    let n = &1000_000_usize;
    // 4ms 13ms 45ms 144ms, linear even slower since they iter primes Vec.
    group.bench_with_input(BenchmarkId::new("Normal", n), n, |b, n| {
        b.iter(|| Sieve::new(black_box(*n)))
    });
    group.bench_with_input(BenchmarkId::new("Linear", n), n, |b, n| {
        b.iter(|| Sieve::linear(black_box(*n)))
    });
    let n = &10_000_000_usize;
    group.bench_with_input(BenchmarkId::new("Normal", n), n, |b, n| {
        b.iter(|| Sieve::new(black_box(*n)))
    });
    group.bench_with_input(BenchmarkId::new("Linear", n), n, |b, n| {
        b.iter(|| Sieve::linear(black_box(*n)))
    });

    // 430ms
    group.bench_with_input(BenchmarkId::new("phi sieve", n), n, |b, n| {
        b.iter(|| phi_sieve(black_box(*n)))
    });
    // 418ms
    group.bench_with_input(BenchmarkId::new("phi direct", n), n, |b, n| {
        b.iter(|| phi_table(black_box(*n)))
    });

    // 164ms
    group.bench_with_input(BenchmarkId::new("mu sieve", n), n, |b, n| {
        b.iter(|| mu_sieve(black_box(*n)))
    });
    // 1.15s
    group.bench_with_input(BenchmarkId::new("mu transform", n), n, |b, n| {
        b.iter(|| mu_table(black_box(*n)))
    });
}
criterion_group!(benches, run);
criterion_main!(benches);
