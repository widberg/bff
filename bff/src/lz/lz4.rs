use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, Endian, args};

use crate::BffResult;

#[binrw::parser(reader, endian)]
pub fn lz4_decompress_body_parser(
    decompressed_size: u32,
    compressed_size: u32,
) -> BinResult<Vec<u8>> {
    // These fields are little endian even on big endian platforms.
    let read_decompressed_size = reader.read_le::<u32>()?;
    let read_compressed_size = reader.read_le::<u32>()?;

    assert_eq!(decompressed_size, read_decompressed_size);
    assert_eq!(compressed_size, read_compressed_size);

    lz4_decompress_data_parser(reader, endian, (decompressed_size, compressed_size - 8))
}

#[binrw::writer(writer)]
pub fn lz4_compress_data_writer(data: &[u8]) -> BinResult<()> {
    // TODO: Don't use asserts. Add proper error handling.
    // TODO: Not the same parameters used by the game.
    let compress_bound = unsafe { lz4::liblz4::LZ4_compressBound(data.len() as i32) as usize };
    let mut compressed_buffer: Vec<u8> = Vec::with_capacity(compress_bound);
    unsafe {
        let result = lz4::liblz4::LZ4_compress_default(
            data.as_ptr() as *const i8,
            compressed_buffer.as_mut_ptr() as *mut i8,
            data.len() as i32,
            compressed_buffer.capacity() as i32,
        );
        assert!(result >= 0);
        compressed_buffer.set_len(result as usize);
    }
    writer.write_all(&compressed_buffer)?;
    Ok(())
}

#[binrw::writer(writer)]
pub fn lz4_compress_data_with_header_writer_internal(data: &[u8]) -> BinResult<()> {
    let begin = writer.stream_position()?;
    writer.seek(SeekFrom::Current(8))?;
    lz4_compress_data_writer(data, writer, Endian::Little, ())?;
    let end = writer.stream_position()?;

    writer.seek(SeekFrom::Start(begin))?;
    (data.len() as u32).write_le(writer)?;
    ((end - begin - 8) as u32).write_le(writer)?;
    writer.seek(SeekFrom::Start(end))?;
    Ok(())
}

pub fn lz4_compress_data_with_header_writer<W: Write + Seek>(
    data: &[u8],
    writer: &mut W,
    endian: Endian,
) -> BffResult<()> {
    Ok(lz4_compress_data_with_header_writer_internal(
        data,
        writer,
        endian,
        (),
    )?)
}

#[binrw::parser(reader)]
pub fn lz4_decompress_data_parser(
    decompressed_size: u32,
    compressed_size: u32,
) -> BinResult<Vec<u8>> {
    // TODO: Don't use asserts. Add proper error handling.
    if compressed_size != 0 {
        let compressed_buffer =
            Vec::<u8>::read_args(reader, args! { count: compressed_size as usize })?;
        let mut decompressed_buffer: Vec<u8> = Vec::with_capacity(decompressed_size as usize);
        unsafe {
            let result = lz4::liblz4::LZ4_decompress_safe(
                compressed_buffer.as_ptr() as *const i8,
                decompressed_buffer.as_mut_ptr() as *mut i8,
                compressed_size as i32,
                decompressed_size as i32,
            );
            assert!(result >= 0);
            decompressed_buffer.set_len(result as usize);
        }
        assert_eq!(decompressed_buffer.len(), decompressed_size as usize);

        Ok(decompressed_buffer)
    } else {
        let decompressed_buffer =
            Vec::<u8>::read_args(reader, args! { count: decompressed_size as usize })?;
        Ok(decompressed_buffer)
    }
}

#[binrw::parser(reader)]
pub fn lz4_decompress_data_with_header_parser_internal() -> BinResult<Vec<u8>> {
    let decompressed_size = u32::read_le(reader)?;
    let compressed_size = u32::read_le(reader)?;
    lz4_decompress_data_parser(reader, Endian::Little, (decompressed_size, compressed_size))
}

pub fn lz4_decompress_data_with_header_parser<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
) -> BffResult<Vec<u8>> {
    Ok(lz4_decompress_data_with_header_parser_internal(
        reader,
        endian,
        (),
    )?)
}
