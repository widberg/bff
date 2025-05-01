use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_06_63_02PC, Vec2f, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct PointsRelated0 {
    data: [u8; 12],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct PointsRelated1 {
    data: [u8; 16],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct MorpherRelated {
    data: [u8; 16],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct MorphTargetDescRelated {
    data: [u8; 16],
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct MorphTargetDesc {
    name: u32,
    morph_target_desc_relateds: DynArray<MorphTargetDescRelated>,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
struct Morpher {
    morpher_relateds: DynArray<MorpherRelated>,
    morph_target_descs: DynArray<MorphTargetDesc>,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
pub struct LinkInfo {
    resource_link_header: ResourceObjectLinkHeaderV1_06_63_02PC,
    vertices: DynArray<Vec3f>,
    points_relateds1: DynArray<PointsRelated1>,
    morpher: Morpher,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames)]
#[br(import(_link_header: &LinkInfo))]
pub struct RotShapeBodyV1_06_63_02PC {
    material_indices: DynArray<u32>,
    local_vertices: DynArray<Vec3f>,
    local_uvs: DynArray<Vec2f>,
    material_anim_names: DynArray<Name>,
    scale: f32,
    rot_shape_type: u16,
}

pub type RotShapeV1_06_63_02PC = TrivialClass<LinkInfo, RotShapeBodyV1_06_63_02PC>;

impl Export for RotShapeV1_06_63_02PC {}
impl Import for RotShapeV1_06_63_02PC {}
