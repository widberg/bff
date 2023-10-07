use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::names::Name;

#[bitsize(16)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits)]
struct SoundFlags {
    paused: u1,
    looping: u1,
    stereo: u1,
    padding: u13,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
pub struct LinkHeader {
    link_name: Name,
    pub sample_rate: u32,
    sound_data_size: u32,
    flags: SoundFlags,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(link_header: &LinkHeader))]
pub struct SoundBodyV1_381_67_09PC {
    #[br(count = link_header.sound_data_size / 2)]
    pub data: Vec<i16>,
}

pub type SoundV1_381_67_09PC = TrivialClass<LinkHeader, SoundBodyV1_381_67_09PC>;
