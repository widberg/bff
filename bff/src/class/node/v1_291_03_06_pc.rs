use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{Mat4f, Quat, RGBA, Rect, Sphere, Vec3f};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct NodeBodyV1_291_03_06PC {
    parent_name: Name,
    head_child_name: Name,
    prev_node_name: Name,
    next_node_name: Name,
    object_node_name: Name,
    user_define_name: Name,
    light_data_name: Name,
    bitmap_name: Name,
    unknown_name: Name,
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
