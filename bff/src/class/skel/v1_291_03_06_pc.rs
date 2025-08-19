use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    DynArray,
    DynBox,
    DynSphere,
    Mat4f,
    Quat,
    ResourceObjectLinkHeaderV1_06_63_02PC,
    Sphere,
    Vec3f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct ObjectDatas {
    flag: u32,
    b_sphere_local: Sphere,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct BoneNode {
    user_define_name: Name,
    local_rotation: Quat,
    scale: Vec3f,
    flags: u32,
    local_translation: Vec3f,
    placeholder_child_ptr: u32,
    model_rot_matrix_row1: Vec3f,
    model_matrix_id: i16,
    inverse_model_matrix_id: i16,
    model_rot_matrix_row2: Vec3f,
    placeholder_model_matrix_ptr: u32,
    model_rot_matrix_row3: Vec3f,
    placeholder_inverse_model_matrix_ptr: u32,
    inverse_local_rotation: Quat,
    unknown_ptrs0: [u32; 3],
    placeholder_parent_ptr: u32,
    unknown_ptrs1: [u32; 3],
    placeholder_prev_sibling_ptr: u32,
    unknown_ptrs2: [u32; 3],
    placeholder_next_sibling_ptr: u32,
    original_model_transform: Mat4f,
    child_bone_id: i32,
    parent_bone_id: i32,
    next_sibling_bone_id: i32,
    prev_sibling_bone_id: i32,
    bone_name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct BoneNodeGroup {
    bone_node_names: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SphereColBone {
    sphere_col: DynSphere,
    bone_node_name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct BoxColBone {
    box_col: DynBox,
    bone_node_name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct SkelBodyV1_291_03_06PC {
    object_datas: ObjectDatas,
    bone_nodes: DynArray<BoneNode>,
    material_names: DynArray<Name>,
    mesh_data_names: DynArray<Name>,
    bone_node_groups: DynArray<BoneNodeGroup>,
    unknown_names: DynArray<Name>,
    sphere_col_bones1: DynArray<SphereColBone>,
    sphere_col_bones2: DynArray<SphereColBone>,
    box_col_bones: DynArray<BoxColBone>,
}

pub type SkelV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, SkelBodyV1_291_03_06PC>;

impl Export for SkelV1_291_03_06PC {}
impl Import for SkelV1_291_03_06PC {}
