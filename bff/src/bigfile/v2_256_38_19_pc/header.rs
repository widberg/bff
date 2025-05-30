use std::io::SeekFrom;

use binrw::{BinRead, BinWrite, binread};

use crate::bigfile::v1_06_63_02_pc::header::BigFileType;
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
    pub _name: Name,
    pub _class_name: Name,
    pub _unk1: u32,
    pub offset: u32,
    pub _compressed_size: u32,
    pub _unk2: u32,
    pub _decompressed_size: u32,
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
    pub _unk4: u64,
    #[br(align_after = 2048)]
    pub _unk5: u32,
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
