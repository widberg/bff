use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;

#[bitsize(16)]
#[derive(DebugBits, BinRead, SerializeBits, BinWrite, DeserializeBits)]
pub struct SoundFlags {
    paused: u1,
    looping: u1,
    pub stereo: u1,
    padding: u13,
}

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &()))]
pub struct SoundBodyV1_291_03_06PC {
    pub sample_rate: u32,
    data_size: u32,
    pub flags: SoundFlags,
    #[br(count = data_size / 2)]
    #[serde(skip_serializing)]
    pub data: Vec<i16>,
}

pub type SoundV1_291_03_06PC = TrivialClass<(), SoundBodyV1_291_03_06PC>;
