use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffMap,
    DynArray,
    FixedStringNull,
    Mat4f,
    ObjectLinkHeaderV1_381_67_09PC,
    PascalStringNull,
    Vec2f,
    Vec3f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Category {
    one: u32,
    node_name_arrays: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct CAFlatSurface {
    zero0: u32,
    mat: Mat4f,
    a: f32,
    b: f32,
    c: f32,
    reciprocal: f32,
    vec: Vec3f,
    unknown1: f32,
    unknown3: f32,
    zero1: u32,
    zero2: u32,
    zero3: u32,
    zero4: u32,
    unknown9: i32,
    unknown4: u8,
    unknown20: u8,
    unknown21: u8,
    unknown22: u8,
    unknown23: u8,
    unknown24: u8,
    unknown2: u8,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Unused10 {
    unused0: u32,
    unused1s: [u32; 8],
    unused2: u32,
    unused3: u32,
    unused4: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct RegionEdge {
    region_vertices_index_a: u32,
    region_vertices_index_b: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Region {
    unknown: u8,
    region_edges_indices: DynArray<u32>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct GenWorldBodyV1_381_67_09PC {
    node_name: Name,
    user_define_name: Name,
    gw_road_name: Name,
    binary_names: DynArray<Name>,
    bitmap_names: DynArray<Name>,
    material_names: DynArray<Name>,
    equals41: u32,
    categories: BffMap<PascalStringNull, Category>,
    ca_flat_surfaces: DynArray<CAFlatSurface>,
    cancel_auto_mesh_placement: DynArray<Mat4f>,
    unused10s: DynArray<Unused10>,
    region_vertices: DynArray<Vec2f>,
    region_edges: DynArray<RegionEdge>,
    regions: BffMap<FixedStringNull<31>, Region>,
}

pub type GenWorldV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, GenWorldBodyV1_381_67_09PC>;

impl Export for GenWorldV1_381_67_09PC {}
impl Import for GenWorldV1_381_67_09PC {}
