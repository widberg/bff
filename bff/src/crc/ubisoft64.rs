use crate::crc::asobo64::CRC64_TABLE;

pub const fn ubisoft64(bytes: &[u8]) -> i64 {
    ubisoft64_options(bytes, 0)
}

pub const fn ubisoft64_options(bytes: &[u8], starting: i64) -> i64 {
    let mut hash = !starting as u64;
    let mut i: usize = 0;
    while i < bytes.len() {
        let c = bytes[i];
        hash = CRC64_TABLE[(((hash >> 56) ^ c.to_ascii_uppercase() as u64) & 0xff) as usize]
            ^ (hash << 8);
        i += 1;
    }

    !hash as i64
}
