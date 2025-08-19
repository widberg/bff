mod wordlist;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write as _};
use std::hash::Hash;
use std::io::{BufRead, Read, Seek, Write};
use std::str::FromStr;
use std::sync::{LazyLock, Mutex};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use const_power_of_two::PowerOfTwoUsize;
use derive_more::{Display, From};
use encoding_rs::WINDOWS_1252;
use num_traits::AsPrimitive;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Deserializer, Serialize};
use string_interner::backend::BucketBackend;
use string_interner::{DefaultSymbol, StringInterner};
pub use wordlist::*;

use crate::BffResult;
use crate::class::class_names;
use crate::crc::{Asobo32, Asobo64, AsoboAlternate32, BlackSheep32, Kalisto32, Ubisoft64};
use crate::traits::NameHashFunction;

const FORCED_NAME_STRING_CHAR: char = '$';

#[derive(PartialEq, Eq, Hash, Copy, Clone, BinRead, BinWrite, Debug, Display)]
pub struct NameVariant<H: NameHashFunction>(H::Target)
where
    for<'a> H::Target:
        PartialEq + Eq + Hash + Copy + Clone + BinRead + BinWrite<Args<'a> = ()> + Display + Debug,
    for<'a> <H::Target as BinRead>::Args<'a>: Default;

impl<H: NameHashFunction> NameVariant<H>
where
    for<'a> H::Target: PartialEq
        + Eq
        + Hash
        + Copy
        + Clone
        + BinRead
        + BinWrite<Args<'a> = ()>
        + Display
        + Debug
        + FromStr,
    <<H as NameHashFunction>::Target as FromStr>::Err: Debug,
    for<'a> <H::Target as BinRead>::Args<'a>: Default,
{
    pub const fn new(value: H::Target) -> Self {
        Self(value)
    }
    pub fn hash(bytes: &[u8]) -> Self {
        Self(H::hash(bytes))
    }
    pub fn hash_string(string: &str) -> Self {
        if let Some((name, _)) = Self::parse_forced_hash_name(string) {
            return name;
        }
        Self::hash(string.as_bytes())
    }
    pub fn parse_forced_hash_name<S: AsRef<str>>(string: S) -> Option<(Self, String)> {
        if let Some(string) = string.as_ref().strip_prefix(FORCED_NAME_STRING_CHAR) {
            if let Some((value, s)) = string.split_once(FORCED_NAME_STRING_CHAR) {
                if let Ok(value) = value.parse::<H::Target>() {
                    return Some((Self::new(value), s.to_owned()));
                }
            }
        }
        None
    }
}

