use std::io::{Seek, SeekFrom, Write};

use binrw::{args, binread, parser, BinRead, BinResult, BinWrite, Endian};
use serde::Serialize;

use crate::lz::{compress_data_with_header_writer_internal, decompress_body_parser};
use crate::names::Name;

#[parser(reader, endian)]
fn data_parser(decompressed_size: u32, compressed_size: u32) -> BinResult<Vec<u8>> {
    if compressed_size == 0 {
        Vec::<u8>::read_options(
            reader,
            endian,
            args! {
                count: decompressed_size as usize,
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
    decompressed_size: u32,
    #[br(temp)]
    compressed_size: u32,
    #[br(calc = compressed_size != 0)]
    pub compress: bool,
    pub class_name: Name,
    pub name: Name,
    #[br(parse_with = data_parser, args(decompressed_size, compressed_size))]
    // #[br(count = if compress { compressed_size } else { decompressed_size })]
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
