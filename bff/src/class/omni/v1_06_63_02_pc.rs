use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{Mat4f, ObjectLinkHeaderV1_06_63_02PC, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct OmniBodyV1_06_63_02PC {
    texture_projection_matrix: Mat4f,
    color: Vec3f,
    intensity: f32,
    scaled_color: Vec3f,
    unused: f32,
    spot_angle_half_rad: f32,
    spot_angle_scale: f32,
    spot_angle_bias: f32,
    start: f32,
    end: f32,
    material_anim_name: Name,
}

pub type OmniV1_06_63_02PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, OmniBodyV1_06_63_02PC>;

impl Export for OmniV1_06_63_02PC {}
impl Import for OmniV1_06_63_02PC {}
