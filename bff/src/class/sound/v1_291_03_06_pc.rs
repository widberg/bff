use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;

#[bitsize(16)]
#[derive(DebugBits, BinRead, SerializeBits, BinWrite)]
struct SoundFlags {
    paused: u1,
    looping: u1,
    stereo: u1,
    padding: u13,
}

#[derive(Debug, BinRead, Serialize, BinWrite)]
#[br(import(_link_header: &()))]
pub struct SoundBodyV1_291_03_06PC {
    sample_rate: u32,
    data_size: u32,
    flags: SoundFlags,
    #[br(count = data_size / 2)]
    #[serde(skip_serializing)]
    _data: Vec<i16>,
}

pub type SoundV1_291_03_06PC = TrivialClass<(), SoundBodyV1_291_03_06PC>;
