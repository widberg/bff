use std::ffi::OsStr;

use binrw::Endian;
use derive_more::Display;
use serde::Serialize;

use crate::error::{InvalidExtensionError, InvalidPlatformStyleError};
use crate::BffResult;

macro_rules! styles_enum {
    ($($a:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize)]
        pub enum Style {
            $($a,)*
        }
    };
}

macro_rules! platforms_enum {
    ($($i:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, Serialize)]
        pub enum Platform {
            $($i,)*
        }
    };
}

macro_rules! extensions_to_platforms {
    ($(($i:ident,[$($c:ident($s:literal)),* $(,)?])),* $(,)?) => {
        impl TryFrom<&OsStr> for Platform {
            type Error = InvalidExtensionError;

            fn try_from(extension: &OsStr) -> Result<Self, Self::Error> {
                match extension.to_ascii_uppercase().to_str() {
                    $($(Some($s) => Ok(Platform::$i),)*)*
                    _ => Err(InvalidExtensionError::new(extension.to_os_string())),
                }
            }
        }

        impl TryFrom<&OsStr> for Style {
            type Error = InvalidExtensionError;

            fn try_from(extension: &OsStr) -> Result<Self, Self::Error> {
                match extension.to_ascii_uppercase().to_str() {
                    $($(Some($s) => Ok(Style::$c),)*)*
                    _ => Err(InvalidExtensionError::new(extension.to_os_string())),
                }
            }
        }
    };
}

macro_rules! platforms_to_extensions {
    ($(($i:ident,[$($c:ident($s:literal)),* $(,)?])),* $(,)?) => {
        pub fn try_platform_style_to_extension(platform: Platform, style: Style) -> BffResult<&'static OsStr> {
            match (platform, style) {
                $($((Platform::$i, Style::$c) => Ok(OsStr::new($s)),)*)*
                _ => Err(InvalidPlatformStyleError::new(platform, style).into()),
            }
        }
    };
}

macro_rules! platforms_to_endian {
    ($(($i:ident,$e:ident)),* $(,)?) => {
        impl From<Platform> for Endian {
            fn from(platform: Platform) -> Self {
                match platform {
                    $(Platform::$i => Endian::$e,)*
                }
            }
        }
    };
}

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

platforms! {
    styles: [BF, D, DB],
    platforms: [
        PC([D("DPC"), BF("BFPC"), DB("DBC")], Little),
        UWP([D("DUA")], Little),
        Maci386([D("DMC")], Little),
        MacPPC([D("DBM")], Big),
        PS2([D("DPS")], Little),
        PS3([D("DP3")], Big),
        PSP([D("DPP")], Little),
        Xbox([D("DXB")], Big),
        Xbox360([D("D36")], Big),
        GameCube([D("DGC")], Big),
        Wii([D("DRV")], Big),
        Switch([D("DNX")], Little),
    ]
}

pub fn try_extension_to_endian(extension: &OsStr) -> Result<Endian, InvalidExtensionError> {
    extension.try_into().map(<Platform as Into<Endian>>::into)
}
