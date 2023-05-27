use std::io::{Seek, Read};

use binrw::{BinResult, BinReaderExt, Endian};

pub fn decompress<R: Seek + Read>(
    reader: &mut R,
    decompressed_size: usize,
    endian: Endian,
) -> BinResult<Vec<u8>> {
    const WINDOW_LOG: u16 = 14;
    const WINDOW_MASK: u16 = (1 << WINDOW_LOG) - 1;

    let mut decompressed_buffer: Vec<u8> = Vec::new();

    let _decompressed_size = reader.read_type::<u32>(endian)?;
    let _compressed_size = reader.read_type::<u32>(endian)? - 8;

    assert_eq!(decompressed_size, _decompressed_size as usize);

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

            if decompressed_buffer.len() as usize >= decompressed_size {
                return Ok(decompressed_buffer);
            }

            flags <<= 1;
        }
    }
}
