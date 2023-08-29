use std::io::{self, Cursor, Read, Write};

use bff::lz::decompress_data_with_header_parser;
use bff::{BufReader, Endian};
use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum LzEndian {
    Big,
    Little,
}

pub fn unlz(endian: &LzEndian) -> Result<(), Box<dyn std::error::Error>> {
    let endian = match endian {
        LzEndian::Big => Endian::Big,
        LzEndian::Little => Endian::Little,
    };

    let stdin = io::stdin();
    let mut buf: Vec<u8> = Vec::new();
    stdin.lock().read_to_end(&mut buf)?;
    let mut reader = BufReader::new(Cursor::new(buf));

    let decompressed = decompress_data_with_header_parser(&mut reader, endian, ())?;

    let stdout = io::stdout();
    Ok(stdout.lock().write_all(decompressed.as_slice())?)
}
