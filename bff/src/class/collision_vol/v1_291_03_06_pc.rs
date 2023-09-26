use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_06_63_02PC;
use crate::math::Mat4f;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct CollisionVolInfo {
    local_transform: Mat4f,
    inv_local_transform: Mat4f,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct CollisionVolBodyV1_291_03_06PC {
    collision_vol_infos: DynArray<CollisionVolInfo>,
    in_message_id: u32,
    out_message_id: u32,
    node_param_crc32s: [u32; 12],
    float_param_crc32s: [u32; 12],
    anim_frame_crc32s: DynArray<Name>,
    collision_vol_agent_crc32: Name,
    anim_start_time: f32,
}

pub type CollisionVolV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, CollisionVolBodyV1_291_03_06PC>;
