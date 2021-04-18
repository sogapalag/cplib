//! Query prefix sum of multiplicative functions when n~10^10.

pub mod dirichlet;
pub mod prime_pos;

pub use self::dirichlet::{PrefixMu, PrefixPhi};
pub use self::prime_pos::PrimeCounter;

#[cfg(test)]
mod tests;
