use std::io::SeekFrom;

use binrw::*;
use serde::Serialize;

use crate::bigfile::v1_06_63_02_pc::header::BigFileType;
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
    pub unk1: u32,
    pub offset: u32,
    pub compressed_size: u32,
    pub unk2: u32,
    pub decompressed_size: u32,
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
    pub unk3: u64,
    pub unk4: u64,
    #[br(align_after = 2048)]
    pub unk5: u32,
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
    pub bigfile_type: BigFileType,
    pub block_description_offset: u32,
    pub unk1: u32,
    pub unk2: u64,
    #[brw(align_after = 4096)]
    pub unk3: u32,
    #[br(seek_before = SeekFrom::Start(block_description_offset as u64 * 2048))]
    pub block_descriptions: DynArray<BlockDescription>,
}
