use std::io::{BufRead, BufReader, ErrorKind, Result};

pub trait TrimRead: BufRead {
    fn read_trim_whitespace(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        read_trim_whitespace(self, buf)
    }
    fn trim(self) -> Trim<Self>
    where
        Self: Sized,
    {
        Trim { buf: self }
    }
}
impl<R: BufRead> TrimRead for BufReader<R> {}

fn read_trim_whitespace<R: BufRead + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize> {
    let mut res = 0;
    // trim prefix
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            match available.iter().position(|c| !c.is_ascii_whitespace()) {
                Some(i) => (true, i),
                None => (false, available.len()),
            }
        };
        r.consume(used);
        if done || used == 0 {
            break;
        }
    }
    // take while
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            match available.iter().position(|c| c.is_ascii_whitespace()) {
                Some(i) => {
                    buf.extend_from_slice(&available[..i]);
                    (true, i + 1)
                }
                None => {
                    buf.extend_from_slice(available);
                    (false, available.len())
                }
            }
        };
        r.consume(used);
        res += used;
        if done || used == 0 {
            return Ok(res);
        }
    }
}
#[derive(Debug)]
pub struct Trim<B> {
    buf: B,
}

impl<B: TrimRead> Iterator for Trim<B> {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Result<Vec<u8>>> {
        let mut buf = Vec::new();
        match self.buf.read_trim_whitespace(&mut buf) {
            Ok(0) => None,
            Ok(_n) => Some(Ok(buf)),
            Err(e) => Some(Err(e)),
        }
    }
}

/// Warning: only handled ASCII whitespace
#[macro_export]
macro_rules! input {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = parse!($iter, $t);
        input!{$iter $($r)*}
    };
}
// iter type = Option<Result<Vec<u8>>>
#[macro_export]
macro_rules! parse {
    // tuple
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(parse!($iter, $t)),* )
    };
    // raw Vec<u8>
    ($iter:expr, raw) => {
        match $iter.next() {
            Some(Ok(x)) => x,
            _ => panic!("No more item now or read fail."),
        }
    };
    // to 0-based
    ($iter:expr, usize1) => {parse!($iter, usize) - 1};
    // [u8;n] := raw
    ($iter:expr, [ u8 ; $n:expr ]) => {
        parse!($iter, raw)
    };
    // [_;n]
    ($iter:expr, [ $t:tt ; $n:expr ]) => {
        (0..$n).map(|_| parse!($iter, $t)).collect::<Vec<_>>()
    };
    // [_]
    ($iter:expr, [ $t:tt ]) => {{
        let n = parse!($iter, usize);
        parse!($iter, [$t; n])
    }};
    // simple type
    ($iter:expr, $t:ty) => {
        match std::str::from_utf8(&(parse!($iter,raw))) {
            Ok(x) => x.parse::<$t>().expect(concat!("Not valid ", stringify!($t))),
            _ => unreachable!(),
        }
    };
}

#[cfg(test)]
mod tests {
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
}
