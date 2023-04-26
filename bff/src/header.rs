use binread::*;
use binread::io::*;
use serde::Serialize;

#[derive(BinRead, Serialize, Debug)]
pub struct BlockDescription {
    object_count: u32,
    padded_size: u32,
    data_size: u32,
    working_buffer_offset: u32,
    first_object_name: u32,
    zero: u32,
}

#[derive(BinRead, Serialize, Debug)]
pub struct Header {
    version: NullString,
    #[br(seek_before = SeekFrom::Start(256))]
    is_not_rtc: u32,
    block_count: u32,
    block_working_buffer_capacity_even: u32,
    block_working_buffer_capacity_odd: u32,
    padded_size: u32,
    version_patch: u32,
    version_minor: u32,
    version_major: u32,
    #[br(count = block_count)]
    block_descriptions: Vec<BlockDescription>,
    #[br(seek_before = SeekFrom::Start(1824))]
    pool_manifest_padded_size: u32,
    pool_manifest_offset: u32,
    pool_manifest_unused0: u32,
    pool_manifest_unused1: u32,
    pool_object_decompression_buffer_capacity: u32,
    block_sector_padding_size: u32,
    pool_sector_padding_size: u32,
    file_size: u32,
    #[br(align_after = 2048)]
    incredi_builder_string: NullString,
}
