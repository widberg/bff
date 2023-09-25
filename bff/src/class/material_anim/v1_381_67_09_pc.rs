use bff_derive::serialize_bits;
use bilge::prelude::{bitsize, u1, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::name::Name;
use crate::keyframer::{KeyframerFlag, KeyframerFloatLinearComp, KeyframerHdl, KeyframerVec2fLinear, KeyframerVec3fLinear, KeyframerVec4fLinear};

#[serialize_bits]
#[bitsize(8)]
#[derive(BinRead, DebugBits)]
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

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
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
    object_flag_keyframer: KeyframerFlag,
    base_material_name: Name,
    duration: f32,
    flags: MaterialAnimFlags,
}

pub type MaterialAnimV1_381_67_09PC = TrivialClass<LinkHeader, MaterialAnimBodyV1_381_67_09PC>;
