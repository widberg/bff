use std::io::{self, BufRead, Read};

use bff::crc64::Asobo64;
use bff::traits::NameHashFunction;
use clap::ValueEnum;

use crate::crc32::{CrcFormat, CrcMode};
use crate::error::BffCliResult;

#[derive(ValueEnum, Clone)]
pub enum Crc64Algorithm {
    Asobo,
}

fn format_hash(hash: i64, format: &CrcFormat) -> String {
    match format {
        CrcFormat::Signed => {
            format!("{}", hash)
        }
        CrcFormat::Unsigned => {
            format!("{}", hash as u64)
        }
        CrcFormat::Hexadecimal => {
            format!("{:#08x}", hash)
        }
    }
}

pub fn crc64(
    string: &Option<String>,
    starting: &i64,
    algorithm: &Crc64Algorithm,
    mode: &CrcMode,
    format: &CrcFormat,
) -> BffCliResult<()> {
    let starting = *starting;
    let hash_function = match algorithm {
        Crc64Algorithm::Asobo => Asobo64::hash_options,
    };

    match (string, mode) {
        (Some(string), CrcMode::Bytes) => {
            let hash = hash_function(string.as_bytes(), starting);
            println!("{} {:?}", format_hash(hash, format), string.as_bytes());
        }
        (Some(string), CrcMode::Lines) => {
            for line in string.lines() {
                let hash = hash_function(line.as_bytes(), starting);
                println!("{} \"{}\"", format_hash(hash, format), line);
            }
        }
        (None, CrcMode::Bytes) => {
            let stdin = io::stdin();
            let mut buf: Vec<u8> = Vec::new();
            stdin.lock().read_to_end(&mut buf)?;
            let hash = hash_function(&buf, starting);
            println!("{} {:?}", format_hash(hash, format), buf);
        }
        (None, CrcMode::Lines) => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = line?;
                let hash = hash_function(line.as_bytes(), starting);
                println!("{} \"{}\"", format_hash(hash, format), line);
            }
        }
    }
    Ok(())
}
