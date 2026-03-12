mod wordlist;

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write as _};
use std::hash::Hash;
use std::io::{BufRead, Read, Seek, Write};
use std::str::FromStr;
use std::sync::Mutex;

use binrw::{BinRead, BinResult, BinWrite, Endian};
use const_power_of_two::PowerOfTwoUsize;
use derive_more::{Display, From};
use encoding_rs::WINDOWS_1252;
use num_traits::AsPrimitive;
use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize};
use serde_context::context_scope;
use string_interner::backend::BucketBackend;
use string_interner::{DefaultSymbol, StringInterner};
pub use wordlist::*;

use crate::BffResult;
use crate::class::class_names;
use crate::crc::{Asobo32, Asobo64, AsoboAlternate32, BlackSheep32, Kalisto32, Ubisoft64};
use crate::traits::NameHashFunction;

const FORCED_NAME_STRING_CHAR: char = '$';

thread_local! {
    static ACTIVE_NAME_CONTEXT_STACK: RefCell<Vec<*const NameContext>> = const { RefCell::new(Vec::new()) };
}

struct NameContextScopeGuard;

impl Drop for NameContextScopeGuard {
    fn drop(&mut self) {
        ACTIVE_NAME_CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}

fn with_active_name_context<R>(f: impl FnOnce(Option<&NameContext>) -> R) -> R {
    ACTIVE_NAME_CONTEXT_STACK.with(|stack| {
        let context = stack.borrow().last().copied().map(|ptr| {
            // SAFETY: Pointers are pushed only from `NameContext::scope` and popped by
            // `NameContextScopeGuard`, so they are valid for the duration of the scope.
            unsafe { &*ptr }
        });
        f(context)
    })
}

fn with_name_context<R>(f: impl FnOnce(Option<&NameContext>) -> R) -> R {
    with_active_name_context(|active_context| {
        if active_context.is_some() {
            return f(active_context);
        }

        context_scope(|cx| f(cx.get::<NameContext>().ok()))
    })
}

fn current_name_type() -> NameType {
    with_name_context(|name_context| {
        name_context
            .map(NameContext::name_type)
            .unwrap_or(NameType::Asobo32)
    })
}

fn parse_forced_hash_name_for_type<S: AsRef<str>>(
    name_type: NameType,
    string: S,
) -> Option<(Name, String)> {
    match name_type {
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
        if let Some(string) = string.as_ref().strip_prefix(FORCED_NAME_STRING_CHAR)
            && let Some((value, s)) = string.split_once(FORCED_NAME_STRING_CHAR)
            && let Ok(value) = value.parse::<H::Target>()
        {
            return Some((Self::new(value), s.to_owned()));
        }
        None
    }
}

pub fn parse_forced_hash_name<S: AsRef<str>>(string: S) -> Option<(Name, String)> {
    parse_forced_hash_name_for_type(current_name_type(), string)
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

pub struct NameWithContext<'a> {
    name: &'a Name,
    name_context: &'a NameContext,
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
    pub fn with_context<'a>(&'a self, name_context: &'a NameContext) -> NameWithContext<'a> {
        NameWithContext {
            name: self,
            name_context,
        }
    }

    fn fmt_without_context(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asobo32(name) => write!(f, "{}", name.0),
            Self::AsoboAlternate32(name) => write!(f, "{}", name.0),
            Self::Kalisto32(name) => write!(f, "{}", name.0),
            Self::BlackSheep32(name) => write!(f, "{}", name.0),
            Self::Asobo64(name) => write!(f, "{}", name.0),
            Self::Ubisoft64(name) => write!(f, "{}", name.0),
        }
    }

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
        match current_name_type() {
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
        let name_type = current_name_type();
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

fn matches_name_type(expected: Option<NameType>, actual: NameType) -> bool {
    expected.is_none_or(|expected| expected == actual)
}

fn serialize_name_value<S: serde::Serializer>(
    name: &Name,
    serializer: S,
    expected_name_type: Option<NameType>,
) -> Result<S::Ok, S::Error> {
    use serde::ser::Error as _;

    match name {
        Name::Asobo32(name) if matches_name_type(expected_name_type, NameType::Asobo32) => {
            name.0.serialize(serializer)
        }
        Name::AsoboAlternate32(name)
            if matches_name_type(expected_name_type, NameType::AsoboAlternate32) =>
        {
            name.0.serialize(serializer)
        }
        Name::Kalisto32(name) if matches_name_type(expected_name_type, NameType::Kalisto32) => {
            name.0.serialize(serializer)
        }
        Name::BlackSheep32(name)
            if matches_name_type(expected_name_type, NameType::BlackSheep32) =>
        {
            name.0.serialize(serializer)
        }
        Name::Asobo64(name) if matches_name_type(expected_name_type, NameType::Asobo64) => {
            name.0.serialize(serializer)
        }
        Name::Ubisoft64(name) if matches_name_type(expected_name_type, NameType::Ubisoft64) => {
            name.0.serialize(serializer)
        }
        _ => Err(S::Error::custom("Cannot convert between name types")),
    }
}

impl Default for Name {
    fn default() -> Self {
        match current_name_type() {
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
        context_scope(|cx| {
            if let Ok(names_context) = cx.get::<SerializeNamesContext>() {
                if let Some(name) = names_context.resolve(self) {
                    return name.serialize(serializer);
                }

                return serialize_name_value(self, serializer, Some(names_context.name_type()));
            }

            with_name_context(|name_context| {
                if let Some(name_context) = name_context {
                    if let Some(name) = name_context.resolve(self) {
                        return name.serialize(serializer);
                    }

                    return serialize_name_value(self, serializer, Some(name_context.name_type()));
                }

                serialize_name_value(self, serializer, None)
            })
        })
    }
}

fn deserialize_name_with_type<'de, D, F>(
    deserializer: D,
    name_type: NameType,
    mut add_name: F,
) -> Result<Name, D::Error>
where
    D: Deserializer<'de>,
    F: FnMut(&str),
{
    match name_type {
        NameType::Asobo32 => {
            let serde_name = SerdeName::deserialize(deserializer)?;
            match serde_name {
                SerdeName::Name(name) => Ok(NameAsobo32::new(name).into()),
                SerdeName::Str(string) => {
                    add_name(string);
                    Ok(NameAsobo32::hash_string(string).into())
                }
                SerdeName::String(string) => {
                    add_name(string.as_str());
                    Ok(NameAsobo32::hash_string(string.as_str()).into())
                }
            }
        }
        NameType::AsoboAlternate32 => {
            let serde_name = SerdeName::deserialize(deserializer)?;
            match serde_name {
                SerdeName::Name(name) => Ok(NameAsoboAlternate32::new(name).into()),
                SerdeName::Str(string) => {
                    add_name(string);
                    Ok(NameAsoboAlternate32::hash_string(string).into())
                }
                SerdeName::String(string) => {
                    add_name(string.as_str());
                    Ok(NameAsoboAlternate32::hash_string(string.as_str()).into())
                }
            }
        }
        NameType::Kalisto32 => {
            let serde_name = SerdeName::deserialize(deserializer)?;
            match serde_name {
                SerdeName::Name(name) => Ok(NameKalisto32::new(name).into()),
                SerdeName::Str(string) => {
                    add_name(string);
                    Ok(NameKalisto32::hash_string(string).into())
                }
                SerdeName::String(string) => {
                    add_name(string.as_str());
                    Ok(NameKalisto32::hash_string(string.as_str()).into())
                }
            }
        }
        NameType::BlackSheep32 => {
            let serde_name = SerdeName::deserialize(deserializer)?;
            match serde_name {
                SerdeName::Name(name) => Ok(NameBlackSheep32::new(name).into()),
                SerdeName::Str(string) => {
                    add_name(string);
                    Ok(NameBlackSheep32::hash_string(string).into())
                }
                SerdeName::String(string) => {
                    add_name(string.as_str());
                    Ok(NameBlackSheep32::hash_string(string.as_str()).into())
                }
            }
        }
        NameType::Asobo64 => {
            let serde_name = SerdeName::deserialize(deserializer)?;
            match serde_name {
                SerdeName::Name(name) => Ok(NameAsobo64::new(name).into()),
                SerdeName::Str(string) => {
                    add_name(string);
                    Ok(NameAsobo64::hash_string(string).into())
                }
                SerdeName::String(string) => {
                    add_name(string.as_str());
                    Ok(NameAsobo64::hash_string(string.as_str()).into())
                }
            }
        }
        NameType::Ubisoft64 => {
            let serde_name = SerdeName::deserialize(deserializer)?;
            match serde_name {
                SerdeName::Name(name) => Ok(NameUbisoft64::new(name).into()),
                SerdeName::Str(string) => {
                    add_name(string);
                    Ok(NameUbisoft64::hash_string(string).into())
                }
                SerdeName::String(string) => {
                    add_name(string.as_str());
                    Ok(NameUbisoft64::hash_string(string.as_str()).into())
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        context_scope(|cx| {
            if let Ok(names_context) = cx.get::<DeserializeNamesContext>() {
                return deserialize_name_with_type(
                    deserializer,
                    names_context.name_type(),
                    |string| names_context.insert(string),
                );
            }

            with_name_context(|name_context| {
                let name_type = name_context
                    .map(NameContext::name_type)
                    .unwrap_or(NameType::Asobo32);
                deserialize_name_with_type(deserializer, name_type, |string| {
                    if let Some(name_context) = name_context {
                        name_context.insert(string);
                    }
                })
            })
        })
    }
}

impl JsonSchema for Name {
    fn is_referenceable() -> bool {
        true
    }

    fn schema_name() -> std::string::String {
        "Name".into()
    }

    fn schema_id() -> Cow<'static, str> {
        concat!(module_path!(), "::Name").into()
    }

    fn json_schema(_schema_generator: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Vec(vec![
                InstanceType::String,
                InstanceType::Integer,
            ])),
            ..Default::default()
        })
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_without_context(f)
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_without_context(f)
    }
}

impl Display for NameWithContext<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = self.name_context.resolve(self.name) {
            return write!(f, "{}", name);
        }

