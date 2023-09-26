use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ResourceObjectLinkHeader;
use crate::name::Name;
use crate::strings::PascalStringNull;

#[derive(BinRead, Debug, Serialize)]
struct Prefab {
    string: PascalStringNull,
    in_world: u32,
    names: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct GameObjBodyV1_381_67_09PC {
    prefabs: DynArray<Prefab>,
}

pub type GameObjV1_381_67_09PC = TrivialClass<ResourceObjectLinkHeader, GameObjBodyV1_381_67_09PC>;
