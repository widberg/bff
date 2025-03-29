use std::io::{Read, Seek, Write};

use binrw::{args, binread, parser, BinRead, BinResult, BinWrite, Endian};
use serde::Serialize;

use crate::bigfile::resource::Resource;
use crate::bigfile::resource::ResourceData::SplitData;
use crate::lz::{lz4_decompress_body_parser, zlib_decompress_body_parser};
use crate::names::{Name, NameAsobo64};

#[parser(reader, endian)]
fn body_parser(
    decompressed_size: u32,
    compressed_size: u32,
    compression_type: CompressionType,
) -> BinResult<Vec<u8>> {
    if compressed_size == 0 {
        Vec::<u8>::read_options(
            reader,
            endian,
            args! {
                count: decompressed_size as usize,
            },
        )
    } else {
        match compression_type {
            CompressionType::Zlib => {
                zlib_decompress_body_parser(reader, endian, (decompressed_size, compressed_size))
            }
            CompressionType::LZ4 => {
                lz4_decompress_body_parser(reader, endian, (decompressed_size, compressed_size))
            }
            CompressionType::None => unreachable!(),
        }
    }
}

#[derive(Serialize, Debug, Eq, PartialEq, BinRead, BinWrite, Copy, Clone)]
#[brw(repr = u8)]
enum CompressionType {
    None = 0,
    LZ4 = 2,
    Zlib = 4,
}

#[binread]
#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct Object {
    pub class_name: Name,
    pub name: Name,
    pub link_name: Name,
    #[br(temp)]
    _size: u32,
    #[br(temp)]
    link_header_size: u32,
    #[br(temp)]
    decompressed_body_size: u32,
    #[br(temp)]
    compressed_body_size: u32,
    padding_size: u16,
    compression_type: CompressionType,
    #[br(calc = compressed_body_size != 0)]
    pub compress: bool,
    #[br(count = link_header_size, pad_after = padding_size)]
    #[serde(skip_serializing)]
    pub link_header: Vec<u8>,
    #[br(parse_with = body_parser, args(decompressed_body_size, compressed_body_size, compression_type))]
    #[serde(skip_serializing)]
    pub body: Vec<u8>,
}

impl Object {
    pub fn dump_resource<W: Write + Seek>(
        resource: &Resource,
        writer: &mut W,
        endian: Endian,
    ) -> BinResult<()> {
        match &resource.data {
            SplitData { link_header, body } => {
                resource.class_name.write_options(writer, endian, ())?;
                resource.name.write_options(writer, endian, ())?;
                resource
                    .link_name
                    .unwrap_or(NameAsobo64::default().into())
                    .write_options(writer, endian, ())?;
                (link_header.len() as u32 + body.len() as u32).write_options(writer, endian, ())?;
                (link_header.len() as u32).write_options(writer, endian, ())?;
                (body.len() as u32).write_options(writer, endian, ())?;
                0u32.write_options(writer, endian, ())?;
                0u32.write_options(writer, endian, ())?;
                CompressionType::None.write_options(writer, endian, ())?;
                writer.write_all(link_header)?;
                writer.write_all(body)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    pub fn read_resource<R: Read + Seek>(reader: &mut R, endian: Endian) -> BinResult<Resource> {
        Ok(Self::read_options(reader, endian, ())?.into())
    }
}

impl From<Object> for Resource {
    fn from(value: Object) -> Self {
        Resource {
            class_name: value.class_name,
            name: value.name,
            link_name: Some(value.link_name),
            compress: value.compress,
            data: SplitData {
                link_header: value.link_header,
                body: value.body,
            },
        }
    }
}
