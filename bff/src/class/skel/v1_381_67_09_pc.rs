use bilge::prelude::{bitsize, u1, u19, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat4f, Quat, Sphere, Vec3, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Bone {
    user_define_name: Name,
    transform_rotation_inverse0: Quat,
    transform_scale: Vec3f,
    bone_flags: u32,
    transform_row3: Vec3f,
    child_bone_begin: u32,
    transform_row0: Vec3f,
    some_placeholder0: u16,
    some_placeholder1: u16,
    transform_row1: Vec3f,
    some_mat_ptr1_placeholder: u32,
    transform_row2: Vec3f,
    some_mat_ptr2_placeholder: u32,
    transform_rotation_inverse1: Quat,
    placeholder_vec0: Vec3<i32>,
    parent_bone_ptr_placeholder: u32,
    placeholder_vec1: Vec3<i32>,
    some_bone_ptr_placeholder: u32,
    placeholder_vec2: Vec3<i32>,
    child_bone_ptr_placeholder: u32,
    transformation: Mat4f,
    parent_index: i32,
    child_bones_index0: i32,
    child_bones_index1: i32,
    some_bone_index: i32,
    bone_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct SphereColBone {
    sphere: Sphere,
    names: [Name; 3],
}

#[derive(BinRead, Debug, Serialize)]
struct BoxColBone {
    mat: Mat4f,
    names: [Name; 3],
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, Serialize)]
struct ObjectDatasFlags {
    fl_objectdatas_hide: u1,
    fl_objectdatas_code_control: u1,
    fl_objectdatas_cloned: u1,
    fl_objectdatas_skinned: u1,
    fl_objectdatas_morphed: u1,
    fl_objectdatas_vreflect: u1,
    fl_objectdatas_hide_shadow: u1,
    fl_objectdatas_static_shadow: u1,
    fl_objectdatas_vp0_hide: u1,
    fl_objectdatas_vp1_hide: u1,
    fl_objectdatas_vp2_hide: u1,
    fl_objectdatas_vp3_hide: u1,
    fl_objectdatas_last: u1,
    padding: u19,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ObjectDatas {
    link_name: Name,
    flags: ObjectDatasFlags,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectDatas))]
pub struct SkelBodyV1_381_67_09PC {
    bounding_sphere_center: Sphere,
    bones: DynArray<Bone>,
    material_names: DynArray<Name>,
    mesh_data_names: DynArray<Name>,
    animation_node_names_arrays: DynArray<DynArray<Name>>,
    some_names: DynArray<Name>,
    sphere_col_bones0: DynArray<SphereColBone>,
    sphere_col_bones1: DynArray<SphereColBone>,
    box_col_bones: DynArray<BoxColBone>,
}

pub type SkelV1_381_67_09PC = TrivialClass<ObjectDatas, SkelBodyV1_381_67_09PC>;
