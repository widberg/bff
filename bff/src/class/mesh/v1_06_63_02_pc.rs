use bff_derive::ReferencedNames;
use binrw::binrw;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::shared::{AABBNode, Strip, Vertices};
use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffBox,
    Cylindre,
    DynArray,
    ObjectLinkHeaderV1_06_63_02PC,
    Sphere,
    Vec2f,
    Vec3,
    Vec3f,
    Vec3i16,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
struct TBVtx {
    unk_vec_1: Vec2f,
    unk_vec_2: Vec2f,
}

#[derive(..BffStruct)]
struct MorpherRelated {
    data: [u8; 16],
}

#[derive(..BffStruct)]
struct MorphTargetDescRelated {
    data: [u8; 16],
}

#[derive(..BffStruct)]
struct MorphTargetDesc {
    name: Name,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[derive(..BffStruct)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morph_target_descs: DynArray<MorphTargetDesc>,
}

#[derive(..BffStruct)]
struct SphereCol {
    col_sph: Sphere,
    flag: u32,
    name: Name,
}

#[derive(..BffStruct)]
struct BoxCol {
    col_box: BffBox,
    flag: u32,
    name: Name,
}

#[derive(..BffStruct)]
struct CylindreCol {
    col_cylindre: Cylindre,
    flag: u32,
    name: Name,
}

#[derive(..BffStruct)]
struct FaceCol {
    first_vertex_id: i16,
    second_vertex_id: i16,
    third_vertex_id: i16,
    material_index: i16,
}

#[derive(..BffStruct)]
struct AABBCol {
    collision_faces: DynArray<FaceCol>,
    collision_aabb_nodes: DynArray<AABBNode>,
}

#[derive(..BffStruct)]
struct PrimitiveInfo {
    placeholder_pointers: Vec3<u32>,
    prim_type: u16,
    shader_type: u16,
    unused0: u16,
    vertex_buffer_offset: u16,
    vertex_count: u16,
    index_buffer_offset_in_shorts: u16,
    face_count: u32,
    vertex_buffer_range_begin: u16,
    vertex_size: u16,
    unused1: u32,
}

#[derive(..BffStruct)]
struct Points {
    positions: DynArray<Vec3f>,
    tb_vtxs: DynArray<TBVtx>,
    morpher: Morpher,
}

#[derive(..BffStruct)]
struct Unused00 {
    unused0: u32,
    unused1: u32,
}

#[derive(..BffStruct)]
struct Unused4 {
    unused0s: DynArray<Unused00>,
}

#[derive(..BffStruct)]
struct MeshBuffers {
    vertex_buffers: DynArray<VertexBufferExt>,
    index_buffers: DynArray<IndexBufferExt>,
    prim_infos: DynArray<PrimitiveInfo>,
}

//TODO: Use PrimInfo to determine the actual layout to use, since can be (and are) many with same size
#[binrw]
#[derive(Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
struct VertexBufferExt {
    #[br(temp)]
    #[bw(calc = vertices.len() as u16)]
    vertex_count: u16,
    #[br(temp)]
    #[bw(calc = vertices.layout() as u16)]
    vertex_size: u16,
    #[br(args(vertex_count as usize, vertex_size as usize))]
    vertices: Vertices,
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
struct IndexBufferExt {
    #[br(temp)]
    #[bw(calc = tris.len() as u16 * 3)]
    index_count: u16,
    #[br(count = index_count / 3)]
    tris: Vec<Vec3i16>,
}

#[derive(..BffStruct)]
#[br(import(link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct MeshBodyV1_06_63_02PC {
    points: Points,
    uv_count: u32,
    #[br(count = uv_count)]
    uvs: Vec<Vec2f>,
    normal_count: u32,
    #[br(count = normal_count)]
    normals: Vec<Vec3f>,
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
    sphere_cols: DynArray<SphereCol>,
    box_cols: DynArray<BoxCol>,
    cylindre_cols: DynArray<CylindreCol>,
    aabb_col: AABBCol,
    aabb_vertices: DynArray<Vec3i16>,
    zero2: u32,
    unk_uints: DynArray<u32>,
    mesh_buffers: MeshBuffers,
}

pub type MeshV1_06_63_02PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, MeshBodyV1_06_63_02PC>;

impl Export for MeshV1_06_63_02PC {}
impl Import for MeshV1_06_63_02PC {}
