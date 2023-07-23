use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Mat4f;
use crate::name::Name;

// #[derive(BinRead, Debug)]
// struct LinkInfo {
//     data_crc32: Name,
//     b_sphere_local: Sphere,
//     unknown_matrix: Mat4f,
//     fade_out_distance: f32,
//     flags: u32,
//     collision_vol_type: u16,
// }

#[derive(BinRead, Debug, Serialize)]
struct CollisionVolInfo {
    local_transform: Mat4f,
    inv_local_transform: Mat4f,
}

#[derive(BinRead, Debug, Serialize)]
pub struct CollisionVolBodyV1_291_03_06PC {
    collision_vol_infos: DynArray<CollisionVolInfo>,
    in_message_id: u32,
    out_message_id: u32,
    #[br(count = 12)]
    node_param_crc32s: Vec<u32>,
    #[br(count = 12)]
    float_param_crc32s: Vec<f32>,
    anim_frame_crc32s: DynArray<Name>,
    collision_vol_agent_crc32: Name,
    anim_start_time: f32,
}

pub type CollisionVolV1_291_03_06PC = TrivialClass<(), CollisionVolBodyV1_291_03_06PC>;
