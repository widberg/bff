use std::io::SeekFrom;

use binrw::*;
use serde::Serialize;

use crate::names::Name;
use crate::strings::FixedStringNull;
use crate::versions::VersionTriple;

#[derive(Serialize, Debug, BinRead, BinWrite)]
pub struct BlockDescription {
    pub object_count: u32,
    pub padded_size: u32,
    pub data_size: u32,
    pub working_buffer_offset: u32,
    pub first_object_name: Name,
    #[br(map = |checksum: i32| if checksum == 0 { None } else { Some(checksum) })]
    #[bw(map = |checksum: &Option<i32>| checksum.unwrap_or(0))]
    pub checksum: Option<i32>,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct Header {
    #[br(map = |is_not_rtc: u32| is_not_rtc == 0)]
    #[bw(map = |is_rtc: &bool| if *is_rtc { 0u32 } else { 1u32 })]
    pub is_rtc: bool,
    #[br(temp)]
    #[bw(calc = block_descriptions.len() as u32)]
    block_count: u32,
    pub block_working_buffer_capacity_even: u32,
    pub block_working_buffer_capacity_odd: u32,
    pub padded_size: u32,
    pub version_triple: VersionTriple,
    #[serde(skip)]
    #[br(count = block_count)]
    pub block_descriptions: Vec<BlockDescription>,
    #[brw(seek_before = SeekFrom::Start(0x720))]
    pub pool_manifest_padded_size: u32,
    #[br(map = |pool_offset: u32| if pool_offset != u32::MAX && pool_offset != 0 { Some(pool_offset * 2048) } else { None })]
    pub pool_offset: Option<u32>,
    #[br(map = |pool_manifest_unused: u32| if pool_manifest_unused != u32::MAX { Some(pool_manifest_unused) } else { None })]
    pub pool_manifest_unused: Option<u32>,
    #[br(temp)]
    #[bw(calc = pool_manifest_unused.unwrap_or(0))]
    _pool_manifest_unused1: u32,
    pub pool_object_decompression_buffer_capacity: u32,
    pub block_sector_padding_size: u32,
    pub pool_sector_padding_size: u32,
    pub file_size: u32,
    #[brw(align_after = 2048)]
    #[br(try, map = |incredi_builder_string: FixedStringNull<128>| Some(incredi_builder_string.into()))]
    #[bw(map = |incredi_builder_string: &Option<String>| incredi_builder_string.as_ref().map(|s| FixedStringNull::<128>::from(s.to_string())).unwrap_or(FixedStringNull::<128>::from(String::new())))]
    pub incredi_builder_string: Option<String>,
}