pub fn parse_forced_hash_name<S: AsRef<str>>(string: S) -> Option<(Name, String)> {
    match names().lock().unwrap().name_type {
        NameType::Asobo32 => {
            NameAsobo32::parse_forced_hash_name(string).map(|(n, s)| (n.into(), s))
        }
        NameType::AsoboAlternate32 => {
            NameAsoboAlternate32::parse_forced_hash_name(string).map(|(n, s)| (n.into(), s))
        }
        NameType::Kalisto32 => {
            NameKalisto32::parse_forced_hash_name(string).map(|(n, s)| (n.into(), s))
        }
        NameType::BlackSheep32 => {
            NameBlackSheep32::parse_forced_hash_name(string).map(|(n, s)| (n.into(), s))
        }
        NameType::Asobo64 => {
            NameAsobo64::parse_forced_hash_name(string).map(|(n, s)| (n.into(), s))
        }
        NameType::Ubisoft64 => {
            NameUbisoft64::parse_forced_hash_name(string).map(|(n, s)| (n.into(), s))
        }
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

fn get_wordlist_encoded_string<T, const N: usize>(x: T, wordlist: [&str; N]) -> String
where
    T: AsPrimitive<usize>,
    usize: PowerOfTwoUsize<N>,
{
    let wordlist_mask = wordlist.len() - 1;
    let wordlist_bits = wordlist_mask.count_ones() as usize;
    let mut out = String::new();
    let mut x = x.as_();
    for _ in 0..(size_of::<T>() * 8).div_ceil(wordlist_bits) {
        let index = x & wordlist_mask;
        out.push_str(wordlist[index]);
        x >>= wordlist_bits;
    }
    out
}

pub fn get_forced_hash_string<S: AsRef<str>>(name: &Name, string: S) -> String {
    let value = name.get_value();
    let string = string.as_ref();
    format!("{FORCED_NAME_STRING_CHAR}{value}{FORCED_NAME_STRING_CHAR}{string}")
}

impl Name {
    pub fn is_default(&self) -> bool {
        match *self {
            Self::Asobo32(name) => name == NameAsobo32::default(),
            Self::AsoboAlternate32(name) => name == NameAsoboAlternate32::default(),
            Self::Kalisto32(name) => name == NameKalisto32::default(),
            Self::BlackSheep32(name) => name == NameBlackSheep32::default(),
            Self::Asobo64(name) => name == NameAsobo64::default(),
            Self::Ubisoft64(name) => name == NameUbisoft64::default(),
        }
    }

    pub fn get_wordlist_encoded_string<const N: usize>(&self, wordlist: [&str; N]) -> String
    where
        usize: PowerOfTwoUsize<N>,
    {
        match self {
            Self::Asobo32(name) => get_wordlist_encoded_string(name.0, wordlist),
            Self::AsoboAlternate32(name) => get_wordlist_encoded_string(name.0, wordlist),
            Self::Kalisto32(name) => get_wordlist_encoded_string(name.0, wordlist),
            Self::BlackSheep32(name) => get_wordlist_encoded_string(name.0, wordlist),
            Self::Asobo64(name) => get_wordlist_encoded_string(name.0, wordlist),
            Self::Ubisoft64(name) => get_wordlist_encoded_string(name.0, wordlist),
        }
    }

    pub fn get_value(&self) -> i64 {
        match self {
            Self::Asobo32(name) => name.0 as i64,
            Self::AsoboAlternate32(name) => name.0 as i64,
            Self::Kalisto32(name) => name.0 as i64,
            Self::BlackSheep32(name) => name.0 as i64,
            Self::Asobo64(name) => name.0,
            Self::Ubisoft64(name) => name.0,
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
            Self::Asobo32(name) if name_type == NameType::Asobo32 => {
                name.write_options(writer, endian, ())
            }
            Self::AsoboAlternate32(name) if name_type == NameType::AsoboAlternate32 => {
                name.write_options(writer, endian, ())
            }
            Self::Kalisto32(name) if name_type == NameType::Kalisto32 => {
                name.write_options(writer, endian, ())
            }
            Self::BlackSheep32(name) if name_type == NameType::BlackSheep32 => {
                name.write_options(writer, endian, ())
            }
            Self::Asobo64(name) if name_type == NameType::Asobo64 => {
                name.write_options(writer, endian, ())
            }
            Self::Ubisoft64(name) if name_type == NameType::Ubisoft64 => {
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
enum SerdeName<'a, T> {
    Name(T),
    Str(&'a str),
    String(String),
}

impl<H: NameHashFunction> From<&str> for NameVariant<H>
where
    for<'a> H::Target: PartialEq
        + Eq
        + Hash
        + Copy
        + Clone
        + BinRead
        + BinWrite<Args<'a> = ()>
        + Display
        + Debug
        + FromStr,
    <<H as NameHashFunction>::Target as FromStr>::Err: Debug,
    for<'a> <H::Target as BinRead>::Args<'a>: Default,
{
    fn from(value: &str) -> Self {
        NAMES.lock().unwrap().insert(value);
        Self::hash_string(value)
    }
}

impl<H: NameHashFunction> Default for NameVariant<H>
where
    for<'a> H::Target: PartialEq
        + Eq
        + Hash
        + Copy
        + Clone
        + BinRead
        + BinWrite<Args<'a> = ()>
        + Display
        + Debug
        + FromStr,
    <<H as NameHashFunction>::Target as FromStr>::Err: Debug,
    for<'a> <H::Target as BinRead>::Args<'a>: Default,
{
    fn default() -> Self {
        Self::hash_string("")
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
                Self::Asobo32(name) if name_type == NameType::Asobo32 => {
                    name.0.serialize(serializer)
                }
                Self::AsoboAlternate32(name) if name_type == NameType::AsoboAlternate32 => {
                    name.0.serialize(serializer)
                }
                Self::Kalisto32(name) if name_type == NameType::Kalisto32 => {
                    name.0.serialize(serializer)
                }
                Self::BlackSheep32(name) if name_type == NameType::BlackSheep32 => {
                    name.0.serialize(serializer)
                }
                Self::Asobo64(name) if name_type == NameType::Asobo64 => {
                    name.0.serialize(serializer)
                }
                Self::Ubisoft64(name) if name_type == NameType::Ubisoft64 => {
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
        let name_type = names().lock().unwrap().name_type;
        match name_type {
            NameType::Asobo32 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameAsobo32::new(name).into()),
                    SerdeName::Str(string) => Ok(NameAsobo32::from(string).into()),
                    SerdeName::String(string) => Ok(NameAsobo32::from(string.as_str()).into()),
                }
            }
            NameType::AsoboAlternate32 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameAsoboAlternate32::new(name).into()),
                    SerdeName::Str(string) => Ok(NameAsoboAlternate32::from(string).into()),
                    SerdeName::String(string) => {
                        Ok(NameAsoboAlternate32::from(string.as_str()).into())
                    }
                }
            }
            NameType::Kalisto32 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameKalisto32::new(name).into()),
                    SerdeName::Str(string) => Ok(NameKalisto32::from(string).into()),
                    SerdeName::String(string) => Ok(NameKalisto32::from(string.as_str()).into()),
                }
            }
            NameType::BlackSheep32 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameBlackSheep32::new(name).into()),
                    SerdeName::Str(string) => Ok(NameBlackSheep32::from(string).into()),
                    SerdeName::String(string) => Ok(NameBlackSheep32::from(string.as_str()).into()),
                }
            }
            NameType::Asobo64 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameAsobo64::new(name).into()),
                    SerdeName::Str(string) => Ok(NameAsobo64::from(string).into()),
                    SerdeName::String(string) => Ok(NameAsobo64::from(string.as_str()).into()),
                }
            }
            NameType::Ubisoft64 => {
                let serde_name = SerdeName::deserialize(deserializer)?;
                match serde_name {
                    SerdeName::Name(name) => Ok(NameUbisoft64::new(name).into()),
                    SerdeName::Str(string) => Ok(NameUbisoft64::from(string).into()),
                    SerdeName::String(string) => Ok(NameUbisoft64::from(string.as_str()).into()),
                }
            }
        }
    }
}

