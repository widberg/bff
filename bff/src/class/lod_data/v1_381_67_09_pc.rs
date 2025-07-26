use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffOption,
    DynArray,
    ObjectDatasFlagsV1_381_67_09PC,
    ResourceObjectLinkHeaderV1_381_67_09PC,
    Vec3f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
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

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct LodDataBodyV1_381_67_09PC {
    flags: ObjectDatasFlagsV1_381_67_09PC,
    mesh_data_names: DynArray<Name>,
    zero: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    extended: BffOption<Extended>,
}

pub type LodDataV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, LodDataBodyV1_381_67_09PC>;

impl Export for LodDataV1_381_67_09PC {}
impl Import for LodDataV1_381_67_09PC {}
