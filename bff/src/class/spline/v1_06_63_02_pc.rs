use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat, Sphere, Vec3f, Vec4f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Box {
    mat: Mat<3, 4>,
    vec: Vec3f,
    scale: f32,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkInfo {
    link_name: Name,
    links: DynArray<u32>,
    data_crc32: Name,
    b_sphere_local: Sphere,
    b_box: Box,
    fade_out_distance: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct Segment {
    vertices: [Vec3f; 2],
    length: f32,
}

#[derive(BinRead, Debug, Serialize)]
struct Spline {
    point_id: [u16; 2],
    tangent_id: [u16; 2],
    flag: u32,
    length: f32,
    segments: [Segment; 8],
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkInfo))]
pub struct SplineBodyV1_06_63_02PC {
    points: DynArray<Vec3f>,
    splines: DynArray<Spline>,
    vec: Vec4f,
    length: f32,
}

pub type SplineV1_06_63_02PC = TrivialClass<LinkInfo, SplineBodyV1_06_63_02PC>;
