use bff_derive::ReferencedNames;
use binrw::helpers::until_eof;
use binrw::{binread, BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct LinkHeaderGeneric {
    pub width: u32,
    pub height: u32,
    pub mipmap_count: u8,
}

#[binread]
#[derive(Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &LinkHeaderGeneric))]
pub struct BitmapBodyGeneric {
    #[serde(skip_serializing)]
    #[br(parse_with = until_eof)]
    pub data: Vec<u8>,
}

pub type BitmapGeneric = TrivialClass<LinkHeaderGeneric, BitmapBodyGeneric>;
