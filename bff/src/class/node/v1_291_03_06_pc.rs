use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::math::{Mat4f, Quat, Rect, Sphere, Vec3f, RGBA};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
pub struct NodeBodyV1_291_03_06PC {
    parent_crc32: Name,
    head_child_crc32: Name,
    prev_node_crc32: Name,
    next_node_crc32: Name,
    object_node_crc32: Name,
    user_define_crc32: Name,
    light_data_crc32: Name,
    bitmap_crc32: Name,
    unknown_crc32: Name,
    inverse_world_transform: Mat4f,
    unknown_vec3f1: Vec3f,
    collide_seads_id1: Name,
    unknown_vec3f2: Vec3f,
    placeholder_world_matrix_ptr: u32,
    display_seads_id1: Name,
    unknown_matrix: Mat4f,
    translation: Vec3f,
    flags: u32,
    rotation: Quat,
    scale: f32,
    other_scale: f32,
    one_over_scale: f32,
    unknown_float1: f32,
    color: RGBA,
    b_sphere: Sphere,
    display_seads_rect: Rect,
    collide_seads_rect: Rect,
    world_transform: Mat4f,
    collide_seads_id2: Name,
    display_seads_id2: Name,
    unknown4: u16,
    unknown5: u16,
    unknown6: u16,
}

pub type NodeV1_291_03_06PC = TrivialClass<(), NodeBodyV1_291_03_06PC>;
