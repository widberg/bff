use binrw::BinRead;
use serde::Serialize;
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct RotFrame {
    time: f32,
    rot: [i16; 4],
}

#[derive(BinRead, Debug, Serialize)]
struct KeyframerRot {
    rot_frames: DynArray<RotFrame>,
}

#[derive(BinRead, Debug, Serialize)]
struct BezierRotFrame {
    time: f32,
    #[serde(with = "BigArray")]
    data: [u8; 36],
}

#[derive(BinRead, Debug, Serialize)]
struct KeyframerBezierRot {
    bezier_rot_frames: DynArray<BezierRotFrame>,
}

#[derive(BinRead, Debug, Serialize)]
struct Vec3CompFrame {
    time: f32,
    i16s: [i16; 10],
}

#[derive(BinRead, Debug, Serialize)]
struct KeyframerVec3Comp {
    unknown: u16,
    vec3_comp_frames: DynArray<Vec3CompFrame>,
}

#[derive(BinRead, Debug, Serialize)]
struct FloatCompFrame {
    time: f32,
    i16s: [i16; 4],
}

#[derive(BinRead, Debug, Serialize)]
struct KeyframerFloatComp {
    unknown: u16,
    float_comp_frames: DynArray<FloatCompFrame>,
}

#[derive(BinRead, Debug, Serialize)]
struct Message {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: f32,
    crc32: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct KeyMessage {
    time: f32,
    messages: DynArray<Message>,
}

#[derive(BinRead, Debug, Serialize)]
struct KeyframerMessage {
    key_messages: DynArray<KeyMessage>,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationNode {
    unknown: u16,
    keyframer_rot: KeyframerRot,
    keyframer_bezier_rot: KeyframerBezierRot,
    keyframer_scale: KeyframerVec3Comp,
    keyframer_translation: KeyframerVec3Comp,
    keyframer_message: KeyframerMessage,
}

#[derive(BinRead, Debug, Serialize)]
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

#[derive(BinRead, Debug, Serialize)]
struct AnimationMaterial {
    keyframer_float_comp0: KeyframerFloatComp,
    keyframer_float_comp1: KeyframerFloatComp,
    keyframer_vec3_comp0: KeyframerVec3Comp,
    keyframer_vec3_comp1: KeyframerVec3Comp,
    keyframer_float_comp2: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize)]
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

#[derive(BinRead, Debug, Serialize)]
struct AnimationMesh {
    keyframer_float_comp: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationMeshModifier {
    mesh_link_crc32: Name,
    mesh_id: u16,
    flag: u16,
    keyframer_float_comp_start_frame: u16,
    keyframer_float_comp_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationMorph {
    keyframer_float_comp: KeyframerFloatComp,
}

#[derive(BinRead, Debug, Serialize)]
struct AnimationMorphModifier {
    mesh_link_crc32: Name,
    mesh_id: u16,
    flag: u16,
    keyframer_float_comp_start_frame: u16,
    keyframer_float_comp_frame_count: u16,
}

#[derive(BinRead, Debug, Serialize)]
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
