use std::ffi::OsStr;

use binrw::Endian;
use derive_more::Display;

use crate::error::{Error, InvalidExtensionError};
use crate::BffResult;

macro_rules! platforms_enum {
    ($($i:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
        pub enum Platform {
            $($i,)*
        }
    };
}

macro_rules! extensions_to_platforms {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        impl TryFrom<&OsStr> for Platform {
            type Error = Error;

            fn try_from(extension: &OsStr) -> Result<Self, Self::Error> {
                match extension.to_ascii_uppercase().to_str() {
                    $(Some($s) => Ok(Platform::$i),)*
                    _ => Err(InvalidExtensionError::new(extension.to_os_string()).into()),
                }
            }
        }
    };
}

macro_rules! platforms_to_extensions {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        impl From<Platform> for &'static OsStr {
            fn from(platform: Platform) -> Self {
                match platform {
                    $(Platform::$i => OsStr::new($s),)*
                }
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
    ($(($i:ident,$s:literal,$e:ident)),* $(,)?) => {
        platforms_enum!($($i),*);
        extensions_to_platforms!($(($i,$s)),*);
        platforms_to_extensions!($(($i,$s)),*);
        platforms_to_endian!($(($i,$e)),*);
    };
}

platforms! {
    (PC, "DPC", Little),
    (UWP, "DUA", Little),
    (Maci386, "DMC", Little),
    (MacPPC, "DBM", Big),
    (PS2, "DPS", Little),
    (PS3, "DP3", Big),
    (PSP, "DPP", Little),
    (Xbox, "DXB", Big),
    (Xbox360, "D36", Big),
    (GameCube, "DGC", Big),
    (Wii, "DRV", Big),
    (Switch, "DNX", Little),
}

pub fn try_extension_to_endian(extension: &OsStr) -> BffResult<Endian> {
    extension.try_into().map(<Platform as Into<Endian>>::into)
}
