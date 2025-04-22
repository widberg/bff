use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ResourceDatasFlagsV1_381_67_09PC, ResourceLinkHeader, Vec3, Vec3f};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceLinkHeader))]
pub struct LightDataBodyV1_381_67_09PC {
    resource_datas_flags: ResourceDatasFlagsV1_381_67_09PC,
    facing: Vec3f,
    local_collision_sphere: Vec3f,
    unused_vec: Vec3<i32>,
    flags: u32,
    local_collision_sphere_facing: Vec3f,
}

pub type LightDataV1_381_67_09PC = TrivialClass<ResourceLinkHeader, LightDataBodyV1_381_67_09PC>;

impl Export for LightDataV1_381_67_09PC {}
impl Import for LightDataV1_381_67_09PC {}
