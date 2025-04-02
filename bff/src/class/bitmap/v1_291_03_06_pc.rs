use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::helpers::until_eof;
use binrw::{BinRead, BinWrite, binread};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::traits::{Export, Import};

#[derive(
    BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass, Clone,
)]
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
#[br(import(_link_header: &()))]
pub struct BitmapBodyV1_291_03_06PC {
    header: BitmapHeader,
    #[br(parse_with = until_eof)]
    #[serde(skip_serializing)]
    #[generic]
    data: Vec<u8>,
}

trivial_class!(
    BitmapV1_291_03_06PC((), BitmapBodyV1_291_03_06PC),
    BitmapGeneric
);

impl Export for BitmapV1_291_03_06PC {}
impl Import for BitmapV1_291_03_06PC {}
