#[test]
fn first() {
    let a = [0, 3, 4, 6, 6, 7, 8, 10];
    let i = bs_first!(0, 8, |i| a[i] > 5);
    assert_eq!(i, 3);
    let i = bs_first!(2, 8, |i| { a[i] > 6 });
    assert_eq!(i, 5);

    assert_eq!(123, bs_first!(-1000, 1000, |i| i >= 123));
}
#[test]
fn last() {
    let a = [0, 3, 4, 6, 6, 7, 8, 10];
    let i = bs_last!(0, 8, |i| a[i] <= 6);
    assert_eq!(i, 4);
    let i = bs_last!(2, 8, |i| { a[i] < 6 });
    assert_eq!(i, 2);

    assert_eq!(123, bs_last!(-1000, 1000, |i| i < 124));
}
