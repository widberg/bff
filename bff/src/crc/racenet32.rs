use crate::traits::NameHashFunction;
use super::blacksheep32::CRC32_TABLE;

pub const fn racenet32(bytes: &[u8]) -> i32 {
    racenet32_options(bytes, 0)
}

// https://github.com/lattera/freebsd/blob/401a161083850a9a4ce916f37520c084cff1543b/sys/libkern/crc32.c#L103-L113
pub const fn racenet32_options(bytes: &[u8], starting: i32) -> i32 {
    let mut hash = !starting as u32;
    let mut i: usize = 0;
    while i < bytes.len() {
        let c = bytes[i];
        hash = (hash >> 8) ^ CRC32_TABLE[((c as u32 ^ hash) & 0xff) as usize];
        i += 1;
    }

    !hash as i32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RaceNet32;
impl NameHashFunction for RaceNet32 {
    type Target = i32;

    fn hash(bytes: &[u8]) -> Self::Target {
        racenet32(bytes)
    }

    fn hash_options(bytes: &[u8], starting: Self::Target) -> Self::Target {
        racenet32_options(bytes, starting)
    }
}
