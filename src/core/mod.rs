//! Many helpers.
#[macro_use]
mod internal_macros;
#[macro_use]
mod binary_search;
mod div_block;
pub mod index_map;
pub mod iter;
//mod log64;
pub mod modular;
mod next_perm;
pub mod num;
mod rng;
mod sub_mask;

pub use self::div_block::div_block;
pub use self::next_perm::next_permutation;
pub use self::rng::Rng;
pub use self::sub_mask::subs;
