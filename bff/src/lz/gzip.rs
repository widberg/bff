use std::io::{Read, Seek, Write};

use binrw::{BinResult, Endian};
use flate2::Compression;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;

use crate::BffResult;

#[binrw::writer(writer)]
pub fn gzip_compress_data_with_header_writer_internal(data: &[u8]) -> BinResult<()> {
    let mut gz = GzEncoder::new(writer, Compression::fast());
    gz.write_all(data)?;
    Ok(())
}

pub fn gzip_compress_data_with_header_writer<W: Write + Seek>(
    data: &[u8],
    writer: &mut W,
    endian: Endian,
) -> BffResult<()> {
    Ok(gzip_compress_data_with_header_writer_internal(
        data,
        writer,
        endian,
        (),
    )?)
}

#[binrw::parser(reader)]
pub fn gzip_decompress_data_with_header_parser_internal() -> BinResult<Vec<u8>> {
    let mut gz = MultiGzDecoder::new(reader);
    let mut buf = Vec::new();
    gz.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn gzip_decompress_data_with_header_parser<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
) -> BffResult<Vec<u8>> {
    Ok(gzip_decompress_data_with_header_parser_internal(
        reader,
        endian,
        (),
    )?)
}
