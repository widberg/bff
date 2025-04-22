use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite, binrw};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::generic::{CollisionAABB, Strip, VertexGroupFlags, Vertices};
use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    DynArray,
    DynBox,
    DynSphere,
    ResourceLinkHeaderV1_06_63_02PC,
    Vec2f,
    Vec3,
    Vec3f,
    Vec3i16,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct PointsRelated0 {
    data: [u8; 12],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct PointsRelated1 {
    data: [u32; 4],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct MorpherRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct MorphTargetDescRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct MorphTargetDesc {
    name: Name,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morph_target_descs: DynArray<MorphTargetDesc>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct CylindreCol {
    #[serde(with = "BigArray")]
    data: [u8; 40],
    name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct AABBColTri {
    first_vertex_id: i16,
    second_vertex_id: i16,
    third_vertex_id: i16,
    material_index: i16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct VertexGroup {
    zeroes: Vec3<u32>,
    flags: VertexGroupFlags,
    unused0: u16,
    vertex_buffer_offset: u16,
    vertex_count: u16,
    index_buffer_offset_in_shorts: u16,
    face_count: u32,
    vertex_buffer_range_begin: u16,
    vertex_layout: u16,
    unused1: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Points {
    points_relateds0: DynArray<Vec3f>,
    points_relateds1: DynArray<PointsRelated1>,
    morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unused00 {
    unused0: u32,
    unused1: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unused4 {
    unused0s: DynArray<Unused00>,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, ReferencedNames)]
struct VertexBuffer {
    #[br(temp)]
    #[bw(calc = vertices.len() as u16)]
    vertex_count: u16,
    #[br(temp)]
    #[bw(calc = vertices.layout() as u16)]
    vertex_layout: u16,
    #[br(args(vertex_count as usize, vertex_layout as usize))]
    vertices: Vertices,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, ReferencedNames)]
struct IndexBuffer {
    #[br(temp)]
    #[bw(calc = tris.len() as u16 * 3)]
    index_count: u16,
    #[br(count = index_count / 3)]
    tris: Vec<Vec3i16>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(link_header: &ResourceLinkHeaderV1_06_63_02PC))]
pub struct MeshBodyV1_06_63_02PC {
    points: Points,
    uv_count: u32,
    #[br(count = 8 * uv_count)]
    unknown0s: Vec<Vec2f>,
    normal_count: u32,
    #[br(count = 12 * normal_count)]
    unknown1s: Vec<Vec3f>,
    strips: DynArray<Strip>,
    #[br(if(link_header.flags & 2 >= 1))]
    #[br(count = 4 * strips.len())]
    unk6: Option<Vec<u8>>,
    unused4s: DynArray<Unused4>,
    material_names: DynArray<Name>,
    drawing_start_distance: f32,
    drawing_cutoff_distance: f32,
    shadow_related: u32,
    related_to_counts: [u32; 3],
    sphere_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylindre_cols: DynArray<CylindreCol>,
    collision_aabb_tris: DynArray<AABBColTri>,
    collision_aabbs: DynArray<CollisionAABB>,
    vertices: DynArray<Vec3i16>,
    zero2: u32,
    unk_uints: DynArray<u32>,
    vertex_buffers: DynArray<VertexBuffer>,
    index_buffers: DynArray<IndexBuffer>,
    vertex_groups: DynArray<VertexGroup>,
}

pub type MeshV1_06_63_02PC = TrivialClass<ResourceLinkHeaderV1_06_63_02PC, MeshBodyV1_06_63_02PC>;

impl Export for MeshV1_06_63_02PC {}
impl Import for MeshV1_06_63_02PC {}
