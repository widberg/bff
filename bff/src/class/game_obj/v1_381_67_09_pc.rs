use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::name::Name;
use crate::strings::PascalStringNull;

#[derive(BinRead, Debug, Serialize)]
struct Prefab {
    string: PascalStringNull,
    in_world: u32,
    names: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize)]
pub struct ResourceObject {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObject))]
pub struct GameObjBodyV1_381_67_09PC {
    prefabs: DynArray<Prefab>,
}

pub type GameObjV1_381_67_09PC = TrivialClass<ResourceObject, GameObjBodyV1_381_67_09PC>;
