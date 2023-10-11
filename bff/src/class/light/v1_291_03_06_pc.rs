use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::link_header::ObjectLinkHeaderV1_06_63_02PC;
use crate::math::{Quat, Vec3f, RGBA};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct LightBodyV1_291_03_06PC {
    rotation: Quat,
    direction: Vec3f,
    color: RGBA,
    ambient: Vec3f,
    position: Vec3f,
}

pub type LightV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, LightBodyV1_291_03_06PC>;
