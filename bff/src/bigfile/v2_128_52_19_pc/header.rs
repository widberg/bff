use std::io::SeekFrom;

use binrw::{BinRead, BinWrite, binread};

use crate::bigfile::versions::VersionOneple;
use crate::helpers::DynArray;
use crate::names::Name;

#[derive(Debug, BinRead, BinWrite)]
pub struct DataDescription {
    pub resource_count: u32,
    pub padded_size: u64,
    pub size: u64,
    pub working_buffer_offset: u64,
}

impl DataDescription {
    const SIZE: u64 = 28;
}

#[derive(Debug, BinRead)]
pub struct Resource {
    pub name: Name,
    pub _class_name: Name,
    pub offset: u32,
    pub _compressed_size: u32,
    pub _unk1: u32,
    pub _decompressed_size: u32,
    pub _unk2: u16,
    pub _unk3: u16,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Unknown {
    pub data: [u8; 16],
}

#[binread]
#[derive(Debug)]
pub struct Resources {
    #[br(temp)]
    pub data_count: u32,
    pub data_offset: u32,
    pub working_buffer_offset: u32,
    pub _unk1: u32,
    pub _unk2: u64,
    pub _padded_size: u64,
    pub _padding_size: u64,
    #[br(count = data_count, pad_after = DataDescription::SIZE * 52 - DataDescription::SIZE * data_count as u64)]
    pub data_descriptions: Vec<DataDescription>,
    // Use a Vec here instead of DynArray because Resource doesn't impl BinWrite and binrw isn't smart with trait bounds
    #[br(temp)]
    pub resource_count: u32,
    #[br(count = resource_count)]
    pub resources: Vec<Resource>,
    pub _unk3: u64,
    pub _unknown: DynArray<Unknown>,
    pub _unk4: DynArray<u32>,
    #[br(temp)]
    pub resource_count2: u32,
    #[br(count = resource_count2, align_after = 16)]
    pub resources2: Vec<Resource>,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct BlockDescription {
    pub unk1: u64,
    pub unk2: u64,
    pub unk3: u64,
    pub resources_map_offset: u32,
    pub data_resources_map_offset: u32,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(repr = u8)]
pub enum BigFileType {
    Rtc = 0,
    Normal = 1,
    Common = 2,
    Updated1 = 4,
}

impl From<BigFileType> for crate::bigfile::manifest::BigFileType {
    fn from(bigfile_type: BigFileType) -> Self {
        match bigfile_type {
            BigFileType::Rtc => Self::Rtc,
            BigFileType::Normal => Self::Normal,
            BigFileType::Common => Self::Common,
            BigFileType::Updated1 => Self::Updated1,
        }
    }
}

impl From<crate::bigfile::manifest::BigFileType> for BigFileType {
    fn from(bigfile_type: crate::bigfile::manifest::BigFileType) -> Self {
        match bigfile_type {
            crate::bigfile::manifest::BigFileType::Rtc => Self::Rtc,
            crate::bigfile::manifest::BigFileType::Normal => Self::Normal,
            crate::bigfile::manifest::BigFileType::Common => Self::Common,
            crate::bigfile::manifest::BigFileType::Updated1 => Self::Updated1,
        }
    }
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Header {
    pub bigfile_type: BigFileType,
    pub version_oneple: VersionOneple,
    pub block_description_offset: u32,
    pub resources_block_size: u32,
    pub resources_block_offset: u32,
    pub map_size: u32,
    #[brw(align_after = 4096)]
    pub map_offset: u32,
    #[br(seek_before = SeekFrom::Start(block_description_offset as u64 * 16))]
    pub block_descriptions: DynArray<BlockDescription>,
}
