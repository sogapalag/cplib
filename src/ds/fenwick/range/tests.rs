use super::*;
use crate::algebra::Add;
use crate::prelude::Rng;

#[test]
fn add_sum() {
    let n = 100;
    let mut s = RangeAddRangeSum::<i64, Add>::new(n);
    let mut a = vec![0i64; n];
    let rng = Rng::new();
    let add = |a: &mut Vec<_>, s: &mut RangeAddRangeSum<_, _>| {
        let mut l = rng.gen() as usize % n;
        let mut r = rng.gen() as usize % n;
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        let x = rng.gen() as i64 % 553;
        s.add(l..r, x);
        for i in l..r {
            a[i] += x;
        }
    };
    let sum = |a: &mut Vec<_>, s: &mut RangeAddRangeSum<_, _>| {
        let mut l = rng.gen() as usize % n;
        let mut r = rng.gen() as usize % n;
        if l == r {
            return;
        }
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        let res_s = s.sum(l..r);
        let res_a: i64 = a[l..r].iter().sum();
        assert_eq!(res_s, res_a,);
    };
    for _ in 0..1000 {
        let c = rng.gen() & 1;
        if c == 0 {
            add(&mut a, &mut s);
        } else {
            sum(&mut a, &mut s);
        }
    }
    // test 0..n
    assert_eq!(s.sum(0..n), a.iter().sum());
}
