use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ObjectLinkHeaderV1_06_63_02PC, Quat, RGBA, Vec3f};
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct LightBodyV1_291_03_06PC {
    rotation: Quat,
    direction: Vec3f,
    color: RGBA,
    ambient: Vec3f,
    position: Vec3f,
}

pub type LightV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, LightBodyV1_291_03_06PC>;

impl Export for LightV1_291_03_06PC {}
impl Import for LightV1_291_03_06PC {}
