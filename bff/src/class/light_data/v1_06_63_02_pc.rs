use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::math::{Vec3f, RGB, RGBA};

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct ObjectDatas {
    unknown: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(_link_header: &()))]
pub struct LightDataBodyV1_06_63_02PC {
    object_datas: ObjectDatas,
    direction: Vec3f,
    color: RGB,
    padding: [u8; 12],
    flag: u32,
    ambient: RGB,
}

pub type LightDataV1_06_63_02PC = TrivialClass<(), LightDataBodyV1_06_63_02PC>;
