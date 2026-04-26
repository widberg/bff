pub mod context;
pub mod json;
mod scope;
pub mod serde_schema;
pub mod value;
pub mod wordlist;

pub use context::NameContext;
pub use value::{
    Name,
    NameWithContext,
    get_forced_hash_string_for_type,
    hash_string_for_type,
    parse_forced_hash_name,
};
pub use wordlist::*;

use crate::crc::{asobo_alternate32, asobo32, asobo64, blacksheep32, kalisto32, ubisoft64};
use crate::macros::names::names;

names! {
    styles: [Z(append_z), Caps(str::to_uppercase)],
    names: [
        Asobo32(Z, u32, i32, asobo32),
        AsoboAlternate32(Caps, u32, i32, asobo_alternate32),
        Kalisto32(Caps, u32, i32, kalisto32),
        BlackSheep32(Caps, u32, i32, blacksheep32),
        Asobo64(Z, u64, i64, asobo64),
        Ubisoft64(Caps, u64, i64, ubisoft64),
    ]
}

// Faster than format!() but more verbose
#[inline]
fn append_z(s: &str) -> String {
    let mut styled = String::with_capacity(s.len() + 2);
    styled.push_str(s);
    styled.push_str("_Z");
    styled
}
