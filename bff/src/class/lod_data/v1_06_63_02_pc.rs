use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat4f, Quat, Sphere};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct ObjectDatas {
    unknown: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize)]
struct UnkStruct {
    unk1_crc32: Name,
    unk2_crc32: Name,
    unk3_crc32: Name,
    unks: DynArray<u32>,
}

#[derive(BinRead, Debug, Serialize)]
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
    unk_crc32: Name,
    unk_structs: DynArray<UnkStruct>,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkInfo))]
struct LodDataBodyV1_06_63_02PC {
    obj_datas: ObjectDatas,
    mesh_data_or_skelcrc32s: DynArray<Name>,
    final_skel_crc32: Name,
    actor_data: BffOption<ActorData>,
}

pub type LodDataV1_06_63_02PC = TrivialClass<LinkInfo, LodDataBodyV1_06_63_02PC>;
