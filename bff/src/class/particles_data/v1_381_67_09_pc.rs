use bilge::prelude::{bitsize, u1, u19, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Vec3f;
use crate::name::Name;

#[bitsize(32)]
#[derive(BinRead, DebugBits, Serialize)]
struct ObjectDatasFlags {
    fl_objectdatas_hide: u1,
    fl_objectdatas_code_control: u1,
    fl_objectdatas_cloned: u1,
    fl_objectdatas_skinned: u1,
    fl_objectdatas_morphed: u1,
    fl_objectdatas_vreflect: u1,
    fl_objectdatas_hide_shadow: u1,
    fl_objectdatas_static_shadow: u1,
    fl_objectdatas_vp0_hide: u1,
    fl_objectdatas_vp1_hide: u1,
    fl_objectdatas_vp2_hide: u1,
    fl_objectdatas_vp3_hide: u1,
    fl_objectdatas_last: u1,
    padding: u19,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct FadeDistances {
    x: f32,
    y: f32,
    fade_close: f32,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct ParticlesDataBodyV1_381_67_09PC {
    flags: ObjectDatasFlags,
    position: Vec3f,
    fade: FadeDistances,
    shorts: DynArray<u16>,
    zero: u32,
}

pub type ParticlesDataV1_381_67_09PC = TrivialClass<LinkHeader, ParticlesDataBodyV1_381_67_09PC>;
