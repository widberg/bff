use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, RGBA};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ResourceDatas {
    unknown: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct UnkStruct1 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct UnkStruct2 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct UnkStruct3 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct UnkStruct4 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct MeshVolume {
    unk_struct1s: DynArray<UnkStruct1>,
    unk_struct2s: DynArray<UnkStruct2>,
    unk_struct3s: DynArray<UnkStruct3>,
    unk_struct4s: DynArray<UnkStruct4>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct MeshDataBodyV1_06_63_02PC {
    resource_datas: ResourceDatas,
    mesh_volume: MeshVolume,
}

pub type MeshDataV1_06_63_02PC = TrivialClass<(), MeshDataBodyV1_06_63_02PC>;

impl Export for MeshDataV1_06_63_02PC {}
impl Import for MeshDataV1_06_63_02PC {}
