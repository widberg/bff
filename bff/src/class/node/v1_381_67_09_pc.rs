use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    Mat4f,
    Quat,
    RGBA,
    Rect,
    ResourceObjectLinkHeaderV1_381_67_09PC,
    Sphere,
    Vec3f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct NodeBodyV1_381_67_09PC {
    parent_name: Name,
    head_child_name: Name,
    prev_sibling: Name,
    next_sibling: Name,
    lod_or_particles_name: Name,
    lod_data_or_particles_data_name: Name,
    user_define_name: Name,
    light_data_name: Name,
    bitmap_name: Name,
    unused_name2: Name,
    rotation: Quat,
    translation: Vec3f,
    flags: u32,
    rotation2: Quat,
    scale: f32,
    scale2: f32,
    reciprocal_scale2: f32,
    unknown10: f32,
    color: RGBA,
    sphere: Sphere,
    display_seads_rect: Rect,
    collide_seads_rect: Rect,
    negative_four: i16,
    world_transform_mat4: Mat4f,
}

pub type NodeV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, NodeBodyV1_381_67_09PC>;

impl Export for NodeV1_381_67_09PC {}
impl Import for NodeV1_381_67_09PC {}
