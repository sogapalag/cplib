use super::*;
#[test]
fn count() {
    assert_eq!(subs(0xff).count(), 0x100);
    assert_eq!(subs(0b10101101).count(), 2_usize.pow(5));
    assert_eq!(subs(0).count(), 1);
}
