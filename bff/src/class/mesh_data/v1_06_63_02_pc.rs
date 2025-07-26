use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, RGBA, ResourceObjectLinkHeaderV1_06_63_02PC};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct ObjectDatas {
    unknown: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct UnkStruct1 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct UnkStruct2 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct UnkStruct3 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct UnkStruct4 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct MeshVolume {
    unk_struct1s: DynArray<UnkStruct1>,
    unk_struct2s: DynArray<UnkStruct2>,
    unk_struct3s: DynArray<UnkStruct3>,
    unk_struct4s: DynArray<UnkStruct4>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct MeshDataBodyV1_06_63_02PC {
    object_datas: ObjectDatas,
    mesh_volume: MeshVolume,
}

pub type MeshDataV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, MeshDataBodyV1_06_63_02PC>;

impl Export for MeshDataV1_06_63_02PC {}
impl Import for MeshDataV1_06_63_02PC {}
