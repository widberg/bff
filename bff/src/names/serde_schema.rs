use std::borrow::Cow;

use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize};

use super::scope::{with_name_context, with_name_context_mut};
use super::{Name, NameType, hash_string_for_type};
use crate::traits::NameHashFunction;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
enum SerdeName<'a, T> {
    Str(&'a str),
    String(String),
    Name(T),
}

pub(super) fn serialize_name_value_for_hash<H, S>(
    name: Name,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    H: NameHashFunction,
    H::Display: Serialize,
    S: serde::Serializer,
{
    let value: H::Target = name.to_hash_target::<H>();
    let display = H::display_from_target(value);
    display.serialize(serializer)
}

impl Serialize for Name {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error as _;

        with_name_context(|name_context| {
            let Some(name_context) = name_context else {
                return Err(S::Error::custom(
                    "Name serialization requires an active NameContext",
                ));
            };

            if let Some(name) = name_context.resolve(*self) {
                return name.serialize(serializer);
            }

            name_context
                .name_type()
                .serialize_name_value(*self, serializer)
        })
    }
}

pub(super) fn deserialize_name_for_hash<'de, H, D, F>(
    deserializer: D,
    name_type: NameType,
    mut add_name: F,
) -> Result<Name, D::Error>
where
    H: NameHashFunction,
    H::Display: Deserialize<'de>,
    D: Deserializer<'de>,
    F: FnMut(&str),
{
    let serde_name: SerdeName<'_, H::Display> = SerdeName::deserialize(deserializer)?;
    match serde_name {
        SerdeName::Str(string) => {
            add_name(string);
            Ok(hash_string_for_type(name_type, string))
        }
        SerdeName::String(string) => {
            add_name(string.as_str());
            Ok(hash_string_for_type(name_type, string))
        }
        SerdeName::Name(name) => Ok(Name::from_hash_target::<H>(H::target_from_display(name))),
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as _;

        with_name_context_mut(|name_context| {
            if let Some(name_context) = name_context {
                return name_context
                    .name_type()
                    .deserialize_name(deserializer, |string| {
                        name_context.insert(string);
                    });
            }
            with_name_context(|name_context| {
                let Some(name_context) = name_context else {
                    return Err(D::Error::custom(
                        "Name deserialization requires an active NameContext",
                    ));
                };
                name_context
                    .name_type()
                    .deserialize_name(deserializer, |string| {
                        let _ = string;
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
