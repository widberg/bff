use std::io::{Read, Write};

use binrw::Endian;
use minilzo_rs::LZO;

use crate::BffResult;

pub fn lzo_compress<W: Write>(data: &[u8], writer: &mut W, _endian: Endian) -> BffResult<()> {
    let mut lzo = LZO::init()?;
    let compressed = lzo.compress(data)?;
    writer.write_all(&compressed)?;
    Ok(())
}

pub fn lzo_decompress<R: Read>(reader: &mut R, _endian: Endian) -> BffResult<Vec<u8>> {
    let lzo = LZO::init()?;
    let mut compressed: Vec<u8> = Vec::new();
    reader.read_to_end(&mut compressed)?;
    let decompressed = lzo.decompress_safe(&compressed, 0x1000000)?;
    Ok(decompressed)
}
