use bff_derive::{GenericClass, ReferencedNames};
use binrw::helpers::until_eof;
use binrw::{BinRead, BinWrite, binread};
use serde::{Deserialize, Serialize};

use super::generic::BitmapGeneric;
use crate::class::trivial_class::TrivialClass;
use crate::macros::trivial_class_generic::trivial_class_generic;
use crate::traits::{Export, Import};

#[derive(
    BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass, Clone,
)]
#[generic(name(BitmapHeaderGeneric))]
pub struct BitmapHeader {
    #[generic]
    width: u32,
    #[generic]
    height: u32,
    #[generic]
    precalculated_size: u32,
    flag: u16,
    format: u8,
    #[generic]
    mipmap_count: u8,
    unknown: u8,
}

#[binread]
#[derive(Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &BitmapHeader))]
pub struct BitmapBodyV1_291_03_06PC {
    header: BitmapHeader,
    #[br(parse_with = until_eof)]
    #[serde(skip)]
    #[generic]
    data: Vec<u8>,
}

pub type BitmapV1_291_03_06PC = TrivialClass<BitmapHeader, BitmapBodyV1_291_03_06PC>;

trivial_class_generic!(BitmapV1_291_03_06PC, BitmapGeneric);

impl Export for BitmapV1_291_03_06PC {}
impl Import for BitmapV1_291_03_06PC {}
