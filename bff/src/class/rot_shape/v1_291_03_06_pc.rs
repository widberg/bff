
use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ObjectLinkHeaderV1_06_63_02PC, Vec2f, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
struct PointsRelated0 {
    data: [u8; 16],
}

#[derive(..BffStruct)]
struct PointsRelated1 {
    data: [u8; 4],
}

#[derive(..BffStruct)]
struct Points {
    vertices: DynArray<Vec3f>,
    points_related0s: DynArray<PointsRelated0>,
    points_related1s: DynArray<PointsRelated1>,
}

#[derive(..BffStruct)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct RotShapeBodyV1_291_03_06PC {
    points: Points,
    material_indices: DynArray<u32>,
    local_vertices: DynArray<Vec3f>,
    local_uvs: DynArray<Vec2f>,
    material_anims: DynArray<Name>,
    scale: f32,
    rot_shape_type: u16,
}

pub type RotShapeV1_291_03_06PC =
    TrivialClass<ObjectLinkHeaderV1_06_63_02PC, RotShapeBodyV1_291_03_06PC>;

impl Export for RotShapeV1_291_03_06PC {}
impl Import for RotShapeV1_291_03_06PC {}
