use crate::crc::asobo32::CRC32_TABLE;
use crate::traits::NameHashFunction;

pub const fn asobo_alternate32(bytes: &[u8]) -> i32 {
    asobo_alternate32_options(bytes, 0)
}

pub const fn asobo_alternate32_options(bytes: &[u8], starting: i32) -> i32 {
    let mut hash = starting as u32;
    let mut i: usize = 0;
    while i < bytes.len() {
        let c = bytes[i];
        hash = (hash << 8)
            ^ CRC32_TABLE[((c.to_ascii_lowercase() as u32 ^ (hash >> 0x18)) & 0xff) as usize];
        i += 1;
    }

    hash as i32
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsoboAlternate32;
impl NameHashFunction for AsoboAlternate32 {
    type Target = i32;

    fn hash(bytes: &[u8]) -> Self::Target {
        asobo_alternate32(bytes)
    }

    fn hash_options(bytes: &[u8], starting: Self::Target) -> Self::Target {
        asobo_alternate32_options(bytes, starting)
    }
}
