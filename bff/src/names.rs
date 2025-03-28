use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write as _};
use std::hash::Hash;
use std::io::{BufRead, Read, Seek, Write};
use std::sync::Mutex;

use binrw::{BinRead, BinResult, BinWrite, Endian};
use derive_more::{Display, From};
use encoding_rs::WINDOWS_1252;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize};
use string_interner::backend::BucketBackend;
use string_interner::{DefaultSymbol, StringInterner};

use crate::class::class_names;
use crate::crc::{Asobo32, Asobo64, AsoboAlternate32, BlackSheep32, Kalisto32, Ubisoft64};
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
    pub fn hash(bytes: &[u8]) -> Self {
        Self(H::hash(bytes))
    }
}

pub type NameAsobo32 = NameVariant<Asobo32>;
pub type NameAsoboAlternate32 = NameVariant<AsoboAlternate32>;
pub type NameKalisto32 = NameVariant<Kalisto32>;
pub type NameBlackSheep32 = NameVariant<BlackSheep32>;
pub type NameAsobo64 = NameVariant<Asobo64>;
pub type NameUbisoft64 = NameVariant<Ubisoft64>;

#[derive(From, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Name {
    Asobo32(NameAsobo32),
    AsoboAlternate32(NameAsoboAlternate32),
    Kalisto32(NameKalisto32),
    BlackSheep32(NameBlackSheep32),
    Asobo64(NameAsobo64),
    Ubisoft64(NameUbisoft64),
}

impl Name {
    pub fn is_default(&self) -> bool {
        match *self {
            Name::Asobo32(name) => name == NameAsobo32::default(),
            Name::AsoboAlternate32(name) => name == NameAsoboAlternate32::default(),
            Name::Kalisto32(name) => name == NameKalisto32::default(),
            Name::BlackSheep32(name) => name == NameBlackSheep32::default(),
            Name::Asobo64(name) => name == NameAsobo64::default(),
            Name::Ubisoft64(name) => name == NameUbisoft64::default(),
        }
    }
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
            NameType::Ubisoft64 => {
                NameUbisoft64::read_options(reader, endian, ()).map(Name::Ubisoft64)
            }
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
            Name::Ubisoft64(name) if name_type == NameType::Ubisoft64 => {
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
    Ubisoft64,
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
            NameType::Ubisoft64 => NameUbisoft64::default().into(),
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
                Name::BlackSheep32(name) if name_type == NameType::BlackSheep32 => {
                    name.0.serialize(serializer)
                }
                Name::Asobo64(name) if name_type == NameType::Asobo64 => {
                    name.0.serialize(serializer)
                }
                Name::Ubisoft64(name) if name_type == NameType::Ubisoft64 => {
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
            NameType::Ubisoft64 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameUbisoft64::new(name).into()),
                    SerdeName::String(string) => Ok(NameUbisoft64::from(string).into()),
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
                Name::Ubisoft64(name) => write!(f, "{}", name.0),
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
                Name::Ubisoft64(name) => write!(f, "{}", name.0),
            }
        }
    }
}

#[derive(Debug)]
pub struct Names {
    pub name_type: NameType,
    strings: StringInterner<BucketBackend>,
    asobo32_names: HashMap<NameAsobo32, DefaultSymbol>,
    asobo_alternate32_names: HashMap<NameAsoboAlternate32, DefaultSymbol>,
    kalisto32_names: HashMap<NameKalisto32, DefaultSymbol>,
    blacksheep32_names: HashMap<NameBlackSheep32, DefaultSymbol>,
    asobo64_names: HashMap<NameAsobo64, DefaultSymbol>,
    ubisoft64_names: HashMap<NameUbisoft64, DefaultSymbol>,
}

