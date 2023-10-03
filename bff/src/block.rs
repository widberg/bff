use binrw::BinRead;
use serde::Serialize;

use crate::header::BlockDescription;
use crate::object::Object;

#[derive(BinRead, Serialize, Debug)]
#[br(import(block_description: &BlockDescription))]
pub struct Block {
    #[br(count = block_description.object_count, align_after = 2048)]
    pub objects: Vec<Object>,
}
