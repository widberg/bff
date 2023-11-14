use std::io::SeekFrom;

use binrw::*;
use serde::Serialize;

use crate::helpers::DynArray;
use crate::names::Name;
use crate::versions::VersionOneple;

#[derive(Serialize, Debug, BinRead, BinWrite)]
pub struct DataDescription {
    pub resource_count: u32,
    pub padded_size: u64,
    pub size: u64,
    pub working_buffer_offset: u64,
}

impl DataDescription {
    const SIZE: u64 = 28;
}

#[derive(Serialize, Debug, BinRead)]
pub struct Resource {
    pub name: Name,
    pub class_name: Name,
    pub offset: u32,
    pub size: u64,
    pub decompressed_size: u64,
}

#[derive(Serialize, Debug, BinRead)]
pub struct Resources {
    pub data_count: u32,
    pub data_offset: u32,
    pub working_buffer_offset: u32,
    pub unk1: u32,
    pub unk2: u64,
    pub padded_size: u64,
    pub padding_size: u64,
    #[br(count = data_count, pad_after = DataDescription::SIZE * 52 - DataDescription::SIZE * data_count as u64)]
    pub data_descriptions: Vec<DataDescription>,
    // Use a Vec here instead of DynArray because Resource doesn't impl BinWrite and binrw isn't smart with trait bounds
    pub resource_count: u32,
    #[br(count = resource_count)]
    pub resources: Vec<Resource>,
    #[br(align_after = 2048)]
    pub unk3: u64,
}

#[derive(Serialize, Debug, BinRead, BinWrite)]
pub struct BlockDescription {
    pub unk1: u64,
    pub unk2: u64,
    pub unk3: u64,
    pub resources_map_offset: u32,
    pub data_resources_map_offset: u32,
}

#[binrw]
#[derive(Serialize, Debug)]
pub struct Header {
    pub version_oneple: VersionOneple,
    #[br(map = |is_not_rtc: u32| is_not_rtc == 0)]
    #[bw(map = |is_rtc: &bool| if *is_rtc { 0u32 } else { 1u32 })]
    pub is_rtc: bool,
    pub block_description_offset: u64,
    pub block_working_buffer_capacity_even: u64,
    pub block_working_buffer_capacity_odd: u64,
    pub total_padded_block_size: u64,
    pub block_sector_padding_size: u64,
    pub file_size: u64,
    pub total_decompressed_size: u64,
    pub zero: u64,
    #[brw(align_after = 2048)]
    pub total_resource_count: u32,
    #[br(seek_before = SeekFrom::Start(block_description_offset * 2048))]
    pub block_descriptions: DynArray<BlockDescription>,
}
