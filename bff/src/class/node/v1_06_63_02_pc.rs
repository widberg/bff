use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{Mat4f, Quat, Rect, Sphere, Vec3f, RGBA};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct NodeBodyV1_06_63_02PC {
    parent_name: Name,
    head_child_name: Name,
    prev_node_name: Name,
    next_node_name: Name,
    object_name: Name,
    user_define_name: Name,
    bitmap_name: Name,
    unk_name: Name,
    inverse_world_transform: Mat4f,
    unk_vec3f: Vec3f,
    collide_seads_id1: u32,
    unk_vec3f2: Vec3f,
    placeholder_world_matrix_ptr: u32,
    unk_vec3f3: Vec3f,
    display_seads_id1: u32,
    unk_mat: Mat4f,
    translation: Vec3f,
    flags: u32,
    rotation: Quat,
    scale: f32,
    other_scale: f32,
    one_over_scale: f32,
    unk_float1: f32,
    colors: RGBA,
    b_sphere: Sphere,
    display_seads_rect: Rect,
    collide_seads_rect: Rect,
    world_transform: Mat4f,
    collide_seads_id2: u32,
    display_seads_id2: u32,
    unknown4: u16,
    unknown5: u32,
    unknown6: u32,
}

pub type NodeV1_06_63_02PC = TrivialClass<(), NodeBodyV1_06_63_02PC>;
