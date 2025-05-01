use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    KeyframerFlag,
    KeyframerFloatLinearComp,
    KeyframerHdl,
    KeyframerVec2fLinear,
    KeyframerVec3fLinear,
    KeyframerVec4fLinear,
    ResourceObjectLinkHeaderV1_381_67_09PC,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[bitsize(8)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames)]
struct MaterialAnimFlags {
    fl_mat_play: u1,
    fl_mat_played: u1,
    fl_mat_playonce: u1,
    fl_mat_neveragain: u1,
    fl_mat_autostart: u1,
    flag_5: u1,
    flag_6: u1,
    flag_7: u1,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct MaterialAnimBodyV1_381_67_09PC {
    bitmap_name_keyframer: KeyframerHdl,
    scroll_keyframer: KeyframerVec2fLinear,
    scale_keyframer: KeyframerVec2fLinear,
    rotation_keyframer: KeyframerFloatLinearComp,
    diffuse_keyframer: KeyframerVec3fLinear,
    emission_keyframer: KeyframerVec3fLinear,
    alpha_keyframer: KeyframerFloatLinearComp,
    vec4f_keyframer0: KeyframerVec4fLinear,
    params_keyframer: KeyframerVec4fLinear,
    render_flag_keyframer: KeyframerFlag,
    resource_flag_keyframer: KeyframerFlag,
    base_material_name: Name,
    duration: f32,
    flags: MaterialAnimFlags,
}

pub type MaterialAnimV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, MaterialAnimBodyV1_381_67_09PC>;

impl Export for MaterialAnimV1_381_67_09PC {}
impl Import for MaterialAnimV1_381_67_09PC {}
