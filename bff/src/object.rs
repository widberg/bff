use std::{io::Cursor, ops::Deref};

use crate::{class::Class, lz::decompress_body_parser, name::Name};
use binrw::{binread, BinRead, BinResult};
use serde::Serialize;

#[binrw::parser(reader, endian)]
fn body_parser(decompressed_size: u32, compressed_size: u32, class_name: u32) -> BinResult<Class> {
    if compressed_size == 0 {
        Class::read_options(reader, endian, (decompressed_size, class_name))
    } else {
        let decompressed =
            decompress_body_parser(reader, endian, (decompressed_size, compressed_size)).unwrap();
        let mut data = Cursor::new(decompressed);
        Class::read_options(&mut data, endian, (decompressed_size, class_name))
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
    #[serde(skip_serializing)]
    link_header: Vec<u8>,
    #[br(parse_with = body_parser, args(decompressed_size, compressed_size, class_name))]
    #[serde(skip_serializing)]
    body: Class,
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

    pub fn body(self: &Self) -> &Class {
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
