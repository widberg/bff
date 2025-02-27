use bff_derive::{GenericClass, ReferencedNames};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::class::user_define::generic::UserDefineBodyGeneric;
use crate::helpers::{PascalString, ResourceObjectLinkHeader};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct UserDefineBodyV1_291_03_06PC {
    #[generic]
    data: PascalString,
}

pub type UserDefineV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeader, UserDefineBodyV1_291_03_06PC>;
