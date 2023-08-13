use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::math::{Mat4f, Quat, Sphere, Vec3f, RGBA};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
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

pub type NodeV1_381_67_09PC = TrivialClass<LinkHeader, NodeBodyV1_381_67_09PC>;
