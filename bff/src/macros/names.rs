macro_rules! names {
    (
        styles: [$($style:ident($style_transform:expr)),* $(,)?],
        names: [
            $($name:ident($name_style:ident, $name_target:ty, $name_display:ty, $name_hash:path)),* $(,)?
        ]
    ) => {
        $crate::macros::names::names!(@emit_empty_hash_asserts
            $($name_hash),*
        );
        $crate::macros::names::names!(@emit_hash_function_types
            $($name($name_target, $name_display, $name_hash)),*
        );
        $crate::macros::names::names!(@emit_name_styles
            $($style($style_transform)),*
        );
        $crate::macros::names::names!(@emit_name_types
            $($name($name_style)),*
        );
        $crate::macros::names::names!(@emit_name_type_impl
            $($name($name_target)),*
        );
    };

    (@emit_empty_hash_asserts $($name_hash:path),* $(,)?) => {
        const _: () = {
            $(assert!($name_hash(b"") == 0);)*
        };
    };

    (@emit_hash_function_types $($name:ident($name_target:ty, $name_display:ty, $name_hash:path)),* $(,)?) => {
        $(
            pastey::paste! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                struct [<$name NameHashFunction>];

                impl $crate::traits::NameHashFunction for [<$name NameHashFunction>] {
                    type Target = $name_target;
                    type Display = $name_display;

                    fn hash(bytes: &[u8]) -> Self::Target {
                        num_traits::AsPrimitive::<Self::Target>::as_($name_hash(bytes))
                    }
                }
            }
        )*
    };

    (@emit_name_styles $($style:ident($style_transform:expr)),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum NameStyle {
            $($style,)*
        }

        pub const ALL_NAME_STYLES: &[NameStyle] = &[
            $(NameStyle::$style,)*
        ];

        pub const fn name_style_transform(style: NameStyle) -> fn(&str) -> String {
            match style {
                $(NameStyle::$style => $style_transform,)*
            }
        }
    };

    (@emit_name_types $($name:ident($name_style:ident)),* $(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, derive_more::FromStr)]
        pub enum NameType {
            $($name,)*
        }

        pub const fn name_type_style(name_type: NameType) -> NameStyle {
            match name_type {
                $(NameType::$name => NameStyle::$name_style,)*
            }
        }
    };

    (@emit_name_type_impl $($name:ident($name_target:ty)),* $(,)?) => {
        pastey::paste! {
            impl NameType {
                pub const fn target_bits(self) -> usize {
                    match self {
                        $(
                            NameType::$name => core::mem::size_of::<$name_target>() * 8,
                        )*
                    }
                }

                pub fn hash_bytes(self, bytes: &[u8]) -> $crate::names::Name {
                    match self {
                        $(NameType::$name => $crate::names::value::hash_bytes_for_hash::<[<$name NameHashFunction>]>(bytes),)*
                    }
                }

                pub fn parse_forced_hash_name<S: AsRef<str>>(self, string: S) -> Option<($crate::names::Name, String)> {
                    match self {
                        $(NameType::$name => $crate::names::value::parse_forced_hash_name_for_hash::<[<$name NameHashFunction>], S>(string),)*
                    }
                }

                pub fn parse_name_value<S: AsRef<str>>(self, string: S) -> Option<$crate::names::Name> {
                    match self {
                        $(NameType::$name => $crate::names::value::parse_name_value_for_hash::<[<$name NameHashFunction>], S>(string),)*
                    }
                }

                pub fn value_string_from_name(self, name: $crate::names::Name) -> String {
                    match self {
                        $(NameType::$name => $crate::names::value::name_value_string_for_hash::<[<$name NameHashFunction>]>(name),)*
                    }
                }

                pub fn fmt_name_value(
                    self,
                    name: $crate::names::Name,
                    f: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    match self {
                        $(NameType::$name => $crate::names::value::fmt_name_for_hash::<[<$name NameHashFunction>]>(name, f),)*
                    }
                }

                pub fn read_name<R: std::io::Read + std::io::Seek>(
                    self,
                    reader: &mut R,
                    endian: binrw::Endian,
                ) -> binrw::BinResult<$crate::names::Name> {
                    match self {
                        $(NameType::$name => $crate::names::value::read_name_for_hash::<[<$name NameHashFunction>], R>(reader, endian),)*
                    }
                }

                pub fn write_name<W: std::io::Write + std::io::Seek>(
                    self,
                    writer: &mut W,
                    endian: binrw::Endian,
                    name: $crate::names::Name,
                ) -> binrw::BinResult<()> {
                    match self {
                        $(NameType::$name => $crate::names::value::write_name_for_hash::<[<$name NameHashFunction>], W>(writer, endian, name),)*
                    }
                }

                pub fn serialize_name_value<S: serde::Serializer>(
                    self,
                    name: $crate::names::Name,
                    serializer: S,
                ) -> Result<S::Ok, S::Error> {
                    match self {
                        $(NameType::$name => $crate::names::serde_schema::serialize_name_value_for_hash::<[<$name NameHashFunction>], S>(name, serializer),)*
                    }
                }

                pub fn deserialize_name<'de, D, F>(
                    self,
                    deserializer: D,
                    add_name: F,
                ) -> Result<$crate::names::Name, D::Error>
                where
                    D: serde::Deserializer<'de>,
                    F: FnMut(&str),
                {
                    match self {
                        $(NameType::$name => $crate::names::serde_schema::deserialize_name_for_hash::<[<$name NameHashFunction>], D, F>(deserializer, self, add_name),)*
                    }
                }
            }
        }
    };
}

pub(crate) use names;
