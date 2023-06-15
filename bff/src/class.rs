use binrw::BinRead;
use serde::Serialize;

use crate::object::Object;

#[derive(BinRead, Serialize, Debug)]
#[serde(untagged)]
#[br(import(object: Object))]
pub enum Class {
    #[br(pre_assert(object.class_name() == 0x52F79F96))]
    UserDefine(),
}
