use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::name::Name;

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &()))]
pub struct GameObjBodyV1_291_03_06PC {
    node_crc32s: DynArray<Name>,
}

pub type GameObjV1_291_03_06PC = TrivialClass<(), GameObjBodyV1_291_03_06PC>;
