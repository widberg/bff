use ascii::AsciiString;
use bff_derive::{bff_forms, NamedClass};
use serde::Serialize;

mod v1_291_03_06_pc;

use v1_291_03_06_pc::UserDefineV1_291_03_06PC;

#[derive(Serialize, Debug, NamedClass)]
#[bff_forms((V1_291_03_06, PC) => UserDefineV1_291_03_06PC)]
pub struct UserDefine {
    data: AsciiString,
}
