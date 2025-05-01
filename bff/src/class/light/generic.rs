use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ObjectLinkHeaderGeneric, Quat, RGBA, Vec3f};

pub struct LightBodyGeneric {
    pub rotation: Quat,
    pub direction: Vec3f,
    pub color: RGBA,
    pub ambient: Vec3f,
    pub position: Vec3f,
}

pub type LightGeneric = TrivialClass<ObjectLinkHeaderGeneric, LightBodyGeneric>;
