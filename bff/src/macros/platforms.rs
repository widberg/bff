macro_rules! styles_enum {
    ($($a:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, serde::Serialize)]
        pub enum Style {
            $($a,)*
        }
    };
}

pub(crate) use styles_enum;

macro_rules! platforms_enum {
    ($($i:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, serde::Serialize)]
        pub enum Platform {
            $($i,)*
        }
    };
}

pub(crate) use platforms_enum;

macro_rules! extensions_to_platforms {
    ($(($i:ident,[$($c:ident($s:literal)),* $(,)?])),* $(,)?) => {
        impl TryFrom<&std::ffi::OsStr> for Platform {
            type Error = crate::error::Error;

            fn try_from(extension: &std::ffi::OsStr) -> Result<Self, Self::Error> {
                match extension.to_ascii_uppercase().to_str() {
                    $($(Some($s) => Ok(Platform::$i),)*)*
                    _ => Err(crate::error::InvalidExtensionError::new(extension.to_os_string()).into()),
                }
            }
        }

        impl TryFrom<&std::ffi::OsStr> for Style {
            type Error = crate::error::Error;

            fn try_from(extension: &std::ffi::OsStr) -> Result<Self, Self::Error> {
                match extension.to_ascii_uppercase().to_str() {
                    $($(Some($s) => Ok(Style::$c),)*)*
                    _ => Err(crate::error::InvalidExtensionError::new(extension.to_os_string()).into()),
                }
            }
        }
    };
}

pub(crate) use extensions_to_platforms;

macro_rules! platforms_to_extensions {
    ($(($i:ident,[$($c:ident($s:literal)),* $(,)?])),* $(,)?) => {
        pub fn try_platform_style_to_extension(platform: Platform, style: Style) -> crate::BffResult<&'static std::ffi::OsStr> {
            match (platform, style) {
                $($((Platform::$i, Style::$c) => Ok(std::ffi::OsStr::new($s)),)*)*
                _ => Err(crate::error::InvalidPlatformStyleError::new(platform, style).into()),
            }
        }
    };
}

pub(crate) use platforms_to_extensions;

macro_rules! platforms_to_endian {
    ($(($i:ident,$e:ident)),* $(,)?) => {
        impl From<Platform> for binrw::Endian {
            fn from(platform: Platform) -> Self {
                match platform {
                    $(Platform::$i => binrw::Endian::$e,)*
                }
            }
        }
    };
}

pub(crate) use platforms_to_endian;

macro_rules! platforms {
    (
        styles: [$($a:ident),* $(,)?],
        platforms: [
            $($i:ident($s:tt,$e:ident)),* $(,)?
        ]
    ) => {
            styles_enum!($($a),*);
            platforms_enum!($($i),*);
            extensions_to_platforms!($(($i,$s)),*);
            platforms_to_extensions!($(($i,$s)),*);
            platforms_to_endian!($(($i,$e)),*);
    };
}

pub(crate) use platforms;
