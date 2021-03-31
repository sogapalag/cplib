//! A family of Fenwick Trees, called Binary Indexed Tree(BIT) also.

mod range;
mod tree;

pub use self::range::RangeAddPointQuery;
pub use self::range::RangeAddRangeQuery;
pub use self::tree::Fenwick;
