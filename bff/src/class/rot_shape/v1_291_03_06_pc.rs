use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat, Sphere, Vec2f, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Box {
    matrix: Mat<3, 4>,
    vector: Vec3f,
    scale: f32,
}

#[derive(BinRead, Debug, Serialize)]
struct PointsRelated0 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
struct PointsRelated1 {
    data: [u8; 4],
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkInfo {
    link_crc32: Name,
    links: DynArray<Name>,
    data_crc32: Name,
    b_sphere_local: Sphere,
    b_box: Box,
    fade_out_distance: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct Points {
    vertices: DynArray<Vec3f>,
    points_related0s: DynArray<PointsRelated0>,
    points_related1s: DynArray<PointsRelated1>,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkInfo))]
pub struct RotShapeBodyV1_291_03_06PC {
    points: Points,
    material_indices: DynArray<u32>,
    local_vertices: DynArray<Vec3f>,
    local_uvs: DynArray<Vec2f>,
    material_anims: DynArray<Name>,
    scale: f32,
    rot_shape_type: u16,
}

pub type RotShapeV1_291_03_06PC = TrivialClass<LinkInfo, RotShapeBodyV1_291_03_06PC>;
