use std::io::{Seek, SeekFrom, Write};

use binrw::{binread, BinResult, BinWrite, Endian};
use serde::Serialize;

use crate::bigfile::v1_06_63_02_pc::object::body_parser;
use crate::lz::compress_data_with_header_writer_internal;
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

impl BinWrite for Object {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let start = writer.stream_position()?;
        writer.seek(SeekFrom::Current(8))?;

        self.class_name.write_options(writer, endian, ())?;
        self.name.write_options(writer, endian, ())?;
        let body_size = if self.compress {
            let body_start = writer.stream_position()?;
            compress_data_with_header_writer_internal(&self.data, writer, endian, ())?;
            let body_end = writer.stream_position()?;
            (body_end - body_start) as u32
        } else {
            self.data.write_options(writer, endian, ())?;
            self.data.len() as u32
        };

        let end = writer.stream_position()?;

        // Now that we know everything, back to the top to write the header
        writer.seek(SeekFrom::Start(start))?;

        let decompressed_size = self.data.len() as u32;
        let compressed_size = if self.compress { body_size } else { 0 };

        decompressed_size.write_options(writer, endian, ())?;
        compressed_size.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::Start(end))?;

        Ok(())
    }
}
