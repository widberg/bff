use binrw::*;
use serde::Serialize;

use crate::bigfile::v1_06_63_02_pc::header::BlockDescription;
use crate::versions::VersionTriple;

#[binread]
#[derive(Serialize, Debug)]
pub struct Header {
    #[br(temp)]
    block_count: u32,
    block_working_buffer_capacity_even: u32,
    block_working_buffer_capacity_odd: u32,
    #[br(temp)]
    _padded_size: u32,
    pub version_triple: VersionTriple,
    #[serde(skip)]
    #[br(count = block_count, align_after = 2048)]
    pub block_descriptions: Vec<BlockDescription>,
}
