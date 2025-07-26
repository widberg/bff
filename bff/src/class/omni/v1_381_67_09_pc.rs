use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{Mat4f, ObjectLinkHeaderV1_381_67_09PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct OmniBodyV1_381_67_09PC {
    scale_matrix: Mat4f,
    translation_matrix: Mat4f,
    unknown0: f32,
    unknown1: f32,
    unknown2: f32,
    unknown3: f32,
    unknown4: f32,
    unknown5: f32,
    unknown6: f32,
    unknown7: f32,
    unknown8: f32,
    unknown9: f32,
    unknown10: i32,
    unknown11: f32,
    unknown12: f32,
    unknown13: f32,
    unknown14: f32,
    unknown15: f32,
    material_anim_name0: Name,
    material_anim_name1: Name,
}

pub type OmniV1_381_67_09PC = TrivialClass<ObjectLinkHeaderV1_381_67_09PC, OmniBodyV1_381_67_09PC>;

impl Export for OmniV1_381_67_09PC {}
impl Import for OmniV1_381_67_09PC {}
