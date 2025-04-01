use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::helpers::until_eof;
use binrw::{BinRead, BinWrite, binrw};
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

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[generic(name(BitmapHeaderGeneric))]
pub struct LinkHeader {
    link_name: Name,
    bitmap_class: BitmapClass,
    #[generic]
    width: u32,
    #[generic]
    height: u32,
    #[generic]
    precalculated_size: u32,
    flags: u8,
    bitmap_type: u8,
    pad: u16,
    layer: f32,
    format0: BmFormat,
    #[generic]
    mipmap_count: u8,
    four: u8,
    bitmap_class2: BitmapClass2,
    format1: BmFormat,
    transparency: BmTransp,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &LinkHeader))]
pub struct BitmapBodyV1_381_67_09PC {
    #[br(parse_with = until_eof)]
    #[serde(skip_serializing)]
    #[generic]
    data: Vec<u8>,
}

trivial_class!(
    BitmapV1_381_67_09PC(LinkHeader, BitmapBodyV1_381_67_09PC),
    BitmapGeneric
);
