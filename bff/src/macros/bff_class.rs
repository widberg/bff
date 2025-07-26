macro_rules! named_class {
    ($class:ident) => {
        // This mess can go away once https://github.com/rust-lang/rust/issues/76001 is stabilized
        pastey::paste! {
            impl crate::traits::NamedClass<crate::names::NameAsobo32> for $class {
                const NAME: crate::names::NameAsobo32 = crate::names::NameAsobo32::new(crate::crc::asobo32(stringify!([<$class _Z>]).as_bytes()));
                const NAME_LEGACY: crate::names::NameAsobo32 = crate::names::NameAsobo32::new(crate::crc::asobo32(stringify!([<$class:upper>]).as_bytes()));
            }

            impl crate::traits::NamedClass<crate::names::NameAsoboAlternate32> for $class {
                const NAME: crate::names::NameAsoboAlternate32 = crate::names::NameAsoboAlternate32::new(crate::crc::asobo_alternate32(stringify!([<$class _Z>]).as_bytes()));
                const NAME_LEGACY: crate::names::NameAsoboAlternate32 = crate::names::NameAsoboAlternate32::new(crate::crc::asobo_alternate32(stringify!([<$class:upper>]).as_bytes()));
            }

            impl crate::traits::NamedClass<crate::names::NameKalisto32> for $class {
                const NAME: crate::names::NameKalisto32 = crate::names::NameKalisto32::new(crate::crc::kalisto32(stringify!([<$class _Z>]).as_bytes()));
                const NAME_LEGACY: crate::names::NameKalisto32 = crate::names::NameKalisto32::new(crate::crc::kalisto32(stringify!([<$class:upper>]).as_bytes()));
            }

            impl crate::traits::NamedClass<crate::names::NameBlackSheep32> for $class {
                const NAME: crate::names::NameBlackSheep32 = crate::names::NameBlackSheep32::new(crate::crc::blacksheep32(stringify!([<$class _Z>]).as_bytes()));
                const NAME_LEGACY: crate::names::NameBlackSheep32 = crate::names::NameBlackSheep32::new(crate::crc::blacksheep32(stringify!([<$class:upper>]).as_bytes()));
            }

            impl crate::traits::NamedClass<crate::names::NameAsobo64> for $class {
                const NAME: crate::names::NameAsobo64 = crate::names::NameAsobo64::new(crate::crc::asobo64(stringify!([<$class _Z>]).as_bytes()));
                const NAME_LEGACY: crate::names::NameAsobo64 = crate::names::NameAsobo64::new(crate::crc::asobo64(stringify!([<$class:upper>]).as_bytes()));
            }

            impl crate::traits::NamedClass<&'static str> for $class {
                const NAME: &'static str = stringify!([<$class _Z>]);
                const NAME_LEGACY: &'static str = stringify!([<$class:upper>]);
            }
        }
    };
}

pub(crate) use named_class;