impl JsonSchema for Name {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> Cow<'static, str> {
        "Name".into()
    }

    fn schema_id() -> Cow<'static, str> {
        concat!(module_path!(), "::Name").into()
    }

    fn json_schema(_schema_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": ["string", "integer"]
        })
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = names().lock().unwrap().get(self) {
            write!(f, "{}", name)
        } else {
            match self {
                Self::Asobo32(name) => write!(f, "{}", name.0),
                Self::AsoboAlternate32(name) => write!(f, "{}", name.0),
                Self::Kalisto32(name) => write!(f, "{}", name.0),
                Self::BlackSheep32(name) => write!(f, "{}", name.0),
                Self::Asobo64(name) => write!(f, "{}", name.0),
                Self::Ubisoft64(name) => write!(f, "{}", name.0),
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
                Self::Asobo32(name) => write!(f, "{}", name.0),
                Self::AsoboAlternate32(name) => write!(f, "{}", name.0),
                Self::Kalisto32(name) => write!(f, "{}", name.0),
                Self::BlackSheep32(name) => write!(f, "{}", name.0),
                Self::Asobo64(name) => write!(f, "{}", name.0),
                Self::Ubisoft64(name) => write!(f, "{}", name.0),
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
    pub fn insert(&mut self, string: &str) {
        let sym = self.strings.get_or_intern(string);

        self.asobo32_names
            .entry(NameAsobo32::hash_string(string))
            .or_insert(sym);
        self.asobo_alternate32_names
            .entry(NameAsoboAlternate32::hash_string(string))
            .or_insert(sym);
        self.kalisto32_names
            .entry(NameKalisto32::hash_string(string))
            .or_insert(sym);
        self.blacksheep32_names
            .entry(NameBlackSheep32::hash_string(string))
            .or_insert(sym);
        self.asobo64_names
            .entry(NameAsobo64::hash_string(string))
            .or_insert(sym);
        self.ubisoft64_names
            .entry(NameUbisoft64::hash_string(string))
            .or_insert(sym);
    }

    pub fn get(&self, name: &Name) -> Option<&str> {
        match name {
            Name::Asobo32(n) => self.strings.resolve(*self.asobo32_names.get(n)?),
            Name::AsoboAlternate32(n) => {
                self.strings.resolve(*self.asobo_alternate32_names.get(n)?)
            }
            Name::Kalisto32(n) => self.strings.resolve(*self.kalisto32_names.get(n)?),
            Name::BlackSheep32(n) => self.strings.resolve(*self.blacksheep32_names.get(n)?),
            Name::Asobo64(n) => self.strings.resolve(*self.asobo64_names.get(n)?),
            Name::Ubisoft64(n) => self.strings.resolve(*self.ubisoft64_names.get(n)?),
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
static NAMES: LazyLock<Mutex<Names>> = LazyLock::new(|| Mutex::new(Names::default()));

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
            if let Some((_, string)) = line.split_once(' ') {
                self.insert(string.trim_matches('"'));
            }
        }

        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W, names: &Option<Vec<&Name>>) -> BffResult<()> {
        let mut out = String::new();
        for (_, string) in &self.strings {
            match self.name_type {
                NameType::Asobo32 => {
                    let name = NameAsobo32::hash_string(string);
                    if let Some(names) = names {
                        if !names.contains(&&Name::Asobo32(name)) {
                            continue;
                        }
                    }
                    writeln!(out, r#"{} "{}""#, name, string)?;
                }
                NameType::AsoboAlternate32 => {
                    let name = NameAsoboAlternate32::hash_string(string);
                    if let Some(names) = names {
                        if !names.contains(&&Name::AsoboAlternate32(name)) {
                            continue;
                        }
                    }
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameAsoboAlternate32::hash_string(string),
                        string
                    )?;
                }
                NameType::Kalisto32 => {
                    let name = NameKalisto32::hash_string(string);
                    if let Some(names) = names {
                        if !names.contains(&&Name::Kalisto32(name)) {
                            continue;
                        }
                    }
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameKalisto32::hash_string(string),
                        string
                    )?;
                }
                NameType::BlackSheep32 => {
                    let name = NameBlackSheep32::hash_string(string);
                    if let Some(names) = names {
                        if !names.contains(&&Name::BlackSheep32(name)) {
                            continue;
                        }
                    }
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameBlackSheep32::hash_string(string),
                        string
                    )?;
                }
                NameType::Asobo64 => {
                    let name = NameAsobo64::hash_string(string);
                    if let Some(names) = names {
                        if !names.contains(&&Name::Asobo64(name)) {
                            continue;
                        }
                    }
                    writeln!(out, r#"{} "{}""#, NameAsobo64::hash_string(string), string)?;
                }
                NameType::Ubisoft64 => {
                    let name = NameUbisoft64::hash_string(string);
                    if let Some(names) = names {
                        if !names.contains(&&Name::Ubisoft64(name)) {
                            continue;
                        }
                    }
                    writeln!(
                        out,
                        r#"{} "{}""#,
                        NameUbisoft64::hash_string(string),
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
