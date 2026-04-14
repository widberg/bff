macro_rules! classes {
    (
        $(
            $class:ident
            $({
                $($pattern:pat => $variant_mod:ident::$variant:ident),+ $(,)?
                $(; $(pub mod $extra_mod:ident;)*)?
            })?
            ,
        )*
    ) => {
        $(
            $crate::macros::classes::classes!(@module
                $class
                $({
                    $($pattern => $variant_mod::$variant),+
                    $(; $(pub mod $extra_mod;)*)?
                })?
            );
        )*

        $crate::macros::classes::classes!(@emit_class_enums $($class)*);
        $crate::macros::classes::classes!(@emit_class_name_maps $($class)*);
        $crate::macros::classes::classes!(@emit_try_from_resource $($class)*);
        $crate::macros::classes::classes!(@emit_try_into_resource $($class)*);
        $crate::macros::classes::classes!(@emit_class_try_your_best_report $($class)*);
        $crate::macros::classes::classes!(@emit_class_try_your_best_impl $($class)*);
        $crate::macros::classes::classes!(@emit_class_try_your_best_display $($class)*);
        $crate::macros::classes::classes!(@emit_class_names_fn $($class)*);
        $crate::macros::classes::classes!(@emit_class_export_impl $($class)*);
        $crate::macros::classes::classes!(@emit_class_import_impl $($class)*);
    };

    (@emit_class_enums $($class:ident)*) => {
        #[derive(serde::Serialize, Debug, derive_more::From, derive_more::IsVariant, serde::Deserialize, bff_derive::ReferencedNames, schemars::JsonSchema)]
        pub enum Class {
            $($class($crate::macros::classes::classes!(@class_ty $class)),)*
        }

        #[derive(serde::Serialize, Debug, Clone, Copy, Eq, PartialEq, serde::Deserialize, schemars::JsonSchema)]
        pub enum ClassType {
            $($class,)*
        }
    };

    (@emit_class_name_maps $($class:ident)*) => {
        impl ClassType {
            pub fn from_name_and_type(
                name: crate::names::Name,
                name_type: crate::names::NameType,
            ) -> Option<Self> {
                use std::collections::HashMap;
                use std::sync::OnceLock;

                fn build(
                    name_type: crate::names::NameType,
                ) -> HashMap<crate::names::Name, ClassType> {
                    let mut map = HashMap::new();
                    $(
                        let styled = crate::names::apply_name_style(
                            stringify!($class),
                            crate::names::name_type_style(name_type),
                        );
                        map.insert(
                            crate::names::hash_string_for_type(name_type, styled),
                            ClassType::$class,
                        );
                    )*
                    map
                }

                static ASOBO32: OnceLock<HashMap<crate::names::Name, ClassType>> = OnceLock::new();
                static ASOBO_ALTERNATE32: OnceLock<HashMap<crate::names::Name, ClassType>> = OnceLock::new();
                static KALISTO32: OnceLock<HashMap<crate::names::Name, ClassType>> = OnceLock::new();
                static BLACKSHEEP32: OnceLock<HashMap<crate::names::Name, ClassType>> = OnceLock::new();
                static ASOBO64: OnceLock<HashMap<crate::names::Name, ClassType>> = OnceLock::new();
                static UBISOFT64: OnceLock<HashMap<crate::names::Name, ClassType>> = OnceLock::new();

                let map = match name_type {
                    crate::names::NameType::Asobo32 => {
                        ASOBO32.get_or_init(|| build(crate::names::NameType::Asobo32))
                    }
                    crate::names::NameType::AsoboAlternate32 => ASOBO_ALTERNATE32
                        .get_or_init(|| build(crate::names::NameType::AsoboAlternate32)),
                    crate::names::NameType::Kalisto32 => {
                        KALISTO32.get_or_init(|| build(crate::names::NameType::Kalisto32))
                    }
                    crate::names::NameType::BlackSheep32 => {
                        BLACKSHEEP32.get_or_init(|| build(crate::names::NameType::BlackSheep32))
                    }
                    crate::names::NameType::Asobo64 => {
                        ASOBO64.get_or_init(|| build(crate::names::NameType::Asobo64))
                    }
                    crate::names::NameType::Ubisoft64 => {
                        UBISOFT64.get_or_init(|| build(crate::names::NameType::Ubisoft64))
                    }
                };

                map.get(&name).copied()
            }
        }

        impl TryFrom<(crate::names::Name, crate::names::NameType)> for ClassType {
            type Error = ();

            fn try_from(
                (name, name_type): (crate::names::Name, crate::names::NameType),
            ) -> Result<ClassType, ()> {
                ClassType::from_name_and_type(name, name_type).ok_or(())
            }
        }
    };

    (@emit_try_from_resource $($class:ident)*) => {
        impl crate::traits::TryFromVersionPlatform<&crate::bigfile::resource::Resource> for Class {
            type Error = crate::error::Error;

            fn try_from_version_platform(resource: &crate::bigfile::resource::Resource, version: crate::bigfile::versions::Version, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<Class> {
                let name_type = (&version).try_into()?;
                if let Some(class_type) = ClassType::from_name_and_type(resource.class_name, name_type) {
                    return match class_type {
                        $(
                            ClassType::$class => Ok(<&crate::bigfile::resource::Resource as crate::traits::TryIntoVersionPlatform<$crate::macros::classes::classes!(@class_ty $class)>>::try_into_version_platform(resource, version, platform)?.into()),
                        )*
                    };
                }
                Err(crate::error::UnimplementedClassError::new(resource.name, resource.class_name, version, platform).into())
            }
        }
    };

    (@emit_try_into_resource $($class:ident)*) => {
        impl crate::traits::TryFromVersionPlatform<&Class> for crate::bigfile::resource::Resource {
            type Error = crate::error::Error;

            fn try_from_version_platform(class: &Class, version: crate::bigfile::versions::Version, platform: crate::bigfile::platforms::Platform) -> crate::BffResult<crate::bigfile::resource::Resource> {
                match class {
                    $(Class::$class(class) => Ok(<&$crate::macros::classes::classes!(@class_ty $class) as crate::traits::TryIntoVersionPlatform<crate::bigfile::resource::Resource>>::try_into_version_platform(class, version, platform)?),)*
                }
            }
        }
    };

    (@emit_class_try_your_best_report $($class:ident)*) => {
        pastey::paste! {
            #[allow(non_snake_case)]
            #[derive(Default, Debug, Clone, Copy)]
            pub struct ClassTryYourBestReport {
                pub total: usize,
                $($class: crate::class::[<#$class:snake>]::[<$class TryYourBestReport>]),*
            }
        }
    };

    (@emit_class_try_your_best_impl $($class:ident)*) => {
        impl crate::traits::TryYourBest<&crate::bigfile::resource::Resource> for Class {
            type Report = ClassTryYourBestReport;
            fn update_report(resource: &crate::bigfile::resource::Resource, platform: crate::bigfile::platforms::Platform, report: &mut Self::Report) {
                report.total += 1;
                let Some(name_type) = crate::names::active_name_type() else {
                    return;
                };
                let Some(class_type) = ClassType::from_name_and_type(resource.class_name, name_type) else {
                    return;
                };
                match class_type {
                    $(
                        ClassType::$class => {
                            <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::TryYourBest<&crate::bigfile::resource::Resource>>::update_report(resource, platform, &mut report.$class);
                        }
                    )*
                }
            }
        }
    };

    (@emit_class_try_your_best_display $($class:ident)*) => {
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
    };

    (@emit_class_names_fn $($class:ident)*) => {
        pub fn class_base_names() -> &'static[&'static str] {
            &[ $(stringify!($class),)* ]
        }
    };

    (@emit_class_export_impl $($class:ident)*) => {
        impl crate::traits::Export for Class {
            fn export(&self) -> crate::BffResult<std::collections::HashMap<std::ffi::OsString, crate::traits::Artifact>> {
                match self {
                    $(Class::$class(class) => <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::Export>::export(class),)*
                }
            }
        }
    };

    (@emit_class_import_impl $($class:ident)*) => {
        impl crate::traits::Import for Class {
            fn import(&mut self, artifacts: &std::collections::HashMap<std::ffi::OsString, crate::traits::Artifact>) -> crate::BffResult<()> {
                match self {
                    $(Class::$class(class) => <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::Import>::import(class, artifacts),)*
                }
            }
        }
    };

    (@class_ty $class:ident) => {
        pastey::paste! {
            crate::class::[<#$class:snake>]::[<$class>]
        }
    };

    (@module $class:ident) => {
        pastey::paste! {
            pub mod [<#$class:snake>] {
                $crate::macros::classes::classes!(@declare_class_kind_impl $class {});
            }
        }
    };
    (@module $class:ident { $($pattern:pat => $variant_mod:ident::$variant:ident),+ $(,)? $(; $(pub mod $extra_mod:ident;)*)? }) => {
        pastey::paste! {
            pub mod [<#$class:snake>] {
                $(
                    $(
                        pub mod $extra_mod;
                    )*
                )?
                $(
                    pub mod $variant_mod;
                    use self::$variant_mod::$variant;
                )*
                $crate::macros::classes::classes!(@declare_class_kind_impl $class { $($pattern => $variant),* });
            }
        }
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
                        crate::error::UnimplementedClassError::new(resource.name, resource.class_name, version, platform).into(),
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
                        crate::error::UnimplementedClassError::new(resource.name, resource.class_name, version, platform).into(),
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
}

pub(crate) use classes;
