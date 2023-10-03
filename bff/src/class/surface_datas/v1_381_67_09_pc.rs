use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::link_header::{ObjectDatasFlagsV1_381_67_09PC, ResourceObjectLinkHeader};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct SurfaceDatasBodyV1_381_67_09PC {
    flags: ObjectDatasFlagsV1_381_67_09PC,
}

pub type SurfaceDatasV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeader, SurfaceDatasBodyV1_381_67_09PC>;
