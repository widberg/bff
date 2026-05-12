use std::io::{self, BufRead as _, Read as _};

use bff::crc::{
    asobo_alternate32_options,
    asobo32_options,
    asobo64_options,
    blacksheep32_options,
    kalisto32_options,
    racenet32_options,
    ubisoft64_options,
};
use clap::ValueEnum;

use crate::error::BffCliResult;

#[derive(ValueEnum, Clone, Copy)]
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
    #[value(alias("net"))]
    RaceNet32,
    #[value(alias("m"))]
    MQFEL32,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum CrcMode {
    Bytes,
    Lines,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum CrcFormat {
    #[value(alias("s"))]
    Signed,
    #[value(alias("u"))]
    Unsigned,
    #[value(alias("h"), alias("hex"))]
    Hexadecimal,
}

fn format_hash(hash: i32, format: CrcFormat) -> String {
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

fn format_hash64(hash: i64, format: CrcFormat) -> String {
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

fn hash(bytes: &[u8], starting: i64, algorithm: CrcAlgorithm, format: CrcFormat) -> String {
    match algorithm {
        CrcAlgorithm::Asobo => format_hash(asobo32_options(bytes, starting as i32), format),
        CrcAlgorithm::AsoboAlternate => {
            format_hash(asobo_alternate32_options(bytes, starting as i32), format)
        }
        CrcAlgorithm::Kalisto => format_hash(kalisto32_options(bytes, starting as i32), format),
        CrcAlgorithm::BlackSheep => {
            format_hash(blacksheep32_options(bytes, starting as i32), format)
        }
        CrcAlgorithm::Asobo64 => format_hash64(asobo64_options(bytes, starting), format),
        CrcAlgorithm::Ubisoft64 => format_hash64(ubisoft64_options(bytes, starting), format),
        CrcAlgorithm::RaceNet32 | CrcAlgorithm::MQFEL32 => {
            format_hash(racenet32_options(bytes, starting as i32), format)
        }
    }
}

pub fn crc(
    string: Option<&str>,
    starting: i64,
    algorithm: CrcAlgorithm,
    mode: CrcMode,
    format: CrcFormat,
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
