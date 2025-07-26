use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_06_63_02PC, Vec2f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SeadEntry {
    next_resource_of_entry: u32,
    prev_resource_of_entry: u32,
    next_entry_of_resource: u32,
    grid_id: u32,
    node_name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct UnkStruct1 {
    data: [u8; 8],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SeadHandle {
    p_min: Vec2f,
    p_max: Vec2f,
    inv_diag: Vec2f,
    size: (u32, u32),
    first_free: u32,
    free_count: u32,
    grid: DynArray<u32>,
    sead_entries: DynArray<SeadEntry>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SubWorldRange {
    data: [u8; 24],
    unk_structs1: DynArray<UnkStruct1>,
    unk0: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SubWorldData {
    data: [u8; 24],
    sub_world_range: SubWorldRange,
    unknown0s: DynArray<u8>,
    unknown1s: DynArray<u32>,
    unknown2s: DynArray<u32>,
    unknown3s: DynArray<u32>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct WorldBodyV1_06_63_02PC {
    linked_names: DynArray<Name>,
    root_node_name: Name,
    sead_handle0: SeadHandle,
    sead_handle1: SeadHandle,
    anim_frame_names: DynArray<Name>,
    camera_zone_names: DynArray<Name>,
    graph_names: DynArray<Name>,
    occluder_names: DynArray<Name>,
    crc32s_unk4: DynArray<Name>,
    sub_world_datas: DynArray<SubWorldData>,
    warp_name: Name,
    game_obj_name: Name,
    crc32_unk5: Name,
    crc32_unk6: Name,
}

pub type WorldV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, WorldBodyV1_06_63_02PC>;

impl Export for WorldV1_06_63_02PC {}
impl Import for WorldV1_06_63_02PC {}
