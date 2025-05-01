use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::generic::{
    AnimationMaterial,
    AnimationMaterialModifier,
    AnimationMesh,
    AnimationMeshModifier,
    AnimationMorph,
    AnimationMorphModifier,
    AnimationNode,
    AnimationNodeModifier,
};
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_381_67_09PC};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[generic(complete)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
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
    AnimationV1_381_67_09PC(
        ResourceObjectLinkHeaderV1_381_67_09PC,
        AnimationBodyV1_381_67_09PC
    ),
    AnimationGeneric,
    false
);

impl Export for AnimationV1_381_67_09PC {}
impl Import for AnimationV1_381_67_09PC {}
