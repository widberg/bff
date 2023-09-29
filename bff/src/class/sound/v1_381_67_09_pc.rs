use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::name::Name;

#[bitsize(16)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite)]
struct SoundFlags {
    paused: u1,
    looping: u1,
    stereo: u1,
    padding: u13,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
pub struct LinkHeader {
    link_name: Name,
    sample_rate: u32,
    sound_data_size: u32,
    flags: SoundFlags,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(link_header: &LinkHeader))]
pub struct SoundBodyV1_381_67_09PC {
    #[br(count = link_header.sound_data_size)]
    data: Vec<u8>,
}

pub type SoundV1_381_67_09PC = TrivialClass<LinkHeader, SoundBodyV1_381_67_09PC>;
