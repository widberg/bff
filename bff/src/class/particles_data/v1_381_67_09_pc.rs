use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceDatasFlagsV1_381_67_09PC, ResourceLinkHeader, Vec3f};
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct FadeDistances {
    x: f32,
    y: f32,
    fade_close: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceLinkHeader))]
pub struct ParticlesDataBodyV1_381_67_09PC {
    flags: ResourceDatasFlagsV1_381_67_09PC,
    position: Vec3f,
    fade: FadeDistances,
    shorts: DynArray<u16>,
    zero: u32,
}

pub type ParticlesDataV1_381_67_09PC =
    TrivialClass<ResourceLinkHeader, ParticlesDataBodyV1_381_67_09PC>;

impl Export for ParticlesDataV1_381_67_09PC {}
impl Import for ParticlesDataV1_381_67_09PC {}
