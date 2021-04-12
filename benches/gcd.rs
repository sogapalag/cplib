use cplib::core::Rng;
use cplib::math::modulo::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
fn gcd(a: u32, b: u32) -> u32 {
    if a < b {
        gcd(b, a)
    } else {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
}

fn gcd_u64(a: u64, b: u64) -> u64 {
    if a < b {
        gcd_u64(b, a)
    } else {
        if b == 0 {
            a
        } else {
            gcd_u64(b, a % b)
        }
    }
}

fn gcd_rec(n: usize) {
    let rng = Rng::new();
    for _ in 0..n {
        let a = rng.gen() as u32;
        let b = rng.gen() as u32;
        gcd(a, b);
    }
}
fn gcd_rec_u64(n: usize) {
    let rng = Rng::new();
    for _ in 0..n {
        let a = rng.gen();
        let b = rng.gen();
        gcd_u64(a, b);
    }
}
fn gcd_iter(n: usize) {
    let rng = Rng::new();
    for _ in 0..n {
        let a = rng.gen() as u32;
        let b = rng.gen() as u32;
        a.gcd(b);
    }
}
fn gcd_iter_u64(n: usize) {
    let rng = Rng::new();
    for _ in 0..n {
        let a = rng.gen();
        let b = rng.gen();
        a.gcd(b);
    }
}

fn _exgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) -> i32 {
    if b == 0 {
        *x = 1;
        *y = 0;
        a
    } else {
        let d = _exgcd(b, a % b, y, x);
        *y -= a / b * *x;
        d
    }
}
fn exgcd(a: i32, b: i32) -> Exgcd<i32> {
    let (mut x, mut y) = (0, 0);
    let mut d = _exgcd(a, b, &mut x, &mut y);
    if d < 0 {
        d *= -1;
        x *= -1;
        y *= -1;
    }
    Exgcd { gcd: d, x, y }
}

#[allow(overflowing_literals)]
fn exgcd_rec(n: usize) {
    let rng = Rng::new();
    let mask = 0xafff_ffff; // avoid -MIN
    for _ in 0..n {
        let a = rng.gen() as i32;
        let b = rng.gen() as i32;
        let a = a | mask;
        let b = b | mask;
        exgcd(a, b);
    }
}
#[allow(overflowing_literals)]
fn exgcd_iter(n: usize) {
    let rng = Rng::new();
    let mask = 0xafff_ffff; // avoid -MIN
    for _ in 0..n {
        let a = rng.gen() as i32;
        let b = rng.gen() as i32;
        let a = a | mask;
        let b = b | mask;
        a.exgcd(b);
    }
}
// 5ms, 5ms, 5ms, 5ms, 19ms, 17ms.
fn run(c: &mut Criterion) {
    let mut group = c.benchmark_group("Gcd");
    let n = &1000_000_usize;
    group.bench_with_input(BenchmarkId::new("Recursive", n), n, |b, n| {
        b.iter(|| gcd_rec(black_box(*n)))
    });
    group.bench_with_input(BenchmarkId::new("Iterative", n), n, |b, n| {
        b.iter(|| gcd_iter(black_box(*n)))
    });
    group.bench_with_input(BenchmarkId::new("Recursive_u64", n), n, |b, n| {
        b.iter(|| gcd_rec_u64(black_box(*n)))
    });
    group.bench_with_input(BenchmarkId::new("Iterative_u64", n), n, |b, n| {
        b.iter(|| gcd_iter_u64(black_box(*n)))
    });
    group.bench_with_input(BenchmarkId::new("Recursive exgcd", n), n, |b, n| {
        b.iter(|| exgcd_rec(black_box(*n)))
    });
    group.bench_with_input(BenchmarkId::new("Iterative exgcd", n), n, |b, n| {
        b.iter(|| exgcd_iter(black_box(*n)))
    });
    group.finish();
}
criterion_group!(benches, run);
criterion_main!(benches);
