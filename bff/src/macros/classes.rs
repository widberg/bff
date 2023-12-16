macro_rules! classes {
    ($($class:ident),* $(,)?) => {
        #[derive(serde::Serialize, Debug, derive_more::From, derive_more::IsVariant, serde::Deserialize, bff_derive::ReferencedNames)]
        #[serde(untagged)]
        pub enum Class {
            $($class(Box<$class>),)*
        }

        #[derive(serde::Serialize, Debug, serde::Deserialize)]
        #[serde(untagged)]
        pub enum ClassType {
            $($class,)*
        }

        pub enum ClassNameStyle {
            Z,
            Caps,
        }

        impl TryFrom<crate::names::Name> for (ClassType, ClassNameStyle, crate::names::NameType) {
            type Error = ();

            fn try_from(name: crate::names::Name) -> Result<(ClassType, ClassNameStyle, crate::names::NameType), ()> {
                use crate::traits::NamedClass;
                match name {
                    $(crate::names::Name::Asobo32($class::NAME) => Ok((ClassType::$class, ClassNameStyle::Z, crate::names::NameType::Asobo32)),
                    crate::names::Name::Asobo32($class::NAME_LEGACY) => Ok((ClassType::$class, ClassNameStyle::Caps, crate::names::NameType::Asobo32)),
                    crate::names::Name::AsoboAlternate32($class::NAME) => Ok((ClassType::$class, ClassNameStyle::Z, crate::names::NameType::AsoboAlternate32)),
                    crate::names::Name::AsoboAlternate32($class::NAME_LEGACY) => Ok((ClassType::$class, ClassNameStyle::Caps, crate::names::NameType::AsoboAlternate32)),
                    crate::names::Name::Kalisto32($class::NAME) => Ok((ClassType::$class, ClassNameStyle::Z, crate::names::NameType::Kalisto32)),
                    crate::names::Name::Kalisto32($class::NAME_LEGACY) => Ok((ClassType::$class, ClassNameStyle::Caps, crate::names::NameType::Kalisto32)),
                    crate::names::Name::BlackSheep32($class::NAME) => Ok((ClassType::$class, ClassNameStyle::Z, crate::names::NameType::BlackSheep32)),
                    crate::names::Name::BlackSheep32($class::NAME_LEGACY) => Ok((ClassType::$class, ClassNameStyle::Caps, crate::names::NameType::BlackSheep32)),
                    crate::names::Name::Asobo64($class::NAME) => Ok((ClassType::$class, ClassNameStyle::Z, crate::names::NameType::Asobo64)),
                    crate::names::Name::Asobo64($class::NAME_LEGACY) => Ok((ClassType::$class, ClassNameStyle::Caps, crate::names::NameType::Asobo64)),)*
                    _ => Err(()),
                }
            }
        }

        impl crate::traits::TryFromVersionPlatform<&crate::bigfile::resource::Resource> for Class {
            type Error = crate::error::Error;

            fn try_from_version_platform(object: &crate::bigfile::resource::Resource, version: crate::bigfile::versions::Version, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<Class> {
                use crate::traits::NamedClass;
                match object.class_name {
                    $(crate::names::Name::Asobo32($class::NAME) | crate::names::Name::Asobo32($class::NAME_LEGACY)
                    | crate::names::Name::AsoboAlternate32($class::NAME) | crate::names::Name::AsoboAlternate32($class::NAME_LEGACY)
                    | crate::names::Name::Kalisto32($class::NAME) | crate::names::Name::Kalisto32($class::NAME_LEGACY)
                    | crate::names::Name::BlackSheep32($class::NAME) | crate::names::Name::BlackSheep32($class::NAME_LEGACY)
                    | crate::names::Name::Asobo64($class::NAME) | crate::names::Name::Asobo64($class::NAME_LEGACY)
                        => Ok(Box::new(<&crate::bigfile::resource::Resource as crate::traits::TryIntoVersionPlatform<$class>>::try_into_version_platform(object, version, platform)?).into()),)*
                    _ => Err(crate::error::UnimplementedClassError::new(object.name, object.class_name, version, platform).into()),
                }
            }
        }

        impl crate::traits::TryFromVersionPlatform<&Class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            fn try_from_version_platform(class: &Class, version: crate::bigfile::versions::Version, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use std::ops::Deref;
                match class {
                    $(Class::$class(class) => Ok(<&$class as crate::traits::TryIntoVersionPlatform<crate::bigfile::resource::Resource>>::try_into_version_platform(class.deref(), version, platform)?),)*
                }
            }
        }

        pub fn class_names() -> Vec<&'static str> {
            use crate::traits::NamedClass;
            vec![$($class::NAME,$class::NAME_LEGACY,)*]
        }
    };
}

pub(crate) use classes;
