use std::fmt::Display;
use std::str::FromStr;

use num_traits::AsPrimitive;

pub trait NameDisplay<Target>: Copy + Display + FromStr + AsPrimitive<i64> {
    fn from_target(target: Target) -> Self;
    fn into_target(self) -> Target;
}

impl NameDisplay<i32> for i32 {
    fn from_target(target: i32) -> Self {
        target
    }

    fn into_target(self) -> i32 {
        self
    }
}

impl NameDisplay<u32> for i32 {
    fn from_target(target: u32) -> Self {
        target as i32
    }

    fn into_target(self) -> u32 {
        self as u32
    }
}

impl NameDisplay<i64> for i64 {
    fn from_target(target: i64) -> Self {
        target
    }

    fn into_target(self) -> i64 {
        self
    }
}

impl NameDisplay<u64> for i64 {
    fn from_target(target: u64) -> Self {
        target as i64
    }

    fn into_target(self) -> u64 {
        self as u64
    }
}
