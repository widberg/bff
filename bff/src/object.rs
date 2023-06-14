use std::ops::Deref;

use crate::{lz::decompress_parser, name::Name};
use binrw::{binread, BinRead, BinResult, VecArgs};
use serde::Serialize;

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
        decompress_parser(reader, endian, (decompressed_size, compressed_size))
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
    link_header: Vec<u8>,
    #[br(parse_with = body_parser, args(decompressed_size, compressed_size))]
    #[serde(skip_serializing)]
    body: Vec<u8>,
}

impl Object {
    pub fn class_name(self: &Self) -> Name {
        self.class_name
    }

    pub fn name(self: &Self) -> Name {
        self.name
    }

    pub fn link_header(self: &Self) -> &Vec<u8> {
        &self.link_header
    }

    pub fn body(self: &Self) -> Option<&Vec<u8>> {
        if self.body.len() != 0 {
            Some(&self.body)
        } else {
            None
        }
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
