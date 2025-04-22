use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffOption,
    DynArray,
    Mat4f,
    RGB,
    ResourceLinkHeaderV1_06_63_02PC,
    Sphere,
    Vec2f,
    Vec3f,
    Vec4f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct PointsRelated0 {
    vector: Vec3f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct PointsRelated1 {
    vector: Vec4f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Points {
    points_related0s: DynArray<PointsRelated0>,
    points_related1s: DynArray<PointsRelated1>,
    morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct MorpherRelated {
    data: [u8; 4],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct EdgeCol {
    sphere: Sphere,
    flag: u32,
    unknown0: u32,
    unknown1: u32,
    edge_id: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ClingLineRelated {
    sphere: Sphere,
    flag: u32,
    edge_id: u32,
    unknown0: u32,
    unknown1: u32,
    unknown2: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct CullCone {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Box {
    transformation: Mat4f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Patch {
    flag: u16,
    should_draw_related_start_index: u16,
    edge_indices: [u16; 4],
    material_anim_index: u32,
    sphere: Sphere,
    cull_cone: CullCone,
    b_box: Box,
    unknown_indices: [u16; 4],
    color_indices: [u16; 4],
    normal_indices: [u16; 4],
    displacement_indices: [u16; 8],
    col_cache_index: u16,
    unknown: u16,
    material_anim_name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Edge {
    p: [u16; 2],
    t: [u16; 2],
}

#[bitsize(16)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames)]
struct ShouldDrawRelated {
    index_in_draw_info_array: u3,
    shift_amount_for_bit: u5,
    other: u8,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct PatchCol {
    sphere: Sphere,
    flag: u32,
    edge_col_id: u16,
    next_patch_col_id: u16,
    cdcdcdcd: [u32; 2],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct SeadVoxel {
    element_entry: u16,
    element_count: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct SeadIndex {
    sead_voxels: DynArray<SeadVoxel>,
    patch_indices: DynArray<u16>,
    unknown_vec0s: [Vec4f; 3],
    axes0: Vec3f,
    unknown_ptr0: u32,
    axes1: Vec3f,
    unknown_ptr1: u32,
    axes2: Vec3f,
    unknown_ptr2: u32,
    unknown_vec1: Vec4f,
    size: Vec4f,
    step: Vec3f,
    unknown_ptr3: u32,
    center: Vec4f,
    unknown_vec2: Vec4f,
    f_size: Vec3f,
    unknown_ptr4: u32,
    used_in_voxel_trace: u32,
    i_size: Vec3f,
    hit_patch_count: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceLinkHeaderV1_06_63_02PC))]
pub struct SurfaceBodyV1_291_03_06PC {
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

pub type SurfaceV1_291_03_06PC =
    TrivialClass<ResourceLinkHeaderV1_06_63_02PC, SurfaceBodyV1_291_03_06PC>;

impl Export for SurfaceV1_291_03_06PC {}
impl Import for SurfaceV1_291_03_06PC {}