macro_rules! bff_class {
    ($class:ident {}) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, derive_more::From, derive_more::IsVariant, bff_derive::ReferencedNames)]
        pub enum $class {}

        crate::macros::bff_class::named_class! { $class }

        impl crate::traits::Export for $class {}
        impl crate::traits::Import for $class {}

        impl crate::traits::TryFromVersionPlatform<&crate::bigfile::resource::Resource> for $class {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                resource: &crate::bigfile::resource::Resource,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
            ) -> crate::BffResult<$class> {
                Err(
                        // TODO: Pick the right name based on the algorithm and suffix for the current BigFile
                        crate::error::UnimplementedClassError::new(resource.name, <Self as crate::traits::NamedClass<crate::names::NameAsobo32>>::NAME.into(), version, platform).into(),
                    )
            }
        }

        impl crate::traits::TryFromVersionPlatform<&$class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            fn try_from_version_platform(
                _class: &$class,
                _version: crate::bigfile::versions::Version,
                _platform: crate::bigfile::platforms::Platform,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                todo!()
            }
        }

        impl schemars::JsonSchema for $class {
            fn inline_schema() -> bool {
                true
            }

            fn schema_name() -> std::borrow::Cow<'static, str> {
                stringify!($class).into()
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                format!("{}::{}", module_path!(), stringify!($class)).into()
            }

            fn json_schema(_schema_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "object"
                })
            }
        }

        pastey::paste! {
            #[allow(non_snake_case)]
            #[derive(Default, Clone, Copy, Debug)]
            pub struct [<$class TryYourBestReport>] {
                pub total: usize,
            }

            impl crate::traits::TryYourBest<&crate::bigfile::resource::Resource> for $class {
                type Report = [<$class TryYourBestReport>];
                fn update_report(_resource: &crate::bigfile::resource::Resource, _platform: crate::bigfile::platforms::Platform, report: &mut Self::Report) {
                    report.total += 1;
                }
            }

            impl std::fmt::Display for [<$class TryYourBestReport>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    writeln!(f, "{}", stringify!($class))?;
                    writeln!(f, "Total: {}", self.total)?;
                    Ok(())
                }
            }
        }
    };
    ($class:ident { $($pattern:pat => $variant:ident),* $(,)? }) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, derive_more::From, derive_more::IsVariant, bff_derive::ReferencedNames, schemars::JsonSchema)]
        pub enum $class {
            $($variant(std::boxed::Box<$variant>)),*
        }

        crate::macros::bff_class::named_class! { $class }

        impl crate::traits::Export for $class {
            fn export(&self) -> crate::BffResult<std::collections::HashMap<std::ffi::OsString, crate::traits::Artifact>> {
                match self {
                    $($class::$variant(class) => <$variant as crate::traits::Export>::export(class),)*
                }
            }
        }

        impl crate::traits::Import for $class {
            fn import(&mut self, artifacts: &std::collections::HashMap<std::ffi::OsString, crate::traits::Artifact>) -> crate::BffResult<()> {
                match self {
                    $($class::$variant(class) => <$variant as crate::traits::Import>::import(class, artifacts),)*
                }
            }
        }

        impl crate::traits::TryFromVersionPlatform<&crate::bigfile::resource::Resource> for $class {
            type Error = crate::error::Error;

            #[allow(unused_imports)]
            fn try_from_version_platform(
                resource: &crate::bigfile::resource::Resource,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
            ) -> crate::BffResult<$class> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                match (version.clone(), platform) {
                    $($pattern => {
                        let shadow_class: $variant = <&crate::bigfile::resource::Resource as crate::traits::TryIntoVersionPlatform<$variant>>::try_into_version_platform(resource, version, platform)?;
                        Ok(std::boxed::Box::new(shadow_class).into())
                    })*
                    _ => Err(
                        // TODO: Pick the right name based on the algorithm and suffix for the current BigFile
                        crate::error::UnimplementedClassError::new(resource.name, <Self as crate::traits::NamedClass<crate::names::NameAsobo32>>::NAME.into(), version, platform).into(),
                    ),
                }
            }
        }

        impl crate::traits::TryFromVersionPlatform<&$class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            #[allow(unused_imports)]
            fn try_from_version_platform(
                class: &$class,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::versions::Version::*;
                use crate::bigfile::platforms::Platform::*;
                match class {
                    $($class::$variant(class) => {
                        <&$variant as crate::traits::TryIntoVersionPlatform<crate::bigfile::resource::Resource>>::try_into_version_platform(class, version, platform)
                    })*
                }
            }
        }

        pastey::paste! {
            #[allow(non_snake_case)]
            #[derive(Default, Clone, Copy, Debug)]
            pub struct [<$class TryYourBestReport>] {
                pub total: usize,
                $($variant: usize),*
            }

            impl crate::traits::TryYourBest<&crate::bigfile::resource::Resource> for $class {
                type Report = [<$class TryYourBestReport>];
                fn update_report(resource: &crate::bigfile::resource::Resource, platform: crate::bigfile::platforms::Platform, report: &mut Self::Report) {
                    report.total += 1;
                    // TODO: Probably need a way to do this without specifying a version.
                    $(
                        report.$variant += <bool as Into<usize>>::into(<&crate::bigfile::resource::Resource as crate::traits::TryIntoVersionPlatform<$variant>>::try_into_version_platform(resource, crate::bigfile::versions::Version::Asobo(0, 0, 0, 0), platform).is_ok());
                    )*
                }
            }

            impl std::fmt::Display for [<$class TryYourBestReport>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    writeln!(f, "{}", stringify!($class))?;
                    writeln!(f, "Total: {}", self.total)?;
                    $(
                        writeln!(f, "{}: {}", stringify!($variant), self.$variant)?;
                    )*
                    Ok(())
                }
            }
        }
    };
    (#![generic] $class:ident {}) => {
        bff_class! {$class {}}

        pastey::paste! {
            impl From<#class> for generic::[<$class Generic>] {
                fn from(
                    class: $class,
                ) -> generic::[<$class Generic>] {
                    todo!()
                }
            }
        }
    };
    (#![generic] $class:ident { $($pattern:pat => $variant:ident),* $(,)? }) => {
        bff_class! {$class { $($pattern => $variant),* }}

        pastey::paste! {
            impl From<$class> for generic::[<$class Generic>] {
                fn from(
                    class: $class,
                ) -> generic::[<$class Generic>] {
                    match class {
                        $($class::$variant(class) => {
                            (*class).into()
                        })*
                    }
                }
            }
        }
    };
}

pub(crate) use bff_class;
