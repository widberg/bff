use binrw::BinRead;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::strings::PascalString;

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &()))]
pub struct UserDefineBodyV1_291_03_06PC {
    data: PascalString,
}

impl UserDefineBodyV1_291_03_06PC {
    pub fn data(&self) -> &PascalString {
        &self.data
    }
}

pub type UserDefineV1_291_03_06PC = TrivialClass<(), UserDefineBodyV1_291_03_06PC>;
