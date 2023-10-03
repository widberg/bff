use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_381_67_09PC;
use crate::math::Mat4f;
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct CollisionVolInfo {
    local_transform: Mat4f,
    local_transform_inverse: Mat4f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct CollisionVolBodyV1_381_67_09PC {
    collision_vol_info: DynArray<CollisionVolInfo>,
    in_message_id: Name,
    out_message_id: Name,
    node_name_params: [Name; 12],
    float_params: [f32; 12],
    anim_frame_names: DynArray<Name>,
    material_anim_names: DynArray<Name>,
    volume_type: Name,
    delay: f32,
}

pub type CollisionVolV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, CollisionVolBodyV1_381_67_09PC>;
