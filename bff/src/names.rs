use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;

use derive_more::{Display, Error, From};

use crate::crc32;

pub type Name = i32;

#[derive(Debug, Default)]
pub struct Names(HashMap<Name, String>);

#[derive(Debug, From, Display, Error)]
pub enum NamesError {
    Io(io::Error),
    ParseInt(std::num::ParseIntError),
    Utf8(std::string::FromUtf8Error),
    #[display(
        fmt = "CRC-32 mismatch for {}: expected {}, actual {}",
        string,
        expected,
        actual
    )]
    MismatchCrc32 {
        string: String,
        expected: Name,
        actual: Name,
    },
}

impl Names {
    pub fn read<R: BufRead>(&mut self, reader: R) -> Result<(), NamesError> {
        let names = &mut self.0;
        for line in reader.lines() {
            let line = line?;
            let mut line = line.split_whitespace();
            let crc = line.next().unwrap();
            let name = line.next().unwrap();
            let name = name.trim_matches('"');
            let crc = Name::from_str(crc)?;
            let expected = crc32::asobo(name.as_bytes());
            if crc != expected {
                return Err(NamesError::MismatchCrc32 {
                    string: name.to_string(),
                    expected,
                    actual: crc,
                });
            }
            names.entry(crc).or_insert_with(|| name.to_string());
        }

        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), NamesError> {
        for (crc, name) in self.0.iter() {
            let expected = crc32::asobo(name.as_bytes());
            if *crc != expected {
                return Err(NamesError::MismatchCrc32 {
                    string: name.to_string(),
                    expected,
                    actual: *crc,
                });
            }
            writeln!(writer, r#"{} "{}""#, crc, name)?;
        }
        Ok(())
    }
}
