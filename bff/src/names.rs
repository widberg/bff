use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::io::{BufRead, Read, Seek, Write};
use std::sync::Mutex;

use binrw::{BinRead, BinResult, BinWrite, Endian};
use derive_more::{Display, From};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize};

use crate::class::class_names;
use crate::crc::{Asobo32, Asobo64, AsoboAlternate32, BlackSheep32, Kalisto32};
use crate::traits::NameHashFunction;
use crate::BffResult;

#[derive(PartialEq, Eq, Hash, Copy, Clone, BinRead, BinWrite, Debug, Display)]
pub struct NameVariant<H: NameHashFunction>(H::Target)
where
    for<'a> H::Target:
        PartialEq + Eq + Hash + Copy + Clone + BinRead + BinWrite<Args<'a> = ()> + Display + Debug,
    for<'a> <H::Target as BinRead>::Args<'a>: Default;

impl<H: NameHashFunction> NameVariant<H>
where
    for<'a> H::Target:
        PartialEq + Eq + Hash + Copy + Clone + BinRead + BinWrite<Args<'a> = ()> + Display + Debug,
    for<'a> <H::Target as BinRead>::Args<'a>: Default,
{
    pub const fn new(value: H::Target) -> Self {
        Self(value)
    }
}

pub type NameAsobo32 = NameVariant<Asobo32>;
pub type NameAsoboAlternate32 = NameVariant<AsoboAlternate32>;
pub type NameKalisto32 = NameVariant<Kalisto32>;
pub type NameBlackSheep32 = NameVariant<BlackSheep32>;
pub type NameAsobo64 = NameVariant<Asobo64>;

#[derive(From, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Name {
    Asobo32(NameAsobo32),
    AsoboAlternate32(NameAsoboAlternate32),
    Kalisto32(NameKalisto32),
    BlackSheep32(NameBlackSheep32),
    Asobo64(NameAsobo64),
}

impl BinRead for Name {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        match names().lock().unwrap().name_type {
            NameType::Asobo32 => NameAsobo32::read_options(reader, endian, ()).map(Name::Asobo32),
            NameType::AsoboAlternate32 => {
                NameAsoboAlternate32::read_options(reader, endian, ()).map(Name::AsoboAlternate32)
            }
            NameType::Kalisto32 => {
                NameKalisto32::read_options(reader, endian, ()).map(Name::Kalisto32)
            }
            NameType::BlackSheep32 => {
                NameBlackSheep32::read_options(reader, endian, ()).map(Name::BlackSheep32)
            }
            NameType::Asobo64 => NameAsobo64::read_options(reader, endian, ()).map(Name::Asobo64),
        }
    }
}

impl BinWrite for Name {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let name_type = names().lock().unwrap().name_type;
        match self {
            Name::Asobo32(name) if name_type == NameType::Asobo32 => {
                name.write_options(writer, endian, ())
            }
            Name::AsoboAlternate32(name) if name_type == NameType::AsoboAlternate32 => {
                name.write_options(writer, endian, ())
            }
            Name::Kalisto32(name) if name_type == NameType::Kalisto32 => {
                name.write_options(writer, endian, ())
            }
            Name::BlackSheep32(name) if name_type == NameType::BlackSheep32 => {
                name.write_options(writer, endian, ())
            }
            Name::Asobo64(name) if name_type == NameType::Asobo64 => {
                name.write_options(writer, endian, ())
            }
            _ => todo!("Cannot convert between name types"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NameType {
    Asobo32,
    AsoboAlternate32,
    Kalisto32,
    BlackSheep32,
    Asobo64,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SerdeName<'a, T> {
    Name(T),
    String(&'a str),
}

impl<H: NameHashFunction> From<&str> for NameVariant<H>
where
    for<'a> H::Target:
        PartialEq + Eq + Hash + Copy + Clone + BinRead + BinWrite<Args<'a> = ()> + Display + Debug,
    for<'a> <H::Target as BinRead>::Args<'a>: Default,
{
    fn from(value: &str) -> Self {
        NAMES.lock().unwrap().insert(value);
        Self(H::hash(value.as_bytes()))
    }
}

impl<H: NameHashFunction> Default for NameVariant<H>
where
    for<'a> H::Target:
        PartialEq + Eq + Hash + Copy + Clone + BinRead + BinWrite<Args<'a> = ()> + Display + Debug,
    for<'a> <H::Target as BinRead>::Args<'a>: Default,
{
    fn default() -> Self {
        Self(H::hash(b""))
    }
}

impl Default for Name {
    fn default() -> Self {
        match names().lock().unwrap().name_type {
            NameType::Asobo32 => NameAsobo32::default().into(),
            NameType::AsoboAlternate32 => NameAsoboAlternate32::default().into(),
            NameType::Kalisto32 => NameKalisto32::default().into(),
            NameType::BlackSheep32 => NameBlackSheep32::default().into(),
            NameType::Asobo64 => NameAsobo64::default().into(),
        }
    }
}

impl Serialize for Name {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name_type = names().lock().unwrap().name_type;
        match NAMES.lock().unwrap().get(self) {
            Some(name) => name.serialize(serializer),
            None => match self {
                Name::Asobo32(name) if name_type == NameType::Asobo32 => {
                    name.0.serialize(serializer)
                }
                Name::AsoboAlternate32(name) if name_type == NameType::AsoboAlternate32 => {
                    name.0.serialize(serializer)
                }
                Name::Kalisto32(name) if name_type == NameType::Kalisto32 => {
                    name.0.serialize(serializer)
                }
                Name::Asobo64(name) if name_type == NameType::Asobo64 => {
                    name.0.serialize(serializer)
                }
                _ => todo!("Cannot convert between name types"),
            },
        }
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match names().lock().unwrap().name_type {
            NameType::Asobo32 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameAsobo32::new(name).into()),
                    SerdeName::String(string) => Ok(NameAsobo32::from(string).into()),
                }
            }
            NameType::AsoboAlternate32 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameAsoboAlternate32::new(name).into()),
                    SerdeName::String(string) => Ok(NameAsoboAlternate32::from(string).into()),
                }
            }
            NameType::Kalisto32 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameKalisto32::new(name).into()),
                    SerdeName::String(string) => Ok(NameKalisto32::from(string).into()),
                }
            }
            NameType::BlackSheep32 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameBlackSheep32::new(name).into()),
                    SerdeName::String(string) => Ok(NameBlackSheep32::from(string).into()),
                }
            }
            NameType::Asobo64 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameAsobo64::new(name).into()),
                    SerdeName::String(string) => Ok(NameAsobo64::from(string).into()),
                }
            }
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = names().lock().unwrap().get(self) {
            write!(f, "{}", name)
        } else {
            match self {
                Name::Asobo32(name) => write!(f, "{}", name.0),
                Name::AsoboAlternate32(name) => write!(f, "{}", name.0),
                Name::Kalisto32(name) => write!(f, "{}", name.0),
                Name::BlackSheep32(name) => write!(f, "{}", name.0),
                Name::Asobo64(name) => write!(f, "{}", name.0),
            }
        }
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = names().lock().unwrap().get(self) {
            write!(f, r#""{}""#, name)
        } else {
            match self {
                Name::Asobo32(name) => write!(f, "{}", name.0),
                Name::AsoboAlternate32(name) => write!(f, "{}", name.0),
                Name::Kalisto32(name) => write!(f, "{}", name.0),
                Name::BlackSheep32(name) => write!(f, "{}", name.0),
                Name::Asobo64(name) => write!(f, "{}", name.0),
            }
        }
    }
}

