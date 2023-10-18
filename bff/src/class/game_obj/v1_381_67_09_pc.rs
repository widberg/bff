use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, PascalStringNull, ResourceObjectLinkHeader};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct Prefab {
    string: PascalStringNull,
    in_world: u32,
    names: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct GameObjBodyV1_381_67_09PC {
    prefabs: DynArray<Prefab>,
}

pub type GameObjV1_381_67_09PC = TrivialClass<ResourceObjectLinkHeader, GameObjBodyV1_381_67_09PC>;
