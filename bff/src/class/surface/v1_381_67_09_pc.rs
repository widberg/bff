use bilge::prelude::{bitsize, u1, u15, u3, u5, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::Serialize;
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat4f, Quat, RangeBeginSize, Vec2f, Vec3f, Vec4f};
use crate::name::Name;
use crate::option::BffOption;

#[derive(BinRead, Debug, Serialize)]
struct Unused2 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize)]
struct Unused3 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize)]
struct Patch {
    flag: u16,
    index_in_unk_short_da: u16,
    edge_indices: [u16; 4],
    material_anim_index: u32,
    data: [u32; 12],
    mat: Mat4f,
    vec4fs_indices: [u16; 4],
    unknown3s: [u16; 17],
    surface_indices_index: u16,
    material_anim_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
struct Edge {
    p: [u16; 2],
    t: [u16; 2],
}

#[bitsize(16)]
#[derive(BinRead, DebugBits, Serialize)]
struct ShouldDrawBitfield {
    index_in_draw_info_array: u3,
    shift_amount_for_bit: u5,
    other: u8,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused12 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize)]
struct SeadVoxel {
    patches_indices_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize)]
struct Unknown15 {
    #[serde(with = "BigArray")]
    data: [u32; 48],
    sead_voxel_count: u32,
    patch_count_related: u32,
    unknown0s: [u32; 2],
}

#[derive(BinRead, Debug, Serialize)]
struct SeadIndex {
    sead_voxels: DynArray<SeadVoxel>,
    patches_indices: DynArray<u16>,
    unknown15: Unknown15,
    patch_count: u32,
}

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
pub struct LinkHeader {
    link_name: Name,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: ObjectFlags,
    r#type: ObjectType,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct SurfaceBodyV1_381_67_09PC {
    points: DynArray<Vec3f>,
    vec4fs: DynArray<Vec4f>,
    unused2s: DynArray<Unused2>,
    unused3s: DynArray<Unused3>,
    patches: DynArray<Patch>,
    edges: DynArray<Edge>,
    normals: DynArray<Vec3f>,
    vertex9s: DynArray<Vec3f>,
    vertex10s: DynArray<Vec2f>,
    should_draw_relateds: DynArray<ShouldDrawBitfield>,
    unused12s: DynArray<Unused12>,
    sead_index: BffOption<SeadIndex>,
}

pub type SurfaceV1_381_67_09PC = TrivialClass<LinkHeader, SurfaceBodyV1_381_67_09PC>;
