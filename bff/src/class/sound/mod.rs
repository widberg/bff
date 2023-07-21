use bff_derive::{bff_forms, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::SoundV1_291_03_06PC;

#[derive(Serialize, Debug, NamedClass)]
#[bff_forms((V1_291_03_06, PC) => SoundV1_291_03_06PC)]
pub struct Sound {
    sample_rate: u32,
    data_size: Option<u32>,
    sound_type: Option<u16>,
    data: Vec<i16>,
}
