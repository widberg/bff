use binrw::*;
use serde::Serialize;

use crate::error::InvalidVersionError;
use crate::strings::FixedStringNull;
use crate::versions::{Version, VersionTriple};

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
    /// So we will calculate it ourselves and store it in this constant. We could mark the struct as
    /// #[repr(C)] and use std::mem::size_of::<BlockDescription>(), but that would imply that we
    /// care about the size of the struct in memory, which we don't; we care about how many bytes
    /// are read. For this trivial struct it does not matter, but I do not want to introduce that
    /// pattern into the code as more complex structs are added.
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
    version_string: FixedStringNull<256>,
    is_not_rtc: u32,
    #[br(temp)]
    block_count: u32,
    block_working_buffer_capacity_even: u32,
    block_working_buffer_capacity_odd: u32,
    #[br(temp)]
    padded_size: u32,
    version_triple: VersionTriple,
    #[serde(skip)]
    #[br(count = block_count, pad_size_to = BlockDescription::SIZE * 64)]
    block_descriptions: Vec<BlockDescription>,
    #[br(temp)]
    pool_manifest_padded_size: u32,
    #[br(map = |pool_offset: u32| if pool_offset != u32::MAX && pool_offset != 0 { Some(pool_offset * 2048) } else { None })]
    pool_offset: Option<u32>,
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
    #[brw(try, align_after = 2048)]
    incredi_builder_string: Option<FixedStringNull<128>>,
}

impl Header {
    pub fn block_descriptions(&self) -> &Vec<BlockDescription> {
        &self.block_descriptions
    }

    pub fn version(&self) -> Result<Version, InvalidVersionError> {
        self.version_string.as_str().try_into()
    }

    pub fn pool_offset(&self) -> Option<u32> {
        self.pool_offset
    }
}
