use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Vec2f;
use crate::name::Name;

#[derive(Debug, BinRead, Serialize)]
struct SeadEntry {
    next_object_of_entry: u32, //name?
    prev_object_of_entry: u32, //name?
    next_entry_of_object: u32,
    grid_id: u32,
    node_crc32: Name,
}

#[derive(Debug, BinRead, Serialize)]
struct SeadHandle {
    p_min: Vec2f,
    p_max: Vec2f,
    ind_diag: Vec2f,
    size: (u32, u32),
    first_free: u32,
    free_count: u32,
    grid: DynArray<u32>,
    sead_entries: DynArray<SeadEntry>,
}

#[derive(Debug, BinRead, Serialize)]
struct Unknown0 {
    data: [u8; 8],
}

#[derive(Debug, BinRead, Serialize)]
struct SubWorldRange {
    data: [u8; 24],
    unknown0s: DynArray<Unknown0>,
    unknown1: u32,
}

#[derive(Debug, BinRead, Serialize)]
struct SubWorldData {
    data: [u8; 24],
    sub_world_range: SubWorldRange,
    unknown0s: DynArray<u8>,
    unknown1s: DynArray<u32>,
    unknown2s: DynArray<u32>,
    unknown3s: DynArray<u32>,
}

#[derive(Debug, BinRead, Serialize)]
#[br(import(_link_header: &()))]
pub struct WorldBodyV1_291_03_06PC {
    root_node_crc32: Name,
    warp_crc32: Name,
    game_obj_crc32: Name,
    unk0_crc32: Name,
    unk1_crc32: Name,
    links: DynArray<Name>,
    sead_handle0: SeadHandle,
    sead_handle1: SeadHandle,
    anim_frame_crc32: DynArray<Name>,
    camera_zone_crc32: DynArray<Name>,
    graph_crc32: DynArray<Name>,
    occluder_crc32: DynArray<Name>,
    unk2_crc32: DynArray<Name>,
    sub_world_datas: DynArray<SubWorldData>,
}

pub type WorldV1_291_03_06PC = TrivialClass<(), WorldBodyV1_291_03_06PC>;
