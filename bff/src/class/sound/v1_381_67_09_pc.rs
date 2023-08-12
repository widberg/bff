use bilge::prelude::{bitsize, u1, u13, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::name::Name;

#[bitsize(16)]
#[derive(BinRead, DebugBits, Serialize)]
struct SoundFlags {
    paused: u1,
    looping: u1,
    stereo: u1,
    padding: u13,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
    sample_rate: u32,
    sound_data_size: u32,
    flags: SoundFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(link_header: &LinkHeader))]
pub struct SoundBodyV1_381_67_09PC {
    #[br(count = link_header.sound_data_size)]
    data: Vec<u8>,
}

pub type SoundV1_381_67_09PC = TrivialClass<LinkHeader, SoundBodyV1_381_67_09PC>;
