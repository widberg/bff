use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{BufRead, Write};

use encoding_rs::WINDOWS_1252;

use super::{ALL_NAME_STYLES, Name, NameType, apply_name_style, hash_string_for_type, scope};
use crate::BffResult;
use crate::class::class_base_names;
use crate::error::{InvalidNameDecodingError, InvalidNameEncodingError};

pub(super) type NameMap = HashMap<Name, String>;

fn insert_name(names: &mut NameMap, name_type: NameType, string: &str) -> Name {
    let name = hash_string_for_type(name_type, string);
    names.entry(name).or_insert_with(|| string.to_owned());
    name
}

fn read_names<R: BufRead>(
    names: &mut NameMap,
    name_type: NameType,
    reader: &mut R,
) -> BffResult<()> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;

    let (cow, encoding_used, had_errors) = WINDOWS_1252.decode(&bytes);
    if encoding_used != WINDOWS_1252 {
        return Err(InvalidNameDecodingError::new(format!(
            "decoder returned `{}` but expected `{}`",
            encoding_used.name(),
            WINDOWS_1252.name()
        ))
        .into());
    }
    if had_errors {
        return Err(InvalidNameDecodingError::new(
            "input contained invalid byte sequences".to_owned(),
        )
        .into());
    }

    for line in cow.lines() {
        if let Some((_, string)) = line.split_once(' ') {
            insert_name(names, name_type, string.trim_matches('"'));
        }
    }

    Ok(())
}

fn write_names<W: Write>(
    names: &NameMap,
    name_type: NameType,
    writer: &mut W,
    only_names: Option<&[Name]>,
) -> BffResult<()> {
    let mut out = String::new();
    let mut entries: Vec<(Name, &String)> =
        names.iter().map(|(name, string)| (*name, string)).collect();
    entries.sort_unstable_by(|(name_a, string_a), (name_b, string_b)| {
        name_a
            .as_raw()
            .cmp(&name_b.as_raw())
            .then_with(|| string_a.cmp(string_b))
    });

    for (name, string) in entries {
        if let Some(only_names) = only_names
            && !only_names.contains(&name)
        {
            continue;
        }
        writeln!(
            out,
            r#"{} \"{}\""#,
            name_type.value_string_from_name(name),
            string
        )?;
    }

    let (cow, encoding_used, had_errors) = WINDOWS_1252.encode(&out);
    if encoding_used != WINDOWS_1252 {
        return Err(InvalidNameEncodingError::new(format!(
            "encoder used `{}` but expected `{}`",
            encoding_used.name(),
            WINDOWS_1252.name()
        ))
        .into());
    }
    if had_errors {
        return Err(InvalidNameEncodingError::new(
            "output contains characters not representable in windows-1252".to_owned(),
        )
        .into());
    }

    writer.write_all(&cow)?;

    Ok(())
}

#[derive(Debug)]
pub struct NameContext {
    name_type: NameType,
    names: NameMap,
}

impl NameContext {
    pub fn new(name_type: NameType) -> Self {
        let mut names = NameMap::default();

        for class_name in class_base_names() {
            for style in ALL_NAME_STYLES {
                let canonical = apply_name_style(class_name, *style);
                insert_name(&mut names, name_type, canonical.as_str());
            }
        }

        names.insert(Name::default(), String::new());

        Self { name_type, names }
    }

    pub fn into_retyped(self, name_type: NameType) -> Self {
        if self.name_type == name_type {
            return self;
        }

        let mut names = self.names;
        let old_names = std::mem::take(&mut names);
        for string in old_names.into_values() {
            names
                .entry(hash_string_for_type(name_type, &string))
                .or_insert(string);
        }

        Self { name_type, names }
    }

    pub fn scope<R>(&self, f: impl FnOnce() -> R) -> R {
        scope::scope(self, f)
    }

    pub fn scope_mut<R>(&mut self, f: impl FnOnce() -> R) -> R {
        scope::scope_mut(self, f)
    }

    pub fn name_type(&self) -> NameType {
        self.name_type
    }

    pub fn insert(&mut self, string: &str) -> Name {
        insert_name(&mut self.names, self.name_type, string)
    }

    pub fn contains(&self, name: Name) -> bool {
        self.names.contains_key(&name)
    }

    pub fn resolve(&self, name: Name) -> Option<String> {
        self.names.get(&name).cloned()
    }

    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> BffResult<()> {
        read_names(&mut self.names, self.name_type, reader)
    }

    pub fn write<W: Write>(&self, writer: &mut W, names: Option<&[Name]>) -> BffResult<()> {
        write_names(&self.names, self.name_type, writer, names)
    }
}
