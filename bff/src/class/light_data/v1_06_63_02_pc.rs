use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{RGB, RGBA, Vec3f};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ObjectDatas {
    unknown: f32,
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
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
