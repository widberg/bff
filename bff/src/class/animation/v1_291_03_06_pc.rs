use bff_derive::{GenericClass, ReferencedNames};
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::generic::{
    AnimationGeneric,
    AnimationMaterial,
    AnimationMaterialModifier,
    AnimationMesh,
    AnimationMeshModifier,
    AnimationMorph,
    AnimationMorphModifier,
    AnimationNode,
    AnimationNodeModifier,
};
use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_06_63_02PC};
use crate::macros::trivial_class_generic::trivial_class_generic;
use crate::traits::{Export, Import};

#[derive(
    BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames, GenericClass,
)]
#[generic(complete)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct AnimationBodyV1_291_03_06PC {
    duration: f32,
    blending: f32,
    unknown: u16,
    #[generic(no_convert)]
    animation_node: AnimationNode,
    #[generic(no_convert)]
    animation_material: AnimationMaterial,
    #[generic(no_convert)]
    animation_mesh: AnimationMesh,
    #[generic(no_convert)]
    animation_morph: AnimationMorph,
    #[generic(no_convert)]
    animation_node_modifiers: DynArray<AnimationNodeModifier>,
    #[generic(no_convert)]
    animation_material_modifiers: DynArray<AnimationMaterialModifier>,
    #[generic(no_convert)]
    animation_mesh_modifiers: DynArray<AnimationMeshModifier>,
    #[generic(no_convert)]
    animation_morph_modifiers: DynArray<AnimationMorphModifier>,
}

pub type AnimationV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, AnimationBodyV1_291_03_06PC>;

trivial_class_generic!(AnimationV1_291_03_06PC, AnimationGeneric);

impl Export for AnimationV1_291_03_06PC {}
impl Import for AnimationV1_291_03_06PC {}
