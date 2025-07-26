use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffOption,
    DynArray,
    Mat4f,
    ObjectLinkHeaderV1_381_67_09PC,
    RangeBeginSize,
    Vec2f,
    Vec3f,
    Vec4f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unused2 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unused3 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
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

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Edge {
    p: [u16; 2],
    t: [u16; 2],
}

#[bitsize(16)]
#[derive(
    BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames, JsonSchema,
)]
struct ShouldDrawBitfield {
    index_in_draw_info_array: u3,
    shift_amount_for_bit: u5,
    other: u8,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unused12 {
    data: [u8; 32],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SeadVoxel {
    patches_indices_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unknown15 {
    data1: [u32; 32],
    data2: [u32; 16],
    sead_voxel_count: u32,
    patch_count_related: u32,
    unknown0s: [u32; 2],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct SeadIndex {
    sead_voxels: DynArray<SeadVoxel>,
    patches_indices: DynArray<u16>,
    unknown15: Unknown15,
    patch_count: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    sead_index: BffOption<SeadIndex>,
}

pub type SurfaceV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, SurfaceBodyV1_381_67_09PC>;

impl Export for SurfaceV1_381_67_09PC {}
impl Import for SurfaceV1_381_67_09PC {}
