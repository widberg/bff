use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ObjectLinkHeaderV1_06_63_02PC, Vec3f, Vec4f};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Segment {
    vertices: [Vec3f; 2],
    length: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Spline {
    point_id: [u16; 2],
    tangent_id: [u16; 2],
    flag: u32,
    length: f32,
    segments: [Segment; 8],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct SplineBodyV1_06_63_02PC {
    points: DynArray<Vec3f>,
    splines: DynArray<Spline>,
    vec: Vec4f,
    length: f32,
}

pub type SplineV1_06_63_02PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, SplineBodyV1_06_63_02PC>;

impl Export for SplineV1_06_63_02PC {}
impl Import for SplineV1_06_63_02PC {}
