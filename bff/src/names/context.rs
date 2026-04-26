use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{BufRead, Write};

use encoding_rs::WINDOWS_1252;

use super::{Name, NameType, apply_name_style, hash_string_for_type, name_type_style};
use crate::BffResult;
use crate::class::class_base_names;

thread_local! {
    static ACTIVE_NAME_CONTEXT_STACK: RefCell<Vec<*const NameContext>> = const { RefCell::new(Vec::new()) };
    static ACTIVE_MUT_NAME_CONTEXT_STACK: RefCell<Vec<*mut NameContext>> = const { RefCell::new(Vec::new()) };
}

struct NameContextScopeGuard {
    is_mut: bool,
}

impl Drop for NameContextScopeGuard {
    fn drop(&mut self) {
        if self.is_mut {
            ACTIVE_MUT_NAME_CONTEXT_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
        ACTIVE_NAME_CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}

pub(super) fn with_name_context<R>(f: impl FnOnce(Option<&NameContext>) -> R) -> R {
    ACTIVE_NAME_CONTEXT_STACK.with(|stack| {
        let context = stack.borrow().last().copied().map(|ptr| {
            // SAFETY: Pointers are pushed only from `NameContext::scope` and popped by
            // `NameContextScopeGuard`, so they are valid for the duration of the scope.
            unsafe { &*ptr }
        });
        f(context)
    })
}

pub(super) fn with_name_context_mut<R>(f: impl FnOnce(Option<&mut NameContext>) -> R) -> R {
    ACTIVE_MUT_NAME_CONTEXT_STACK.with(|stack| {
        let context = stack.borrow().last().copied().map(|ptr| {
            // SAFETY: Pointers are pushed only from `NameContext::scope_mut` and popped by
            // `NameContextScopeGuard`, so they are valid for the duration of the scope.
            unsafe { &mut *ptr }
        });
        f(context)
    })
}

pub(crate) fn current_name_type() -> Option<NameType> {
    with_name_context(|name_context| name_context.map(NameContext::name_type))
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

fn parse_i32_or_hash_name(names: &mut NameMap, name_type: NameType, token: &str) -> Name {
    if let Some(name) = name_type.parse_name_value(token) {
        name
    } else {
        insert_name(names, name_type, token)
    }
}

fn insert_name(names: &mut NameMap, name_type: NameType, string: &str) -> Name {
    let name = hash_string_for_type(name_type, string);
    names.entry(name).or_insert_with(|| string.to_owned());
    name
}

fn get_name<'a>(names: &'a NameMap, name: Name) -> Option<&'a str> {
    names.get(&name).map(String::as_str)
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
    // TODO: Handle errors
    assert_eq!(encoding_used, WINDOWS_1252);
    assert!(!had_errors, "Name encoding failed");

    writer.write_all(&cow)?;

    Ok(())
}

#[derive(Debug)]
pub struct NameContext {
    name_type: NameType,
    default_name: Name,
    names: NameMap,
}

impl NameContext {
    pub fn new(name_type: NameType) -> Self {
        Self {
            name_type,
            default_name: hash_string_for_type(name_type, ""),
            names: new_names(name_type),
        }
    }

    pub fn into_retyped(self, name_type: NameType) -> Self {
        if self.name_type == name_type {
            return self;
        }

        let names = into_retyped_names(self.names, self.name_type, name_type);
        Self {
            name_type,
            default_name: hash_string_for_type(name_type, ""),
            names,
        }
    }

    pub fn scope<R>(&self, f: impl FnOnce() -> R) -> R {
        ACTIVE_NAME_CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().push(self as *const Self);
        });
        let _guard = NameContextScopeGuard { is_mut: false };
        f()
    }

    pub fn scope_mut<R>(&mut self, f: impl FnOnce() -> R) -> R {
        ACTIVE_NAME_CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().push(self as *const Self);
        });
        ACTIVE_MUT_NAME_CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().push(self as *mut Self);
        });
        let _guard = NameContextScopeGuard { is_mut: true };
        f()
    }

    pub fn name_type(&self) -> NameType {
        self.name_type
    }

    pub fn default_name(&self) -> Name {
        self.default_name
    }

    pub fn parse_i32_or_hash_name(&mut self, token: &str) -> Name {
        parse_i32_or_hash_name(&mut self.names, self.name_type, token)
    }

    pub fn insert(&mut self, string: &str) -> Name {
        insert_name(&mut self.names, self.name_type, string)
    }

    pub fn contains(&self, name: Name) -> bool {
        get_name(&self.names, name).is_some()
    }

    pub fn resolve(&self, name: Name) -> Option<String> {
        get_name(&self.names, name).map(std::borrow::ToOwned::to_owned)
    }

    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> BffResult<()> {
        read_names(&mut self.names, self.name_type, reader)
    }

    pub fn write<W: Write>(&self, writer: &mut W, names: Option<&[Name]>) -> BffResult<()> {
        write_names(&self.names, self.name_type, writer, names)
    }
}
