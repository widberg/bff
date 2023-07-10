use std::ffi::OsStr;

use binrw::Endian;

macro_rules! platforms_enum {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        pub enum Platform {
            $($i,)*
        }
    };
}

macro_rules! extensions_to_platforms {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        pub fn extension_to_platform(extension: &OsStr) -> Option<Platform> {
            match extension.to_ascii_uppercase().to_str() {
                $(Some($s) => Some(Platform::$i),)*
                _ => None,
            }
        }
    };
}

macro_rules! platforms_to_extensions {
    ($(($i:ident,$s:literal)),* $(,)?) => {
        pub fn platform_to_extension(platform: Platform) -> &'static OsStr {
            match platform {
                $(Platform::$i => OsStr::new($s),)*
            }
        }
    };
}

macro_rules! platforms_to_endian {
    ($(($i:ident,$e:ident)),* $(,)?) => {
        pub fn platform_to_endian(platform: Platform) -> Endian {
            match platform {
                $(Platform::$i => Endian::$e,)*
            }
        }
    };
}

macro_rules! platforms {
    ($(($i:ident,$s:literal,$e:ident)),* $(,)?) => {
        platforms_enum!($(($i,$s)),*);
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

pub fn extension_to_endian(extension: &OsStr) -> Option<Endian> {
    match extension_to_platform(extension) {
        Some(platform) => Some(platform_to_endian(platform)),
        None => None,
    }
}
