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
    };

    (@emit_try_from_resource $($class:ident)*) => {
        impl crate::traits::FromResource for Class {
            fn from_resource(
                resource: &crate::bigfile::resource::Resource,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<Class> {
                let name_type = version.name_type()?;
                if let Some(class_type) = ClassType::from_name_and_type(resource.class_name, name_type) {
                    return match class_type {
                        $(
                            ClassType::$class => Ok(<$crate::macros::classes::classes!(@class_ty $class) as crate::traits::FromResource>::from_resource(resource, version, platform, name_context)?.into()),
                        )*
                    };
                }
                Err(crate::error::UnimplementedClassError::new(resource.name, resource.class_name, version, platform).into())
            }
        }
    };

    (@emit_try_into_resource $($class:ident)*) => {
        impl crate::traits::ToResource for Class {
            fn to_resource(
                &self,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                match self {
                    $(Class::$class(class) => <$crate::macros::classes::classes!(@class_ty $class) as crate::traits::ToResource>::to_resource(class, version, platform, name_context),)*
                }
            }
        }
    };

    (@emit_class_names_fn $($class:ident)*) => {
        pub const fn class_base_names() -> &'static[&'static str] {
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

        impl crate::traits::FromResource for $class {
            fn from_resource(
                resource: &crate::bigfile::resource::Resource,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
                _name_context: &crate::names::NameContext,
            ) -> crate::BffResult<$class> {
                Err(
                        crate::error::UnimplementedClassError::new(resource.name, resource.class_name, version, platform).into(),
                    )
            }
        }

        impl crate::traits::ToResource for $class {
            fn to_resource(
                &self,
                _version: crate::bigfile::versions::Version,
                _platform: crate::bigfile::platforms::Platform,
                _name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                match *self {}
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

        impl crate::traits::FromResource for $class {
            fn from_resource(
                resource: &crate::bigfile::resource::Resource,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<$class> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                match (version.clone(), platform) {
                    $($pattern => {
                        let shadow_class: $variant = <$variant as crate::traits::FromResource>::from_resource(resource, version, platform, name_context)?;
                        Ok(std::boxed::Box::new(shadow_class).into())
                    })*
                    _ => Err(
                        crate::error::UnimplementedClassError::new(resource.name, resource.class_name, version, platform).into(),
                    ),
                }
            }
        }

        impl crate::traits::ToResource for $class {
            #[expect(unused_imports)]
            fn to_resource(
                &self,
                version: crate::bigfile::versions::Version,
                platform: crate::bigfile::platforms::Platform,
                name_context: &crate::names::NameContext,
            ) -> crate::BffResult<crate::bigfile::resource::Resource> {
                use crate::bigfile::platforms::Platform::*;
                use crate::bigfile::versions::Version::*;
                match self {
                    $($class::$variant(class) => {
                        <$variant as crate::traits::ToResource>::to_resource(class, version, platform, name_context)
                    })*
                }
            }
        }

    };
}

pub(crate) use classes;
