use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ObjectLinkHeaderV1_06_63_02PC;
use crate::math::{Vec3f, Vec4f};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Segment {
    vertices: [Vec3f; 2],
    length: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Spline {
    point_id: [u16; 2],
    tangent_id: [u16; 2],
    flag: u32,
    length: f32,
    segments: [Segment; 8],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct SplineBodyV1_06_63_02PC {
    points: DynArray<Vec3f>,
    splines: DynArray<Spline>,
    vec: Vec4f,
    length: f32,
}

pub type SplineV1_06_63_02PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, SplineBodyV1_06_63_02PC>;
