use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;
use std::sync::Mutex;

use binrw::{BinRead, BinWrite};
use derive_more::{Deref, DerefMut, Display, Error, From};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize};

use crate::crc32;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    Copy,
    Clone,
    Display,
    From,
    Deref,
    DerefMut,
    BinRead,
    BinWrite,
    Default,
)]
pub struct Name(i32);

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SerdeName {
    Name(Name),
    String(String),
}

impl Name {
    pub const fn new(name: i32) -> Self {
        Self(name)
    }
}

impl Serialize for Name {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(name) = NAMES.lock().unwrap().get(self) {
            SerdeName::String(name.clone()).serialize(serializer)
        } else {
            SerdeName::Name(*self).serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serde_name = SerdeName::deserialize(deserializer)?;
        match serde_name {
            SerdeName::Name(name) => Ok(name),
            SerdeName::String(string) => {
                NAMES
                    .lock()
                    .unwrap()
                    .entry(crc32::asobo(string.as_bytes()).into())
                    .or_insert_with(|| string.clone());
                Ok(crc32::asobo(string.as_bytes()).into())
            }
        }
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct Names(HashMap<Name, String>);

static NAMES: Lazy<Mutex<Names>> = Lazy::new(|| Mutex::new(Names::default()));

pub fn names() -> &'static Mutex<Names> {
    &NAMES
}

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
            let actual = line.next().unwrap();
            let name = line.next().unwrap();
            let name = name.trim_matches('"');
            let actual = i32::from_str(actual)?.into();
            let expected = crc32::asobo(name.as_bytes()).into();
            if actual != expected {
                return Err(NamesError::MismatchCrc32 {
                    string: name.to_string(),
                    expected,
                    actual,
                });
            }
            names.entry(actual).or_insert_with(|| name.to_string());
        }

        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), NamesError> {
        for (crc, name) in self.0.iter() {
            let expected = crc32::asobo(name.as_bytes());
            if *crc != Name::from(expected) {
                return Err(NamesError::MismatchCrc32 {
                    string: name.to_string(),
                    expected: expected.into(),
                    actual: *crc,
                });
            }
            writeln!(writer, r#"{} "{}""#, crc, name)?;
        }
        Ok(())
    }
}
