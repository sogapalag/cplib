pub trait Signed {}
pub trait Unsigned {}

macro_rules! impl_signed {
    (for $($t:ty)*) => {
        $(
        impl Signed for $t {}
    )*
    };
}
impl_signed!(for i8 i16 i32 i64 i128 isize);
macro_rules! impl_unsigned {
    (for $($t:ty)*) => {
        $(
        impl Unsigned for $t {}
    )*
    };
}
impl_unsigned!(for u8 u16 u32 u64 u128 usize);
