pub trait NameTarget: Copy {
    fn from_i32(value: i32) -> Self;
    fn from_raw(raw: u64) -> Self;
    fn into_raw(self) -> u64;
}

impl NameTarget for u32 {
    fn from_i32(value: i32) -> Self {
        value as u32
    }

    fn from_raw(raw: u64) -> Self {
        raw as u32
    }

    fn into_raw(self) -> u64 {
        u64::from(self)
    }
}

impl NameTarget for u64 {
    fn from_i32(value: i32) -> Self {
        i64::from(value) as u64
    }

    fn from_raw(raw: u64) -> Self {
        raw
    }

    fn into_raw(self) -> u64 {
        self
    }
}
