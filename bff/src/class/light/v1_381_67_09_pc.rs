use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::link_header::ObjectLinkHeaderV1_381_67_09PC;
use crate::math::{Quat, Vec3f, Vec4f};

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct LightBodyV1_381_67_09PC {
    rotation: Quat,
    direction: Vec3f,
    color: Vec4f,
    ambient: Vec3f,
    position: Vec3f,
}

pub type LightV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, LightBodyV1_381_67_09PC>;
