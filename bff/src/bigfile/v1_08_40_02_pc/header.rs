use binrw::binrw;
use serde::Serialize;

use crate::bigfile::v1_06_63_02_pc::header::BlockDescription;
use crate::versions::VersionTriple;

#[binrw]
#[derive(Serialize, Debug)]
pub struct Header {
    #[br(temp)]
    #[bw(calc = block_descriptions.len() as u32)]
    block_count: u32,
    pub block_working_buffer_capacity_even: u32,
    pub block_working_buffer_capacity_odd: u32,
    pub total_padded_block_size: u32,
    pub version_triple: VersionTriple,
    #[serde(skip)]
    #[br(count = block_count)]
    #[brw(align_after = 2048)]
    pub block_descriptions: Vec<BlockDescription>,
}
