//! Custom input and ouput.
//!
//! ### Examples
//!
//! ```
//! use std::io::{self, BufReader};
//! use cplib::io::TrimRead;
//!
//! let stdin = io::stdin();
//! let buf = BufReader::new(stdin.lock());
//! let mut iter = buf.trim();
//! ```
//!
//! For more detail, check `input` and `parse`.
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, ErrorKind, Result};

/// An extend trait for `BufRead`, to trim whitespace.
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
/// An iterator over the contents of an instance of `TrimRead` split on ASCII whitespace.
///
/// This struct is generally created by calling [`trim`] on a `TrimRead`.
/// Please see the documentation of [`trim`] for more details.
///
/// [`trim`]: TrimRead::trim
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

/// Wrapper for to display `Vec`.
pub struct Veco<'a, T>(pub &'a [T]);
impl<'a, T> Display for Veco<'a, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.0.len() {
            write!(
                f,
                "{}{}",
                &self.0[i],
                if i + 1 == self.0.len() { "\n" } else { " " }
            )?
        }
        Ok(())
    }
}

/// Warning: only handled ASCII whitespace.
#[macro_export]
macro_rules! input {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = parse!($iter, $t);
        input!{$iter $($r)*}
    };
}
/// Warning: only handled ASCII whitespace.
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
#[allow(unused_variables)]
mod tests;
