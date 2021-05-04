// verified, luogu: p3165 p3391 p5055.
use super::persistent::*;
use super::pnode::*;
use super::*;
use crate::algebra::*;
use crate::core::Rng;
#[test]
fn persistent() {
    let mut t = TreapPersistent::<i32, Add>::new();
    t.insert_at(0, 0, 1);
    t.insert_at(1, 1, 1);
    //t.insert_at(2, 2, 1); //->3
    //t.walk(2, |i, x| println!("{} {}\n", i, x));
    t.remove_at(2, 1); // ver->5
                       //t.walk(2, |i, x| println!("{} {}\n", i, x));
    t.insert_at(3, 1, 10);
    assert_eq!(t.sum(4, 0..2), 11);
    //t.walk(2, |i, x| println!("{} {}\n", i, x));
    //assert_eq!(t.sum(3, 0..1), 1); //
}

#[test]
fn insert_only() {
    let mut t = TreapPersistent::<i64, Add>::new();
    let rng = Rng::new();

    let mut a = vec![vec![]; 1];
    let n = 40_usize;
    for v in 0..n {
        let v = rng.gen() as usize % (v + 1);
        let len = t.versions[v].len();
        assert_eq!(len, a[v].len());
        let i = rng.gen() as usize % (len + 1);
        let x: i64 = rng.gen() as i64 % 32947;

        let mut b = a[v].clone();
        b.insert(i, x);
        a.push(b);
        t.insert_at(v, i, x);
    }

    for v in 0..=n {
        assert_eq!(a[v], to_vec(&mut t.versions[v]));
    }
}

#[test]
fn remove_only() {
    let mut t = TreapPersistent::<i64, Add>::new();
    let rng = Rng::new();

    let mut a = vec![vec![]; 1];
    let n = 40_usize;
    for v in 0..n {
        let x: i64 = rng.gen() as i64 % 32947;
        a[0].push(x);
        t.insert_at(v, v, x);
    }
    for v in 0..n {
        let v = rng.gen() as usize % (v + 1);
        let len = t.versions[v + n].len();
        assert_eq!(len, a[v].len());
        let i = rng.gen() as usize % (len);

        let mut b = a[v].clone();
        b.remove(i);
        a.push(b);
        t.remove_at(n + v, i);
    }

    for v in 0..=n {
        assert_eq!(a[v], to_vec(&mut t.versions[n + v]));
    }
}
#[test]
fn persistent_as_normal() {
    let mut t = TreapPersistent::<i32, Add>::new();

    let n = 10;
    for i in 0..n {
        t.insert_at(i, i, i as i32);
    }
    dbg!(to_vec(&mut t.versions[n]));
    t.rev(n, 3..7);
    dbg!(to_vec(&mut t.versions[n]));
    dbg!(to_vec(&mut t.versions[n + 1])); // 6543
    assert_eq!(t.sum(n + 1, 2..4), 2 + 6);
    assert_eq!(t.sum(n, 2..4), 2 + 3);

    t.remove_at(n, 5);
    dbg!(to_vec(&mut t.versions[n]));
    dbg!(to_vec(&mut t.versions[n + 1]));
    dbg!(to_vec(&mut t.versions[n + 4])); // 3467
    assert_eq!(t.sum(n + 4, 4..6), 4 + 6);

    t.remove_at(n, 3);
    dbg!("ver = n + {}\n", 6);
    dbg!(to_vec(&mut t.versions[n]));
    dbg!(to_vec(&mut t.versions[n + 1]));
    dbg!(to_vec(&mut t.versions[n + 4]));
    dbg!(to_vec(&mut t.versions[n + 6])); // 245

    t.insert_at(n + 4, 5, 11);
    dbg!("ver = n + {}\n", 7);
    dbg!(to_vec(&mut t.versions[n + 4]));
    dbg!(to_vec(&mut t.versions[n + 6])); // 245
    dbg!(to_vec(&mut t.versions[n + 7])); //4 11 6
}
#[test]
fn persistent1() {
    let mut t = TreapPersistent::<i32, Add>::new();
    t.insert_at(0, 0, 1);
    t.insert_at(1, 1, 1);
    t.insert_at(2, 2, 1);
    t.insert_at(3, 3, 1);
    t.insert_at(4, 4, 1);
    assert_eq!(t.sum(5, 0..5), 5); // ver->6
    t.remove_at(3, 1); // ver->7
    assert_eq!(t.sum(7, 0..2), 2); // 8
    t.insert_at(8, 1, 10); //9
    assert_eq!(t.sum(9, 0..3), 12); //10
    assert_eq!(t.sum(4, 0..3), 3); // 11
}
#[test]
fn rev() {
    let mut t = TreapPersistent::<i32, Add>::new();
    for i in 0..10 {
        t.insert_at(i, i, i as i32); // >10
    }
    assert_eq!(t.sum(10, 3..6), 12);

    t.rev(10, 3..6);
    assert_eq!(t.sum(12, 3..6), 12); //13
    assert_eq!(t.sum(12, 3..4), 5);
    assert_eq!(t.sum(10, 3..4), 3); //15

    t.rev(10, 2..7);
    assert_eq!(t.sum(16, 3..5), 5 + 4); //17
    assert_eq!(t.sum(12, 3..4), 5);
    assert_eq!(t.sum(10, 3..4), 3);
}
