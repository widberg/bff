use std::io::{self, BufRead, Read};

use bff::crc::{Asobo32, Asobo64, AsoboAlternate32, BlackSheep32, Kalisto32, Ubisoft64};
use bff::traits::NameHashFunction;
use clap::ValueEnum;

use crate::error::BffCliResult;

#[derive(ValueEnum, Clone)]
pub enum CrcAlgorithm {
    #[value(alias("a"))]
    Asobo,
    #[value(alias("alt"))]
    AsoboAlternate,
    #[value(alias("k"))]
    Kalisto,
    #[value(alias("bs"))]
    BlackSheep,
    #[value(alias("a64"))]
    Asobo64,
    #[value(alias("u64"))]
    Ubisoft64,
}

#[derive(ValueEnum, Clone)]
pub enum CrcMode {
    Bytes,
    Lines,
}

#[derive(ValueEnum, Clone)]
pub enum CrcFormat {
    #[value(alias("s"))]
    Signed,
    #[value(alias("u"))]
    Unsigned,
    #[value(alias("h"), alias("hex"))]
    Hexadecimal,
}

fn format_hash(hash: i32, format: &CrcFormat) -> String {
    match format {
        CrcFormat::Signed => {
            format!("{}", hash)
        }
        CrcFormat::Unsigned => {
            format!("{}", hash as u32)
        }
        CrcFormat::Hexadecimal => {
            format!("{:#08x}", hash)
        }
    }
}

fn format_hash64(hash: i64, format: &CrcFormat) -> String {
    match format {
        CrcFormat::Signed => {
            format!("{}", hash)
        }
        CrcFormat::Unsigned => {
            format!("{}", hash as u64)
        }
        CrcFormat::Hexadecimal => {
            format!("{:#016x}", hash)
        }
    }
}

fn hash(bytes: &[u8], starting: &i64, algorithm: &CrcAlgorithm, format: &CrcFormat) -> String {
    let starting = *starting;
    match algorithm {
        CrcAlgorithm::Asobo => format_hash(Asobo32::hash_options(bytes, starting as i32), format),
        CrcAlgorithm::AsoboAlternate => format_hash(
            AsoboAlternate32::hash_options(bytes, starting as i32),
            format,
        ),
        CrcAlgorithm::Kalisto => {
            format_hash(Kalisto32::hash_options(bytes, starting as i32), format)
        }
        CrcAlgorithm::BlackSheep => {
            format_hash(BlackSheep32::hash_options(bytes, starting as i32), format)
        }
        CrcAlgorithm::Asobo64 => format_hash64(Asobo64::hash_options(bytes, starting), format),
        CrcAlgorithm::Ubisoft64 => format_hash64(Ubisoft64::hash_options(bytes, starting), format),
    }
}

pub fn crc(
    string: &Option<String>,
    starting: &i64,
    algorithm: &CrcAlgorithm,
    mode: &CrcMode,
    format: &CrcFormat,
) -> BffCliResult<()> {
    match (string, mode) {
        (Some(string), CrcMode::Bytes) => {
            println!("{}", hash(string.as_bytes(), starting, algorithm, format));
        }
        (Some(string), CrcMode::Lines) => {
            for line in string.lines() {
                println!(
                    r#"{} "{}""#,
                    hash(line.as_bytes(), starting, algorithm, format),
                    line
                );
            }
        }
        (None, CrcMode::Bytes) => {
            let stdin = io::stdin();
            let mut buf: Vec<u8> = Vec::new();
            stdin.lock().read_to_end(&mut buf)?;
            println!("{}", hash(&buf, starting, algorithm, format));
        }
        (None, CrcMode::Lines) => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = line?;
                println!(
                    r#"{} "{}""#,
                    hash(line.as_bytes(), starting, algorithm, format),
                    line
                );
            }
        }
    }
    Ok(())
}
