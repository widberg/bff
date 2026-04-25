macro_rules! names {
    (
        styles: [$($style:ident($style_transform:expr)),* $(,)?],
        names: [
            $($name:ident($name_style:ident, $name_hash_function:ty)),* $(,)?
        ]
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum NameStyle {
            $($style,)*
        }

        pub const fn name_style_transform(style: NameStyle) -> fn(&str) -> String {
            match style {
                $(NameStyle::$style => $style_transform,)*
            }
        }

        #[inline]
        pub fn apply_name_style<S: AsRef<str>>(string: S, style: NameStyle) -> String {
            name_style_transform(style)(string.as_ref())
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, derive_more::FromStr)]
        pub enum NameType {
            $($name,)*
        }

        pub const fn name_type_style(name_type: NameType) -> NameStyle {
            match name_type {
                $(NameType::$name => NameStyle::$name_style,)*
            }
        }

        impl NameType {
            pub const fn target_bits(self) -> usize {
                match self {
                    $(
                        NameType::$name => core::mem::size_of::<<$name_hash_function as $crate::traits::NameHashFunction>::Target>() * 8,
                    )*
                }
            }

            pub const fn is_32_bit(self) -> bool {
                self.target_bits() == 32
            }

            pub fn hash_bytes(self, bytes: &[u8]) -> $crate::names::Name {
                match self {
                    $(NameType::$name => $crate::names::value::hash_bytes_for_hash::<$name_hash_function>(bytes),)*
                }
            }

            pub fn parse_forced_hash_name<S: AsRef<str>>(self, string: S) -> Option<($crate::names::Name, String)> {
                match self {
                    $(NameType::$name => $crate::names::value::parse_forced_hash_name_for_hash::<$name_hash_function, S>(string),)*
                }
            }

            pub fn name_from_i32(self, value: i32) -> $crate::names::Name {
                match self {
                    $(NameType::$name => $crate::names::value::name_from_i32_for_hash::<$name_hash_function>(value),)*
                }
            }

            pub fn value_from_name(self, name: $crate::names::Name) -> i64 {
                match self {
                    $(NameType::$name => $crate::names::value::name_value_for_hash::<$name_hash_function>(name),)*
                }
            }

            pub fn read_name<R: std::io::Read + std::io::Seek>(
                self,
                reader: &mut R,
                endian: binrw::Endian,
            ) -> binrw::BinResult<$crate::names::Name> {
                match self {
                    $(NameType::$name => $crate::names::value::read_name_for_hash::<$name_hash_function, R>(reader, endian),)*
                }
            }

            pub fn write_name<W: std::io::Write + std::io::Seek>(
                self,
                writer: &mut W,
                endian: binrw::Endian,
                name: $crate::names::Name,
            ) -> binrw::BinResult<()> {
                match self {
                    $(NameType::$name => $crate::names::value::write_name_for_hash::<$name_hash_function, W>(writer, endian, name),)*
                }
            }

            pub fn serialize_name_value<S: serde::Serializer>(
                self,
                name: $crate::names::Name,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                match self {
                    $(NameType::$name => $crate::names::serde_schema::serialize_name_value_for_hash::<$name_hash_function, S>(name, serializer),)*
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
                    $(NameType::$name => $crate::names::serde_schema::deserialize_name_for_hash::<$name_hash_function, D, F>(deserializer, self, add_name),)*
                }
            }
        }
    };
}

pub(crate) use names;
