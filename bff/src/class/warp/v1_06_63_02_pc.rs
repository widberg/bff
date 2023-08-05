use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::Vec3f;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &()))]
pub struct WarpBodyV1_06_63_02PC {
    flag: u32,
    vertices: [Vec3f; 8],
    vec: Vec3f,
    material_anim_names: [Name; 6],
    node_name: Name,
    anim_frame_names: DynArray<Name>,
}

pub type WarpV1_06_63_02PC = TrivialClass<(), WarpBodyV1_06_63_02PC>;
