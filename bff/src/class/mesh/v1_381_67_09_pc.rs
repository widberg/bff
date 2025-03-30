use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffMap, DynArray, DynBox, DynSphere, NumeratorFloat, ObjectLinkHeaderV1_381_67_09PC,
    PascalString, RangeBeginSize, RangeFirstLast, Vec2f, Vec3f, Vec3i16,
};
use crate::names::Name;

use super::generic::{CollisionAABB, Strip, Vertex, VertexGroupFlags};

type DisplacementVectorComponent = NumeratorFloat<i16, 1024>;
type ShortVecWeird = [NumeratorFloat<i16, 1024>; 3];

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct FadeDistances {
    x: f32,
    y: f32,
    fade_close: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct LinkHeader {
    object_link_header: ObjectLinkHeaderV1_381_67_09PC,
    names: DynArray<Name>,
    fade: FadeDistances,
    dyn_spheres: DynArray<DynSphere>,
    dyn_boxes: DynArray<DynBox>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unused0 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
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

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct CollisionFace {
    short_vec_weirds_indices: [u16; 3],
    surface_type: u16,
}

// same as Unknown6 in wall-e
#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unused8 {
    unuseds: [u32; 8],
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, Deserialize, ReferencedNames)]
struct D3DFlags {
    d3d_pool_default: u1,
    d3d_pool_managed: u1,
    d3d_pool_systemmem: u1,
    d3d_pool_scratch: u1,

    d3d_usage_dynamic: u1,
    d3d_usage_writeonly: u1,
    padding0: u1,
    unknown: u1,
    padding1: u24,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct VertexBufferExt {
    vertex_count: u32,
    vertex_layout: u32,
    flags: D3DFlags,
    #[br(args { count: vertex_count as usize, inner: (vertex_layout,) })]
    vertices: Vec<Vertex>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct IndexBufferExt {
    index_count: u32,
    flags: D3DFlags,
    #[br(count = index_count / 3)]
    data: Vec<Vec3i16>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Quad {
    vertices: [Vec3f; 4],
    normal: Vec3f,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unused1 {
    unused0: u32,
    unused1: u32,
    unused2: u32,
    unused3: u32,
    unused4: u32,
    unused5: u32,
    unused6: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct VertexGroup {
    vertex_buffer_index: u32,
    index_buffer_index: u32,
    quad_range: RangeBeginSize<u32>,
    flags: VertexGroupFlags,
    vertex_buffer_range: RangeFirstLast,
    vertex_count: u32,
    index_buffer_index_begin: u32,
    face_count: u32,
    zero: u32,
    vertex_buffer_range_begin_or_zero: u32,
    vertex_layout: u16,
    material_index: i16,
    unused1s: DynArray<Unused1>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct AABBMorphTrigger {
    min: Vec3f,
    aabb_morph_triggers_range: RangeFirstLast,
    max: Vec3f,
    map_index_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct DisplacementVector {
    displacement: [DisplacementVectorComponent; 3],
    displacement_vectors_self_index: u16,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct MorphTargetDesc {
    name: PascalString,
    base_vertex_buffer_id: u32,
    displacement_vertex_buffer_index: u16,
    displacement_vectors_indicies: DynArray<u16>,
    displacement_vectors: DynArray<DisplacementVector>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Morpher {
    aabb_morph_triggers: DynArray<AABBMorphTrigger>,
    map: BffMap<u16, u16>,
    displacement_vectors_indices: DynArray<u16>,
    morphs: DynArray<MorphTargetDesc>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct MeshBuffers {
    vertex_buffers: DynArray<VertexBufferExt>,
    index_buffers: DynArray<IndexBufferExt>,
    quads: DynArray<Quad>,
    vertex_groups: DynArray<VertexGroup>,
    morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &LinkHeader))]
pub struct MeshBodyV1_381_67_09PC {
    strip_vertices: DynArray<Vec3f>,
    unused0s: DynArray<Unused0>,
    texcoords: DynArray<Vec2f>,
    normals: DynArray<Vec3f>,
    strips: DynArray<Strip>,
    unused4s: DynArray<Unused4>,
    material_names: DynArray<Name>,
    collision_aabbs: DynArray<CollisionAABB>,
    collision_faces: DynArray<CollisionFace>,
    unused8s: DynArray<Unused8>,
    mesh_buffers: MeshBuffers,
    short_vec_weirds: DynArray<ShortVecWeird>,
}

pub type MeshV1_381_67_09PC = TrivialClass<LinkHeader, MeshBodyV1_381_67_09PC>;
