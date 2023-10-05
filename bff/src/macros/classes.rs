macro_rules! classes_enum {
    ($($i:ident),* $(,)?) => {
        #[derive(serde::Serialize, Debug, derive_more::From, derive_more::IsVariant, serde::Deserialize)]
        #[serde(untagged)]
        pub enum Class {
            $($i(Box<$i>),)*
        }
    };
}

pub(crate) use classes_enum;

macro_rules! objects_to_classes {
    ($($i:ident),* $(,)?) => {
        impl crate::traits::TryFromVersionPlatform<&crate::bigfile::resource::Resource> for Class {
            type Error = crate::error::Error;

            fn try_from_version_platform(object: &crate::bigfile::resource::Resource, version: crate::versions::Version, platform: crate::platforms::Platform) -> crate::BffResult<Class> {
                match object.class_name {
                    $(<$i as crate::traits::NamedClass>::NAME => Ok(Box::new(<&crate::bigfile::resource::Resource as crate::traits::TryIntoVersionPlatform<$i>>::try_into_version_platform(object, version, platform)?).into()),)*
                    _ => Err(crate::error::UnimplementedClassError::new(object.name, object.class_name, version, platform).into()),
                }
            }
        }
    };
}

pub(crate) use objects_to_classes;

macro_rules! classes_to_objects {
    ($($i:ident),* $(,)?) => {
        impl crate::traits::TryFromVersionPlatform<&Class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            fn try_from_version_platform(class: &Class, version: crate::versions::Version, platform: crate::platforms::Platform) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use std::ops::Deref;
                match class {
                    $(Class::$i(class) => Ok(<&$i as crate::traits::TryIntoVersionPlatform<crate::bigfile::resource::Resource>>::try_into_version_platform(class.deref(), version, platform)?),)*
                }
            }
        }
    };
}

pub(crate) use classes_to_objects;

macro_rules! class_name_map {
    ($($i:ident),* $(,)?) => {
        pub fn class_name_map() -> std::collections::HashMap<crate::names::Name, String> {
            let mut map = std::collections::HashMap::new();
            $(map.insert(<$i as crate::traits::NamedClass>::NAME, <$i as crate::traits::NamedClass>::NAME_STR.to_string());)*
            map
        }
    };
}

pub(crate) use class_name_map;

macro_rules! classes {
    ($($i:ident),* $(,)?) => {
        classes_enum!($($i),*);
        objects_to_classes!($($i),*);
        classes_to_objects!($($i),*);
        class_name_map!($($i),*);
    };
}

pub(crate) use classes;
