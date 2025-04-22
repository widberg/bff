use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{ResourceDatasFlagsV1_381_67_09PC, ResourceLinkHeader};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceLinkHeader))]
pub struct MeshDataBodyV1_381_67_09PC {
    flags: ResourceDatasFlagsV1_381_67_09PC,
    zeroes: [u32; 4],
}

pub type MeshDataV1_381_67_09PC = TrivialClass<ResourceLinkHeader, MeshDataBodyV1_381_67_09PC>;

impl Export for MeshDataV1_381_67_09PC {}
impl Import for MeshDataV1_381_67_09PC {}
