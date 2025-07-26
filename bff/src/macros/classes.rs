macro_rules! classes {
    ($($class:ident),* $(,)?) => {
        #[derive(serde::Serialize, Debug, derive_more::From, derive_more::IsVariant, serde::Deserialize, bff_derive::ReferencedNames, schemars::JsonSchema)]
        pub enum Class {
            $($class($class),)*
        }

        #[derive(serde::Serialize, Debug, serde::Deserialize, schemars::JsonSchema)]
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

            fn try_from_version_platform(resource: &crate::bigfile::resource::Resource, version: crate::bigfile::versions::Version, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<Class> {
                use crate::traits::NamedClass;
                match resource.class_name {
                    $(crate::names::Name::Asobo32($class::NAME) | crate::names::Name::Asobo32($class::NAME_LEGACY)
                    | crate::names::Name::AsoboAlternate32($class::NAME) | crate::names::Name::AsoboAlternate32($class::NAME_LEGACY)
                    | crate::names::Name::Kalisto32($class::NAME) | crate::names::Name::Kalisto32($class::NAME_LEGACY)
                    | crate::names::Name::BlackSheep32($class::NAME) | crate::names::Name::BlackSheep32($class::NAME_LEGACY)
                    | crate::names::Name::Asobo64($class::NAME) | crate::names::Name::Asobo64($class::NAME_LEGACY)
                        => Ok(<&crate::bigfile::resource::Resource as crate::traits::TryIntoVersionPlatform<$class>>::try_into_version_platform(resource, version, platform)?.into()),)*
                    _ => Err(crate::error::UnimplementedClassError::new(resource.name, resource.class_name, version, platform).into()),
                }
            }
        }

        impl crate::traits::TryFromVersionPlatform<&Class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            fn try_from_version_platform(class: &Class, version: crate::bigfile::versions::Version, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<crate::bigfile::resource::Resource> {
                match class {
                    $(Class::$class(class) => Ok(<&$class as crate::traits::TryIntoVersionPlatform<crate::bigfile::resource::Resource>>::try_into_version_platform(class, version, platform)?),)*
                }
            }
        }

        pastey::paste! {
            #[allow(non_snake_case)]
            #[derive(Default, Debug, Clone, Copy)]
            pub struct ClassTryYourBestReport {
                pub total: usize,
                $($class: crate::class::[<#$class:snake>]::[<$class TryYourBestReport>]),*
            }
        }

        impl crate::traits::TryYourBest<&crate::bigfile::resource::Resource> for Class {
            type Report = ClassTryYourBestReport;
            fn update_report(resource: &crate::bigfile::resource::Resource, platform: crate::bigfile::platforms::Platform, report: &mut Self::Report) {
                use crate::traits::NamedClass;
                report.total += 1;
                match resource.class_name {
                    $(crate::names::Name::Asobo32($class::NAME) | crate::names::Name::Asobo32($class::NAME_LEGACY)
                    | crate::names::Name::AsoboAlternate32($class::NAME) | crate::names::Name::AsoboAlternate32($class::NAME_LEGACY)
                    | crate::names::Name::Kalisto32($class::NAME) | crate::names::Name::Kalisto32($class::NAME_LEGACY)
                    | crate::names::Name::BlackSheep32($class::NAME) | crate::names::Name::BlackSheep32($class::NAME_LEGACY)
                    | crate::names::Name::Asobo64($class::NAME) | crate::names::Name::Asobo64($class::NAME_LEGACY)
                        => <$class as crate::traits::TryYourBest<&crate::bigfile::resource::Resource>>::update_report(resource, platform, &mut report.$class),)*
                    _ => {},
                }
            }
        }

        pastey::paste! {
            impl std::fmt::Display for ClassTryYourBestReport {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    writeln!(f, "ClassTryYourBestReport")?;
                    writeln!(f, "Total: {}", self.total)?;
                    $(if self.$class.total != 0 {
                        <crate::class::[<#$class:snake>]::[<$class TryYourBestReport>] as std::fmt::Display>::fmt(&self.$class, f)?;
                    })*
                    Ok(())
                }
            }
        }

        pub fn class_names() -> &'static[&'static str] {
            use crate::traits::NamedClass;
            &[$($class::NAME,$class::NAME_LEGACY,)*]
        }

        impl crate::traits::Export for Class {
            fn export(&self) -> crate::BffResult<std::collections::HashMap<std::ffi::OsString, crate::traits::Artifact>> {
                match self {
                    $(Class::$class(class) => <$class as crate::traits::Export>::export(class),)*
                }
            }
        }

        impl crate::traits::Import for Class {
            fn import(&mut self, artifacts: &std::collections::HashMap<std::ffi::OsString, crate::traits::Artifact>) -> crate::BffResult<()> {
                match self {
                    $(Class::$class(class) => <$class as crate::traits::Import>::import(class, artifacts),)*
                }
            }
        }
    };
}

pub(crate) use classes;
