use binrw::BinRead;

use super::header::BlockDescription;
use crate::bigfile::v1_06_63_02_pc::resource::Resource;

#[derive(BinRead, Debug)]
#[br(import(block_description: &BlockDescription))]
pub struct Block {
    #[br(count = block_description.resource_count, align_after = 2048)]
    pub resources: Vec<Resource>,
}
