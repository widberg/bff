#[macro_use(derive)]
extern crate derive_aliases;

pub mod bigfile;
pub mod class;
pub mod crc;
pub mod error;
pub mod fat_lin;
pub mod helpers;
pub mod lz;
pub mod macros;
pub mod names;
pub mod traits;
pub mod tsc;
pub use petgraph; // Re-export petgraph for use with BigFile::reference_graph

pub type BffError = crate::error::Error;
pub type BffResult<T> = std::result::Result<T, BffError>;
pub type Endian = binrw::Endian;
pub type BufReader<T> = binrw::io::BufReader<T>;

mod derive_alias {
    derive_aliases::define! {
        BffStruct = ::binrw::BinRead, ::binrw::BinWrite, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, ::bff_derive::ReferencedNames, ::std::fmt::Debug;
        // BffStructBits is useless because bilge can't parse the derive_aliases::derive syntax
        // BffStructBits = ::binrw::BinRead, ::binrw::BinWrite, ::bilge::SerializeBits, ::bilge::DeserializeBits, ::bilge::JsonSchemaBits, ::bff_derive::ReferencedNames, ::bilge::DebugBits;
    }
}
