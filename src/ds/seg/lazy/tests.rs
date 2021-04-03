use super::*;
use crate::algebra::*;
use crate::prelude::Rng;
// verified: https://judge.yosupo.jp/problem/range_affine_range_sum
#[test]
fn max_add_i32() {
    let n = 512;
    let mut a = vec![0; n];
    affine_new!(impl F<i32> for i32, |x,y| x+y);
    let mut s = SegLazy::<_, _, Max, Add, F>::from(&a);
    let rng = Rng::new();
    let add = |a: &mut Vec<_>, s: &mut SegLazy<_, _, _, _, _>| {
        let mut l = rng.gen() as usize % n;
        let mut r = rng.gen() as usize % n;
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        let x = rng.gen() as i32 % 55;
        s.add(l..r, x);
        dbg!(l, r, x);
        for i in l..r {
            a[i] += x;
        }
    };
    let max = |a: &mut Vec<_>, s: &mut SegLazy<_, _, _, _, _>| {
        let mut l = rng.gen() as usize % n;
        let mut r = rng.gen() as usize % n;
        if l == r {
            return;
        }
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        let res_s = s.sum(l..r);
        let res_a = *a[l..r].iter().max().unwrap();
        dbg!(l, r, res_s, res_a);
        assert_eq!(
            res_s,
            res_a,
            "{:?}\n{:?}\n{:?}\n",
            dbg!(a),
            dbg!(&s.a),
            dbg!(&s.d)
        );
    };
    for _ in 0..1000 {
        let c = rng.gen() & 1;
        if c == 0 {
            add(&mut a, &mut s);
        } else {
            max(&mut a, &mut s);
        }
    }
}
