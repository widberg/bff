use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_06_63_02PC, Vec2f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct SeadEntry {
    next_resource_of_entry: u32,
    prev_resource_of_entry: u32,
    next_entry_of_resource: u32,
    grid_id: u32,
    node_name: Name,
}

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames)]
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

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unknown0 {
    data: [u8; 8],
}

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct SubWorldRange {
    data: [u8; 24],
    unknown0s: DynArray<Unknown0>,
    unknown1: u32,
}

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct SubWorldData {
    data: [u8; 24],
    sub_world_range: SubWorldRange,
    unknown0s: DynArray<u8>,
    unknown1s: DynArray<u32>,
    unknown2s: DynArray<u32>,
    unknown3s: DynArray<u32>,
}

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct WorldBodyV1_291_03_06PC {
    root_node_name: Name,
    warp_name: Name,
    game_obj_name: Name,
    unk0_name: Name,
    unk1_name: Name,
    links: DynArray<Name>,
    sead_handle0: SeadHandle,
    sead_handle1: SeadHandle,
    anim_frame_names: DynArray<Name>,
    camera_zone_names: DynArray<Name>,
    graph_names: DynArray<Name>,
    occluder_names: DynArray<Name>,
    unk2_names: DynArray<Name>,
    sub_world_datas: DynArray<SubWorldData>,
}

pub type WorldV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, WorldBodyV1_291_03_06PC>;

impl Export for WorldV1_291_03_06PC {}
impl Import for WorldV1_291_03_06PC {}