        self.name.fmt_without_context(f)
    }
}

impl Debug for NameWithContext<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = self.name_context.resolve(self.name) {
            return write!(f, r#"\"{}\""#, name);
        }

        self.name.fmt_without_context(f)
    }
}

#[derive(Debug)]
struct Names {
    name_type: NameType,
    strings: StringInterner<BucketBackend>,
    asobo32_names: HashMap<NameAsobo32, DefaultSymbol>,
    asobo_alternate32_names: HashMap<NameAsoboAlternate32, DefaultSymbol>,
    kalisto32_names: HashMap<NameKalisto32, DefaultSymbol>,
    blacksheep32_names: HashMap<NameBlackSheep32, DefaultSymbol>,
    asobo64_names: HashMap<NameAsobo64, DefaultSymbol>,
    ubisoft64_names: HashMap<NameUbisoft64, DefaultSymbol>,
}

impl Names {
    fn name_type(&self) -> NameType {
        self.name_type
    }

    fn set_name_type(&mut self, name_type: NameType) {
        self.name_type = name_type;
    }

    fn name_from_i32(&self, value: i32) -> Name {
        match self.name_type {
            NameType::Asobo32 => NameAsobo32::new(value).into(),
            NameType::AsoboAlternate32 => NameAsoboAlternate32::new(value).into(),
            NameType::Kalisto32 => NameKalisto32::new(value).into(),
            NameType::BlackSheep32 => NameBlackSheep32::new(value).into(),
            NameType::Asobo64 => NameAsobo64::new(value as i64).into(),
            NameType::Ubisoft64 => NameUbisoft64::new(value as i64).into(),
        }
    }

