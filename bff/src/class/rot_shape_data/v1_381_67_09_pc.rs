use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ObjectDatasFlagsV1_381_67_09PC, ResourceObjectLinkHeader};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct RotShapeDataBodyV1_381_67_09PC {
    flags: ObjectDatasFlagsV1_381_67_09PC,
    zeros: DynArray<u16>,
    #[br(count = zeros.len() * 28)]
    pad: Vec<u8>,
}

pub type RotShapeDataV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeader, RotShapeDataBodyV1_381_67_09PC>;
