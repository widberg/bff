use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Vec3f;
use crate::names::Name;
use crate::option::BffOption;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct Extended {
    padding: [u8; 24],
    flags: u32,
    zero0: u32,
    unknown0: u32,
    zero1s: [u32; 3],
    scale: Vec3f,
    zero2s: [u32; 3],
    unknown1: u32,
    zero3s: [u32; 4],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &()))]
pub struct LodDataBodyV1_291_03_06PC {
    flags: u32,
    mesh_data_crc32s: DynArray<Name>,
    zero: u32,
    extended: BffOption<Extended>,
}

pub type LodDataV1_291_03_06PC = TrivialClass<(), LodDataBodyV1_291_03_06PC>;
