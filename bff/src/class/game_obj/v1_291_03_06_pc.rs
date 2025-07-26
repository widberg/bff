use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, ResourceObjectLinkHeaderV1_06_63_02PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(Debug, BinRead, Serialize, BinWrite, Deserialize, ReferencedNames, JsonSchema)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct GameObjBodyV1_291_03_06PC {
    node_names: DynArray<Name>,
}

pub type GameObjV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, GameObjBodyV1_291_03_06PC>;

impl Export for GameObjV1_291_03_06PC {}
impl Import for GameObjV1_291_03_06PC {}
