use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::link_header::ResourceObjectLinkHeader;
use crate::strings::PascalString;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct UserDefineBodyV1_291_03_06PC {
    data: PascalString,
}

pub type UserDefineV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeader, UserDefineBodyV1_291_03_06PC>;
