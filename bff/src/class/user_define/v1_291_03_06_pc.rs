use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::strings::PascalString;

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &()))]
pub struct UserDefineBodyV1_291_03_06PC {
    data: PascalString,
}

pub type UserDefineV1_291_03_06PC = TrivialClass<(), UserDefineBodyV1_291_03_06PC>;
