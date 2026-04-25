use std::io::{Error, ErrorKind, Read, Write};

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_context::{deserialize_with_context, serialize_with_context};
use serde_json::Value;

use crate::bigfile::versions::Version;

use super::context::{DeserializeNamesContext, new_names};
use super::{NameContext, NameType};

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

pub fn from_reader<R, T>(reader: R, name_context: &mut NameContext) -> serde_json::Result<T>
where
    R: Read,
    T: DeserializeOwned,
{
    let name_type = name_context.name_type();
    let names_context = DeserializeNamesContext::new(
        name_type,
        name_context.replace_names(new_names(name_type)),
    );
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let result = deserialize_with_context(&mut deserializer, &names_context);
    let _ = name_context.replace_names(names_context.into_names());
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
    let mut serializer = serde_json::Serializer::pretty(writer);
    serialize_with_context(value, &mut serializer, name_context)
}

pub fn to_string_pretty<T>(value: &T, name_context: &NameContext) -> serde_json::Result<String>
where
    T: Serialize + ?Sized,
{
    let mut serializer = serde_json::Serializer::pretty(Vec::new());
    serialize_with_context(value, &mut serializer, name_context)?;
    let bytes = serializer.into_inner();
    String::from_utf8(bytes).map_err(|error| {
        serde_json::Error::io(Error::new(
            ErrorKind::InvalidData,
            format!("serialized JSON was not valid UTF-8: {}", error),
        ))
    })
}
