use binrw::BinRead;
use serde::Serialize;

use crate::object::Object;

#[derive(BinRead, Serialize, Debug)]
#[br(import(object_count : u32))]
pub struct Block {
    #[br(count = object_count, align_after = 2048)]
    objects: Vec<Object>,
}