    fn parse_i32_or_hash_name(&mut self, token: &str) -> Name {
        if let Ok(value) = token.parse::<i32>() {
            self.name_from_i32(value)
        } else {
            self.insert(token)
        }
    }

    fn insert(&mut self, string: &str) -> Name {
        let sym = self.strings.get_or_intern(string);

        let asobo32 = NameAsobo32::hash_string(string);
        self.asobo32_names.entry(asobo32).or_insert(sym);
        let asobo_alternate32 = NameAsoboAlternate32::hash_string(string);
        self.asobo_alternate32_names
            .entry(asobo_alternate32)
            .or_insert(sym);
        let kalisto32 = NameKalisto32::hash_string(string);
        self.kalisto32_names.entry(kalisto32).or_insert(sym);
        let blacksheep32 = NameBlackSheep32::hash_string(string);
        self.blacksheep32_names.entry(blacksheep32).or_insert(sym);
        let asobo64 = NameAsobo64::hash_string(string);
        self.asobo64_names.entry(asobo64).or_insert(sym);
        let ubisoft64 = NameUbisoft64::hash_string(string);
        self.ubisoft64_names.entry(ubisoft64).or_insert(sym);

        match self.name_type {
            NameType::Asobo32 => asobo32.into(),
            NameType::AsoboAlternate32 => asobo_alternate32.into(),
            NameType::Kalisto32 => kalisto32.into(),
            NameType::BlackSheep32 => blacksheep32.into(),
            NameType::Asobo64 => asobo64.into(),
            NameType::Ubisoft64 => ubisoft64.into(),
        }
    }

