use std::ops::Deref;

use binrw::{binread, BinRead, BinResult, VecArgs};
use serde::Serialize;

use crate::lz::decompress_body_parser;
use crate::name::Name;

#[binrw::parser(reader, endian)]
fn body_parser(decompressed_size: u32, compressed_size: u32) -> BinResult<Vec<u8>> {
    if compressed_size == 0 {
        Vec::<u8>::read_options(
            reader,
            endian,
            VecArgs {
                count: decompressed_size as usize,
                inner: <_>::default(),
            },
        )
    } else {
        decompress_body_parser(reader, endian, (decompressed_size, compressed_size))
    }
}

#[binread]
#[derive(Serialize, Debug, Default)]
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
    // #[serde(skip_serializing)]
    link_header: Vec<u8>,
    #[br(parse_with = body_parser, args(decompressed_size, compressed_size))]
    // #[serde(skip_serializing)]
    body: Vec<u8>,
}

impl Object {
    pub fn class_name(&self) -> Name {
        self.class_name
    }

    pub fn name(&self) -> Name {
        self.name
    }

    pub fn link_header(&self) -> &Vec<u8> {
        &self.link_header
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
}

#[derive(BinRead, Serialize, Debug)]
pub struct PoolObject {
    #[br(align_after(2048))]
    #[serde(flatten)]
    object: Object,
}

impl Deref for PoolObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
