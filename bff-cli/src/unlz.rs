use std::io::{self, Cursor, Read, Write};

use bff::lz::{decompress_data_with_header_parser, lz4_decompress, lzo_decompress};
use bff::{BufReader, Endian};

use crate::error::BffCliResult;
use crate::lz::{LzAlgorithm, LzEndian};

pub fn unlz(endian: &LzEndian, algorithm: &LzAlgorithm) -> BffCliResult<()> {
    let endian = match endian {
        LzEndian::Big => Endian::Big,
        LzEndian::Little => Endian::Little,
    };

    let stdin = io::stdin();
    let mut buf: Vec<u8> = Vec::new();
    stdin.lock().read_to_end(&mut buf)?;
    let mut reader = BufReader::new(Cursor::new(buf));

    let decompressed = match algorithm {
        LzAlgorithm::Lzrs => decompress_data_with_header_parser(&mut reader, endian)?,
        LzAlgorithm::Lzo => lzo_decompress(&mut reader, endian)?,
        LzAlgorithm::Lz4 => lz4_decompress(&mut reader, endian)?,
    };

    let stdout = io::stdout();
    Ok(stdout.lock().write_all(&decompressed)?)
}
