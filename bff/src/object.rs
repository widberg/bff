use crate::{name::Name, lz::decompress_parser};
use binrw::{binread, BinResult, BinRead, VecArgs};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum BodySize {
    Uncompressed {
        size: u32,
    },
    Compressed {
        decompressed_size: u32,
        compressed_size: u32,
    },
}

#[binrw::parser(reader, endian)]
fn body_parser(data_size: u32, link_header_size: u32, decompressed_size: u32, compressed_size: u32) -> BinResult<Option<Vec<u8>>> {
    if data_size == link_header_size {
        // The body is in the pool
        Ok(None)
    } else if compressed_size == 0 {
        // The body is not compressed
        Ok(Some(Vec::<u8>::read_options(reader, endian, VecArgs {
            count: decompressed_size as usize,
            inner: <_>::default(),
        })?))
    } else {
        // The body is compressed
        Ok(Some(decompress_parser(reader, endian, (decompressed_size as usize, compressed_size as usize))?))
    }
}

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
    class_name: Name,
    name: Name,
    #[br(count = link_header_size)]
    #[serde(skip_serializing)]
    _link_header: Vec<u8>,
    #[br(parse_with = body_parser, args(data_size, link_header_size, decompressed_size, compressed_size))]
    #[serde(skip_serializing)]
    _body: Option<Vec<u8>>,
}

impl Object {
    pub fn class_name(self: &Self) -> Name {
        self.class_name
    }

    pub fn name(self: &Self) -> Name {
        self.name
    }
}
