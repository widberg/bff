use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{args, BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use crate::BffResult;

#[binrw::parser(reader, endian)]
pub fn zlib_decompress_body_parser(
    decompressed_size: u32,
    compressed_size: u32,
) -> BinResult<Vec<u8>> {
    // These fields are little endian even on big endian platforms.
    let read_decompressed_size = reader.read_le::<u32>()?;
    let read_compressed_size = reader.read_le::<u32>()?;

    assert_eq!(decompressed_size, read_decompressed_size);
    assert_eq!(compressed_size - 8, read_compressed_size);

    zlib_decompress_data_parser(reader, endian, (decompressed_size, compressed_size - 8))
}

#[binrw::writer(writer)]
pub fn zlib_compress_data_writer(data: &[u8]) -> BinResult<()> {
    // TODO: Don't use asserts. Add proper error handling.
    // TODO: Not the same parameters used by the game.
    let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(data)?;
    let compressed_buffer = encoder.finish()?;
    writer.write_all(&compressed_buffer)?;
    Ok(())
}

#[binrw::writer(writer)]
pub fn zlib_compress_data_with_header_writer_internal(data: &[u8]) -> BinResult<()> {
    let begin = writer.stream_position()?;
    writer.seek(SeekFrom::Current(8))?;
    zlib_compress_data_writer(data, writer, Endian::Little, ())?;
    let end = writer.stream_position()?;

    writer.seek(SeekFrom::Start(begin))?;
    (data.len() as u32).write_le(writer)?;
    ((end - begin - 8) as u32).write_le(writer)?;
    writer.seek(SeekFrom::Start(end))?;
    Ok(())
}

pub fn zlib_compress_data_with_header_writer<W: Write + Seek>(
    data: &[u8],
    writer: &mut W,
    endian: Endian,
) -> BffResult<()> {
    Ok(zlib_compress_data_with_header_writer_internal(
        data,
        writer,
        endian,
        (),
    )?)
}

#[binrw::parser(reader)]
pub fn zlib_decompress_data_parser(
    decompressed_size: u32,
    compressed_size: u32,
) -> BinResult<Vec<u8>> {
    // TODO: Don't use asserts. Add proper error handling.
    if compressed_size != 0 {
        let mut decoder = ZlibDecoder::new(reader.take(compressed_size as u64));
        let mut decompressed_buffer = Vec::with_capacity(decompressed_size as usize);
        decoder.read_to_end(&mut decompressed_buffer)?;

        assert_eq!(decompressed_buffer.len(), decompressed_size as usize);

        Ok(decompressed_buffer)
    } else {
        let decompressed_buffer =
            Vec::<u8>::read_args(reader, args! { count: decompressed_size as usize })?;
        Ok(decompressed_buffer)
    }
}

#[binrw::parser(reader)]
pub fn zlib_decompress_data_with_header_parser_internal() -> BinResult<Vec<u8>> {
    let decompressed_size = u32::read_le(reader)?;
    let compressed_size = u32::read_le(reader)?;
    zlib_decompress_data_parser(reader, Endian::Little, (decompressed_size, compressed_size))
}

pub fn zlib_decompress_data_with_header_parser<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
) -> BffResult<Vec<u8>> {
    Ok(zlib_decompress_data_with_header_parser_internal(
        reader,
        endian,
        (),
    )?)
}
