use std::io::SeekFrom;

use binrw::*;
use serde::Serialize;

use crate::bigfile::v1_06_63_02_pc::header::{BigFileType, BlockDescription};
use crate::versions::VersionOneple;

#[binrw]
#[derive(Serialize, Debug)]
pub struct Header {
    pub version_oneple: VersionOneple,
    pub bigfile_type: BigFileType,
    #[br(temp)]
    #[bw(calc = block_descriptions.len() as u32)]
    block_count: u32,
    pub block_working_buffer_capacity_even: u32,
    pub block_working_buffer_capacity_odd: u32,
    pub total_padded_block_size: u32,
    #[serde(skip)]
    #[br(count = block_count)]
    pub block_descriptions: Vec<BlockDescription>,
    #[br(ignore)]
    pub tag: Option<Vec<u8>>,
    #[brw(seek_before = SeekFrom::Start(0x72C))]
    pub block_sector_padding_size: u32,
    pub pool_sector_padding_size: u32,
    pub file_size: u32,
    pub total_decompressed_size: u32,
    #[brw(align_after = 2048)]
    pub total_resource_count: u32,
}
