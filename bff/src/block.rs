use binrw::BinRead;
use serde::Serialize;

use crate::object::Object;

#[derive(BinRead, Serialize, Debug)]
pub struct Block {
    objects: Vec<Object>,
}
