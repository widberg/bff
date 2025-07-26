use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ObjectDatasFlagsV1_381_67_09PC, ResourceObjectLinkHeaderV1_381_67_09PC};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct SurfaceDatasBodyV1_381_67_09PC {
    flags: ObjectDatasFlagsV1_381_67_09PC,
}

pub type SurfaceDatasV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, SurfaceDatasBodyV1_381_67_09PC>;

impl Export for SurfaceDatasV1_381_67_09PC {}
impl Import for SurfaceDatasV1_381_67_09PC {}
