use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, Mat4f, ResourceLinkHeaderV1_381_67_09PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct CollisionVolInfo {
    local_transform: Mat4f,
    local_transform_inverse: Mat4f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceLinkHeaderV1_381_67_09PC))]
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
    TrivialClass<ResourceLinkHeaderV1_381_67_09PC, CollisionVolBodyV1_381_67_09PC>;

impl Export for CollisionVolV1_381_67_09PC {}
impl Import for CollisionVolV1_381_67_09PC {}
