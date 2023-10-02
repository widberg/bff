use std::io::{self, Cursor, Read, Write};

use bff::lz::compress_data_with_header_writer;
use bff::Endian;
use clap::ValueEnum;

use crate::error::BffCliResult;

#[derive(ValueEnum, Clone)]
pub enum LzEndian {
    Big,
    Little,
}

pub fn lz(endian: &LzEndian) -> BffCliResult<()> {
    let endian = match endian {
        LzEndian::Big => Endian::Big,
        LzEndian::Little => Endian::Little,
    };

    let stdin = io::stdin();
    let mut buf: Vec<u8> = Vec::new();
    stdin.lock().read_to_end(&mut buf)?;

    let mut compressed: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut compressed);

    compress_data_with_header_writer(&buf, &mut writer, endian)?;

    let stdout = io::stdout();
    stdout.lock().write_all(writer.into_inner())?;
    Ok(())
}
