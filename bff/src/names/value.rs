use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use const_power_of_two::PowerOfTwoUsize;

use super::scope::{current_name_type, with_name_context};
use super::{NameContext, NameType};
use crate::traits::{NameHashFunction, NameTarget};

const FORCED_NAME_STRING_CHAR: char = '$';

#[derive(PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct Name(u64);

impl Name {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_raw(self) -> u64 {
        self.0
    }

    pub const fn with_context<'a>(&self, name_context: &'a NameContext) -> NameWithContext<'a> {
        NameWithContext {
            name: *self,
            name_context,
        }
    }

    pub(super) fn from_hash_target<H>(value: H::Target) -> Self
    where
        H: NameHashFunction,
    {
        Self(value.into_raw())
    }

    pub(super) fn to_hash_target<H>(self) -> H::Target
    where
        H: NameHashFunction,
    {
        H::Target::from_raw(self.0)
    }

    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

pub struct NameWithContext<'a> {
    name: Name,
    name_context: &'a NameContext,
}

impl NameWithContext<'_> {
    pub fn get_wordlist_encoded_string<const N: usize>(&self, wordlist: [&str; N]) -> String
    where
        usize: PowerOfTwoUsize<N>,
    {
        let target_bits = self.name_context.name_type().target_bits();
        let wordlist_mask = (wordlist.len() - 1) as u64;
        let wordlist_bits = wordlist_mask.count_ones() as usize;
        let mut out = String::new();
        let mut value = self.name.0;
        for _ in 0..target_bits.div_ceil(wordlist_bits) {
            let index = (value & wordlist_mask) as usize;
            out.push_str(wordlist[index]);
            value >>= wordlist_bits;
        }
        out
    }
}

pub fn get_forced_hash_string_for_type<S: AsRef<str>>(
    name_type: NameType,
    name: Name,
    string: S,
) -> String {
    let value = name_type.value_string_from_name(name);
    let string = string.as_ref();
    format!("{FORCED_NAME_STRING_CHAR}{value}{FORCED_NAME_STRING_CHAR}{string}")
}

pub(super) fn parse_name_value_for_hash<H, S>(string: S) -> Option<Name>
where
    H: NameHashFunction,
    S: AsRef<str>,
{
    H::parse_display(string.as_ref()).map(Name::from_hash_target::<H>)
}

pub(super) fn parse_forced_hash_name_for_hash<H, S>(string: S) -> Option<(Name, String)>
where
    H: NameHashFunction,
    S: AsRef<str>,
{
    let string = string.as_ref();
    if let Some(string) = string.strip_prefix(FORCED_NAME_STRING_CHAR)
        && let Some((value, name_string)) = string.split_once(FORCED_NAME_STRING_CHAR)
        && let Some(name) = parse_name_value_for_hash::<H, _>(value)
    {
        return Some((name, name_string.to_owned()));
    }
    None
}

pub(super) fn hash_bytes_for_hash<H>(bytes: &[u8]) -> Name
where
    H: NameHashFunction,
{
    Name::from_hash_target::<H>(H::hash(bytes))
}

pub(super) fn name_value_string_for_hash<H>(name: Name) -> String
where
    H: NameHashFunction,
{
    let value: H::Target = name.to_hash_target::<H>();
    H::display_from_target(value).to_string()
}

pub(super) fn fmt_name_for_hash<H>(name: Name, f: &mut Formatter<'_>) -> fmt::Result
where
    H: NameHashFunction,
{
    let value: H::Target = name.to_hash_target::<H>();
    write!(f, "{}", H::display_from_target(value))
}

pub(super) fn read_name_for_hash<H, R>(reader: &mut R, endian: Endian) -> BinResult<Name>
where
    H: NameHashFunction,
    H::Target: for<'a> BinRead<Args<'a> = ()>,
    R: Read + Seek,
{
    let value = H::Target::read_options(reader, endian, ())?;
    Ok(Name::from_hash_target::<H>(value))
}

pub(super) fn write_name_for_hash<H, W>(writer: &mut W, endian: Endian, name: Name) -> BinResult<()>
where
    H: NameHashFunction,
    H::Target: for<'a> BinWrite<Args<'a> = ()>,
    W: Write + Seek,
{
    let value: H::Target = name.to_hash_target::<H>();
    value.write_options(writer, endian, ())
}

pub fn parse_forced_hash_name<S: AsRef<str>>(string: S) -> Option<(Name, String)> {
    current_name_type()?.parse_forced_hash_name(string)
}

pub fn hash_string_for_type<S: AsRef<str>>(name_type: NameType, string: S) -> Name {
    name_type
        .parse_forced_hash_name(string.as_ref())
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

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        with_name_context(|name_context| {
            let Some(name_context) = name_context else {
                return Err(fmt::Error);
            };
            name_context.name_type().fmt_name_value(*self, f)
        })
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name_type) = current_name_type() {
            name_type.fmt_name_value(*self, f)
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

        self.name_context.name_type().fmt_name_value(self.name, f)
    }
}

impl Debug for NameWithContext<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = self.name_context.resolve(self.name) {
            return write!(f, r#"\"{}\""#, name);
        }

        self.name_context.name_type().fmt_name_value(self.name, f)
    }
}