#[derive(Debug)]
pub struct Names {
    pub name_type: NameType,
    // TODO: Avoid duplicating strings here
    names: HashMap<Name, String>,
}

impl Names {
    fn insert(&mut self, string: &str) {
        // TODO: optimize this
        let asobo32_hash = <Asobo32 as NameHashFunction>::hash(string.as_bytes());
        let asobo_alternate32_hash =
            <AsoboAlternate32 as NameHashFunction>::hash(string.as_bytes());
        let kalisto32_hash = <Kalisto32 as NameHashFunction>::hash(string.as_bytes());
        let blacksheep32_hash = <BlackSheep32 as NameHashFunction>::hash(string.as_bytes());
        let asobo64_hash = <Asobo64 as NameHashFunction>::hash(string.as_bytes());

        self.names
            .entry(NameAsobo32::new(asobo32_hash).into())
            .or_insert_with(|| string.to_string());
        self.names
            .entry(NameAsoboAlternate32::new(asobo_alternate32_hash).into())
            .or_insert_with(|| string.to_string());
        self.names
            .entry(NameKalisto32::new(kalisto32_hash).into())
            .or_insert_with(|| string.to_string());
        self.names
            .entry(NameBlackSheep32::new(blacksheep32_hash).into())
            .or_insert_with(|| string.to_string());
        self.names
            .entry(NameAsobo64::new(asobo64_hash).into())
            .or_insert_with(|| string.to_string());
    }

    fn get(&self, name: &Name) -> Option<&String> {
        self.names.get(name)
    }
}

impl Default for Names {
    fn default() -> Self {
        let mut names = Self {
            name_type: NameType::Asobo32,
            names: Default::default(),
        };

        for class_name in class_names() {
            names.insert(class_name);
        }

        names.insert("");

        names
    }
}

// TODO: This should NOT be a global. It should be passed around as a parameter to the serialize
// and deserialize functions. Doing that with derive is a bit tricky though.
// https://docs.rs/serde_state/latest/serde_state/ outdated.
// Until this is done bff is not thread safe.
static NAMES: Lazy<Mutex<Names>> = Lazy::new(|| Mutex::new(Names::default()));

pub fn names() -> &'static Mutex<Names> {
    &NAMES
}

impl Names {
    pub fn read<R: BufRead>(&mut self, reader: R) -> BffResult<()> {
        for line in reader.lines() {
            let line = line?;
            let (_, string) = line.split_once(' ').unwrap();
            let string = string.trim_matches('"');
            self.insert(string);
        }

        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> BffResult<()> {
        for (name, string) in self.names.iter() {
            writeln!(writer, r#"{} "{}""#, name, string)?;
        }

        Ok(())
    }
}
