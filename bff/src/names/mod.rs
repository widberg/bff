mod wordlist;

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write as _};
use std::io::{BufRead, Read, Seek, Write};
use std::sync::Mutex;

use binrw::{BinRead, BinResult, BinWrite, Endian};
use const_power_of_two::PowerOfTwoUsize;
use encoding_rs::WINDOWS_1252;
use num_traits::AsPrimitive;
use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize};
use serde_context::context_scope;
pub use wordlist::*;

use crate::BffResult;
use crate::class::class_base_names;
use crate::crc::{Asobo32, Asobo64, AsoboAlternate32, BlackSheep32, Kalisto32, Ubisoft64};
use crate::macros::names::names;
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

fn current_name_type() -> Option<NameType> {
    with_name_context(|name_context| name_context.map(NameContext::name_type))
}

pub fn active_name_type() -> Option<NameType> {
    current_name_type()
}

trait NameTarget: Copy + AsPrimitive<i64> {
    fn from_i32(value: i32) -> Self;
    fn from_raw(raw: u64) -> Self;
    fn into_raw(self) -> u64;
    fn parse_forced(string: &str) -> Option<Self>;
}

impl NameTarget for i32 {
    fn from_i32(value: i32) -> Self {
        value
    }

    fn from_raw(raw: u64) -> Self {
        let raw_u32: u32 = raw.as_();
        raw_u32.as_()
    }

    fn into_raw(self) -> u64 {
        let raw_u32: u32 = self.as_();
        raw_u32.as_()
    }

    fn parse_forced(string: &str) -> Option<Self> {
        string.parse().ok()
    }
}

impl NameTarget for i64 {
    fn from_i32(value: i32) -> Self {
        value.as_()
    }

    fn from_raw(raw: u64) -> Self {
        raw.as_()
    }

    fn into_raw(self) -> u64 {
        self.as_()
    }

