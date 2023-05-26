use binrw::binread;
use serde::Serialize;

#[binread]
#[derive(Serialize, Debug)]
pub struct Object {
    #[br(temp)]
    data_size: u32,
    #[br(temp)]
    link_header_size: u32,
    #[br(temp)]
    decompressed_size: u32,
    #[br(temp)]
    compressed_size: u32,
    class_name: u32,
    name: u32,
    #[br(count = data_size)]
    #[serde(skip_serializing)]
    data: Vec<u8>,
}
