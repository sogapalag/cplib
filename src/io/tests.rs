use super::*;
use std::io::Cursor;

#[test]
fn parse_i32_slice() {
    let s = br" 7  3221
         -216  3318 -312 
         
        320 0422 +32
        ";
    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    assert_eq!(parse!(trim, [i32]).len(), 7);
}

#[test]
fn input_i32_slice() {
    let s = br" 7  3221
         -216  3318 -312 
         
        320 0422 +32
        ";
    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    input!(trim, a: [i32]);
    assert_eq!(a.len(), 7);
}
#[test]
fn parse_f64_slice() {
    let s = br"+2310.032 -3.01 18 1e8 -3.0e-7";
    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    dbg!(parse!(trim, [f64; 5]));
}
#[test]
fn input_tuple() {
    let s = br"x food -3.0 188";
    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    input!(trim, a: (char, raw, f64, usize));
}
#[test]
fn input_same_mix() {
    let s = br"1321 -321 940";
    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    input!(trim, a: i32, b: i32, c: i32);
}
#[test]
fn test_mix() {
    let s = br"
        7
        32 543 2131  -432 231 432 342
        kfsinx9432ls340
        + 120 299.3
        - 213 -123.3
        ";

    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    input!(trim, a: [i32], b: raw, ops: [(char, usize, f64); 2]);
}
#[test]
fn raw_u8_slice() {
    let s = br"32 123abc?!@#^$%^&*()-=_+,.<>/;':[]";

    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    input!(trim, a: [u8]);
}
#[test]
#[should_panic]
fn overflow_i32() {
    let s = br"1000000000000";
    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    parse!(trim, i32);
}
#[test]
#[should_panic]
fn want_more() {
    let s = br"312 321";
    let buf = BufReader::new(Cursor::new(s));
    let mut trim = buf.trim();
    parse!(trim, i32);
    parse!(trim, i32);
    parse!(trim, i32);
}
