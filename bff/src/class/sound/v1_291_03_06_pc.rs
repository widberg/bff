use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;

#[derive(Debug, BinRead, Serialize)]
#[br(import(_link_header: &()))]
pub struct SoundBodyV1_291_03_06PC {
    sample_rate: u32,
    #[brw(if(sample_rate != 0))]
    data_size: Option<u32>,
    #[brw(if(sample_rate != 0))]
    r#type: Option<u16>,
    #[br(count = data_size.expect("sample rate is 0") / 2)]
    data: Vec<i16>,
}

pub type SoundV1_291_03_06PC = TrivialClass<(), SoundBodyV1_291_03_06PC>;
