//! FFT and related.

pub mod fwht;
mod ntt;

pub enum Direction {
    Forward,
    Inverse,
}
pub use self::ntt::Ntt;

#[cfg(test)]
mod tests;
