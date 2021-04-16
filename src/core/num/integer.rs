use super::Num;

pub trait Integer: Num + PartialOrd + Ord + Eq {}

impl<T> Integer for T where T: Num + PartialOrd + Ord + Eq {}
