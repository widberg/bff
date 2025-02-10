use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::DynArray;
use crate::names::Name;

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct GameObjBodyV1_291_03_06PC {
    node_names: DynArray<Name>,
}

pub type GameObjV1_291_03_06PC = TrivialClass<(), GameObjBodyV1_291_03_06PC>;
