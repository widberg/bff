use bilge::prelude::{bitsize, u1, u13, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use bff_derive::serialize_bits;

#[serialize_bits]
#[bitsize(16)]
#[derive(DebugBits, BinRead)]
struct SoundFlags {
    paused: u1,
    looping: u1,
    stereo: u1,
    _padding: u13,
}

#[derive(Debug, BinRead, Serialize)]
#[br(import(_link_header: &()))]
pub struct SoundBodyV1_291_03_06PC {
    sample_rate: u32,
    data_size: u32,
    flags: SoundFlags,
    #[br(count = data_size / 2)]
    #[serde(skip_serializing)]
    data: Vec<i16>,
}

pub type SoundV1_291_03_06PC = TrivialClass<(), SoundBodyV1_291_03_06PC>;
