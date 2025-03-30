macro_rules! extensions_to_platforms {
    ($(($platform:ident,[$($style:ident($extension:literal, $name_extension:literal)),* $(,)?])),* $(,)?) => {
        impl TryFrom<&std::ffi::OsStr> for Platform {
            type Error = crate::error::Error;

            fn try_from(extension: &std::ffi::OsStr) -> Result<Self, Self::Error> {
                match extension {
                    $($(extension if extension.eq_ignore_ascii_case($extension) => Ok(Platform::$platform),)*)*
                    _ => Err(crate::error::InvalidExtensionError::new(extension.to_os_string()).into()),
                }
            }
        }

        impl TryFrom<&std::ffi::OsStr> for Style {
            type Error = crate::error::Error;

            fn try_from(extension: &std::ffi::OsStr) -> Result<Self, Self::Error> {
                match extension {
                    $($(extension if extension.eq_ignore_ascii_case($extension) => Ok(Style::$style),)*)*
                    _ => Err(crate::error::InvalidExtensionError::new(extension.to_os_string()).into()),
                }
            }
        }

        pub fn try_platform_style_to_extension(platform: Platform, style: Style) -> crate::BffResult<&'static std::ffi::OsStr> {
            match (platform, style) {
                $($((Platform::$platform, Style::$style) => Ok(std::ffi::OsStr::new($extension)),)*)*
                _ => Err(crate::error::InvalidPlatformStyleError::new(platform, style).into()),
            }
        }

        pub fn extensions() -> Vec<&'static std::ffi::OsStr> {
            Vec::from([
                $($(std::ffi::OsStr::new($extension),)*)*
            ])
        }

        pub fn try_platform_style_to_name_extension(platform: Platform, style: Style) -> crate::BffResult<&'static std::ffi::OsStr> {
            match (platform, style) {
                $($((Platform::$platform, Style::$style) => Ok(std::ffi::OsStr::new($name_extension)),)*)*
                _ => Err(crate::error::InvalidPlatformStyleError::new(platform, style).into()),
            }
        }

        pub fn name_extensions() -> Vec<&'static std::ffi::OsStr> {
            Vec::from([
                $($(std::ffi::OsStr::new($name_extension),)*)*
            ])
        }
    };
}

pub(crate) use extensions_to_platforms;

macro_rules! platforms {
    (
        styles: [$($style:ident),* $(,)?],
        platforms: [
            $($platform:ident($styles:tt,$endian:ident)),* $(,)?
        ]
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, serde::Serialize, serde::Deserialize, derive_more::FromStr)]
        pub enum Style {
            $($style,)*
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, serde::Serialize, serde::Deserialize, derive_more::FromStr)]
        pub enum Platform {
            $($platform,)*
        }

        crate::macros::platforms::extensions_to_platforms!($(($platform,$styles)),*);

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
