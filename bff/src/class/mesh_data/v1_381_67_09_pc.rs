use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::link_header::{ObjectDatasFlagsV1_381_67_09PC, ResourceObjectLinkHeader};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct MeshDataBodyV1_381_67_09PC {
    flags: ObjectDatasFlagsV1_381_67_09PC,
    zero0: u32,
    zero1: u32,
    zero2: u32,
    zero3: u32,
}

pub type MeshDataV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeader, MeshDataBodyV1_381_67_09PC>;
