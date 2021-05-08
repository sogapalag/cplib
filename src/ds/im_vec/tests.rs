// verified by luogu p3402, MLE last case.
// need to optimize LEN=1/2, usize->u32, Leaf Vec<Op> -> Array<MaybeUninit>.
// TODO: optimize Vec with Chunk/Array, Option with MaybeUninit?
use super::*;

#[test]
fn push_to_another() {
    let mut a = ImVec::<i32>::with_capacity(LEN);
    let mut b = a.clone();
    b.push(3);
    b.push(6);
    assert_eq!(a.len, 0);
    assert_eq!(b.len, 2);

    a.push(7);
    assert_eq!(a.len, 1);
    assert_eq!(b.len, 2);
    assert_eq!(a.iter().map(|x| x.clone()).collect::<Vec<i32>>(), vec![7]);
    assert_eq!(
        b.iter().map(|x| x.clone()).collect::<Vec<i32>>(),
        vec![3, 6]
    );
}

#[test]
fn init_unaligned() {
    let a = ImVec::<i32>::with_capacity(LEN - 1);
    assert!(a.cap == LEN);
    let a = ImVec::<i32>::with_capacity(2 * LEN - 1);
    assert!(a.cap == LEN * 2);
    let a = ImVec::<i32>::with_capacity(LEN * LEN - 1);
    assert!(a.cap == LEN * LEN);
}

#[test]
fn init_aligned() {
    let a = ImVec::<i32>::with_capacity(LEN);
    assert!(a.cap == LEN);
    let a = ImVec::<i32>::with_capacity(2 * LEN);
    assert!(a.cap == LEN * 2);
    let a = ImVec::<i32>::with_capacity(LEN * LEN);
    assert!(a.cap == LEN * LEN);
}

#[test]
fn get() {
    let mut a = ImVec::<i32>::with_capacity(LEN * 6);
    for i in 0..LEN * 4 {
        a.push(i as _);
    }

    for i in 0..LEN * 6 {
        let should = if i < LEN * 4 { Some(i as i32) } else { None };
        assert_eq!(a.get(i), should.as_ref());
    }
    for i in 0..LEN * 4 {
        assert_eq!(a[i], i as _);
    }
}
#[test]
fn get_mut() {
    let mut a = ImVec::<i32>::with_capacity(LEN * 6);
    for i in 0..LEN * 4 {
        a.push(i as _);
    }

    for i in 0..LEN * 6 {
        let mut should = if i < LEN * 4 { Some(i as i32) } else { None };
        assert_eq!(a.get_mut(i), should.as_mut());
    }

    for i in 0..LEN * 2 {
        a[i] = -(i as i32);
    }
    for i in 0..LEN * 2 {
        assert_eq!(a[i], -(i as i32));
    }
    for i in LEN * 2..LEN * 4 {
        assert_eq!(a[i], i as _);
    }
}

#[test]
#[should_panic]
fn push_extra() {
    let mut a = ImVec::<i32>::with_capacity(LEN);
    for _ in 0..=LEN {
        a.push(0);
    }
}

#[test]
fn pop_empty() {
    let mut a = ImVec::<i32>::with_capacity(LEN);
    assert!(a.pop().is_none());
}
