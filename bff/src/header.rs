use binrw::*;
use serde::Serialize;

use crate::error::InvalidVersionError;
use crate::strings::FixedStringNull;
use crate::versions::{Version, VersionTriple};

#[binread]
#[derive(Serialize, Debug, BinWrite)]
pub struct BlockDescription {
    object_count: u32,
    padded_size: u32,
    data_size: u32,
    working_buffer_offset: u32,
    first_object_name: u32,
    #[br(temp)]
    #[bw(calc = 0)]
    _zero: u32,
}

impl BlockDescription {
    // binrw doesn't have a way to calculate the number of bytes read by even a simple structure.
    // So we will calculate it ourselves and store it in this constant. We could mark the struct as
    // #[repr(C)] and use `std::mem::size_of::<BlockDescription>()`, but that would imply that we
    // care about the size of the struct in memory, which we don't; we care about how many bytes
    // are read. For this trivial struct it does not matter, but I do not want to introduce that
    // pattern into the code as more complex structs are added.
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
    #[br(map = |version_string: FixedStringNull<256>| version_string.as_str().to_string())]
    pub version_string: String,
    #[br(map = |is_not_rtc: u32| is_not_rtc == 0)]
    pub is_rtc: bool,
    #[br(temp)]
    block_count: u32,
    block_working_buffer_capacity_even: u32,
    block_working_buffer_capacity_odd: u32,
    #[br(temp)]
    _padded_size: u32,
    pub version_triple: VersionTriple,
    #[serde(skip)]
    #[br(count = block_count, pad_size_to = BlockDescription::SIZE * 64)]
    pub block_descriptions: Vec<BlockDescription>,
    #[br(temp)]
    _pool_manifest_padded_size: u32,
    #[br(map = |pool_offset: u32| if pool_offset != u32::MAX && pool_offset != 0 { Some(pool_offset * 2048) } else { None })]
    pub pool_offset: Option<u32>,
    #[br(map = |pool_manifest_unused: u32| if pool_manifest_unused != u32::MAX { Some(pool_manifest_unused) } else { None })]
    pub pool_manifest_unused: Option<u32>,
    #[br(temp)]
    _pool_manifest_unused1: u32,
    #[br(temp)]
    _pool_object_decompression_buffer_capacity: u32,
    #[br(temp)]
    _block_sector_padding_size: u32,
    #[br(temp)]
    _pool_sector_padding_size: u32,
    #[br(temp)]
    _file_size: u32,
    #[brw(try, align_after = 2048)]
    #[br(map = |incredi_builder_string: Option<FixedStringNull<128>>| incredi_builder_string
    .as_ref()
    .map(|x| x.as_str().to_string()))]
    pub incredi_builder_string: Option<String>,
}

impl Header {
    pub fn version(&self) -> Result<Version, InvalidVersionError> {
        self.version_string.as_str().try_into()
    }
}
