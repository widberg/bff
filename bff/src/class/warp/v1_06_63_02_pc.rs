use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_06_63_02PC, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct WarpBodyV1_06_63_02PC {
    flag: u32,
    vertices: [Vec3f; 8],
    vec: Vec3f,
    material_anim_names: [Name; 6],
    node_name: Name,
    anim_frame_names: DynArray<Name>,
}

pub type WarpV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, WarpBodyV1_06_63_02PC>;

impl Export for WarpV1_06_63_02PC {}
impl Import for WarpV1_06_63_02PC {}
