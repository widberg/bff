use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffOption,
    DynArray,
    RGB,
    ResourceObjectLinkHeaderV1_06_63_02PC,
    Sphere,
    Vec2f,
    Vec3f,
    Vec4f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct PointsRelated0 {
    vec3: Vec3f,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct PointsRelated1 {
    vec4: Vec4f,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct MorpherRelated {
    data: [u8; 16],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct MorphTargetDescRelated {
    data: [u8; 16],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct MorphTargetDesc {
    name: u32,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morph_target_descs: DynArray<MorphTargetDesc>,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct Points {
    points_relateds0: DynArray<PointsRelated0>,
    points_relateds1: DynArray<PointsRelated1>,
    morpher: Morpher,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct PatchCol {
    sphere: Sphere,
    flag: u32,
    edge_col_id: u16,
    next_patch_col_id: u16,
    cdcdcdcd: [u32; 2],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct EdgeCol {
    sphere: Sphere,
    flag: u32,
    edge_id: u32,
    cache_index_maybe: u32,
    unk_placeholder_ptr3: u32,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct ClingLineRelated {
    sphere: Sphere,
    flag: u32,
    edge_id: u32,
    unk_uints: [u32; 2],
    unk_float: f32,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct CullCone {
    data: [u8; 32],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct Patch {
    flag: u16,
    should_draw_related_start_index: u16,
    edge_indices: [u16; 4],
    material_anim_index: u32,
    sphere: Sphere,
    cull_cone: CullCone,
    b_box: crate::helpers::BffBox,
    unknown_indices: [u16; 4],
    color_indices: [u16; 4],
    normal_indices: [u16; 4],
    displacement_indices: [u16; 8],
    col_cache_index: u16,
    unknown: u16,
    material_anim_name: Name,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct Edge {
    p: [u16; 2],
    t: [u16; 2],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct SeadVoxel {
    element_entry: u16,
    element_count: u16,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct SeadIndex {
    sead_voxels: DynArray<SeadVoxel>,
    patch_indices: DynArray<u16>,
    unk_vec4_1: Vec4f,
    unk_vec4_2: Vec4f,
    unk_vec4_3: Vec4f,
    axes_1: Vec3f,
    unk_ptr1: u32,
    axes_2: Vec3f,
    unk_ptr2: u32,
    axes_3: Vec3f,
    unk_ptr3: u32,
    unk_vec4_4: Vec4f,
    size: Vec4f,
    step: Vec3f,
    unk_ptr4: u32,
    center: Vec4f,
    unk_vec4_7: Vec4f,
    f_size: Vec3f,
    unk_ptr5: u32,
    unk_used_in_voxel_trace: u32,
    i_size: Vec3f,
    hit_patch_count: u32,
}

#[bitsize(16)]
#[derive(BinRead, BinWrite, DebugBits, SerializeBits, DeserializeBits, ReferencedNames)]
struct ShouldDrawRelated {
    index_in_draw_info_array: u3,
    shift_amount_for_bit: u5,
    other: u8,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct SurfaceBodyV1_06_63_02PC {
    points: Points,
    edge_cols: DynArray<EdgeCol>,
    cling_line_relateds: DynArray<ClingLineRelated>,
    patches: DynArray<Patch>,
    edges: DynArray<Edge>,
    normals: DynArray<Vec3f>,
    colors: DynArray<RGB>,
    displacement_relateds: DynArray<Vec2f>,
    should_draw_relateds: DynArray<ShouldDrawRelated>,
    patch_cols: DynArray<PatchCol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    sead_index: BffOption<SeadIndex>,
}

pub type SurfaceV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, SurfaceBodyV1_06_63_02PC>;

impl Export for SurfaceV1_06_63_02PC {}
impl Import for SurfaceV1_06_63_02PC {}
