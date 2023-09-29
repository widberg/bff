use binrw::{BinRead, BinWrite};
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::link_header::ObjectLinkHeaderV1_381_67_09PC;
use crate::math::Mat4f;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct OmniBodyV1_381_67_09PC {
    scale_matrix: Mat4f,
    translation_matrix: Mat4f,
    trs_mat: Mat4f,
    material_anim_name0: Name,
    material_anim_name1: Name,
}

pub type OmniV1_381_67_09PC = TrivialClass<ObjectLinkHeaderV1_381_67_09PC, OmniBodyV1_381_67_09PC>;
