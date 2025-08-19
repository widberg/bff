use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ObjectLinkHeaderV1_381_67_09PC, Vec2f, Vec3f};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[brw(repr = u16)]
enum BillboardMode {
    YBillboard = 0,
    CompleteBillboard = 1,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct RotShapeBodyV1_381_67_09PC {
    origins: DynArray<Vec3f>,
    zero: f32,
    material_anim_names_indices: DynArray<u32>,
    sizes: DynArray<Vec3f>,
    texcoords: DynArray<Vec2f>,
    material_anim_names: DynArray<Name>,
    scale: f32,
    billboard_mode: BillboardMode,
}

pub type RotShapeV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, RotShapeBodyV1_381_67_09PC>;

impl Export for RotShapeV1_381_67_09PC {}
impl Import for RotShapeV1_381_67_09PC {}
