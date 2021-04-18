//! Many helpers.
#[macro_use]
mod internal_macros;
#[macro_use]
mod binary_search;
mod div_block;
pub mod index_map;
pub mod iter;
pub mod modular;
pub mod num;
mod rng;
mod sub_mask;

pub use self::div_block::div_block;
pub use self::rng::Rng;
pub use self::sub_mask::subs;
