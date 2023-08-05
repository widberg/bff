use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Vec2f;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct SeadEntry {
    next_object_of_entry: u32,
    prev_object_of_entry: u32,
    next_entry_of_object: u32,
    grid_id: u32,
    node_crc32: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct UnkStruct1 {
    data: [u8; 8],
}

#[derive(BinRead, Debug, Serialize)]
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

#[derive(BinRead, Debug, Serialize)]
struct SubWorldRange {
    data: [u8; 24],
    unk_structs1: DynArray<UnkStruct1>,
    unk0: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct SubWorldData {
    data: [u8; 24],
    sub_world_range: SubWorldRange,
    unknown0s: DynArray<u8>,
    unknown1s: DynArray<u32>,
    unknown2s: DynArray<u32>,
    unknown3s: DynArray<u32>,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &()))]
pub struct WorldBodyV1_06_63_02PC {
    linked_crc32s: DynArray<Name>,
    root_node_crc32: Name,
    sead_handle0: SeadHandle,
    sead_handle1: SeadHandle,
    anim_frame_crc32s: DynArray<Name>,
    camera_zone_crc32s: DynArray<Name>,
    graph_crc32s: DynArray<Name>,
    occluder_crc32s: DynArray<Name>,
    crc32s_unk4: DynArray<Name>,
    sub_world_datas: DynArray<SubWorldData>,
    warp_crc32: Name,
    game_obj_crc32: Name,
    crc32_unk5: Name,
    crc32_unk6: Name,
}

pub type WorldV1_06_63_02PC = TrivialClass<(), WorldBodyV1_06_63_02PC>;
