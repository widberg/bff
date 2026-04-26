use std::fmt::Display;
use std::str::FromStr;

pub trait NameDisplay<Target>: Copy + Display + FromStr {
    fn from_target(target: Target) -> Self;
    fn into_target(self) -> Target;
}

impl NameDisplay<u32> for i32 {
    fn from_target(target: u32) -> Self {
        target as Self
    }

    fn into_target(self) -> u32 {
        self as u32
    }
}

impl NameDisplay<u64> for i64 {
    fn from_target(target: u64) -> Self {
        target as Self
    }

    fn into_target(self) -> u64 {
        self as u64
    }
}
