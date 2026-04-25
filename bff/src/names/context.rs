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

#[derive(Debug)]
pub(super) struct Names {
    name_type: NameType,
    names: HashMap<Name, String>,
}

impl Names {
    pub(super) fn new(name_type: NameType) -> Self {
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

    pub(super) fn name_type(&self) -> NameType {
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

    pub(super) fn insert(&mut self, string: &str) -> Name {
        let name = hash_string_for_type(self.name_type, string);
        self.names.entry(name).or_insert_with(|| string.to_owned());
        name
    }

    pub(super) fn get(&self, name: &Name) -> Option<&str> {
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
                .as_raw()
                .cmp(&name_b.as_raw())
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

pub(super) struct SerializeNamesContext {
    names: Names,
}

impl SerializeNamesContext {
    pub(super) fn new(names: Names) -> Self {
        Self { names }
    }

    pub(super) fn into_names(self) -> Names {
        self.names
    }

    pub(super) fn name_type(&self) -> NameType {
        self.names.name_type()
    }

    pub(super) fn resolve(&self, name: &Name) -> Option<&str> {
        self.names.get(name)
    }
}

pub(super) struct DeserializeNamesContext {
    names: RefCell<Names>,
}

impl DeserializeNamesContext {
    pub(super) fn new(names: Names) -> Self {
        Self {
            names: RefCell::new(names),
        }
    }

    pub(super) fn into_names(self) -> Names {
        self.names.into_inner()
    }

    pub(super) fn name_type(&self) -> NameType {
        self.names.borrow().name_type()
    }

    pub(super) fn insert(&self, string: &str) {
        self.names.borrow_mut().insert(string);
    }
}

#[derive(Debug)]
pub struct NameContext {
    pub(super) names: Mutex<Names>,
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
