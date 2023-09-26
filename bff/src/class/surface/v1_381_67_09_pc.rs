use bff_derive::serialize_bits;
use bilge::prelude::*;
use binrw::BinRead;
use serde::Serialize;
use serde_big_array::BigArray;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_381_67_09PC;
use crate::math::{Mat4f, RangeBeginSize, Vec2f, Vec3f, Vec4f};
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

#[serialize_bits]
#[bitsize(16)]
#[derive(BinRead, DebugBits)]
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

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
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

pub type SurfaceV1_381_67_09PC = TrivialClass<ObjectLinkHeaderV1_381_67_09PC, SurfaceBodyV1_381_67_09PC>;
