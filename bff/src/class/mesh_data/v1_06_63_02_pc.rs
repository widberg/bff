use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::RGBA;

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct ObjectDatas {
    unknown: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct UnkStruct1 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct UnkStruct2 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct UnkStruct3 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct UnkStruct4 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct MeshVolume {
    unk_struct1s: DynArray<UnkStruct1>,
    unk_struct2s: DynArray<UnkStruct2>,
    unk_struct3s: DynArray<UnkStruct3>,
    unk_struct4s: DynArray<UnkStruct4>,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(_link_header: &()))]
pub struct MeshDataBodyV1_06_63_02PC {
    object_datas: ObjectDatas,
    mesh_volume: MeshVolume,
}

pub type MeshDataV1_06_63_02PC = TrivialClass<(), MeshDataBodyV1_06_63_02PC>;
