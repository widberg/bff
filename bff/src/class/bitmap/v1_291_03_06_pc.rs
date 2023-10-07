use binrw::{binread, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;

#[binread]
#[derive(Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &()))]
pub struct BitmapBodyV1_291_03_06PC {
    pub size: (u32, u32),
    #[br(temp)]
    precalculated_size: u32,
    flag: u16,
    format: u8,
    mipmap_count: u8,
    unknown: u8,
    #[br(count = precalculated_size)]
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

pub type BitmapV1_291_03_06PC = TrivialClass<(), BitmapBodyV1_291_03_06PC>;
