use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &()))]
pub struct BitmapBodyV1_06_63_02PC {
    size: (u32, u32),
    precalculated_size: u32,
    format: u8,
    format_copy: u8,
    palette_format: u8,
    transp_format: u8,
    mip_count: u8,
    unk_set_to4: u8,
    flag: u16,
    // might be faulty?
    #[br(count = precalculated_size, if(precalculated_size != 0))]
    dds: Option<Vec<u8>>,
    #[br(if(precalculated_size == 0))]
    #[br(count = size.0 * size.1 * match format {
        12 => 4,
        _ => 3,
    })]
    tex: Option<Vec<u8>>,
}

impl BitmapBodyV1_06_63_02PC {
    pub fn dds(&self) -> Option<&Vec<u8>> {
        self.dds.as_ref()
    }
    pub fn tex(&self) -> Option<&Vec<u8>> {
        self.tex.as_ref()
    }
    pub fn size(&self) -> &(u32, u32) {
        &self.size
    }
    pub fn format(&self) -> &u8 {
        &self.format
    }
}

pub type BitmapV1_06_63_02PC = TrivialClass<(), BitmapBodyV1_06_63_02PC>;
