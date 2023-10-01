use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::keyframer::{
    KeyframerBezierRot,
    KeyframerFloatComp,
    KeyframerMessage,
    KeyframerRot,
    KeyframerVec3fComp,
};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AnimationNode {
    unknown: u16,
    keyframer_rot: KeyframerRot,
    keyframer_bezier_rot: KeyframerBezierRot,
    keyframer_scale: KeyframerVec3fComp,
    keyframer_translation: KeyframerVec3fComp,
    keyframer_message: KeyframerMessage,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AnimationNodeModifier {
    bone_name_crc32: Name,
    bone_id: u16,
    flag: u16,
    translation_start_frame: u16,
    translation_frame_count: u16,
    rot_start_frame: u16,
    rot_frame_count: u16,
    bezier_start_frame: u16,
    bezier_frame_count: u16,
    scale_start_frame: u16,
    scale_frame_count: u16,
    message_start_frame: u16,
    message_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AnimationMaterial {
    keyframer_float_comp0: KeyframerFloatComp,
    keyframer_float_comp1: KeyframerFloatComp,
    keyframer_vec3_comp0: KeyframerVec3fComp,
    keyframer_vec3_comp1: KeyframerVec3fComp,
    keyframer_float_comp2: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AnimationMaterialModifier {
    material_link_crc32: Name,
    material_id: u16,
    flag: u16,
    keyframer_float_comp0_start_frame: u16,
    keyframer_float_comp0_frame_count: u16,
    keyframer_float_comp1_start_frame: u16,
    keyframer_float_comp1_frame_count: u16,
    keyframer_vec3_comp0_start_frame: u16,
    keyframer_vec3_comp0_frame_count: u16,
    keyframer_vec3_comp1_start_frame: u16,
    keyframer_vec3_comp1_frame_count: u16,
    keyframer_float_comp2_start_frame: u16,
    keyframer_float_comp2_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AnimationMesh {
    keyframer_float_comp: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AnimationMeshModifier {
    mesh_link_crc32: Name,
    mesh_id: u16,
    flag: u16,
    keyframer_float_comp_start_frame: u16,
    keyframer_float_comp_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AnimationMorph {
    keyframer_float_comp: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AnimationMorphModifier {
    mesh_link_crc32: Name,
    mesh_id: u16,
    flag: u16,
    keyframer_float_comp_start_frame: u16,
    keyframer_float_comp_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &()))]
pub struct AnimationBodyV1_291_03_06PC {
    duration: f32,
    unknown0: f32,
    unknown1: u16,
    anim_node: AnimationNode,
    anim_material: AnimationMaterial,
    anim_mesh: AnimationMesh,
    anim_morph: AnimationMorph,
    anim_node_modifiers: DynArray<AnimationNodeModifier>,
    anim_material_modifiers: DynArray<AnimationMaterialModifier>,
    anim_mesh_modifiers: DynArray<AnimationMeshModifier>,
    anim_morph_modifiers: DynArray<AnimationMorphModifier>,
}

pub type AnimationV1_291_03_06PC = TrivialClass<(), AnimationBodyV1_291_03_06PC>;
