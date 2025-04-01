use bff_derive::{GenericClass, ReferencedNames, trivial_class};
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{PascalString, ResourceObjectLinkHeader};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames, GenericClass)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct UserDefineBodyV1_291_03_06PC {
    #[generic]
    pub data: PascalString,
}

trivial_class!(
    UserDefineV1_291_03_06PC(ResourceObjectLinkHeader, UserDefineBodyV1_291_03_06PC),
    UserDefineGeneric
);
