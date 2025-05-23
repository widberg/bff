use binrw::BinRead;

use super::resource::Resource;
use crate::bigfile::v1_06_63_02_pc::header::BlockDescription;

#[derive(BinRead, Debug)]
#[br(import(block_description: &BlockDescription))]
pub struct Block {
    #[br(count = block_description.resource_count, align_after = 2048)]
    pub resources: Vec<Resource>,
}
