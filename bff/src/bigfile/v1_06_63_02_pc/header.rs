use std::io::SeekFrom;

use binrw::*;
use serde::Serialize;

use crate::bigfile::versions::VersionTriple;
use crate::helpers::FixedStringNull;
use crate::names::Name;

#[derive(Serialize, Debug, BinRead, BinWrite)]
pub struct BlockDescription {
    pub object_count: u32,
    pub padded_size: u32,
    pub data_size: u32,
    pub working_buffer_offset: u32,
    pub first_object_name: Name,
    #[br(map = |checksum: i32| (checksum != 0).then_some(checksum))]
    #[bw(map = |checksum: &Option<i32>| checksum.unwrap_or(0))]
    pub checksum: Option<i32>,
}

#[derive(Serialize, Debug, BinRead, BinWrite, Copy, Clone)]
#[brw(repr = u32)]
pub enum BigFileType {
    Rtc = 0,
    Normal = 1,
}

impl From<BigFileType> for crate::bigfile::manifest::BigFileType {
    fn from(bigfile_type: BigFileType) -> Self {
        match bigfile_type {
            BigFileType::Rtc => Self::Rtc,
            BigFileType::Normal => Self::Normal,
        }
    }
}

impl From<crate::bigfile::manifest::BigFileType> for BigFileType {
    fn from(bigfile_type: crate::bigfile::manifest::BigFileType) -> Self {
        match bigfile_type {
            crate::bigfile::manifest::BigFileType::Rtc => Self::Rtc,
            _ => Self::Normal,
        }
    }
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct Header {
    pub bigfile_type: BigFileType,
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
    #[br(ignore)]
    pub tag: Option<Vec<u8>>,
    #[brw(seek_before = SeekFrom::Start(0x720))]
    pub pool_manifest_padded_size: u32,
    #[br(map = |pool_offset: u32| (pool_offset != u32::MAX && pool_offset != 0).then(|| pool_offset * 2048))]
    #[bw(map = |pool_offset: &Option<u32>| pool_offset.map(|pool_offset| pool_offset / 2048).unwrap_or(0))]
    pub pool_offset: Option<u32>,
    #[br(map = |pool_manifest_unused: u32| (pool_manifest_unused != u32::MAX).then_some(pool_manifest_unused))]
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
