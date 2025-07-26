use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite, args};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::{DynArray, RangeBeginSize, Vec2f, Vec3f};
use crate::names::Name;

type VertexVectorComponent = u8;
type VertexVector3u8 = [VertexVectorComponent; 3];
type VertexBlendIndex = f32;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(is_leaf: bool))]
pub enum CollisionFacesRange {
    #[br(pre_assert(is_leaf))]
    Range(RangeBeginSize),
    #[br(pre_assert(!is_leaf))]
    Pointer(u32),
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct AABBNode {
    min: Vec3f,
    #[br(map = |x: (u16, u16)| (x != (0, 0)).then(|| (x.0 - 1, x.1 - 1)))]
    #[bw(map = |x: &Option<(u16, u16)>| x.map(|x| (x.0 + 1, x.1 + 1)).unwrap_or((0, 0)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    collision_aabb_children: Option<(u16, u16)>,
    max: Vec3f,
    #[br(args(collision_aabb_children.is_none()))]
    collision_faces_range: CollisionFacesRange,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct Strip {
    strip_vertices_indices: DynArray<u16>,
    material_name: Name,
    tri_order: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct LayoutPosition {
    pub position: Vec3f,
}

impl LayoutPosition {
    pub const SIZE: usize = 12;
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct LayoutPositionUV {
    pub position: Vec3f,
    pub unknown: f32,
    pub uv: Vec2f,
}

impl LayoutPositionUV {
    pub const SIZE: usize = 24;
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct LayoutNoBlend {
    pub position: Vec3f,
    pub tangent: VertexVector3u8,
    pub tangent_w: VertexVectorComponent,
    pub normal: VertexVector3u8,
    pub normal_w: VertexVectorComponent,
    pub uv: Vec2f,
    pub luv: Vec2f,
}

impl LayoutNoBlend {
    pub const SIZE: usize = 36;
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct Layout1Blend {
    pub position: Vec3f,
    pub tangent: VertexVector3u8,
    pub tangent_w: VertexVectorComponent,
    pub normal: VertexVector3u8,
    pub normal_w: VertexVectorComponent,
    pub uv: Vec2f,
    pub blend_index: VertexBlendIndex,
    pub pad2: [i32; 3],
    pub blend_weight: f32,
}

impl Layout1Blend {
    pub const SIZE: usize = 48;
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct Layout4Blend {
    pub position: Vec3f,
    pub tangent: VertexVector3u8,
    pub tangent_w: VertexVectorComponent,
    pub normal: VertexVector3u8,
    pub normal_w: VertexVectorComponent,
    pub uv: Vec2f,
    pub blend_indices: [VertexBlendIndex; 4],
    pub blend_weights: [f32; 4],
}

impl Layout4Blend {
    pub const SIZE: usize = 60;
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(count: usize, layout: usize))]
pub enum Vertices {
    #[br(pre_assert(layout == LayoutPosition::SIZE))]
    LayoutPosition(#[br(count = count)] Vec<LayoutPosition>),
    #[br(pre_assert(layout == LayoutPositionUV::SIZE))]
    LayoutPositionUV(#[br(count = count)] Vec<LayoutPositionUV>),
    #[br(pre_assert(layout == LayoutNoBlend::SIZE))]
    LayoutNoBlend(#[br(count = count)] Vec<LayoutNoBlend>),
    #[br(pre_assert(layout == Layout1Blend::SIZE))]
    Layout1Blend(#[br(count = count)] Vec<Layout1Blend>),
    #[br(pre_assert(layout == Layout4Blend::SIZE))]
    Layout4Blend(#[br(count = count)] Vec<Layout4Blend>),
    LayoutUnknown {
        #[br(calc = layout)]
        #[bw(ignore)]
        layout: usize,
        #[br(args { count: count, inner: args! { count: layout } })]
        data: Vec<Vec<u8>>,
    },
}

impl Vertices {
    pub fn len(&self) -> usize {
        match self {
            Self::LayoutPosition(layout_positions) => layout_positions.len(),
            Self::LayoutPositionUV(layout_position_uvs) => layout_position_uvs.len(),
            Self::LayoutNoBlend(layout_no_blends) => layout_no_blends.len(),
            Self::Layout1Blend(layout1_blends) => layout1_blends.len(),
            Self::Layout4Blend(layout4_blends) => layout4_blends.len(),
            Self::LayoutUnknown { data, .. } => data.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::LayoutPosition(layout_positions) => layout_positions.is_empty(),
            Self::LayoutPositionUV(layout_position_uvs) => layout_position_uvs.is_empty(),
            Self::LayoutNoBlend(layout_no_blends) => layout_no_blends.is_empty(),
            Self::Layout1Blend(layout1_blends) => layout1_blends.is_empty(),
            Self::Layout4Blend(layout4_blends) => layout4_blends.is_empty(),
            Self::LayoutUnknown { data, .. } => data.is_empty(),
        }
    }

    pub fn layout(&self) -> usize {
        match self {
            Self::LayoutPosition(_) => LayoutPosition::SIZE,
            Self::LayoutPositionUV(_) => LayoutPositionUV::SIZE,
            Self::LayoutNoBlend(_) => LayoutNoBlend::SIZE,
            Self::Layout1Blend(_) => Layout1Blend::SIZE,
            Self::Layout4Blend(_) => Layout4Blend::SIZE,
            Self::LayoutUnknown { layout, .. } => *layout,
        }
    }
}

#[bitsize(32)]
#[derive(
    BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames, JsonSchema,
)]
pub struct VertexGroupFlags {
    padding: u2,
    visible: u1,
    padding0: u13,
    unknown0: u1,
    unknown1: u1,
    unknown2: u1,
    morph: u1,
    padding1: u12,
}
