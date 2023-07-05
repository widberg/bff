use binrw::BinRead;
use serde::Serialize;

use crate::strings::PascalString;

// TODO: add objects for different bigfile versions
#[derive(BinRead, Serialize, Debug)]
#[serde(untagged)]
#[br(import(decompressed_size: u32, class_name: u32))]
pub enum Class {
    #[br(pre_assert(class_name == 0x52F79F96))]
    UserDefine { data: PascalString },
    #[br(pre_assert(class_name == 0x57B1F99E))]
    Bitmap {
        width: u32,
        height: u32,
        precalculated_size: u32,
        flag: u16,
        format: u8,
        mipmap_count: u8,
        unknown: u8,
        #[br(count = precalculated_size)]
        data: Vec<u8>,
    },
    #[br(pre_assert(class_name == 0x329ECCE8))]
    Sound {
        sample_rate: u32,
        #[brw(if(sample_rate != 0))]
        data_size: Option<u32>,
        #[brw(if(sample_rate != 0))]
        sound_type: Option<u16>,
        // #[br(restore_position)]
        // #[brw(if(sample_rate != 0))]
        // zero_check: Option<u16>,
        // #[brw(if(zero_check == Some(0) && sample_rate != 0))]
        // zero: Option<u16>,
        #[br(count = data_size.expect("sample rate is 0") / 2)]
        data: Vec<i16>,
    },
    Other {
        #[br(count = decompressed_size)]
        data: Vec<u8>,
    },
}

impl Default for Class {
    fn default() -> Self {
        Class::Other { data: vec![] }
    }
}
