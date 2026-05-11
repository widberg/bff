use std::io::{Error, ErrorKind, Read, Write};

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::{NameContext, NameType};
use crate::bigfile::versions::Version;

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
    version.name_type().map_err(|err: crate::BffError| {
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

pub fn from_reader<R, T>(reader: R, name_context: &mut NameContext) -> serde_json::Result<T>
where
    R: Read,
    T: DeserializeOwned,
{
    name_context.scope_mut(|| serde_json::from_reader(reader))
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
    name_context.scope(|| serde_json::to_writer_pretty(writer, value))
}

pub fn to_string_pretty<T>(value: &T, name_context: &NameContext) -> serde_json::Result<String>
where
    T: Serialize + ?Sized,
{
    name_context.scope(|| serde_json::to_string_pretty(value))
}
