use binrw::{until_eof, BinRead};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u16)]
enum BitmapClass {
    SINGLE = 0,
    CUBEMAP = 2,
}

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u8)]
enum BmFormat {
    BmMultipleBitmaps = 0,
    BmA8l8 = 7,
    BmDxt1 = 14,
    BmDxt5 = 16,
}

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u8)]
enum BitmapClass2 {
    CUBEMAP2 = 0,
    SINGLE2 = 3,
}

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u8)]
enum BmTransp {
    BmNoTransp = 0,
    BmTranspOne = 1,
    BmTransp = 2,
    BmCubemap = 255,
}

#[derive(BinRead, Debug, Serialize)]
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

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct BitmapBodyV1_381_67_09PC {
    #[br(parse_with = until_eof)]
    data: Vec<u8>,
}

pub type BitmapV1_381_67_09PC = TrivialClass<LinkHeader, BitmapBodyV1_381_67_09PC>;
