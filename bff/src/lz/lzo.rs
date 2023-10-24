use std::io::{Read, Write};

use binrw::Endian;
use minilzo3::{compress, decompress};

use crate::BffResult;

pub fn lzo_compress<W: Write>(data: &[u8], writer: &mut W, _endian: Endian) -> BffResult<()> {
    let mut compressed = vec![0; 0x00105800];
    compress(&data, &mut compressed, false)?;
    writer.write_all(&compressed)?;
    Ok(())
}

pub fn lzo_decompress<R: Read>(reader: &mut R, _endian: Endian) -> BffResult<Vec<u8>> {
    let mut compressed = Vec::new();
    reader.read_to_end(&mut compressed)?;
    let mut decompressed = vec![0; 0x00105800];
    let _decompressed_length = decompress(&compressed, &mut decompressed)?;
    Ok(decompressed)
}
