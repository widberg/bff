use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffMap,
    BffOption,
    DynArray,
    DynBox,
    DynSphere,
    ObjectLinkHeaderV1_381_67_09PC,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct FadeDistances {
    x: f32,
    y: f32,
    fade_close: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ObjectLinkHeaderV1_381_67_09PC))]
pub struct LodBodyV1_381_67_09PC {
    collision_spheres: DynArray<DynSphere>,
    collision_boxes: DynArray<DynBox>,
    fade: FadeDistances,
    skin_or_mesh_or_particles_names: DynArray<Name>,
    zero: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    animation_entries: BffOption<BffMap<u32, Name>, u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    sound_entries: BffOption<BffMap<u32, Name>, u32>,
    user_define_name: Name,
}

pub type LodV1_381_67_09PC = TrivialClass<ObjectLinkHeaderV1_381_67_09PC, LodBodyV1_381_67_09PC>;

impl Export for LodV1_381_67_09PC {}
impl Import for LodV1_381_67_09PC {}
