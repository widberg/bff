use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ObjectLinkHeaderV1_381_67_09PC, Quat, RGBA, Vec3f};
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct LightBodyV1_381_67_09PC {
    rotation: Quat,
    direction: Vec3f,
    color: RGBA,
    ambient: Vec3f,
    position: Vec3f,
}

pub type LightV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, LightBodyV1_381_67_09PC>;

impl Export for LightV1_381_67_09PC {}
impl Import for LightV1_381_67_09PC {}
