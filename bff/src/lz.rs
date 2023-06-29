use binrw::{BinReaderExt, BinResult};

#[binrw::parser(reader, endian)]
pub fn decompress_body_parser(decompressed_size: u32, compressed_size: u32) -> BinResult<Vec<u8>> {
    // These fields are little endian even on big endian platforms.
    let read_decompressed_size = reader.read_le::<u32>()?;
    let read_compressed_size = reader.read_le::<u32>()?;

    // Ensure the values from the object header match the values
    // in the compressed data.
    // compressed_size includes the 8 bytes taken up by the duplicate
    // size fields.
    assert_eq!(decompressed_size, read_decompressed_size);
    assert_eq!(compressed_size, read_compressed_size);

    decompress_data_parser(reader, endian, (decompressed_size, compressed_size - 8))
}

#[binrw::parser(reader, endian)]
pub fn decompress_data_with_header_parser() -> BinResult<Vec<u8>> {
    // These fields are little endian even on big endian platforms.
    let decompressed_size = reader.read_le::<u32>()?;
    let compressed_size = reader.read_le::<u32>()?;

    decompress_data_parser(reader, endian, (decompressed_size, compressed_size - 8))
}

#[binrw::parser(reader)]
pub fn decompress_data_parser(decompressed_size: u32, _compressed_size: u32) -> BinResult<Vec<u8>> {
    const WINDOW_LOG: u16 = 14;
    const WINDOW_MASK: u16 = (1 << WINDOW_LOG) - 1;

    let mut decompressed_buffer: Vec<u8> = Vec::new();

    loop {
        let mut flags = reader.read_be::<u32>()?;
        let len = (flags & 0b11) as u16;
        let temp_shift = WINDOW_LOG - len;
        let temp_mask = WINDOW_MASK >> len;

        for _ in 0..30 {
            if (flags & 0x80000000) != 0 {
                let temp = reader.read_be::<u16>()?;
                let start = decompressed_buffer.len() - (temp & temp_mask) as usize - 1;
                let end = start + (temp >> temp_shift) as usize + 3;

                for i in start..end {
                    decompressed_buffer.push(decompressed_buffer[i]);
                }
            } else {
                decompressed_buffer.push(reader.read_be::<u8>()?);
            }

            if decompressed_buffer.len() >= decompressed_size as usize {
                return Ok(decompressed_buffer);
            }

            flags <<= 1;
        }
    }
}
