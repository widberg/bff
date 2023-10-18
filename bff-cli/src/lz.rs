use std::io::{self, Cursor, Read, Write};

use bff::lz::{
    lzrs_compress_data_with_header_writer,
    lzrs_decompress_data_with_header_parser,
    lz4_compress_data_with_header_writer,
    lz4_decompress_data_with_header_parser,
    lzo_compress,
    lzo_decompress,
};
use bff::{BufReader, Endian};
use clap::ValueEnum;

use crate::error::BffCliResult;

#[derive(ValueEnum, Clone)]
pub enum LzEndian {
    Big,
    Little,
}

#[derive(ValueEnum, Clone)]
pub enum LzAlgorithm {
    Lzrs,
    Lzo,
    Lz4,
}

pub fn lz(endian: &LzEndian, algorithm: &LzAlgorithm) -> BffCliResult<()> {
    let endian = match endian {
        LzEndian::Big => Endian::Big,
        LzEndian::Little => Endian::Little,
    };

    let stdin = io::stdin();
    let mut buf: Vec<u8> = Vec::new();
    stdin.lock().read_to_end(&mut buf)?;

    let mut compressed: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut compressed);

    match algorithm {
        LzAlgorithm::Lzrs => lzrs_compress_data_with_header_writer(&buf, &mut writer, endian)?,
        LzAlgorithm::Lzo => lzo_compress(&buf, &mut writer, endian)?,
        LzAlgorithm::Lz4 => lz4_compress_data_with_header_writer(&buf, &mut writer, endian)?,
    };

    let stdout = io::stdout();
    stdout.lock().write_all(writer.into_inner())?;
    Ok(())
}

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
        LzAlgorithm::Lzrs => lzrs_decompress_data_with_header_parser(&mut reader, endian)?,
        LzAlgorithm::Lzo => lzo_decompress(&mut reader, endian)?,
        LzAlgorithm::Lz4 => lz4_decompress_data_with_header_parser(&mut reader, endian)?,
    };

    let stdout = io::stdout();
    Ok(stdout.lock().write_all(&decompressed)?)
}
