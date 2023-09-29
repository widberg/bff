use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::math::RGBA;

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct ObjectDatas {
    unknown_float: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(_link_header: &()))]
pub struct MeshDataBodyV1_291_03_06PC {
    object_datas: ObjectDatas,
}

pub type MeshDataV1_291_03_06PC = TrivialClass<(), MeshDataBodyV1_291_03_06PC>;
