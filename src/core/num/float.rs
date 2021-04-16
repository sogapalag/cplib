use super::Num;

pub trait Float: Num + PartialOrd {}
impl Float for f32 {}
impl Float for f64 {}
