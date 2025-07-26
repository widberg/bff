use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{BffOption, DynArray, RGBA, ResourceObjectLinkHeaderV1_06_63_02PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct ObjectDatas {
    flags: u32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct UnkStruct {
    unk1_name: Name,
    unk2_name: Name,
    unk3_name: Name,
    unks: DynArray<u32>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct ActorData {
    placeholder1: [u32; 7],
    zero1: u32,
    placeholder2: [u32; 3],
    flag: u32,
    placeholder3: [u32; 4],
    unk_bytes1: [u8; 4],
    unk_floats1: [f32; 6],
    placeholder4: [u32; 3],
    unk_uints1: [u32; 4],
    unk_name: Name,
    unk_structs: DynArray<UnkStruct>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct LodDataBodyV1_06_63_02PC {
    object_datas: ObjectDatas,
    mesh_data_or_skelcrc32s: DynArray<Name>,
    final_skel_name: Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    actor_data: BffOption<ActorData>,
}

pub type LodDataV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, LodDataBodyV1_06_63_02PC>;

impl Export for LodDataV1_06_63_02PC {}
impl Import for LodDataV1_06_63_02PC {}
