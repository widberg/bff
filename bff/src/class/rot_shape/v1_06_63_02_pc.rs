use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat, Sphere, Vec2f, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
struct Box {
    mat: Mat<3, 4>,
    vec: Vec3f,
    maybe_scale: f32,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkInfo {
    link_crc32: Name,
    linked_crc32: DynArray<Name>,
    data_crc32: Name,
    b_sphere_local: Sphere,
    b_box: Box,
    fade_out_distance: f32,
    flags: u32,
    r#type: u16,
}

#[derive(BinRead, Debug, Serialize)]
struct ObjectHeader {
    data_size: u32,
    link_size: u32,
    decompressed_size: u32,
    compressed_size: u32,
    class_crc32: Name,
    name_crc32: Name,
    link_info: LinkInfo,
}

#[derive(BinRead, Debug, Serialize)]
struct PointsRelated0 {
    data: [u8; 12],
}

#[derive(BinRead, Debug, Serialize)]
struct PointsRelated1 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
struct MorpherRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
struct MorphTargetDescRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize)]
struct MorphTargetDesc {
    name: u32,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[derive(BinRead, Debug, Serialize)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morph_target_descs: DynArray<MorphTargetDesc>,
}

#[derive(BinRead, Debug, Serialize)]
struct Points {
    header: ObjectHeader,
    vertices: DynArray<Vec3f>,
    points_relateds1: DynArray<PointsRelated1>,
    morpher: Morpher,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkInfo))]
pub struct RotShapeBodyV1_06_63_02PC {
    points: Points,
    material_indices: DynArray<u32>,
    local_vertices: DynArray<Vec3f>,
    local_uvs: DynArray<Vec2f>,
    material_anim_crc32s: DynArray<Name>,
    scale: f32,
    rot_shape_type: u16,
}

pub type RotShapeV1_06_63_02PC = TrivialClass<LinkInfo, RotShapeBodyV1_06_63_02PC>;
