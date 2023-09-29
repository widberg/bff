use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::{ObjectDatasFlagsV1_381_67_09PC, ResourceObjectLinkHeader};
use crate::math::Vec3f;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct FadeDistances {
    x: f32,
    y: f32,
    fade_close: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct ParticlesDataBodyV1_381_67_09PC {
    flags: ObjectDatasFlagsV1_381_67_09PC,
    position: Vec3f,
    fade: FadeDistances,
    shorts: DynArray<u16>,
    zero: u32,
}

pub type ParticlesDataV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeader, ParticlesDataBodyV1_381_67_09PC>;
