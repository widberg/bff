use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{BffOption, DynArray, ResourceObjectLinkHeaderV1_06_63_02PC, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
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

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct LodDataBodyV1_291_03_06PC {
    flags: u32,
    mesh_data_names: DynArray<Name>,
    zero: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    extended: BffOption<Extended>,
}

pub type LodDataV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, LodDataBodyV1_291_03_06PC>;

impl Export for LodDataV1_291_03_06PC {}
impl Import for LodDataV1_291_03_06PC {}
