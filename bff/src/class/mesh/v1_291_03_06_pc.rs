use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite, binrw};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::generic::{AABBNode, Strip, VertexGroupFlags, Vertices};
use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    Cylindre,
    DynArray,
    DynBox,
    DynSphere,
    ObjectLinkHeaderV1_06_63_02PC,
    Vec2f,
    Vec3,
    Vec3f,
    Vec3i16,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct PointsRelated0 {
    data: [u8; 12],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct PointsRelated1 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct PointsRelated2 {
    data: [u8; 4],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unknown5 {
    unknown8_count: u32,
    #[br(count = unknown8_count * 8)]
    unknown8: Vec<u8>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unknown6 {
    unknowns: [u32; 8],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unknown7 {
    data1: [u8; 32],
    data2: [u8; 12],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unknown8 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Points {
    points_related0: DynArray<PointsRelated0>,
    points_related1: DynArray<PointsRelated1>,
    points_related2: DynArray<PointsRelated2>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct CylindreCol {
    cylindre: Cylindre,
    flag: u32,
    name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct AABBColTri {
    first_vertex_id: i16,
    second_vertex_id: i16,
    third_vertex_id: i16,
    material_index: i16,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
pub struct VertexBuffer {
    #[br(temp)]
    #[bw(calc = vertices.len() as u32)]
    vertex_count: u32,
    #[br(temp)]
    #[bw(calc = vertices.layout() as u32)]
    vertex_layout: u32,
    flags: u32,
    #[br(args(vertex_count as usize, vertex_layout as usize))]
    pub vertices: Vertices,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
pub struct IndexBuffer {
    #[br(temp)]
    #[bw(calc = tris.len() as u32 * 3)]
    index_count: u32,
    flags: u32,
    #[br(count = index_count / 3)]
    pub tris: Vec<Vec3i16>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct VertexGroup {
    zeroes: Vec3<u32>,
    flags: VertexGroupFlags,
    pub vertex_offset_in_groups: u16,
    unknown0: u16,
    pub vertex_count: u32,
    pub index_buffer_index_begin: u32,
    pub face_count: u32,
    zero: u32,
    vertex_buffer_range_begin: u32,
    vertex_layout: u16,
    unused: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct MorpherRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct MorphTargetDescRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct MorpherTargetDesc {
    name: Name,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morpher_descs: DynArray<MorpherTargetDesc>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct MeshBuffers {
    pub vertex_buffers: DynArray<VertexBuffer>,
    pub index_buffers: DynArray<IndexBuffer>,
    pub vertex_groups: DynArray<VertexGroup>,
    unknowns: DynArray<Unknown7>,
    morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct MeshBodyV1_291_03_06PC {
    points: Points,
    texcoords: DynArray<Vec2f>,
    normals: DynArray<Vec3f>,
    strips: DynArray<Strip>,
    #[br(if(link_header.flags & 2 > 0))]
    #[br(count = strips.len() * 4)]
    unknown4s: Option<Vec<u8>>,
    unknown5s: DynArray<Unknown5>,
    material_names: DynArray<Name>,
    drawing_start_distance: f32,
    drawing_cutoff_distance: f32,
    shadow_related: u32,
    related_to_counts: [u32; 3],
    sphere_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylindre_cols: DynArray<CylindreCol>,
    collision_aabb_tris: DynArray<AABBColTri>,
    collision_aabbs: DynArray<AABBNode>,
    vertices: DynArray<Vec3i16>,
    unknown6s: DynArray<Unknown6>,
    pub mesh_buffers: MeshBuffers,
    unknown8s: DynArray<Unknown8>,
}

pub type MeshV1_291_03_06PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, MeshBodyV1_291_03_06PC>;

impl Export for MeshV1_291_03_06PC {}
impl Import for MeshV1_291_03_06PC {}
