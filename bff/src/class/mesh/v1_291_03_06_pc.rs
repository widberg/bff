use binrw::binread;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{DynBox, DynSphere, Mat4f, Quat, Vec2f, Vec3, Vec3f};
use crate::name::Name;

#[binread]
#[derive(Debug, Serialize)]
struct PointsRelated0 {
    #[br(count = 12)]
    data: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct PointsRelated1 {
    #[br(count = 16)]
    data: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct PointsRelated2 {
    #[br(count = 4)]
    data: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Unknown1 {
    #[br(count = 8)]
    unknown1: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Unknown2 {
    #[br(count = 12)]
    unknown2: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Unknown3 {
    unknown4_count: u32,
    #[br(count = unknown4_count * 2)]
    unknown4: Vec<u8>,
    #[br(count = 8)]
    unknown5: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Unknown5 {
    unknown8_count: u32,
    #[br(count = unknown8_count * 8)]
    unknown8: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Unknown6 {
    #[br(count = 32)]
    data: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Unknown7 {
    #[br(count = 44)]
    data: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Unknown8 {
    #[br(count = 16)]
    data: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Points {
    mesh_data_crc32: Name,
    rotation: Quat,
    transform: Mat4f,
    radius: f32,
    flags: u32,
    mesh_type: u16,
    points_related0: DynArray<PointsRelated0>,
    points_related1: DynArray<PointsRelated1>,
    points_related2: DynArray<PointsRelated2>,
}

#[binread]
#[derive(Debug, Serialize)]
struct CyilndreCol {
    #[br(count = 40)]
    data: Vec<u8>,
    name: Name,
}

#[binread]
#[derive(Debug, Serialize)]
struct AABBCol {
    #[br(count = 4)]
    unknowns: Vec<u16>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Vertex {
    position: Vec3<i16>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Triangle {
    indices: Vec3<i16>,
}

#[binread]
#[derive(Debug, Serialize)]
#[br(import(length: u32))]
enum VertexStruct {
    #[br(assert(length == 24))]
    VertexStruct24 {
        position: Vec3f,
        unknown: f32,
        uv: Vec2f,
    },
    #[br(assert(length == 36))]
    VertexStruct36 {
        position: Vec3f,
        tangent: Vec3<u8>,
        tangent_padding: u8,
        normal: Vec3<u8>,
        normal_padding: u8,
        uv: Vec2f,
        luv: Vec2f,
    },
    #[br(assert(length == 48))]
    VertexStruct48 {
        position: Vec3f,
        tangent: Vec3<u8>,
        tangent_padding: u8,
        normal: Vec3<u8>,
        normal_padding: u8,
        uv: Vec2f,
        #[br(count = 5)]
        unknown: Vec<f32>,
    },
    #[br(assert(length == 60))]
    VertexStruct60 {
        position: Vec3f,
        tangent: Vec3<u8>,
        tangent_padding: u8,
        normal: Vec3<u8>,
        normal_padding: u8,
        uv: Vec2f,
        #[br(count = 4)]
        blend_indices: Vec<f32>,
        #[br(count = 4)]
        blends: Vec<f32>,
    },
    VertexStructUnknown {
        #[br(count = length)]
        data: Vec<u8>,
    },
}

#[binread]
#[derive(Debug, Serialize)]
struct VertexBuffer {
    vertex_struct_count: u32,
    vertex_struct_length: u32,
    unknown: u32,
    #[br(count = vertex_struct_count)]
    vertex_structs: Vec<VertexStruct>,
}

#[binread]
#[derive(Debug, Serialize)]
struct IndexBuffer {
    index_count: u32,
    unknown: u32,
    #[br(count = index_count / 3)]
    tris: Vec<Triangle>,
}

#[binread]
#[derive(Debug, Serialize)]
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

#[binread]
#[derive(Debug, Serialize)]
struct MorpherRelated {
    #[br(count = 16)]
    morpher_relateds: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct MorphTargetDescRelated {
    #[br(count = 16)]
    morpher_relateds: Vec<u8>,
}

#[binread]
#[derive(Debug, Serialize)]
struct MorpherTargetDesc {
    name: Name,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[binread]
#[derive(Debug, Serialize)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morpher_descs: DynArray<MorpherTargetDesc>,
}

#[binread]
#[derive(Debug, Serialize)]
struct MeshBuffer {
    vertex_buffers: DynArray<VertexBuffer>,
    index_buffers: DynArray<IndexBuffer>,
    vertex_groups: DynArray<VertexGroup>,
    unknowns: DynArray<Unknown7>,
    morpher: Morpher,
}

#[binread]
#[derive(Debug, Serialize)]
pub struct MeshBodyV1_291_03_06PC {
    points: Points,
    unknown1s: DynArray<Unknown1>,
    unknown2s: DynArray<Unknown2>,
    #[br(restore_position)]
    unknown3_count: u32,
    unknown3s: DynArray<Unknown3>,
    #[br(if(points.flags & 2 > 0))]
    #[br(count = unknown3_count * 4)]
    unknown4s: Option<Vec<u8>>,
    unknown5s: DynArray<Unknown5>,
    material_crc32s: DynArray<Name>,
    #[br(count = 24)]
    related_to_counts: Vec<u8>,
    sphere_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylindre_cols: DynArray<CyilndreCol>,
    aabb_cols: DynArray<AABBCol>,
    vertices: DynArray<Vertex>,
    unknown6s: DynArray<Unknown6>,
    mesh_buffer: MeshBuffer,
    unknown8s: DynArray<Unknown8>,
}

pub type MeshV1_291_03_06PC = TrivialClass<(), MeshBodyV1_291_03_06PC>;
