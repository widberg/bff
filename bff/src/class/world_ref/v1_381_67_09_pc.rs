use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_381_67_09PC;
use crate::math::{Mat4f, Vec3f};
use crate::names::Name;
use crate::strings::PascalStringNull;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct UUIDPair {
    uuid0: u32,
    uuid1: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct WorldRefBodyV1_381_67_09PC {
    node_name0: Name,
    warp_name: Name,
    game_obj_name: Name,
    unused14: Name,
    gen_world_name: Name,
    node_name1: Name,
    unused17s: DynArray<u32>,
    unuseds: DynArray<u8>,
    mats: DynArray<Mat4f>,
    point_a: Vec3f,
    point_b: Vec3f,
    uuid_pairs: DynArray<UUIDPair>,
    init_script: PascalStringNull,
    node_name2: DynArray<Name>,
    zero: u32,
}

pub type WorldRefV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, WorldRefBodyV1_381_67_09PC>;
