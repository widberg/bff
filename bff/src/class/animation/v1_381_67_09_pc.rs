use super::shared::{
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
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_381_67_09PC};
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct AnimationBodyV1_381_67_09PC {
    duration: f32,
    blending: f32,
    unknown: u16,
    animation_node: AnimationNode,
    animation_material: AnimationMaterial,
    animation_mesh: AnimationMesh,
    animation_morph: AnimationMorph,
    animation_node_modifiers: DynArray<AnimationNodeModifier>,
    animation_material_modifiers: DynArray<AnimationMaterialModifier>,
    animation_mesh_modifiers: DynArray<AnimationMeshModifier>,
    animation_morph_modifiers: DynArray<AnimationMorphModifier>,
}

pub type AnimationV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, AnimationBodyV1_381_67_09PC>;

impl Export for AnimationV1_381_67_09PC {}
impl Import for AnimationV1_381_67_09PC {}
