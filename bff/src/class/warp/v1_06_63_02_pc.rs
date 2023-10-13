use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeader, Vec3f};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct WarpBodyV1_06_63_02PC {
    flag: u32,
    vertices: [Vec3f; 8],
    vec: Vec3f,
    material_anim_names: [Name; 6],
    node_name: Name,
    anim_frame_names: DynArray<Name>,
}

pub type WarpV1_06_63_02PC = TrivialClass<ResourceObjectLinkHeader, WarpBodyV1_06_63_02PC>;
