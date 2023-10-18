use super::asobo32_options;
use crate::traits::NameHashFunction;

pub const fn kalisto32(bytes: &[u8]) -> i32 {
    kalisto32_options(bytes, -1)
}

pub const fn kalisto32_options(bytes: &[u8], starting: i32) -> i32 {
    !asobo32_options(bytes, starting)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kalisto32;
impl NameHashFunction for Kalisto32 {
    type Target = i32;

    fn hash(bytes: &[u8]) -> Self::Target {
        kalisto32(bytes)
    }

    fn hash_options(bytes: &[u8], starting: Self::Target) -> Self::Target {
        kalisto32_options(bytes, starting)
    }
}
