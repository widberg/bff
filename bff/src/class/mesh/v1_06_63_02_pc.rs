use binrw::{BinRead, BinWrite};
use serde::Serialize;
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{DynBox, DynSphere, Mat, Sphere, Vec2f, Vec3, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Box {
    mat: Mat<3, 4>,
    vec: Vec3f,
    maybe_scale: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
pub struct LinkInfo {
    link_crc32: Name,
    linked_crc32: DynArray<Name>,
    data_crc32: Name,
    b_sphere_local: Sphere,
    b_box: Box,
    fade_out_distance: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct PointsRelated0 {
    data: [u8; 12],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct PointsRelated1 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct MorpherRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct MorphTargetDescRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct MorphTargetDesc {
    name: u32,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morph_target_descs: DynArray<MorphTargetDesc>,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct CylindreCol {
    #[serde(with = "BigArray")]
    data: [u8; 40],
    name_crc32: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct AABBColTri {
    first_vertex_id: i16,
    second_vertex_id: i16,
    third_vertex_id: i16,
    material_index: i16,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct AABBCol {
    unk1: f32,
    unk2: f32,
    unk3: f32,
    unk4: i16,
    unk5: i16,
    unk6: f32,
    unk7: f32,
    unk8: f32,
    unk9: i16,
    unk10: i16,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Vertex {
    position: Vec3<i16>,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct VertexGroup {
    zero0: u32,
    zero1: u32,
    zero2: u32,
    maybe_primitive: u32,
    cdcd: u16,
    vertex_buffer_offset: u16,
    vertex_count: u16,
    index_buffer_offset_in_shorts: u16,
    face_count: u32,
    unk1: u16,
    vertex_size: u16,
    cdcdcdcd: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Triangle {
    index1: i16,
    index2: i16,
    index3: i16,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Points {
    points_relateds0: DynArray<PointsRelated0>,
    points_relateds1: DynArray<PointsRelated1>,
    morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Unknown0 {
    u: f32,
    v: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Unknown1 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Unknown2 {
    unk4_count: u32,
    #[br(count = 2 * unk4_count)]
    unk4s: Vec<u8>,
    unk5: [u8; 8],
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct Unknown3 {
    unk8_count: u32,
    #[br(count = 8 * unk8_count)]
    unk8s: Vec<u8>,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(length: u16))]
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
        unknown1: Vec3f,
        unknown2: Vec3f,
        blend_indices: Vec3f,
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

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct VertexBuffer {
    vertex_struct_count: u16,
    vertex_struct_length: u16,
    #[br(args { count: vertex_struct_count as usize, inner: (vertex_struct_length,) })]
    vertex_structs: Vec<VertexStruct>,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
struct IndexBuffer {
    index_count: u16,
    #[br(count = index_count / 3)]
    tris: Vec<Triangle>,
}

#[derive(BinRead, Debug, Serialize, BinWrite)]
#[br(import(link_header: &LinkInfo))]
pub struct MeshBodyV1_06_63_02PC {
    points: Points,
    uv_count: u32,
    #[br(count = 8 * uv_count)]
    unknown0s: Vec<Unknown0>,
    normal_count: u32,
    #[br(count = 12 * normal_count)]
    unknown1s: Vec<Unknown1>,
    strip_count: u32,
    #[br(count = strip_count)]
    unknown2s: Vec<Unknown2>,
    #[br(if(link_header.flags & 2 >= 1))]
    #[br(count = 4 * strip_count)]
    unk6: Option<Vec<u8>>,
    strip_ext_count: u32,
    #[br(count = strip_ext_count)]
    unknown3s: Vec<Unknown3>,
    material_crc32s: DynArray<Name>,
    drawing_start_distance: f32,
    drawing_cutoff_distance: f32,
    shadow_related: u32,
    unk_uint_related_to_count2: u32,
    unk_uint_related_to_count3: u32,
    unk_uint_related_to_count4: u32,
    sphere_cols: DynArray<DynSphere>,
    box_cols: DynArray<DynBox>,
    cylindre_cols: DynArray<CylindreCol>,
    aabb_col_tris: DynArray<AABBColTri>,
    aabb_cols: DynArray<AABBCol>,
    vertices: DynArray<Vertex>,
    zero2: u32,
    unk_uints: DynArray<u32>,
    vertex_buffer_count: u32,
    vertex_buffer: VertexBuffer,
    index_buffer_count: u32,
    #[br(count = index_buffer_count)]
    index_buffers: Vec<IndexBuffer>,
    vertex_groups: DynArray<VertexGroup>,
}

pub type MeshV1_06_63_02PC = TrivialClass<LinkInfo, MeshBodyV1_06_63_02PC>;
