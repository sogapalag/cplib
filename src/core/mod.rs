pub use self::rng::Rng;

#[macro_use]
mod internal_macros;
#[macro_use]
mod binary_search;
mod rng;
mod sub_mask;

pub mod modular;

pub use self::sub_mask::subs;
