use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    Mat3x4f,
    RGB,
    RGBA,
    ResourceObjectLinkHeaderV1_06_63_02PC,
    Vec2f,
    Vec3f,
    Vec4f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct MaterialBodyV1_06_63_02PC {
    diffuse: RGBA,
    emission: RGB,
    cdcdcdcd: u32,
    uv_transform_matrix: Mat3x4f,
    specular: Vec3f,
    specular_pow: f32,
    params: Vec4f,
    rotation: f32,
    translation: Vec2f,
    scale: Vec2f,
    collision_flag: u32,
    render_flag: u32,
    object_flag: u32,
    general_flag: u8,
    textures: [Name; 4],
}

pub type MaterialV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, MaterialBodyV1_06_63_02PC>;

impl Export for MaterialV1_06_63_02PC {}
impl Import for MaterialV1_06_63_02PC {}
