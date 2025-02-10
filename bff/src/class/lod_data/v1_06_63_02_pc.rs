use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{BffOption, DynArray, RGBA};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ObjectDatas {
    unknown: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct UnkStruct {
    unk1_name: Name,
    unk2_name: Name,
    unk3_name: Name,
    unks: DynArray<u32>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
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

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct LodDataBodyV1_06_63_02PC {
    obj_datas: ObjectDatas,
    mesh_data_or_skelcrc32s: DynArray<Name>,
    final_skel_name: Name,
    actor_data: BffOption<ActorData>,
}

pub type LodDataV1_06_63_02PC = TrivialClass<(), LodDataBodyV1_06_63_02PC>;
