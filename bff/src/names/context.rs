use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{BufRead, Write};
use std::sync::Mutex;

use encoding_rs::WINDOWS_1252;
use serde_context::context_scope;

use crate::BffResult;
use crate::class::class_base_names;

use super::value::parse_forced_hash_name_for_type;
use super::{Name, NameType, apply_name_style, hash_string_for_type, name_type_style};

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

pub(super) fn with_name_context<R>(f: impl FnOnce(Option<&NameContext>) -> R) -> R {
    with_active_name_context(|active_context| {
        if active_context.is_some() {
            return f(active_context);
        }

        context_scope(|cx| f(cx.get::<NameContext>().ok()))
    })
}

pub(super) fn current_name_type() -> Option<NameType> {
    with_name_context(|name_context| name_context.map(NameContext::name_type))
}

pub(super) fn current_default_name() -> Option<Name> {
    with_name_context(|name_context| name_context.map(NameContext::default_name))
}

pub(super) type NameMap = HashMap<Name, String>;

pub(super) fn new_names(name_type: NameType) -> NameMap {
    let mut names = NameMap::default();

    for class_name in class_base_names() {
        let canonical = apply_name_style(class_name, name_type_style(name_type));
        insert_name(&mut names, name_type, canonical.as_str());
    }

    names.insert(hash_string_for_type(name_type, ""), String::new());

    names
}

fn into_retyped_names(
    mut names: NameMap,
    old_name_type: NameType,
    new_name_type: NameType,
) -> NameMap {
    if old_name_type == new_name_type {
        return names;
    }

    let old_names = std::mem::take(&mut names);
    for string in old_names.into_values() {
        names
            .entry(hash_string_for_type(new_name_type, &string))
            .or_insert(string);
    }
    names
}

fn name_from_i32(name_type: NameType, value: i32) -> Name {
    name_type.name_from_i32(value)
}

fn parse_i32_or_hash_name(names: &mut NameMap, name_type: NameType, token: &str) -> Name {
    if let Ok(value) = token.parse::<i32>() {
        name_from_i32(name_type, value)
    } else {
        insert_name(names, name_type, token)
    }
}

fn insert_name(names: &mut NameMap, name_type: NameType, string: &str) -> Name {
    let name = hash_string_for_type(name_type, string);
    names.entry(name).or_insert_with(|| string.to_owned());
    name
}

fn get_name<'a>(names: &'a NameMap, name: &Name) -> Option<&'a str> {
    names.get(name).map(String::as_str)
}

fn read_names<R: BufRead>(
    names: &mut NameMap,
    name_type: NameType,
    reader: &mut R,
) -> BffResult<()> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;

    let (cow, encoding_used, had_errors) = WINDOWS_1252.decode(&bytes);
    // TODO: Handle errors
    assert_eq!(encoding_used, WINDOWS_1252);
    assert!(!had_errors, "Name decoding failed");

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
    only_names: &Option<Vec<&Name>>,
) -> BffResult<()> {
    let mut out = String::new();
    let mut entries: Vec<(&Name, &String)> = names.iter().collect();
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
            name_type.value_from_name(*name),
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

pub(super) struct SerializeNamesContext {
    name_type: NameType,
    names: NameMap,
}

impl SerializeNamesContext {
    pub(super) fn new(name_type: NameType, names: NameMap) -> Self {
        Self { name_type, names }
    }

    pub(super) fn into_names(self) -> NameMap {
        self.names
    }

    pub(super) fn name_type(&self) -> NameType {
        self.name_type
    }

    pub(super) fn resolve(&self, name: &Name) -> Option<&str> {
        get_name(&self.names, name)
    }
}

pub(super) struct DeserializeNamesContext {
    name_type: NameType,
    names: RefCell<NameMap>,
}

impl DeserializeNamesContext {
    pub(super) fn new(name_type: NameType, names: NameMap) -> Self {
        Self {
            name_type,
            names: RefCell::new(names),
        }
    }

    pub(super) fn into_names(self) -> NameMap {
        self.names.into_inner()
    }

    pub(super) fn name_type(&self) -> NameType {
        self.name_type
    }

    pub(super) fn insert(&self, string: &str) {
        insert_name(&mut self.names.borrow_mut(), self.name_type, string);
    }
}

#[derive(Debug)]
pub struct NameContext {
    name_type: NameType,
    default_name: Name,
    pub(super) names: Mutex<NameMap>,
}

impl NameContext {
    pub fn new(name_type: NameType) -> Self {
        Self {
            name_type,
            default_name: hash_string_for_type(name_type, ""),
            names: Mutex::new(new_names(name_type)),
        }
    }

    pub fn into_retyped(self, name_type: NameType) -> Self {
        if self.name_type == name_type {
            return self;
        }

        let names = into_retyped_names(self.names.into_inner().unwrap(), self.name_type, name_type);
        Self {
            name_type,
            default_name: hash_string_for_type(name_type, ""),
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
        self.name_type
    }

    pub fn default_name(&self) -> Name {
        self.default_name
    }

    pub fn name_from_i32(&self, value: i32) -> Name {
        name_from_i32(self.name_type, value)
    }

    pub fn parse_i32_or_hash_name(&self, token: &str) -> Name {
        parse_i32_or_hash_name(&mut self.names.lock().unwrap(), self.name_type, token)
    }

    pub fn insert(&self, string: &str) -> Name {
        insert_name(&mut self.names.lock().unwrap(), self.name_type, string)
    }

    pub fn contains(&self, name: &Name) -> bool {
        get_name(&self.names.lock().unwrap(), name).is_some()
    }

    pub fn resolve(&self, name: &Name) -> Option<String> {
        let names = self.names.lock().unwrap();
        get_name(&names, name).map(std::borrow::ToOwned::to_owned)
    }

    pub fn read<R: BufRead>(&self, reader: &mut R) -> BffResult<()> {
        read_names(&mut self.names.lock().unwrap(), self.name_type, reader)
    }

    pub fn write<W: Write>(&self, writer: &mut W, names: &Option<Vec<&Name>>) -> BffResult<()> {
        write_names(&self.names.lock().unwrap(), self.name_type, writer, names)
    }

    pub fn parse_forced_hash_name<S: AsRef<str>>(&self, string: S) -> Option<(Name, String)> {
        parse_forced_hash_name_for_type(self.name_type(), string)
    }
}
