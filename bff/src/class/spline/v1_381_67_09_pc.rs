use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ObjectLinkHeaderV1_381_67_09PC, Vec3f, Vec4f};
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
struct SplineSegmentSubdivision {
    p: [Vec3f; 2],
    length: f32,
}

#[derive(..BffStruct)]
struct SplineSegment {
    p: [u16; 2],
    t: [u16; 2],
    flags: u32,
    length: f32,
    spline_segment_subdivisions: [SplineSegmentSubdivision; 8],
}

#[derive(..BffStruct)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct SplineBodyV1_381_67_09PC {
    points: DynArray<Vec3f>,
    spline_segments: DynArray<SplineSegment>,
    vec: Vec4f,
    length: f32,
}

pub type SplineV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, SplineBodyV1_381_67_09PC>;

impl Export for SplineV1_381_67_09PC {}
impl Import for SplineV1_381_67_09PC {}
