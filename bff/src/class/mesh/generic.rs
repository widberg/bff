use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::helpers::{DynArray, RangeBeginSize, RangeFirstLast, Vec2f, Vec3f};
use crate::names::Name;

type VertexVectorComponent = u8;
type VertexVector3u8 = [VertexVectorComponent; 3];
type VertexBlendIndex = f32;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct CollisionAABB {
    min: Vec3f,
    collision_aabb_range: RangeFirstLast,
    max: Vec3f,
    collision_faces_range: RangeBeginSize,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
pub struct Strip {
    strip_vertices_indices: DynArray<u16>,
    material_name: Name,
    tri_order: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(vertex_layout: u32))]
pub enum Vertex {
    #[br(pre_assert(vertex_layout == 12))]
    Format12 { position: Vec3f },
    #[br(pre_assert(vertex_layout == 24))]
    Format24 {
        position: Vec3f,
        unknown: f32,
        uv: Vec2f,
    },
    #[br(pre_assert(vertex_layout == 36))]
    Format36 {
        position: Vec3f,
        tangent: VertexVector3u8,
        tangent_w: VertexVectorComponent,
        normal: VertexVector3u8,
        normal_w: VertexVectorComponent,
        uv: Vec2f,
        luv: Vec2f,
    },
    #[br(pre_assert(vertex_layout == 48))]
    Format48 {
        position: Vec3f,
        tangent: VertexVector3u8,
        tangent_w: VertexVectorComponent,
        normal: VertexVector3u8,
        normal_w: VertexVectorComponent,
        uv: Vec2f,
        blend_index: VertexBlendIndex,
        pad2: [i32; 3],
        blend_weight: f32,
    },
    #[br(pre_assert(vertex_layout == 60))]
    Format60 {
        position: Vec3f,
        tangent: VertexVector3u8,
        tangent_w: VertexVectorComponent,
        normal: VertexVector3u8,
        normal_w: VertexVectorComponent,
        uv: Vec2f,
        blend_indices: [VertexBlendIndex; 4],
        blend_weights: [f32; 4],
    },
    FormatUnknown {
        #[br(count = vertex_layout)]
        data: Vec<u8>,
    },
}

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames)]
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
