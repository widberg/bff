macro_rules! classes {
    (
        $(
            $(#![$generic:ident])?
            $class:ident
            $({
                $($pattern:pat => $variant_mod:ident::$variant:ident),+ $(,)?
                $(; $(pub mod $extra_mod:ident;)*)?
            })?
            ,
        )*
    ) => {
        $(
            $crate::macros::classes::classes!(@entry
                $(#![$generic])?
                $class
                $({
                    $($pattern => $variant_mod::$variant),+
                    $(; $(pub mod $extra_mod;)*)?
                })?
            );
        )*

        #[derive(serde::Serialize, Debug, derive_more::From, derive_more::IsVariant, serde::Deserialize, bff_derive::ReferencedNames, schemars::JsonSchema)]
        pub enum Class {
            $($class($crate::macros::classes::classes!(@class_ty $class)),)*
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
                $(
                    if let Some((style, name_type)) = $crate::macros::classes::classes!(@class_name_style_type $class name) {
                        return Ok((ClassType::$class, style, name_type));
                    }
                )*
                Err(())
            }
        }

        impl crate::traits::TryFromVersionPlatform<&crate::bigfile::resource::Resource> for Class {
            type Error = crate::error::Error;

            fn try_from_version_platform(resource: &crate::bigfile::resource::Resource, version: crate::bigfile::versions::Version, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<Class> {
                $(
                    if $crate::macros::classes::classes!(@class_name_style_type $class resource.class_name).is_some() {
                        return Ok(<&crate::bigfile::resource::Resource as crate::traits::TryIntoVersionPlatform<$crate::macros::classes::classes!(@class_ty $class)>>::try_into_version_platform(resource, version, platform)?.into());
                    }
                )*
                Err(crate::error::UnimplementedClassError::new(resource.name, resource.class_name, version, platform).into())
            }
        }

        impl crate::traits::TryFromVersionPlatform<&Class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            fn try_from_version_platform(class: &Class, version: crate::bigfile::versions::Version, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<crate::bigfile::resource::Resource> {
                match class {
                    $(Class::$class(class) => Ok(<&$crate::macros::classes::classes!(@class_ty $class) as crate::traits::TryIntoVersionPlatform<crate::bigfile::resource::Resource>>::try_into_version_platform(class, version, platform)?),)*
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
                report.total += 1;
                $(
                    if $crate::macros::classes::classes!(@class_name_style_type $class resource.class_name).is_some() {
                        <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::TryYourBest<&crate::bigfile::resource::Resource>>::update_report(resource, platform, &mut report.$class);
                        return;
                    }
                )*
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
            &[$(<$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<&'static str>>::NAME,<$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<&'static str>>::NAME_LEGACY,)*]
        }

        impl crate::traits::Export for Class {
            fn export(&self) -> crate::BffResult<std::collections::HashMap<std::ffi::OsString, crate::traits::Artifact>> {
                match self {
                    $(Class::$class(class) => <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::Export>::export(class),)*
                }
            }
        }

        impl crate::traits::Import for Class {
            fn import(&mut self, artifacts: &std::collections::HashMap<std::ffi::OsString, crate::traits::Artifact>) -> crate::BffResult<()> {
                match self {
                    $(Class::$class(class) => <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::Import>::import(class, artifacts),)*
                }
            }
        }
    };

    (@entry #![generic] $class:ident { $($body:tt)* }) => {
        $crate::macros::classes::classes!(@module generic $class { $($body)* });
    };
    (@entry #![generic] $class:ident) => {
        $crate::macros::classes::classes!(@module generic $class {});
    };
    (@entry $class:ident { $($body:tt)* }) => {
        $crate::macros::classes::classes!(@module plain $class { $($body)* });
    };
    (@entry $class:ident) => {
        $crate::macros::classes::classes!(@module plain $class {});
    };

    (@class_ty $class:ident) => {
        pastey::paste! {
            crate::class::[<#$class:snake>]::[<$class>]
        }
    };

    (@class_name_style_type $class:ident $name:expr) => {{
        match $name {
            crate::names::Name::Asobo32(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameAsobo32>>::NAME => Some((ClassNameStyle::Z, crate::names::NameType::Asobo32)),
            crate::names::Name::Asobo32(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameAsobo32>>::NAME_LEGACY => Some((ClassNameStyle::Caps, crate::names::NameType::Asobo32)),
            crate::names::Name::AsoboAlternate32(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameAsoboAlternate32>>::NAME => Some((ClassNameStyle::Z, crate::names::NameType::AsoboAlternate32)),
            crate::names::Name::AsoboAlternate32(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameAsoboAlternate32>>::NAME_LEGACY => Some((ClassNameStyle::Caps, crate::names::NameType::AsoboAlternate32)),
            crate::names::Name::Kalisto32(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameKalisto32>>::NAME => Some((ClassNameStyle::Z, crate::names::NameType::Kalisto32)),
            crate::names::Name::Kalisto32(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameKalisto32>>::NAME_LEGACY => Some((ClassNameStyle::Caps, crate::names::NameType::Kalisto32)),
            crate::names::Name::BlackSheep32(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameBlackSheep32>>::NAME => Some((ClassNameStyle::Z, crate::names::NameType::BlackSheep32)),
            crate::names::Name::BlackSheep32(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameBlackSheep32>>::NAME_LEGACY => Some((ClassNameStyle::Caps, crate::names::NameType::BlackSheep32)),
            crate::names::Name::Asobo64(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameAsobo64>>::NAME => Some((ClassNameStyle::Z, crate::names::NameType::Asobo64)),
            crate::names::Name::Asobo64(n) if n == <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::NamedClass<crate::names::NameAsobo64>>::NAME_LEGACY => Some((ClassNameStyle::Caps, crate::names::NameType::Asobo64)),
            _ => None,
        }
    }};

    (@module_kind_prelude generic) => {
        pub mod generic;
    };
    (@module_kind_prelude plain) => {};

    (@module $kind:ident $class:ident {}) => {
        pastey::paste! {
            pub mod [<#$class:snake>] {
                $crate::macros::classes::classes!(@module_kind_prelude $kind);
                $crate::macros::classes::classes!(@declare_class_kind $kind $class {});
            }
        }
    };
    (@module $kind:ident $class:ident { $($pattern:pat => $variant_mod:ident::$variant:ident),+ $(,)? $(; $(pub mod $extra_mod:ident;)*)? }) => {
        pastey::paste! {
            pub mod [<#$class:snake>] {
                $crate::macros::classes::classes!(@module_kind_prelude $kind);
                $(
                    $(
                        pub mod $extra_mod;
                    )*
                )?
                $(
                    mod $variant_mod;
                    use self::$variant_mod::$variant;
                )*
                $crate::macros::classes::classes!(@declare_class_kind $kind $class { $($pattern => $variant),* });
            }
        }
    };

    (@declare_class_kind generic $class:ident $variants:tt) => {
        $crate::macros::classes::classes!(@declare_class #![generic] $class $variants);
    };
    (@declare_class_kind plain $class:ident $variants:tt) => {
        $crate::macros::classes::classes!(@declare_class $class $variants);
    };

    (@named_class $class:ident) => {
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

    (@declare_class $class:ident $variants:tt) => {
        $crate::macros::classes::classes!(@named_class $class);
        $crate::macros::classes::classes!(@declare_class_kind_impl $class $variants);
    };

    (@declare_class_kind_impl $class:ident {}) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, derive_more::From, derive_more::IsVariant, bff_derive::ReferencedNames)]
        pub enum $class {}

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
                class: &$class,
                _version: crate::bigfile::versions::Version,
                _platform: crate::bigfile::platforms::Platform,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                match *class {}
            }
        }

        impl ::schemars::JsonSchema for $class {
            fn is_referenceable() -> bool {
                true
            }

            fn schema_name() -> ::std::string::String {
                stringify!($class).into()
            }

            fn schema_id() -> ::std::borrow::Cow<'static, str> {
                concat!(module_path!(), "::", stringify!($class)).into()
            }

            fn json_schema(_schema_generator: &mut ::schemars::SchemaGenerator) -> ::schemars::schema::Schema {
                ::schemars::schema::Schema::Object(::schemars::schema::SchemaObject {
                    instance_type: ::core::option::Option::Some(::schemars::schema::InstanceType::Object.into()),
                    ..::core::default::Default::default()
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

    (@declare_class_kind_impl $class:ident { $($pattern:pat => $variant:ident),* $(,)? }) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, derive_more::From, derive_more::IsVariant, bff_derive::ReferencedNames, schemars::JsonSchema)]
        pub enum $class {
            $($variant(std::boxed::Box<$variant>)),*
        }

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
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
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
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
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

    (@declare_class #![generic] $class:ident { $($pattern:pat => $variant:ident),* $(,)? }) => {
        $crate::macros::classes::classes!(@declare_class $class { $($pattern => $variant),* });
        $crate::macros::classes::classes!(@declare_class_generic_from $class { $($pattern => $variant),* });
    };

    (@declare_class_generic_from $class:ident { $($pattern:pat => $variant:ident),* $(,)? }) => {
        pastey::paste! {
            impl From<$class> for generic::[<$class Generic>] {
                fn from(class: $class) -> generic::[<$class Generic>] {
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

pub(crate) use classes;
