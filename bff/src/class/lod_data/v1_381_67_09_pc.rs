use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffOption,
    DynArray,
    ObjectDatasFlagsV1_381_67_09PC,
    ResourceObjectLinkHeader,
    Vec3f,
};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Extended {
    pad: [u8; 24],
    flags1: u32,
    zero1: u32,
    equals0x004000000: u32,
    zero2: u32,
    zero3: u32,
    zero4: u32,
    scale: Vec3f,
    zero5: u32,
    zero6: u32,
    zero7: u32,
    equals0x004000001: u32,
    zero8: u32,
    zero9: u32,
    zero10: u32,
    zero11: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct LodDataBodyV1_381_67_09PC {
    flags: ObjectDatasFlagsV1_381_67_09PC,
    mesh_data_names: DynArray<Name>,
    zero: u32,
    extended: BffOption<Extended>,
}

pub type LodDataV1_381_67_09PC = TrivialClass<ResourceObjectLinkHeader, LodDataBodyV1_381_67_09PC>;
