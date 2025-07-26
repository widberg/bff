use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::ObjectLinkHeaderV1_381_67_09PC;
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct CameraBodyV1_381_67_09PC {
    angle_of_view: f32,
    zero: f32,
    node_name: Name,
}

pub type CameraV1_381_67_09PC =
    TrivialClass<ObjectLinkHeaderV1_381_67_09PC, CameraBodyV1_381_67_09PC>;

impl Export for CameraV1_381_67_09PC {}
impl Import for CameraV1_381_67_09PC {}
