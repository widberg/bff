macro_rules! platforms {
    (
        styles: [$($style:ident),* $(,)?],
        platforms: [
            $($platform:ident($styles:tt,$endian:ident)),* $(,)?
        ]
    ) => {
        $crate::macros::platforms::platforms!(@emit_styles $($style),*);
        $crate::macros::platforms::platforms!(@emit_platforms $($platform),*);
        $crate::macros::platforms::platforms!(@emit_try_from_extension_impls $(($platform, $styles)),*);
        $crate::macros::platforms::platforms!(@emit_try_platform_style_to_extension_fn $(($platform, $styles)),*);
        $crate::macros::platforms::platforms!(@emit_extensions_fn $(($platform, $styles)),*);
        $crate::macros::platforms::platforms!(@emit_try_platform_style_to_name_extension_fn $(($platform, $styles)),*);
        $crate::macros::platforms::platforms!(@emit_name_extensions_fn $(($platform, $styles)),*);
        $crate::macros::platforms::platforms!(@emit_endian_impl $($platform($endian)),*);
    };

    (@emit_styles $($style:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, serde::Serialize, serde::Deserialize, derive_more::FromStr, schemars::JsonSchema)]
        pub enum Style {
            $($style,)*
        }
    };

    (@emit_platforms $($platform:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, serde::Serialize, serde::Deserialize, derive_more::FromStr, binrw::BinRead, binrw::BinWrite, schemars::JsonSchema)]
        #[brw(repr = u8)]
        pub enum Platform {
            $($platform,)*
        }
    };

    (@emit_try_from_extension_impls $(($platform:ident,[$($style:ident($extension:literal, $name_extension:literal)),* $(,)?])),* $(,)?) => {
        impl TryFrom<&std::ffi::OsStr> for Platform {
            type Error = $crate::error::Error;

            fn try_from(extension: &std::ffi::OsStr) -> Result<Self, Self::Error> {
                match extension {
                    $($(extension if extension.eq_ignore_ascii_case($extension) => Ok(Platform::$platform),)*)*
                    _ => Err($crate::error::InvalidExtensionError::new(extension.to_os_string()).into()),
                }
            }
        }

        impl TryFrom<&std::ffi::OsStr> for Style {
            type Error = $crate::error::Error;

            fn try_from(extension: &std::ffi::OsStr) -> Result<Self, Self::Error> {
                match extension {
                    $($(extension if extension.eq_ignore_ascii_case($extension) => Ok(Style::$style),)*)*
                    _ => Err($crate::error::InvalidExtensionError::new(extension.to_os_string()).into()),
                }
            }
        }
    };

    (@emit_try_platform_style_to_extension_fn $(($platform:ident,[$($style:ident($extension:literal, $name_extension:literal)),* $(,)?])),* $(,)?) => {
        pub fn try_platform_style_to_extension(
            platform: Platform,
            style: Style,
        ) -> $crate::BffResult<&'static std::ffi::OsStr> {
            match (platform, style) {
                $($((Platform::$platform, Style::$style) => Ok(std::ffi::OsStr::new($extension)),)*)*
                _ => Err($crate::error::InvalidPlatformStyleError::new(platform, style).into()),
            }
        }
    };

    (@emit_extensions_fn $(($platform:ident,[$($style:ident($extension:literal, $name_extension:literal)),* $(,)?])),* $(,)?) => {
        pub fn extensions() -> Vec<&'static std::ffi::OsStr> {
            Vec::from([
                $($(std::ffi::OsStr::new($extension),)*)*
            ])
        }
    };

    (@emit_try_platform_style_to_name_extension_fn $(($platform:ident,[$($style:ident($extension:literal, $name_extension:literal)),* $(,)?])),* $(,)?) => {
        pub fn try_platform_style_to_name_extension(
            platform: Platform,
            style: Style,
        ) -> $crate::BffResult<&'static std::ffi::OsStr> {
            match (platform, style) {
                $($((Platform::$platform, Style::$style) => Ok(std::ffi::OsStr::new($name_extension)),)*)*
                _ => Err($crate::error::InvalidPlatformStyleError::new(platform, style).into()),
            }
        }
    };

    (@emit_name_extensions_fn $(($platform:ident,[$($style:ident($extension:literal, $name_extension:literal)),* $(,)?])),* $(,)?) => {
        pub fn name_extensions() -> Vec<&'static std::ffi::OsStr> {
            Vec::from([
                $($(std::ffi::OsStr::new($name_extension),)*)*
            ])
        }
    };

    (@emit_endian_impl $($platform:ident($endian:ident)),* $(,)?) => {
        impl From<Platform> for binrw::Endian {
            fn from(platform: Platform) -> Self {
                match platform {
                    $(Platform::$platform => binrw::Endian::$endian,)*
                }
            }
        }
    };
}

pub(crate) use platforms;
