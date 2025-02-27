use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{PascalString, ResourceObjectLinkHeader};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct UserDefineBodyGeneric {
    pub data: PascalString,
}

pub type UserDefineGeneric =
    TrivialClass<ResourceObjectLinkHeader, UserDefineBodyGeneric>;
