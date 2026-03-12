use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};

#[bitsize(16)]
#[derive(
    DebugBits,
    Clone,
    BinRead,
    SerializeBits,
    BinWrite,
    DeserializeBits,
    ReferencedNames,
    JsonSchemaBits,
)]
pub struct SoundFlags {
    pub paused: bool,
    pub looping: bool,
    pub stereo: bool,
    pub padding: u13,
}
