use std::io::Cursor;

use binrw::{binread, Endian};
use serde::Serialize;

use crate::lz::decompress;

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
    #[br(count = link_header_size)]
    #[serde(skip_serializing)]
    _link_header: Vec<u8>,
    #[br(count = data_size - link_header_size, try_map = |x: Vec<u8>| if compressed_size != 0 { decompress(&mut Cursor::new(x), decompressed_size as usize, Endian::Little) } else { Ok(x) })]
    #[serde(skip_serializing)]
    _body: Vec<u8>,
}