    fn get(&self, name: &Name) -> Option<&str> {
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

    fn read<R: BufRead>(&mut self, reader: &mut R) -> BffResult<()> {
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

    fn write<W: Write>(&self, writer: &mut W, names: &Option<Vec<&Name>>) -> BffResult<()> {
        let mut out = String::new();
        for (_, string) in &self.strings {
            match self.name_type {
                NameType::Asobo32 => {
                    let name = NameAsobo32::hash_string(string);
                    if let Some(names) = names
                        && !names.contains(&&Name::Asobo32(name))
                    {
                        continue;
                    }
                    writeln!(out, r#"{} \"{}\""#, name, string)?;
                }
                NameType::AsoboAlternate32 => {
                    let name = NameAsoboAlternate32::hash_string(string);
                    if let Some(names) = names
                        && !names.contains(&&Name::AsoboAlternate32(name))
                    {
                        continue;
                    }
                    writeln!(
                        out,
                        r#"{} \"{}\""#,
                        NameAsoboAlternate32::hash_string(string),
                        string
                    )?;
                }
                NameType::Kalisto32 => {
                    let name = NameKalisto32::hash_string(string);
                    if let Some(names) = names
                        && !names.contains(&&Name::Kalisto32(name))
                    {
                        continue;
                    }
                    writeln!(
                        out,
                        r#"{} \"{}\""#,
                        NameKalisto32::hash_string(string),
                        string
                    )?;
                }
                NameType::BlackSheep32 => {
                    let name = NameBlackSheep32::hash_string(string);
                    if let Some(names) = names
                        && !names.contains(&&Name::BlackSheep32(name))
                    {
                        continue;
                    }
                    writeln!(
                        out,
                        r#"{} \"{}\""#,
                        NameBlackSheep32::hash_string(string),
                        string
                    )?;
                }
                NameType::Asobo64 => {
                    let name = NameAsobo64::hash_string(string);
                    if let Some(names) = names
                        && !names.contains(&&Name::Asobo64(name))
                    {
                        continue;
                    }
                    writeln!(
                        out,
                        r#"{} \"{}\""#,
                        NameAsobo64::hash_string(string),
                        string
                    )?;
                }
                NameType::Ubisoft64 => {
                    let name = NameUbisoft64::hash_string(string);
                    if let Some(names) = names
                        && !names.contains(&&Name::Ubisoft64(name))
                    {
                        continue;
                    }
                    writeln!(
                        out,
                        r#"{} \"{}\""#,
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

pub(crate) struct SerializeNamesContext {
    names: Names,
    name_type: Cell<NameType>,
}

impl SerializeNamesContext {
    fn new(names: Names) -> Self {
        Self {
            name_type: Cell::new(names.name_type()),
            names,
        }
    }

    fn into_names(mut self) -> Names {
        self.names.set_name_type(self.name_type.get());
        self.names
    }

    fn name_type(&self) -> NameType {
        self.name_type.get()
    }

    pub(crate) fn set_name_type(&self, name_type: NameType) {
        self.name_type.set(name_type);
    }

    fn resolve(&self, name: &Name) -> Option<&str> {
        self.names.get(name)
    }
}

pub(crate) struct DeserializeNamesContext {
    names: RefCell<Names>,
}

impl DeserializeNamesContext {
    fn new(names: Names) -> Self {
        Self {
            names: RefCell::new(names),
        }
    }

    fn into_names(self) -> Names {
        self.names.into_inner()
    }

    fn name_type(&self) -> NameType {
        self.names.borrow().name_type()
    }

    pub(crate) fn set_name_type(&self, name_type: NameType) {
        self.names.borrow_mut().set_name_type(name_type);
    }

    pub(crate) fn insert(&self, string: &str) {
        self.names.borrow_mut().insert(string);
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

#[derive(Debug, Default)]
pub struct NameContext {
    names: Mutex<Names>,
}

impl NameContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scope<R>(&self, f: impl FnOnce() -> R) -> R {
        ACTIVE_NAME_CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().push(self as *const Self);
        });
        let _guard = NameContextScopeGuard;
        f()
    }

    pub fn name_type(&self) -> NameType {
        self.names.lock().unwrap().name_type()
    }

    pub fn set_name_type(&self, name_type: NameType) {
        self.names.lock().unwrap().set_name_type(name_type);
    }

    pub fn name_from_i32(&self, value: i32) -> Name {
        self.names.lock().unwrap().name_from_i32(value)
    }

    pub fn parse_i32_or_hash_name(&self, token: &str) -> Name {
        self.names.lock().unwrap().parse_i32_or_hash_name(token)
    }

    pub fn insert(&self, string: &str) -> Name {
        self.names.lock().unwrap().insert(string)
    }

    pub fn contains(&self, name: &Name) -> bool {
        self.names.lock().unwrap().get(name).is_some()
    }

    pub fn resolve(&self, name: &Name) -> Option<String> {
        self.names
            .lock()
            .unwrap()
            .get(name)
            .map(std::borrow::ToOwned::to_owned)
    }

    pub fn read<R: BufRead>(&self, reader: &mut R) -> BffResult<()> {
        self.names.lock().unwrap().read(reader)
    }

    pub fn write<W: Write>(&self, writer: &mut W, names: &Option<Vec<&Name>>) -> BffResult<()> {
        self.names.lock().unwrap().write(writer, names)
    }

    pub fn parse_forced_hash_name<S: AsRef<str>>(&self, string: S) -> Option<(Name, String)> {
        parse_forced_hash_name_for_type(self.name_type(), string)
    }
}

pub mod json {
    use std::io::{Error, ErrorKind, Read, Write};

    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use serde_context::{deserialize_with_context, serialize_with_context};

    use super::{DeserializeNamesContext, NameContext, SerializeNamesContext};

    pub fn from_reader<R, T>(reader: R, name_context: &NameContext) -> serde_json::Result<T>
    where
        R: Read,
        T: DeserializeOwned,
    {
        let mut names_guard = name_context.names.lock().unwrap();
        let names_context = DeserializeNamesContext::new(std::mem::take(&mut *names_guard));
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        let result = deserialize_with_context(&mut deserializer, &names_context);
        *names_guard = names_context.into_names();
        result
    }

    pub fn to_writer_pretty<W, T>(
        writer: W,
        value: &T,
        name_context: &NameContext,
    ) -> serde_json::Result<()>
    where
        W: Write,
        T: Serialize + ?Sized,
    {
        let mut names_guard = name_context.names.lock().unwrap();
        let names_context = SerializeNamesContext::new(std::mem::take(&mut *names_guard));
        let mut serializer = serde_json::Serializer::pretty(writer);
        let result = serialize_with_context(value, &mut serializer, &names_context);
        *names_guard = names_context.into_names();
        result
    }

    pub fn to_string_pretty<T>(value: &T, name_context: &NameContext) -> serde_json::Result<String>
    where
        T: Serialize + ?Sized,
    {
        let mut names_guard = name_context.names.lock().unwrap();
        let names_context = SerializeNamesContext::new(std::mem::take(&mut *names_guard));
        let mut serializer = serde_json::Serializer::pretty(Vec::new());
        let serialize_result = serialize_with_context(value, &mut serializer, &names_context);
        *names_guard = names_context.into_names();
        serialize_result?;
        let bytes = serializer.into_inner();
        String::from_utf8(bytes).map_err(|error| {
            serde_json::Error::io(Error::new(
                ErrorKind::InvalidData,
                format!("serialized JSON was not valid UTF-8: {}", error),
            ))
        })
    }
}
