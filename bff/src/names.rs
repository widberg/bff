use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, Write};
use std::str::FromStr;
use std::sync::Mutex;

use binrw::{BinRead, BinWrite};
use derive_more::{Deref, DerefMut, From};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize};

use crate::class::class_name_map;
use crate::error::MismatchCrc32Error;
use crate::{crc32, BffResult};

// If games with 64 bit names are added then this should be a generic and type aliases for Name32
// and Name64 should be defined.
#[derive(PartialEq, Eq, Hash, Copy, Clone, From, BinRead, BinWrite, Default)]
pub struct Name(i32);

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SerdeName {
    Name(i32),
    String(String),
}

impl Name {
    // This is a const fn so it can be used in const contexts i.e. bff-derive generated NamedClass
    // impl
    pub const fn new(name: i32) -> Self {
        Self(name)
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        let name = crc32::asobo(value.as_bytes()).into();
        NAMES
            .lock()
            .unwrap()
            .entry(name)
            .or_insert_with(|| value.to_string());
        name
    }
}

impl Serialize for Name {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match NAMES.lock().unwrap().get(self) {
            Some(name) => SerdeName::String(name.clone()).serialize(serializer),
            None => SerdeName::Name(self.0).serialize(serializer),
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
            SerdeName::Name(name) => Ok(Name(name)),
            SerdeName::String(string) => Ok(Name::from(string.as_str())),
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = NAMES.lock().unwrap().get(self) {
            write!(f, "{}", name)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = NAMES.lock().unwrap().get(self) {
            write!(f, r#""{}""#, name)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct Names(HashMap<Name, String>);

impl Default for Names {
    fn default() -> Self {
        // By default Names should have all the class names and the empty string
        let mut names = class_name_map();
        names.insert(Name::default(), "".to_string());

        Self(names)
    }
}

static NAMES: Lazy<Mutex<Names>> = Lazy::new(|| Mutex::new(Names::default()));

pub fn names() -> &'static Mutex<Names> {
    &NAMES
}

impl Names {
    pub fn read<R: BufRead>(&mut self, reader: R) -> BffResult<()> {
        let names = &mut self.0;
        for line in reader.lines() {
            let line = line?;
            // We really should only split on the first whitespace
            let mut line = line.split_whitespace();
            let actual = line.next().unwrap();
            let name = line.next().unwrap();
            // We should also validate that the quotes are there
            let name = name.trim_matches('"');
            let actual = i32::from_str(actual)?.into();
            let expected = crc32::asobo(name.as_bytes()).into();
            if actual != expected {
                return Err(MismatchCrc32Error::new(name.to_string(), expected, actual).into());
            }
            names.entry(actual).or_insert_with(|| name.to_string());
        }

        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> BffResult<()> {
        for (actual, name) in self.0.iter() {
            let expected = crc32::asobo(name.as_bytes());
            if *actual != Name::from(expected) {
                return Err(
                    MismatchCrc32Error::new(name.to_string(), expected.into(), *actual).into(),
                );
            }
            writeln!(writer, r#"{} "{}""#, actual, name)?;
        }
        Ok(())
    }
}
