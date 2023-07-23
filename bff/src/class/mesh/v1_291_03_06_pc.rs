use binrw::BinRead;
use serde::Serialize;
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{DynBox, DynSphere, Mat4f, Quat, Vec2, Vec2f, Vec3, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct PointsRelated0 {
    data: [u8; 12],
}

#[derive(BinRead, Debug, Serialize)]
struct PointsRelated1 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
struct PointsRelated2 {
    data: [u8; 4],
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown1 {
    unknown1: [u8; 8],
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown2 {
    unknown2: [u8; 12],
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown3 {
    unknown4_count: u32,
    #[br(count = unknown4_count * 2)]
    unknown4: Vec<u8>,
    unknown5: [u8; 8],
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown5 {
    unknown8_count: u32,
    #[br(count = unknown8_count * 8)]
    unknown8: Vec<u8>,
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown6 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown7 {
    // Big array helper for serde.
    // The purpose of this crate is to make (de-)serializing arrays of sizes > 32 easy.
    // This solution is needed until serde adopts const generics support.
    // https://github.com/serde-rs/serde/issues/1937
    #[serde(with = "BigArray")]
    data: [u8; 44],
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown8 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkInfo {
    link_crc32: Name,
    links: DynArray<Name>,
    mesh_data_crc32: Name,
    rotation: Quat,
    transform: Mat4f,
    radius: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct Points {
    points_related0: DynArray<PointsRelated0>,
    points_related1: DynArray<PointsRelated1>,
    points_related2: DynArray<PointsRelated2>,
}

#[derive(BinRead, Debug, Serialize)]
struct CylindreCol {
    #[serde(with = "BigArray")]
    data: [u8; 40],
    name: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct AABBColRelated {
    unknowns: [u16; 4],
}

#[derive(BinRead, Debug, Serialize)]
struct AABBCol {
    unknown1s: Vec3f,
    unknown2s: Vec2<i16>,
    unknown3s: Vec3f,
    unknown4s: Vec2<i16>,
}

#[derive(BinRead, Debug, Serialize)]
struct Vertex {
    position: Vec3<i16>,
}

#[derive(BinRead, Debug, Serialize)]
struct Triangle {
    indices: Vec3<i16>,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(length: u32))]
enum VertexStruct {
    #[br(pre_assert(length == 24))]
    VertexStruct24 {
        position: Vec3f,
        unknown: f32,
        uv: Vec2f,
    },
    #[br(pre_assert(length == 36))]
    VertexStruct36 {
        position: Vec3f,
        tangent: Vec3<u8>,
        tangent_padding: u8,
        normal: Vec3<u8>,
        normal_padding: u8,
        uv: Vec2f,
        luv: Vec2f,
    },
    #[br(pre_assert(length == 48))]
    VertexStruct48 {
        position: Vec3f,
        tangent: Vec3<u8>,
        tangent_padding: u8,
        normal: Vec3<u8>,
        normal_padding: u8,
        uv: Vec2f,
        unknown: [f32; 5],
    },
    #[br(pre_assert(length == 60))]
    VertexStruct60 {
        position: Vec3f,
        tangent: Vec3<u8>,
        tangent_padding: u8,
        normal: Vec3<u8>,
        normal_padding: u8,
        uv: Vec2f,
        blend_indices: [f32; 4],
        blends: [f32; 4],
    },
    VertexStructUnknown {
        #[br(count = length)]
        data: Vec<u8>,
    },
}

#[derive(BinRead, Debug, Serialize)]
struct VertexBuffer {
    vertex_struct_count: u32,
    vertex_struct_length: u32,
    unknown: u32,
    #[br(args { count: vertex_struct_count as usize, inner: (vertex_struct_length,) })]
    vertex_structs: Vec<VertexStruct>,
}

#[derive(BinRead, Debug, Serialize)]
struct IndexBuffer {
    index_count: u32,
    unknown: u32,
    #[br(count = index_count / 3)]
    tris: Vec<Triangle>,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexGroup {
    zeroes: Vec3<u32>,
    primitive: u32,
    vertex_offset_in_groups: u16,
    unknown0: u16,
    vertex_count: u32,
    index_buffer_offset_in_shorts: u32,
    face_count: u32,
    unknown1: u32,
    unknown2: u32,
    vertex_size: u16,
    cdcdcdcd: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct MorpherRelated {
    morpher_relateds: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
struct MorphTargetDescRelated {
    morpher_relateds: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
struct MorpherTargetDesc {
    name: Name,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[derive(BinRead, Debug, Serialize)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morpher_descs: DynArray<MorpherTargetDesc>,
}

#[derive(BinRead, Debug, Serialize)]
struct MeshBuffer {
    vertex_buffers: DynArray<VertexBuffer>,
    index_buffers: DynArray<IndexBuffer>,
    vertex_groups: DynArray<VertexGroup>,
    unknowns: DynArray<Unknown7>,
    morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(link_header: &LinkInfo))]
pub struct MeshBodyV1_291_03_06PC {
    points: Points,
    unknown1s: DynArray<Unknown1>,
    unknown2s: DynArray<Unknown2>,
    #[br(restore_position)]
    unknown3_count: u32,
    unknown3s: DynArray<Unknown3>,
    #[br(if(link_header.flags & 2 > 0))]
    #[br(count = unknown3_count * 4)]
    unknown4s: Option<Vec<u8>>,
    unknown5s: DynArray<Unknown5>,
    material_crc32s: DynArray<Name>,
    related_to_counts: [u8; 24],
    sphere_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylindre_cols: DynArray<CylindreCol>,
    aabb_col_relateds: DynArray<AABBColRelated>,
    aabb_cols: DynArray<AABBCol>,
    vertices: DynArray<Vertex>,
    unknown6s: DynArray<Unknown6>,
    mesh_buffer: MeshBuffer,
    unknown8s: DynArray<Unknown8>,
}

pub type MeshV1_291_03_06PC = TrivialClass<LinkInfo, MeshBodyV1_291_03_06PC>;
