use super::asobo32_options;

pub const fn kalisto32(bytes: &[u8]) -> i32 {
    kalisto32_options(bytes, 0)
}

pub const fn kalisto32_options(bytes: &[u8], starting: i32) -> i32 {
    // Same as asobo32_options but negate the starting and result values
    !asobo32_options(bytes, !starting)
}
