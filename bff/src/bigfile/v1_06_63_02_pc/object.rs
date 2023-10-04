use std::io::{Seek, SeekFrom, Write};

use binrw::{binread, parser, BinRead, BinResult, BinWrite, Endian, VecArgs};
use derive_more::{Deref, DerefMut};
use serde::Serialize;

use crate::lz::{compress_data_with_header_writer_internal, decompress_body_parser};
use crate::names::Name;

#[parser(reader, endian)]
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
#[derive(Serialize, Debug, Default, Eq, PartialEq)]
pub struct Object {
    #[br(temp)]
    _data_size: u32,
    #[br(temp)]
    link_header_size: u32,
    #[br(temp)]
    decompressed_size: u32,
    #[br(temp)]
    compressed_size: u32,
    #[br(calc = compressed_size != 0)]
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

impl BinWrite for Object {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let start = writer.stream_position()?;
        writer.seek(SeekFrom::Current(16))?;

        self.class_name.write_options(writer, endian, ())?;
        self.name.write_options(writer, endian, ())?;
        self.link_header.write_options(writer, endian, ())?;
        let link_header_size = self.link_header.len() as u32;
        let body_size = if self.compress {
            let body_start = writer.stream_position()?;
            compress_data_with_header_writer_internal(&self.body, writer, endian, ())?;
            let body_end = writer.stream_position()?;
            (body_end - body_start) as u32
        } else {
            self.body.write_options(writer, endian, ())?;
            self.body.len() as u32
        };

        let end = writer.stream_position()?;

        // Now that we know everything, back to the top to write the header
        writer.seek(SeekFrom::Start(start))?;

        let data_size = link_header_size + body_size;
        let decompressed_size = self.body.len() as u32;
        let compressed_size = if self.compress { body_size } else { 0 };

        data_size.write_options(writer, endian, ())?;
        link_header_size.write_options(writer, endian, ())?;
        decompressed_size.write_options(writer, endian, ())?;
        compressed_size.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::Start(end))?;

        Ok(())
    }
}

#[derive(BinRead, Serialize, Debug, BinWrite, Deref, DerefMut)]
pub struct PoolObject {
    #[brw(align_after(2048))]
    #[serde(flatten)]
    pub object: Object,
}
