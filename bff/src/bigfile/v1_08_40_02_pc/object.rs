use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian, binread};
use serde::Serialize;

use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::{Data, SplitData};
use crate::bigfile::v1_06_63_02_pc::object::body_parser;
use crate::names::Name;

#[binread]
#[derive(Serialize, Debug, Default, Eq, PartialEq)]
pub struct Object {
    #[br(temp)]
    decompressed_size: u32,
    #[br(temp)]
    compressed_size: u32,
    #[br(calc = compressed_size != 0)]
    pub compress: bool,
    pub class_name: Name,
    pub name: Name,
    #[br(parse_with = body_parser, args(decompressed_size, compressed_size))]
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

impl Object {
    pub fn dump_resource<W: Write + Seek>(
        resource: &Resource,
        writer: &mut W,
        endian: Endian,
    ) -> BinResult<()> {
        match &resource.data {
            Data(data) => {
                (data.len() as u32).write_options(writer, endian, ())?;
                0u32.write_options(writer, endian, ())?;
                resource.class_name.write_options(writer, endian, ())?;
                resource.name.write_options(writer, endian, ())?;
                data.write_options(writer, endian, ())?;
            }
            SplitData { link_header, body } => {
                ((link_header.len() + body.len()) as u32).write_options(writer, endian, ())?;
                0u32.write_options(writer, endian, ())?;
                resource.class_name.write_options(writer, endian, ())?;
                resource.name.write_options(writer, endian, ())?;
                link_header.write_options(writer, endian, ())?;
                body.write_options(writer, endian, ())?;
            }
        }

        Ok(())
    }

    pub fn read_resource<R: Read + Seek>(reader: &mut R, endian: Endian) -> BinResult<Resource> {
        Ok(Self::read_options(reader, endian, ())?.into())
    }
}

impl From<Object> for Resource {
    fn from(value: Object) -> Self {
        Self {
            class_name: value.class_name,
            name: value.name,
            link_name: None,
            data: Data(value.data),
        }
    }
}
