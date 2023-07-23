use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::name::Name;

#[derive(Debug, BinRead, Serialize)]
#[br(import(_link_header: &()))]
pub struct GameObjBodyV1_291_03_06PC {
    node_crc32s: DynArray<Name>,
}

pub type GameObjV1_291_03_06PC = TrivialClass<(), GameObjBodyV1_291_03_06PC>;
