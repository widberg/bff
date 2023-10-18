use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{BffMap, DynArray, ResourceObjectLinkHeader};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct MaterialObjBodyV1_381_67_09PC {
    entries: BffMap<Name, DynArray<Name>>,
}

pub type MaterialObjV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeader, MaterialObjBodyV1_381_67_09PC>;
