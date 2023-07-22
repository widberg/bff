use bff_derive::{bff_forms, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::CollisionVolV1_291_03_06PC;

use crate::{
    dynarray::DynArray,
    math::{Mat4f, Sphere},
    name::Name,
};

#[derive(Serialize, Debug, NamedClass)]
struct LinkInfo {
    data_crc32: Name,
    b_sphere_local: Sphere,
    unknown_matrix: Mat4f,
    fade_out_distance: f32,
    flags: u32,
    collision_vol_type: u16,
}

#[derive(Serialize, Debug, NamedClass)]
struct CollisionVolInfo {
    local_transform: Mat4f,
    inv_local_transform: Mat4f,
}

#[derive(Serialize, Debug, NamedClass)]
#[bff_forms((V1_291_03_06, PC) | (V1_291_03_01, PSP) | (V1_06_63_02, PC) => CollisionVolV1_291_03_06PC)]
pub struct CollisionVol {
    collision_vol_infos: Vec<CollisionVolInfo>,
    in_message_id: u32,
    out_message_id: u32,
    node_param_crc32s: Vec<u32>,
    float_param_crc32s: Vec<f32>,
    anim_frame_crc32s: DynArray<Name>,
    collision_vol_agent_crc32: Name,
    anim_start_time: f32,
}
