use std::io::SeekFrom;

use binrw::*;
use serde::Serialize;

use crate::strings::FixedStringNull;
use crate::versions::VersionTriple;

#[derive(Serialize, Debug, BinRead, BinWrite)]
pub struct BlockDescription {
    pub object_count: u32,
    padded_size: u32,
    data_size: u32,
    pub working_buffer_offset: u32,
    first_object_name: u32,
    #[br(map = |checksum: i32| if checksum == 0 { None } else { Some(checksum) })]
    pub checksum: Option<i32>,
}

#[binread]
#[derive(Serialize, Debug)]
pub struct Header {
    #[br(map = |is_not_rtc: u32| is_not_rtc == 0)]
    pub is_rtc: bool,
    #[br(temp)]
    block_count: u32,
    block_working_buffer_capacity_even: u32,
    block_working_buffer_capacity_odd: u32,
    #[br(temp)]
    _padded_size: u32,
    pub version_triple: VersionTriple,
    #[serde(skip)]
    #[br(count = block_count)]
    pub block_descriptions: Vec<BlockDescription>,
    #[br(temp, seek_before = SeekFrom::Start(0x720))]
    _pool_manifest_padded_size: u32,
    #[br(map = |pool_offset: u32| if pool_offset != u32::MAX && pool_offset != 0 { Some(pool_offset * 2048) } else { None })]
    pub pool_offset: Option<u32>,
    #[br(map = |pool_manifest_unused: u32| if pool_manifest_unused != u32::MAX { Some(pool_manifest_unused) } else { None })]
    pub pool_manifest_unused: Option<u32>,
    #[br(temp)]
    _pool_manifest_unused1: u32,
    #[br(temp)]
    _pool_object_decompression_buffer_capacity: u32,
    #[br(temp)]
    _block_sector_padding_size: u32,
    #[br(temp)]
    _pool_sector_padding_size: u32,
    #[br(temp)]
    _file_size: u32,
    #[br(try, align_after = 2048)]
    #[br(map = |incredi_builder_string: FixedStringNull<128>| Some(incredi_builder_string.into()))]
    pub incredi_builder_string: Option<String>,
}
