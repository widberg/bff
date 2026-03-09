use binrw::*;

use crate::bigfile::versions::VersionTriple;
use crate::names::Name;

#[derive(Debug, BinRead, BinWrite)]
pub struct BlockDescription {
    pub resource_count: u32,
    pub padded_size: u32,
    pub data_size: u32,
    pub working_buffer_offset: u32,
    pub first_resource_name: Name,
    #[br(map = |checksum: i32| (checksum != 0).then_some(checksum))]
    #[bw(map = |checksum: &Option<i32>| checksum.unwrap_or(0))]
    pub checksum: Option<i32>,
}

#[derive(Debug, BinRead, BinWrite, Copy, Clone)]
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
#[derive(Debug)]
pub struct Header {
    pub bigfile_type: BigFileType,
    #[br(temp)]
    #[bw(calc = block_descriptions.len() as u32)]
    block_count: u32,
    pub block_working_buffer_capacity_even: u32,
    pub block_working_buffer_capacity_odd: u32,
    pub padded_size: u32,
    pub version_triple: VersionTriple,
    #[br(count = block_count)]
    #[brw(align_after = 2048)]
    pub block_descriptions: Vec<BlockDescription>,
    #[br(ignore)]
    pub tag: Option<Vec<u8>>,
}
