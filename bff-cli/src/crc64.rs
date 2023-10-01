use std::io::{self, BufRead, Read};

use bff::crc64::asobo_crc64_options;
use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum Crc64Algorithm {
    Asobo,
}

#[derive(ValueEnum, Clone)]
pub enum Crc64Mode {
    Bytes,
    Lines,
}

#[derive(ValueEnum, Clone)]
pub enum Crc64Format {
    Signed,
    Unsigned,
    #[value(alias("hex"))]
    Hexadecimal,
}

fn format_hash(hash: u64, format: &Crc64Format) -> String {
    match format {
        Crc64Format::Signed => {
            format!("{}", hash as i64)
        }
        Crc64Format::Unsigned => {
            format!("{}", hash)
        }
        Crc64Format::Hexadecimal => {
            format!("{:#08x}", hash)
        }
    }
}

pub fn crc64(
    string: &Option<String>,
    starting: &i64,
    algorithm: &Crc64Algorithm,
    mode: &Crc64Mode,
    format: &Crc64Format,
) -> Result<(), Box<dyn std::error::Error>> {
    let starting = *starting;
    let hash_function = match algorithm {
        Crc64Algorithm::Asobo => asobo_crc64_options,
    };

    match (string, mode) {
        (Some(string), Crc64Mode::Bytes) => {
            let hash = hash_function(string.as_bytes(), starting);
            println!("{} {:?}", format_hash(hash, format), string.as_bytes());
        }
        (Some(string), Crc64Mode::Lines) => {
            for line in string.lines() {
                let hash = hash_function(line.as_bytes(), starting);
                println!("{} \"{}\"", format_hash(hash, format), line);
            }
        }
        (None, Crc64Mode::Bytes) => {
            let stdin = io::stdin();
            let mut buf: Vec<u8> = Vec::new();
            stdin.lock().read_to_end(&mut buf)?;
            let hash = hash_function(buf.as_slice(), starting);
            println!("{} {:?}", format_hash(hash, format), buf);
        }
        (None, Crc64Mode::Lines) => {
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
