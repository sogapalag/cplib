use super::*;
use crate::algebra::*;

#[test]
fn test_basic() {
    let mut fen = Fenwick::<i32, Add>::new(100);
    fen.add(3, 10);
    fen.add(5, 20);
    fen.add(9, 33);
    assert_eq!(fen.prefix(4), 10);
    assert_eq!(fen.prefix(6), 30);
    assert_eq!(fen.prefix(10), 63);
    assert_eq!(fen.sum(4..9), 20);
    assert_eq!(fen.sum(5..10), 53);
}
#[test]
fn bs() {
    let mut fen = Fenwick::<i32, Add>::new(100);
    fen.add(3, 10);
    fen.add(5, 20);
    fen.add(9, 33);
    assert_eq!(fen.binary_search(30), 5);
    assert_eq!(fen.binary_search(33), 9);
    assert_eq!(fen.binary_search(64), 100);
}
