//! A family of Fenwick Trees, called Binary Indexed Tree(BIT) also.

mod range;
mod tree;

pub use self::range::RangeAddPointGet;
pub use self::range::RangeAddRangeSum;
pub use self::tree::Fenwick;
