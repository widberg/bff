use std::ops::Deref;

use binrw::{binrw, BinRead, BinResult, BinWrite, VecArgs};
use serde::Serialize;

use crate::lz::decompress_body_parser;
use crate::names::Name;

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

#[binrw]
#[derive(Serialize, Debug, Default, Eq, PartialEq)]
pub struct Object {
    #[br(temp)]
    #[bw(calc = link_header.len() as u32 + body.len() as u32)]
    _data_size: u32,
    #[br(temp)]
    #[bw(calc = link_header.len() as u32)]
    link_header_size: u32,
    #[br(temp)]
    #[bw(calc = body.len() as u32)]
    decompressed_size: u32,
    #[br(temp)]
    #[bw(calc = 0)]
    compressed_size: u32,
    #[br(calc = compressed_size != 0)]
    #[bw(ignore)]
    pub compress: bool,
    pub class_name: Name,
    pub name: Name,
    #[br(count = link_header_size)]
    #[serde(skip_serializing)]
    pub link_header: Vec<u8>,
    #[br(parse_with = body_parser, args(decompressed_size, compressed_size))]
    #[serde(skip_serializing)]
    pub body: Vec<u8>,
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

    pub fn compress(&self) -> bool {
        self.compress
    }
}

#[derive(BinRead, Serialize, Debug, BinWrite)]
pub struct PoolObject {
    #[br(align_after(2048))]
    #[serde(flatten)]
    pub object: Object,
}

impl Deref for PoolObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
