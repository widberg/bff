use bilge::prelude::{bitsize, u1, u19, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
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
pub struct ObjectDatas {
    link_name: Name,
    flags: ObjectDatasFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectDatas))]
pub struct MeshDataBodyV1_381_67_09PC {
    zero0: u32,
    zero1: u32,
    zero2: u32,
    zero3: u32,
}

pub type MeshDataV1_381_67_09PC = TrivialClass<ObjectDatas, MeshDataBodyV1_381_67_09PC>;
