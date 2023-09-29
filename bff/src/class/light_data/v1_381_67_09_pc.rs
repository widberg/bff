use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::link_header::{ObjectDatasFlagsV1_381_67_09PC, ResourceObjectLinkHeader};
use crate::math::{Vec3, Vec3f};

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct LightDataBodyV1_381_67_09PC {
    object_datas_flags: ObjectDatasFlagsV1_381_67_09PC,
    facing: Vec3f,
    local_collision_sphere: Vec3f,
    unused_vec: Vec3<i32>,
    flags: u32,
    local_collision_sphere_facing: Vec3f,
}

pub type LightDataV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeader, LightDataBodyV1_381_67_09PC>;
