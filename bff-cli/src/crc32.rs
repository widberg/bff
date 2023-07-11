use std::io::{self, BufRead, Read};

use bff::crc32::{asobo_alternate_options, asobo_options};
use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum Crc32Algorithm {
    Asobo,
    #[value(alias("alt"))]
    AsoboAlternate,
}

#[derive(ValueEnum, Clone)]
pub enum Crc32Mode {
    Bytes,
    Lines,
}

#[derive(ValueEnum, Clone)]
pub enum Crc32Format {
    Signed,
    Unsigned,
    #[value(alias("hex"))]
    Hexadecimal,
}

fn format_hash(hash: u32, format: &Crc32Format) -> String {
    match format {
        Crc32Format::Signed => {
            format!("{}", hash as i32)
        }
        Crc32Format::Unsigned => {
            format!("{}", hash)
        }
        Crc32Format::Hexadecimal => {
            format!("{:#08x}", hash)
        }
    }
}

pub fn crc32(
    string: &Option<String>,
    starting: &u32,
    algorithm: &Crc32Algorithm,
    mode: &Crc32Mode,
    format: &Crc32Format,
) -> Result<(), Box<dyn std::error::Error>> {
    let starting = *starting;
    let hash_function = match algorithm {
        Crc32Algorithm::Asobo => asobo_options,
        Crc32Algorithm::AsoboAlternate => asobo_alternate_options,
    };

    match (string, mode) {
        (Some(string), Crc32Mode::Bytes) => {
            let hash = hash_function(string.as_bytes(), starting);
            println!("{} {:?}", format_hash(hash, format), string.as_bytes());
        }
        (Some(string), Crc32Mode::Lines) => {
            for line in string.lines() {
                let hash = hash_function(line.as_bytes(), starting);
                println!("{} \"{}\"", format_hash(hash, format), line);
            }
        }
        (None, Crc32Mode::Bytes) => {
            let stdin = io::stdin();
            let mut buf: Vec<u8> = Vec::new();
            stdin.lock().read_to_end(&mut buf)?;
            let hash = hash_function(buf.as_slice(), starting);
            println!("{} {:?}", format_hash(hash, format), buf);
        }
        (None, Crc32Mode::Lines) => {
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
