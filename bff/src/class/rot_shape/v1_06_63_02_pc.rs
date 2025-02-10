use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::helpers::DynArray;
use crate::helpers::{Mat, Sphere, Vec2f, Vec3f};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize, ReferencedNames)]
struct PointsRelated0 {
    data: [u8; 12],
}

#[derive(BinRead, Debug, Serialize, ReferencedNames)]
struct PointsRelated1 {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, ReferencedNames)]
struct MorpherRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, ReferencedNames)]
struct MorphTargetDescRelated {
    data: [u8; 16],
}

#[derive(BinRead, Debug, Serialize, ReferencedNames)]
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
struct LinkInfo {
    object_link_header: ObjectV1_06_63_02PC,
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
    material_anim_names: DynArray<Name>,
    scale: f32,
    rot_shape_type: u16,
}

pub type RotShapeV1_06_63_02PC = TrivialClass<LinkInfo, RotShapeBodyV1_06_63_02PC>;
