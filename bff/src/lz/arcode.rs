use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use arcode::bitbit::{BitReader, BitWriter, LSB};
use arcode::{ArithmeticDecoder, ArithmeticEncoder, EOFKind, Model};
use binrw::{args, BinRead, BinResult, BinWrite, Endian};

use crate::BffResult;

#[binrw::writer(writer)]
pub fn arcode_compress_data_writer(data: &[u8]) -> BinResult<()> {
    let mut model = Model::builder().num_bits(8).eof(EOFKind::EndAddOne).build();

    let mut compressed_writer = BitWriter::new(writer);

    let mut encoder = ArithmeticEncoder::new(61);

    for sym in data {
        encoder.encode(*sym as u32, &model, &mut compressed_writer)?;
        model.update_symbol(*sym as u32);
    }

    encoder.encode(model.eof(), &model, &mut compressed_writer)?;
    encoder.finish_encode(&mut compressed_writer)?;
    compressed_writer.pad_to_byte()?;

    Ok(())
}

#[binrw::writer(writer)]
pub fn arcode_compress_data_with_header_writer_internal(data: &[u8]) -> BinResult<()> {
    let begin = writer.stream_position()?;
    writer.seek(SeekFrom::Current(8))?;
    arcode_compress_data_writer(data, writer, Endian::Little, ())?;
    let end = writer.stream_position()?;

    writer.seek(SeekFrom::Start(begin))?;
    (data.len() as u32).write_le(writer)?;
    ((end - begin) as u32).write_le(writer)?;
    writer.seek(SeekFrom::Start(end))?;
    Ok(())
}

pub fn arcode_compress_data_with_header_writer<W: Write + Seek>(
    data: &[u8],
    writer: &mut W,
    endian: Endian,
) -> BffResult<()> {
    Ok(arcode_compress_data_with_header_writer_internal(
        data,
        writer,
        endian,
        (),
    )?)
}

#[binrw::parser(reader)]
pub fn arcode_decompress_data_parser(
    decompressed_size: u32,
    compressed_size: u32,
) -> BinResult<Vec<u8>> {
    if compressed_size != 0 {
        let mut model = Model::builder()
            .num_symbols(256)
            .eof(EOFKind::EndAddOne)
            .build();

        let mut input_reader = BitReader::<_, LSB>::new(reader);
        let mut decoder = ArithmeticDecoder::new(61);

        let mut decompressed_buffer_cursor =
            Cursor::new(Vec::with_capacity(decompressed_size as usize));

        while !decoder.finished() {
            let sym = decoder.decode(&model, &mut input_reader)?;
            model.update_symbol(sym);
            u8::write(&(sym as u8), &mut decompressed_buffer_cursor)?;
        }

        Ok(decompressed_buffer_cursor.into_inner())
    } else {
        let decompressed_buffer =
            Vec::<u8>::read_args(reader, args! { count: decompressed_size as usize })?;
        Ok(decompressed_buffer)
    }
}

#[binrw::parser(reader)]
pub fn arcode_decompress_data_with_header_parser_internal() -> BinResult<Vec<u8>> {
    let decompressed_size = u32::read_le(reader)?;
    let compressed_size = u32::read_le(reader)?;
    arcode_decompress_data_parser(
        reader,
        Endian::Little,
        (decompressed_size, compressed_size - 8),
    )
}

pub fn arcode_decompress_data_with_header_parser<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
) -> BffResult<Vec<u8>> {
    Ok(arcode_decompress_data_with_header_parser_internal(
        reader,
        endian,
        (),
    )?)
}