    fn parse_forced(string: &str) -> Option<Self> {
        string.parse().ok()
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Name(u64);

impl Name {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_raw(self) -> u64 {
        self.0
    }

    pub fn with_context<'a>(&'a self, name_context: &'a NameContext) -> NameWithContext<'a> {
        NameWithContext {
            name: self,
            name_context,
        }
    }

    fn from_hash_target<H>(value: H::Target) -> Self
    where
        H: NameHashFunction,
        H::Target: NameTarget,
    {
        Self(value.into_raw())
    }

    fn to_hash_target<H>(self) -> H::Target
    where
        H: NameHashFunction,
        H::Target: NameTarget,
    {
        H::Target::from_raw(self.0)
    }

    fn fmt_number_for_type(&self, f: &mut Formatter<'_>, name_type: NameType) -> fmt::Result {
        write!(f, "{}", name_type.value_from_name(*self))
    }

    pub fn is_default(&self) -> bool {
        current_name_type().is_some_and(|name_type| *self == hash_string_for_type(name_type, ""))
    }

    pub fn get_wordlist_encoded_string<const N: usize>(&self, wordlist: [&str; N]) -> String
    where
        usize: PowerOfTwoUsize<N>,
    {
        if current_name_type().is_some_and(NameType::is_32_bit) || self.0 <= u32::MAX as u64 {
            let value_u32: u32 = self.0.as_();
            get_wordlist_encoded_string(value_u32, wordlist)
        } else {
            get_wordlist_encoded_string(self.0, wordlist)
        }
    }

    pub fn get_value(&self) -> i64 {
        match current_name_type() {
            Some(name_type) => name_type.value_from_name(*self),
            None if self.0 <= u32::MAX as u64 => {
                let value_u32: u32 = self.0.as_();
                let value_i32: i32 = value_u32.as_();
                value_i32.as_()
            }
            None => self.0.as_(),
        }
    }
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

fn parse_forced_hash_name_for_hash<H, S>(string: S) -> Option<(Name, String)>
where
    H: NameHashFunction,
    H::Target: NameTarget,
    S: AsRef<str>,
{
    let string = string.as_ref();
    if let Some(string) = string.strip_prefix(FORCED_NAME_STRING_CHAR)
        && let Some((value, name_string)) = string.split_once(FORCED_NAME_STRING_CHAR)
        && let Some(value) = H::Target::parse_forced(value)
    {
        return Some((Name::from_hash_target::<H>(value), name_string.to_owned()));
    }
    None
}

fn hash_bytes_for_hash<H>(bytes: &[u8]) -> Name
where
    H: NameHashFunction,
    H::Target: NameTarget,
{
    Name::from_hash_target::<H>(H::hash(bytes))
}

fn name_from_i32_for_hash<H>(value: i32) -> Name
where
    H: NameHashFunction,
    H::Target: NameTarget,
{
    Name::from_hash_target::<H>(H::Target::from_i32(value))
}

fn name_value_for_hash<H>(name: Name) -> i64
where
    H: NameHashFunction,
    H::Target: NameTarget,
{
    let value: H::Target = name.to_hash_target::<H>();
    value.as_()
}

fn read_name_for_hash<H, R>(reader: &mut R, endian: Endian) -> BinResult<Name>
where
    H: NameHashFunction,
    H::Target: NameTarget + for<'a> BinRead<Args<'a> = ()>,
    R: Read + Seek,
{
    let value = H::Target::read_options(reader, endian, ())?;
    Ok(Name::from_hash_target::<H>(value))
}

fn write_name_for_hash<H, W>(writer: &mut W, endian: Endian, name: Name) -> BinResult<()>
where
    H: NameHashFunction,
    H::Target: NameTarget + for<'a> BinWrite<Args<'a> = ()>,
    W: Write + Seek,
{
    let value: H::Target = name.to_hash_target::<H>();
    value.write_options(writer, endian, ())
}

fn parse_forced_hash_name_for_type<S: AsRef<str>>(
    name_type: NameType,
    string: S,
) -> Option<(Name, String)> {
    name_type.parse_forced_hash_name(string)
}

pub fn parse_forced_hash_name<S: AsRef<str>>(string: S) -> Option<(Name, String)> {
    current_name_type().and_then(|name_type| parse_forced_hash_name_for_type(name_type, string))
}

pub fn hash_bytes_for_type(name_type: NameType, bytes: &[u8]) -> Name {
    name_type.hash_bytes(bytes)
}

pub fn hash_string_for_type<S: AsRef<str>>(name_type: NameType, string: S) -> Name {
    parse_forced_hash_name_for_type(name_type, string.as_ref())
        .map(|(name, _)| name)
        .unwrap_or_else(|| name_type.hash_bytes(string.as_ref().as_bytes()))
}

impl BinRead for Name {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let pos = reader.stream_position().unwrap_or(0);
        let Some(name_type) = current_name_type() else {
            return Err(binrw::Error::AssertFail {
                pos,
                message: "Name read requires an active NameContext".to_owned(),
            });
        };
        name_type.read_name(reader, endian)
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
        let pos = writer.stream_position().unwrap_or(0);
        let Some(name_type) = current_name_type() else {
            return Err(binrw::Error::AssertFail {
                pos,
                message: "Name write requires an active NameContext".to_owned(),
            });
        };
        name_type.write_name(writer, endian, *self)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
enum SerdeName<'a, T> {
    Name(T),
    Str(&'a str),
    String(String),
}

fn serialize_name_value_for_hash<H, S>(
    name: Name,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    H: NameHashFunction,
    H::Target: NameTarget + Serialize,
    S: serde::Serializer,
{
    let value: H::Target = name.to_hash_target::<H>();
    value.serialize(serializer)
}

fn serialize_name_value_for_type<S: serde::Serializer>(
    name: Name,
    serializer: S,
    name_type: NameType,
) -> Result<S::Ok, S::Error> {
    name_type.serialize_name_value(name, serializer)
}

impl Default for Name {
    fn default() -> Self {
        current_name_type()
            .map(|name_type| hash_string_for_type(name_type, ""))
            .unwrap_or_else(|| Name(0))
    }
}

impl Serialize for Name {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error as _;

        context_scope(|cx| {
            if let Ok(names_context) = cx.get::<SerializeNamesContext>() {
                if let Some(name) = names_context.resolve(self) {
                    return name.serialize(serializer);
                }
                return serialize_name_value_for_type(*self, serializer, names_context.name_type());
            }

            with_name_context(|name_context| {
                let Some(name_context) = name_context else {
                    return Err(S::Error::custom(
                        "Name serialization requires an active NameContext",
                    ));
                };

                if let Some(name) = name_context.resolve(self) {
                    return name.serialize(serializer);
                }

                serialize_name_value_for_type(*self, serializer, name_context.name_type())
            })
        })
    }
}

fn deserialize_name_for_hash<'de, H, D, F>(
    deserializer: D,
    name_type: NameType,
    mut add_name: F,
) -> Result<Name, D::Error>
where
    H: NameHashFunction,
    H::Target: NameTarget + Deserialize<'de>,
    D: Deserializer<'de>,
    F: FnMut(&str),
{
    let serde_name: SerdeName<'_, H::Target> = SerdeName::deserialize(deserializer)?;
    match serde_name {
        SerdeName::Name(name) => Ok(Name::from_hash_target::<H>(name)),
        SerdeName::Str(string) => {
            add_name(string);
            Ok(hash_string_for_type(name_type, string))
        }
        SerdeName::String(string) => {
            add_name(string.as_str());
            Ok(hash_string_for_type(name_type, string))
        }
    }
}

fn deserialize_name_with_type<'de, D, F>(
    deserializer: D,
    name_type: NameType,
    add_name: F,
) -> Result<Name, D::Error>
where
    D: Deserializer<'de>,
    F: FnMut(&str),
{
    name_type.deserialize_name(deserializer, add_name)
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as _;

        context_scope(|cx| {
            if let Ok(names_context) = cx.get::<DeserializeNamesContext>() {
                return deserialize_name_with_type(deserializer, names_context.name_type(), |string| {
                    names_context.insert(string);
                });
            }

            with_name_context(|name_context| {
                let Some(name_context) = name_context else {
                    return Err(D::Error::custom(
                        "Name deserialization requires an active NameContext",
                    ));
                };
                deserialize_name_with_type(deserializer, name_context.name_type(), |string| {
                    name_context.insert(string);
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
        with_name_context(|name_context| {
            let Some(name_context) = name_context else {
                return Err(fmt::Error);
            };
            self.fmt_number_for_type(f, name_context.name_type())
        })
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name_type) = current_name_type() {
            self.fmt_number_for_type(f, name_type)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Display for NameWithContext<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = self.name_context.resolve(self.name) {
            return write!(f, "{}", name);
        }

        self.name.fmt_number_for_type(f, self.name_context.name_type())
    }
}

impl Debug for NameWithContext<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = self.name_context.resolve(self.name) {
            return write!(f, r#"\"{}\""#, name);
        }

        self.name.fmt_number_for_type(f, self.name_context.name_type())
    }
}

#[derive(Debug)]
struct Names {
    name_type: NameType,
    names: HashMap<Name, String>,
}

impl Names {
    fn new(name_type: NameType) -> Self {
        let mut names = Self {
            name_type,
            names: Default::default(),
        };

        for class_name in class_base_names() {
            let canonical = apply_name_style(class_name, name_type_style(name_type));
            names.insert(canonical.as_str());
        }

        names.insert("");

        names
    }

    fn into_retyped(mut self, name_type: NameType) -> Self {
        if self.name_type == name_type {
            return self;
        }

        self.name_type = name_type;
        let old_names = std::mem::take(&mut self.names);
        for string in old_names.into_values() {
            self.names
                .entry(hash_string_for_type(self.name_type, &string))
                .or_insert(string);
        }
        self
    }

    fn name_type(&self) -> NameType {
        self.name_type
    }

    fn name_from_i32(&self, value: i32) -> Name {
        self.name_type.name_from_i32(value)
    }

    fn parse_i32_or_hash_name(&mut self, token: &str) -> Name {
        if let Ok(value) = token.parse::<i32>() {
            self.name_from_i32(value)
        } else {
            self.insert(token)
        }
    }

    fn insert(&mut self, string: &str) -> Name {
        let name = hash_string_for_type(self.name_type, string);
        self.names.entry(name).or_insert_with(|| string.to_owned());
        name
    }

    fn get(&self, name: &Name) -> Option<&str> {
        self.names.get(name).map(String::as_str)
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
        let mut entries: Vec<(&Name, &String)> = self.names.iter().collect();
        entries.sort_unstable_by(|(name_a, string_a), (name_b, string_b)| {
            name_a
                .0
                .cmp(&name_b.0)
                .then_with(|| string_a.cmp(string_b))
        });

        for (name, string) in entries {
            if let Some(names) = names
                && !names.contains(&name)
            {
                continue;
            }
            writeln!(
                out,
                r#"{} \"{}\""#,
                self.name_type.value_from_name(*name),
                string
            )?;
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
}

impl SerializeNamesContext {
    fn new(names: Names) -> Self {
        Self { names }
    }

    fn into_names(self) -> Names {
        self.names
    }

    fn name_type(&self) -> NameType {
        self.names.name_type()
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

    pub(crate) fn insert(&self, string: &str) {
        self.names.borrow_mut().insert(string);
    }
}

#[derive(Debug)]
pub struct NameContext {
    names: Mutex<Names>,
}

impl NameContext {
    pub fn new(name_type: NameType) -> Self {
        Self {
            names: Mutex::new(Names::new(name_type)),
        }
    }

    pub fn into_retyped(self, name_type: NameType) -> Self {
        let names = self.names.into_inner().unwrap().into_retyped(name_type);
        Self {
            names: Mutex::new(names),
        }
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
    use serde_json::Value;
    use serde_context::{deserialize_with_context, serialize_with_context};

    use crate::bigfile::versions::Version;

    use super::{DeserializeNamesContext, NameContext, NameType, SerializeNamesContext};

    fn probe_name_type_from_value(value: &Value) -> serde_json::Result<NameType> {
        let version_string = value
            .get("version")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                serde_json::Error::io(Error::new(
                    ErrorKind::InvalidData,
                    "missing string field `version`",
                ))
            })?;
        let version: Version = version_string.into();
        (&version).try_into().map_err(|err: crate::BffError| {
            serde_json::Error::io(Error::new(
                ErrorKind::InvalidData,
                format!("unable to derive NameType from version `{version_string}`: {err}"),
            ))
        })
    }

    pub fn probe_name_type_from_manifest_reader<R: Read>(reader: R) -> serde_json::Result<NameType> {
        let value: Value = serde_json::from_reader(reader)?;
        probe_name_type_from_value(&value)
    }

    pub fn probe_name_type_from_bff_class_reader<R: Read>(reader: R) -> serde_json::Result<NameType> {
        let value: Value = serde_json::from_reader(reader)?;
        let header = value.get("header").ok_or_else(|| {
            serde_json::Error::io(Error::new(
                ErrorKind::InvalidData,
                "missing object field `header`",
            ))
        })?;
        probe_name_type_from_value(header)
    }

    pub fn from_reader<R, T>(reader: R, name_context: &NameContext) -> serde_json::Result<T>
    where
        R: Read,
        T: DeserializeOwned,
    {
        let mut names_guard = name_context.names.lock().unwrap();
        let name_type = names_guard.name_type();
        let names_context = DeserializeNamesContext::new(std::mem::replace(
            &mut *names_guard,
            super::Names::new(name_type),
        ));
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
        let name_type = names_guard.name_type();
        let names_context = SerializeNamesContext::new(std::mem::replace(
            &mut *names_guard,
            super::Names::new(name_type),
        ));
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
        let name_type = names_guard.name_type();
        let names_context = SerializeNamesContext::new(std::mem::replace(
            &mut *names_guard,
            super::Names::new(name_type),
        ));
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

names! {
    styles: [Z(append_z), Caps(str::to_uppercase)],
    names: [
        Asobo32(Z, Asobo32),
        AsoboAlternate32(Caps, AsoboAlternate32),
        Kalisto32(Caps, Kalisto32),
        BlackSheep32(Caps, BlackSheep32),
        Asobo64(Z, Asobo64),
        Ubisoft64(Caps, Ubisoft64),
    ]
}

// Faster than format!() but more verbose
#[inline]
fn append_z(s: &str) -> String {
    let mut styled = String::with_capacity(s.len() + 2);
    styled.push_str(s);
    styled.push_str("_Z");
    styled
}
