use bff_derive::{GenericClass, ReferencedNames};
use binrw::helpers::until_eof;
use binrw::{binread, BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
pub struct BitmapHeader {
    #[generic]
    width: u32,
    #[generic]
    height: u32,
    precalculated_size: u32,
    flag: u16,
    format: u8,
    #[generic]
    mipmap_count: u8,
    unknown: u8,
}

#[binread]
#[derive(Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &()))]
pub struct BitmapBodyV1_291_03_06PC {
    #[generic(non_primitive)]
    bitmap_header: BitmapHeader,
    #[br(parse_with = until_eof)]
    #[serde(skip_serializing)]
    #[generic]
    pub data: Vec<u8>,
}

pub type BitmapV1_291_03_06PC = TrivialClass<(), BitmapBodyV1_291_03_06PC>;
