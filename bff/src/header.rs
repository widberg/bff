use crate::strings::FixedStringNULL;
use binrw::*;
use serde::Serialize;

#[binread]
#[derive(Serialize, Debug)]
pub struct BlockDescription {
    object_count: u32,
    padded_size: u32,
    data_size: u32,
    working_buffer_offset: u32,
    first_object_name: u32,
    #[br(temp)]
    zero: u32,
}

impl BlockDescription {
    /// binrw doesn't have a way to calculate the number of bytes read by even a simple structure.
    /// So we will calculate it ourselves and store it in this constant.
    pub const SIZE: usize = 0x18;

    pub fn object_count(&self) -> u32 {
        self.object_count
    }

    pub fn working_buffer_offset(&self) -> u32 {
        self.working_buffer_offset
    }
}

#[binread]
#[derive(Serialize, Debug)]
pub struct Header {
    version: FixedStringNULL<256>,
    is_not_rtc: u32,
    #[br(temp)]
    block_count: u32,
    block_working_buffer_capacity_even: u32,
    block_working_buffer_capacity_odd: u32,
    #[br(temp)]
    padded_size: u32,
    version_patch: u32,
    version_minor: u32,
    version_major: u32,
    #[serde(skip)]
    #[br(count = block_count, pad_size_to = BlockDescription::SIZE * 64)]
    block_descriptions: Vec<BlockDescription>,
    #[br(temp)]
    pool_manifest_padded_size: u32,
    #[br(calc = pool_manifest_padded_size != u32::MAX && pool_manifest_padded_size != 0)]
    has_pool: bool,
    #[br(temp)]
    pool_manifest_offset: u32,
    pool_manifest_unused0: u32,
    #[br(temp)]
    pool_manifest_unused1: u32,
    #[br(temp)]
    pool_object_decompression_buffer_capacity: u32,
    #[br(temp)]
    block_sector_padding_size: u32,
    #[br(temp)]
    pool_sector_padding_size: u32,
    #[br(temp)]
    file_size: u32,
    #[br(temp, restore_position)]
    incredi_builder_string_char: u8,
    #[brw(if(incredi_builder_string_char != u8::MAX))]
    incredi_builder_string: Option<FixedStringNULL<128>>,
    #[br(temp, align_after = 2048)]
    padding: (),
}

impl Header {
    pub fn block_descriptions(&self) -> &Vec<BlockDescription> {
        &self.block_descriptions
    }

    pub fn has_pool(&self) -> bool {
        self.has_pool
    }
}
