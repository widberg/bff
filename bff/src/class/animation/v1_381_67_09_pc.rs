use bff_derive::{trivial_class, GenericClass, ReferencedNames};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeader};

use super::generic::{
    AnimationMaterial, AnimationMaterialModifier, AnimationMesh, AnimationMeshModifier,
    AnimationMorph, AnimationMorphModifier, AnimationNode, AnimationNodeModifier,
};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[generic(complete)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct AnimationBodyV1_381_67_09PC {
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

trivial_class!(
    AnimationV1_381_67_09PC(ResourceObjectLinkHeader, AnimationBodyV1_381_67_09PC),
    AnimationGeneric,
    false
);
