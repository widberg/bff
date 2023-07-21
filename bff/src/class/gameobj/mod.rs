use bff_derive::{bff_forms, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;
use v1_291_03_06_pc::GameObjV1_291_03_06PC;

use crate::dynarray::DynArray;
use crate::name::Name;

#[derive(Serialize, Debug, NamedClass)]
#[bff_forms((V1_291_03_06, PC) | (V1_291_03_01, PSP) => GameObjV1_291_03_06PC)]
pub struct GameObj {
    node_crc32s: DynArray<Name>,
}
