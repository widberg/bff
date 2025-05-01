use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};

use crate::class::trivial_class::TrivialClass;

#[bitsize(16)]
#[derive(DebugBits, Clone, BinRead, SerializeBits, BinWrite, DeserializeBits, ReferencedNames)]
pub struct SoundFlags {
    pub paused: u1,
    pub looping: u1,
    pub stereo: u1,
    pub padding: u13,
}

pub struct SoundHeaderGeneric {
    pub sample_rate: u32,
    pub data_size: u32,
    pub flags: SoundFlags,
}

pub struct SoundBodyGeneric {
    pub data: Vec<i16>,
}

pub type SoundGeneric = TrivialClass<SoundHeaderGeneric, SoundBodyGeneric>;
