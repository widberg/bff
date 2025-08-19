use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, Mat4f, ResourceObjectLinkHeaderV1_381_67_09PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Unknown2 {
    placeholder0: u32,
    placeholder1: u32,
    index: u32,
    placeholder2: u32,
    unknown4: u32,
    zero: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct WorldBodyV1_381_67_09PC {
    node_name0: Name,
    warp_name: Name,
    game_obj_name: Name,
    unused14: Name,
    gen_world_name: Name,
    node_name1: Name,
    unused17s: DynArray<u32>,
    unuseds: DynArray<u8>,
    unknown0: Mat4f,
    indices0: DynArray<i32>,
    unknown2s: DynArray<Unknown2>,
    unknown3s: Mat4f,
    indices1: DynArray<i32>,
    unknown5s: DynArray<Unknown2>,
    unused6s: DynArray<Name>,
    unused7s: DynArray<Name>,
    unused8s: DynArray<Name>,
    unused9s: DynArray<Name>,
    unused10s: DynArray<Name>,
    spline_graph_names: DynArray<Name>,
    unused12s: DynArray<Name>,
    material_anim_name: DynArray<Name>,
}

pub type WorldV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, WorldBodyV1_381_67_09PC>;

impl Export for WorldV1_381_67_09PC {}
impl Import for WorldV1_381_67_09PC {}