impl Names {
    fn insert(&mut self, string: &str) {
        let bytes = string.as_bytes();
        let sym = self.strings.get_or_intern(string);

        self.asobo32_names
            .entry(NameAsobo32::hash(bytes))
            .or_insert(sym);
        self.asobo_alternate32_names
            .entry(NameAsoboAlternate32::hash(bytes))
            .or_insert(sym);
        self.kalisto32_names
            .entry(NameKalisto32::hash(bytes))
            .or_insert(sym);
        self.blacksheep32_names
            .entry(NameBlackSheep32::hash(bytes))
            .or_insert(sym);
        self.asobo64_names
            .entry(NameAsobo64::hash(bytes))
            .or_insert(sym);
        self.ubisoft64_names
            .entry(NameUbisoft64::hash(bytes))
            .or_insert(sym);
    }

    fn get(&self, name: &Name) -> Option<&str> {
        match name {
            Name::Asobo32(n) => self
                .asobo32_names
                .get(n)
                .and_then(|x| self.strings.resolve(*x)),
            Name::AsoboAlternate32(n) => self
                .asobo_alternate32_names
                .get(n)
                .and_then(|x| self.strings.resolve(*x)),
            Name::Kalisto32(n) => self
                .kalisto32_names
                .get(n)
                .and_then(|x| self.strings.resolve(*x)),
            Name::BlackSheep32(n) => self
                .blacksheep32_names
                .get(n)
                .and_then(|x| self.strings.resolve(*x)),
            Name::Asobo64(n) => self
                .asobo64_names
                .get(n)
                .and_then(|x| self.strings.resolve(*x)),
            Name::Ubisoft64(n) => self
                .ubisoft64_names
                .get(n)
                .and_then(|x| self.strings.resolve(*x)),
        }
    }
}

impl Default for Names {
    fn default() -> Self {
        let mut names = Self {
            name_type: NameType::Asobo32,
            strings: StringInterner::new(),
            asobo32_names: Default::default(),
            asobo_alternate32_names: Default::default(),
            kalisto32_names: Default::default(),
            blacksheep32_names: Default::default(),
            asobo64_names: Default::default(),
            ubisoft64_names: Default::default(),
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
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> BffResult<()> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        let (cow, encoding_used, had_errors) = WINDOWS_1252.decode(&bytes);
        // TODO: Handle errors
        assert_eq!(encoding_used, WINDOWS_1252);
        assert!(!had_errors, "Name decoding failed");

        for line in cow.lines() {
            let (_, string) = line.split_once(' ').unwrap();
            let string = string.trim_matches('"');
            self.insert(string);
        }

        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> BffResult<()> {
        let mut out = String::new();
        match self.name_type {
            NameType::Asobo32 => {
                for (_, string) in &self.strings {
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameAsobo32::hash(string.as_bytes()),
                        string
                    )?;
                }
            }
            NameType::AsoboAlternate32 => {
                for (_, string) in &self.strings {
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameAsoboAlternate32::hash(string.as_bytes()),
                        string
                    )?;
                }
            }
            NameType::Kalisto32 => {
                for (_, string) in &self.strings {
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameKalisto32::hash(string.as_bytes()),
                        string
                    )?;
                }
            }
            NameType::BlackSheep32 => {
                for (_, string) in &self.strings {
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameBlackSheep32::hash(string.as_bytes()),
                        string
                    )?;
                }
            }
            NameType::Asobo64 => {
                for (_, string) in &self.strings {
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameAsobo64::hash(string.as_bytes()),
                        string
                    )?;
                }
            }
            NameType::Ubisoft64 => {
                for (_, string) in &self.strings {
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameUbisoft64::hash(string.as_bytes()),
                        string
                    )?;
                }
            }
        }

        let (cow, encoding_used, had_errors) = WINDOWS_1252.encode(&out);
        // TODO: Handle errors
        assert_eq!(encoding_used, WINDOWS_1252);
        assert!(!had_errors, "Name encoding failed");

        writer.write_all(&cow)?;

        Ok(())
    }
}
