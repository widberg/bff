use bilge::prelude::{bitsize, u1, u12, u15, u2, u24, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::map::BffMap;
use crate::math::{
    DynBox, DynSphere, Mat4f, NumeratorFloat, Quat, RangeBeginSize, RangeFirstLast, Vec2f, Vec3f,
};
use crate::name::Name;
use crate::strings::PascalString;

type VertexVectorComponent = u8;
type VertexVector3u8 = [VertexVectorComponent; 3];
type VertexBlendIndex = f32;
type DisplacementVectorComponent = NumeratorFloat<i16, 1024>;
type ShortVecWeird = [NumeratorFloat<i16, 1024>; 3];

#[bitsize(32)]
#[derive(BinRead, DebugBits, Serialize)]
struct ObjectFlags {
    fl_object_init: u1,
    fl_object_max_bsphere: u1,
    fl_object_skinned: u1,
    fl_object_morphed: u1,
    fl_object_orientedbbox: u1,
    fl_object_no_seaddisplay: u1,
    fl_object_no_seadcollide: u1,
    fl_object_no_display: u1,
    fl_object_transparent: u1,
    fl_object_optimized_vertex: u1,
    fl_object_linear_mapping: u1,
    fl_object_skinned_with_one_bone: u1,
    fl_object_light_baked: u1,
    fl_object_light_baked_with_material: u1,
    fl_object_shadow_receiver: u1,
    fl_object_no_tesselate: u1,
    fl_object_last: u1,
    padding: u15,
}

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u16)]
enum ObjectType {
    Points = 0,
    Surface = 1,
    Spline = 2,
    Skin = 3,
    RotShape = 4,
    Lod = 5,
    Mesh = 6,
    Camera = 7,
    SplineZone = 9,
    Occluder = 10,
    CameraZone = 11,
    Light = 12,
    HFog = 13,
    CollisionVol = 14,
    Emiter = 15,
    Omni = 16,
    Graph = 17,
    Particles = 18,
    Flare = 19,
    HField = 20,
    Tree = 21,
    GenWorld = 22,
    Road = 23,
    GenWorldSurface = 24,
    SplineGraph = 25,
    WorldRef = 26,
}

#[derive(BinRead, Debug, Serialize)]
struct FadeDistances {
    x: f32,
    y: f32,
    fade_close: f32,
}

#[derive(BinRead, Debug, Serialize)]
pub struct MeshLinkHeader {
    link_name: Name,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: ObjectFlags,
    r#type: ObjectType,
    names: DynArray<Name>,
    fade: FadeDistances,
    dyn_spheres: DynArray<DynSphere>,
    dyn_boxes: DynArray<DynBox>,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused0 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Strip {
    strip_vertices_indices: DynArray<u16>,
    material_name: Name,
    tri_order: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused00 {
    unused0: u32,
    unused1: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused4 {
    unused0s: DynArray<Unused00>,
}

#[derive(BinRead, Debug, Serialize)]
struct CollisionAABB {
    min: Vec3f,
    collision_aabb_range: RangeFirstLast,
    max: Vec3f,
    collision_faces_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize)]
struct CollisionFace {
    short_vec_weirds_indices: [u16; 3],
    surface_type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused8 {
    unused0: u32,
    unused1: u32,
    unused2: u32,
    unused3: u32,
    unused4: u32,
    unused5: u32,
    unused6: u32,
    unused7: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexLayoutPosition {
    position: Vec3f,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexLayoutNoBlend {
    position: Vec3f,
    tangent: VertexVector3u8,
    tangent_w: VertexVectorComponent,
    normal: VertexVector3u8,
    normal_w: VertexVectorComponent,
    uv: Vec2f,
    luv: Vec2f,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexLayout1Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    tangent_w: VertexVectorComponent,
    normal: VertexVector3u8,
    normal_w: VertexVectorComponent,
    uv: Vec2f,
    blend_index: VertexBlendIndex,
    pad2: [i32; 3],
    blend_weight: f32,
}

#[derive(BinRead, Debug, Serialize)]
struct VertexLayout4Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    tangent_w: VertexVectorComponent,
    normal: VertexVector3u8,
    normal_w: VertexVectorComponent,
    uv: Vec2f,
    blend_indices: [VertexBlendIndex; 4],
    blend_weights: [f32; 4],
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(vertex_layout: u32))]
enum Vertex {
    #[br(pre_assert(vertex_layout == 12))]
    VertexLayoutPosition(VertexLayoutPosition),
    #[br(pre_assert(vertex_layout == 36))]
    VertexLayoutNoBlend(VertexLayoutNoBlend),
    #[br(pre_assert(vertex_layout == 48))]
    VertexLayout1Blend(VertexLayout1Blend),
    #[br(pre_assert(vertex_layout == 60))]
    VertexLayout4Blend(VertexLayout4Blend),
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, Serialize)]
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

#[derive(BinRead, Debug, Serialize)]
struct VertexBufferExt {
    vertex_count: u32,
    vertex_layout: u32,
    flags: D3DFlags,
    #[br(args { count: vertex_count as usize, inner: (vertex_layout,) })]
    vertices: Vec<Vertex>,
}

#[derive(BinRead, Debug, Serialize)]
struct IndexBufferExt {
    index_count: u32,
    flags: D3DFlags,
    #[br(count = index_count)]
    data: Vec<u16>,
}

#[derive(BinRead, Debug, Serialize)]
struct Quad {
    vertices: [Vec3f; 4],
    normal: Vec3f,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused1 {
    unused0: u32,
    unused1: u32,
    unused2: u32,
    unused3: u32,
    unused4: u32,
    unused5: u32,
    unused6: u32,
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, Serialize)]
struct VertexGroupFlags {
    padding: u2,
    visible: u1,
    padding0: u16,
    morph: u1,
    padding1: u12,
}

#[derive(BinRead, Debug, Serialize)]
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

#[derive(BinRead, Debug, Serialize)]
struct AABBMorphTrigger {
    min: Vec3f,
    aabb_morph_triggers_range: RangeFirstLast,
    max: Vec3f,
    map_index_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize)]
struct DisplacementVector {
    displacement: [DisplacementVectorComponent; 3],
    displacement_vectors_self_index: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct MorphTargetDesc {
    name: PascalString,
    base_vertex_buffer_id: u32,
    displacement_vertex_buffer_index: u16,
    displacement_vectors_indicies: DynArray<u16>,
    displacement_vectors: DynArray<DisplacementVector>,
}

#[derive(BinRead, Debug, Serialize)]
struct Morpher {
    aabb_morph_triggers: DynArray<AABBMorphTrigger>,
    map: BffMap<u16, u16>,
    displacement_vectors_indices: DynArray<u16>,
    morphs: DynArray<MorphTargetDesc>,
}

#[derive(BinRead, Debug, Serialize)]
struct MeshBuffers {
    vertex_buffers: DynArray<VertexBufferExt>,
    index_buffers: DynArray<IndexBufferExt>,
    quads: DynArray<Quad>,
    vertex_groups: DynArray<VertexGroup>,
    morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &MeshLinkHeader))]
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

pub type MeshV1_381_67_09PC = TrivialClass<MeshLinkHeader, MeshBodyV1_381_67_09PC>;
