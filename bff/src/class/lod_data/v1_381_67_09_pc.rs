use bff_derive::serialize_bits;
use bilge::prelude::{bitsize, u1, u19, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Vec3f;
use crate::name::Name;
use crate::option::BffOption;

#[derive(BinRead, Debug, Serialize)]
struct Extended {
    pad: [u8; 24],
    flags1: u32,
    zero1: u32,
    equals0x004000000: u32,
    zero2: u32,
    zero3: u32,
    zero4: u32,
    scale: Vec3f,
    zero5: u32,
    zero6: u32,
    zero7: u32,
    equals0x004000001: u32,
    zero8: u32,
    zero9: u32,
    zero10: u32,
    zero11: u32,
}

#[serialize_bits]
#[bitsize(32)]
#[derive(BinRead, DebugBits)]
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
    flags: ObjectDatasFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct LodDataBodyV1_381_67_09PC {
    mesh_data_names: DynArray<Name>,
    zero: u32,
    extended: BffOption<Extended>,
}

pub type LodDataV1_381_67_09PC = TrivialClass<LinkHeader, LodDataBodyV1_381_67_09PC>;
