use bff_derive::{NamedClass, bff_forms};
use serde::Serialize;

mod v1_291_03_06_pc;
use v1_291_03_06_pc::BitmapV1_291_03_06PC;

#[derive(Serialize, Debug, NamedClass)]
#[bff_forms((V1_291_03_06, PC) => BitmapV1_291_03_06PC)]
pub struct Bitmap {
    size: (u32, u32),
    flag: u16,
    format: u8,
    mipmap_count: u8,
    unknown: u8,
    data: Vec<u8>,
}
