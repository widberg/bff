use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    DynArray, KeyframerBezierRot, KeyframerFloatComp, KeyframerMessage, KeyframerRot,
    KeyframerVec3fComp, ResourceObjectLinkHeader,
};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct AnimationNode {
    pub unknown: u16,
    pub keyframer_rot: KeyframerRot,
    pub keyframer_bezier_rot: KeyframerBezierRot,
    pub keyframer_scale: KeyframerVec3fComp,
    pub keyframer_translation: KeyframerVec3fComp,
    pub keyframer_message: KeyframerMessage,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct AnimationNodeModifier {
    pub bone_name: Name,
    pub bone_id: u16,
    pub flag: u16,
    pub translation_start_frame: u16,
    pub translation_frame_count: u16,
    pub rot_start_frame: u16,
    pub rot_frame_count: u16,
    pub bezier_start_frame: u16,
    pub bezier_frame_count: u16,
    pub scale_start_frame: u16,
    pub scale_frame_count: u16,
    pub message_start_frame: u16,
    pub message_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct AnimationMaterial {
    pub keyframer_float_comp0: KeyframerFloatComp,
    pub keyframer_float_comp1: KeyframerFloatComp,
    pub keyframer_vec3_comp0: KeyframerVec3fComp,
    pub keyframer_vec3_comp1: KeyframerVec3fComp,
    pub keyframer_float_comp2: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct AnimationMaterialModifier {
    pub material_link_name: Name,
    pub material_id: u16,
    pub flag: u16,
    pub keyframer_float_comp0_start_frame: u16,
    pub keyframer_float_comp0_frame_count: u16,
    pub keyframer_float_comp1_start_frame: u16,
    pub keyframer_float_comp1_frame_count: u16,
    pub keyframer_vec3_comp0_start_frame: u16,
    pub keyframer_vec3_comp0_frame_count: u16,
    pub keyframer_vec3_comp1_start_frame: u16,
    pub keyframer_vec3_comp1_frame_count: u16,
    pub keyframer_float_comp2_start_frame: u16,
    pub keyframer_float_comp2_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct AnimationMesh {
    pub keyframer_float_comp: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct AnimationMeshModifier {
    pub mesh_link_name: Name,
    pub mesh_id: u16,
    pub flag: u16,
    pub keyframer_float_comp_start_frame: u16,
    pub keyframer_float_comp_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct AnimationMorph {
    pub keyframer_float_comp: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct AnimationMorphModifier {
    pub mesh_link_name: Name,
    pub mesh_id: u16,
    pub flag: u16,
    pub keyframer_float_comp_start_frame: u16,
    pub keyframer_float_comp_frame_count: u16,
}

pub struct AnimationBodyGeneric {
    pub duration: f32,
    pub blending: f32,
    pub unknown: u16,
    pub animation_node: AnimationNode,
    pub animation_material: AnimationMaterial,
    pub animation_mesh: AnimationMesh,
    pub animation_morph: AnimationMorph,
    pub animation_node_modifiers: DynArray<AnimationNodeModifier>,
    pub animation_material_modifiers: DynArray<AnimationMaterialModifier>,
    pub animation_mesh_modifiers: DynArray<AnimationMeshModifier>,
    pub animation_morph_modifiers: DynArray<AnimationMorphModifier>,
}

pub type AnimationGeneric = TrivialClass<Option<ResourceObjectLinkHeader>, AnimationBodyGeneric>;
