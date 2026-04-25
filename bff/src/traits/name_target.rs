use num_traits::AsPrimitive;

pub trait NameTarget: Copy + AsPrimitive<i64> {
    fn from_i32(value: i32) -> Self;
    fn from_raw(raw: u64) -> Self;
    fn into_raw(self) -> u64;
    fn parse_forced(string: &str) -> Option<Self>;
}

impl NameTarget for i32 {
    fn from_i32(value: i32) -> Self {
        value
    }

    fn from_raw(raw: u64) -> Self {
        let raw_u32: u32 = raw.as_();
        raw_u32.as_()
    }

    fn into_raw(self) -> u64 {
        let raw_u32: u32 = self.as_();
        raw_u32.as_()
    }

    fn parse_forced(string: &str) -> Option<Self> {
        string.parse().ok()
    }
}

impl NameTarget for i64 {
    fn from_i32(value: i32) -> Self {
        value.as_()
    }

    fn from_raw(raw: u64) -> Self {
        raw.as_()
    }

    fn into_raw(self) -> u64 {
        self.as_()
    }

    fn parse_forced(string: &str) -> Option<Self> {
        string.parse().ok()
    }
}
