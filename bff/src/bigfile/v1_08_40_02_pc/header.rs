use binrw::*;
use serde::Serialize;

use crate::strings::FixedStringNull;
use crate::versions::VersionTriple;

#[derive(Serialize, Debug, BinRead, BinWrite)]
pub struct BlockDescription {
    pub object_count: u32,
    padded_size: u32,
    data_size: u32,
    pub working_buffer_offset: u32,
    first_object_name: u32,
    #[br(map = |checksum: i32| if checksum == 0 { None } else { Some(checksum) })]
    #[bw(map = |checksum: &Option<i32>| checksum.unwrap_or(0))]
    pub checksum: Option<i32>,
}

impl BlockDescription {
    // binrw doesn't have a way to calculate the number of bytes read by even a simple structure.
    // So we will calculate it ourselves and store it in this constant. We could mark the struct as
    // #[repr(C)] and use `std::mem::size_of::<BlockDescription>()`, but that would imply that we
    // care about the size of the struct in memory, which we don't; we care about how many bytes
    // are read. For this trivial struct it does not matter, but I do not want to introduce that
    // pattern into the code as more complex structs are added.
    pub const SIZE: usize = 0x18;
}

#[binread]
#[derive(Serialize, Debug)]
pub struct Header {
    #[br(temp)]
    block_count: u32,
    block_working_buffer_capacity_even: u32,
    block_working_buffer_capacity_odd: u32,
    #[br(temp)]
    _padded_size: u32,
    pub version_triple: VersionTriple,
    #[serde(skip)]
    #[br(count = block_count, align_after = 2048)]
    pub block_descriptions: Vec<BlockDescription>,
}
