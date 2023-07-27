use binrw::binread;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;

#[binread]
#[derive(Debug, Serialize)]
#[br(import(_link_header: &()))]
pub struct BitmapBodyV1_291_03_06PC {
    size: (u32, u32),
    #[br(temp)]
    precalculated_size: u32,
    flag: u16,
    format: u8,
    mipmap_count: u8,
    unknown: u8,
    #[br(count = precalculated_size)]
    data: Vec<u8>,
}

impl BitmapBodyV1_291_03_06PC {
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

pub type BitmapV1_291_03_06PC = TrivialClass<(), BitmapBodyV1_291_03_06PC>;
