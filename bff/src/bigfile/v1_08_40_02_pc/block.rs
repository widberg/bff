use binrw::BinRead;
use serde::Serialize;

use super::object::Object;
use crate::bigfile::v1_06_63_02_pc::header::BlockDescription;

#[derive(BinRead, Serialize, Debug)]
#[br(import(block_description: &BlockDescription))]
pub struct Block {
    #[br(count = block_description.object_count, align_after = 2048)]
    pub objects: Vec<Object>,
}
