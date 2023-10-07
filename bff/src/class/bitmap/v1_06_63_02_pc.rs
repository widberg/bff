use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &()))]
pub struct BitmapBodyV1_06_63_02PC {
    pub size: (u32, u32),
    precalculated_size: u32,
    pub format: u8,
    format_copy: u8,
    palette_format: u8,
    transp_format: u8,
    mip_count: u8,
    unk_set_to4: u8,
    flag: u16,
    // might be faulty?
    #[br(count = precalculated_size, if(precalculated_size != 0))]
    pub dds: Option<Vec<u8>>,
    #[br(if(precalculated_size == 0))]
    #[br(count = size.0 * size.1 * match format {
        12 => 4,
        _ => 3,
    })]
    pub tex: Option<Vec<u8>>,
}

pub type BitmapV1_06_63_02PC = TrivialClass<(), BitmapBodyV1_06_63_02PC>;
