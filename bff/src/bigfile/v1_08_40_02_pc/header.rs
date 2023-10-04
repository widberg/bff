use binrw::*;
use serde::Serialize;

use crate::versions::VersionTriple;

#[derive(Serialize, Debug, BinRead, BinWrite)]
pub struct BlockDescription {
    pub object_count: u32,
    padded_size: u32,
    data_size: u32,
    pub working_buffer_offset: u32,
    first_object_name: u32,
    #[br(map = |checksum: i32| if checksum == 0 { None } else { Some(checksum) })]
    #[bw(map = |checksum: &Option<i32>| checksum.unwrap_or(0))]
    pub checksum: Option<i32>,
}

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
