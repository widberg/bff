use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::ResourceLinkHeaderV1_381_67_09PC;
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceLinkHeaderV1_381_67_09PC))]
pub struct CameraBodyV1_381_67_09PC {
    angle_of_view: f32,
    zero: f32,
    node_name: Name,
}

pub type CameraV1_381_67_09PC =
    TrivialClass<ResourceLinkHeaderV1_381_67_09PC, CameraBodyV1_381_67_09PC>;

impl Export for CameraV1_381_67_09PC {}
impl Import for CameraV1_381_67_09PC {}
