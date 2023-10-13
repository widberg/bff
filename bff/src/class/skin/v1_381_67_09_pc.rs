use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ObjectLinkHeaderV1_381_67_09PC};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(bone_name_count: u32))]
struct SkinSubsection {
    animation_node_names: [Name; 4],
    #[br(count = bone_name_count)]
    bone_names: Vec<Name>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(bone_name_count: u32))]
struct SkinSection {
    #[br(args(bone_name_count))]
    skin_subsections: DynArray<SkinSubsection>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct SkinBodyV1_381_67_09PC {
    mesh_names: DynArray<Name>,
    zeros: [u32; 4],
    one_and_a_half: f32,
    bone_name_count: u32,
    #[br(args(bone_name_count))]
    skin_sections: DynArray<SkinSection>,
}

pub type SkinV1_381_67_09PC = TrivialClass<ObjectLinkHeaderV1_381_67_09PC, SkinBodyV1_381_67_09PC>;
