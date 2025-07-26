use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, PascalStringNull, ResourceObjectLinkHeaderV1_381_67_09PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
struct Prefab {
    string: PascalStringNull,
    in_world: u32,
    names: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_381_67_09PC))]
pub struct GameObjBodyV1_381_67_09PC {
    prefabs: DynArray<Prefab>,
}

pub type GameObjV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeaderV1_381_67_09PC, GameObjBodyV1_381_67_09PC>;

impl Export for GameObjV1_381_67_09PC {}
impl Import for GameObjV1_381_67_09PC {}
