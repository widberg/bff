use bff_derive::ReferencedNames;
use binrw::helpers::until_eof;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[brw(repr = u16)]
enum BitmapClass {
    Single = 0,
    Cubemap = 2,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[brw(repr = u8)]
enum BmFormat {
    BmMultipleBitmaps = 0,
    BmA8l8 = 7,
    BmDxt1 = 14,
    BmDxt5 = 16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[brw(repr = u8)]
enum BitmapClass2 {
    Cubemap2 = 0,
    Single2 = 3,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[brw(repr = u8)]
enum BmTransp {
    NoTransp = 0,
    TranspOne = 1,
    Transp = 2,
    Cubemap = 255,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct LinkHeader {
    link_name: Name,
    bitmap_class: BitmapClass,
    width: u32,
    height: u32,
    bitmap_data_size: u32,
    flags: u8,
    bitmap_type: u8,
    pad: u16,
    layer: f32,
    format0: BmFormat,
    mip_map_count: u8,
    four: u8,
    bitmap_class2: BitmapClass2,
    format1: BmFormat,
    transparency: BmTransp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &LinkHeader))]
pub struct BitmapBodyV1_381_67_09PC {
    #[br(parse_with = until_eof)]
    pub data: Vec<u8>,
}

pub type BitmapV1_381_67_09PC = TrivialClass<LinkHeader, BitmapBodyV1_381_67_09PC>;
