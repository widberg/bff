pub mod context;
pub mod json;
pub mod serde_schema;
pub mod value;
pub mod wordlist;

pub use context::NameContext;
pub use value::{
    Name, NameWithContext, get_forced_hash_string, hash_string_for_type, parse_forced_hash_name,
};
pub use wordlist::*;

use crate::crc::{Asobo32, Asobo64, AsoboAlternate32, BlackSheep32, Kalisto32, Ubisoft64};
use crate::macros::names::names;

names! {
    styles: [Z(append_z), Caps(str::to_uppercase)],
    names: [
        Asobo32(Z, Asobo32),
        AsoboAlternate32(Caps, AsoboAlternate32),
        Kalisto32(Caps, Kalisto32),
        BlackSheep32(Caps, BlackSheep32),
        Asobo64(Z, Asobo64),
        Ubisoft64(Caps, Ubisoft64),
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
