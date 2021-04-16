//! Traits wrapper for number generic.
//!
//! Almost same as crate [num](https://docs.rs/num/0.4.0/num/index.html), slightly modified.
pub mod float;
pub mod identities;
pub mod integer;
pub mod modulo;
pub mod number;

pub use self::float::Float;
pub use self::integer::Integer;
pub use self::number::Num;
