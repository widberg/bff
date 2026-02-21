use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_381_67_09PC, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct WarpBodyV1_381_67_09PC {
    flag: u32,
    vertices: [Vec3f; 8],
    vec: Vec3f,
    material_anim_names: [Name; 6],
    node_name: Name,
    anim_frame_names: DynArray<Name>,
}

pub type WarpV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, WarpBodyV1_381_67_09PC>;

impl Export for WarpV1_381_67_09PC {}
impl Import for WarpV1_381_67_09PC {}
