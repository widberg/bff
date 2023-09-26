use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::link_header::ResourceObjectLinkHeader;
use crate::map::BffMap;
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct MaterialObjBodyV1_381_67_09PC {
    entries: BffMap<Name, DynArray<Name>>,
}

pub type MaterialObjV1_381_67_09PC =
    TrivialClass<ResourceObjectLinkHeader, MaterialObjBodyV1_381_67_09PC>;
