use std::ffi::OsStr;

use binrw::Endian;

pub enum Platform {
    PC,
    UWP,
    Maci386,
    MacPPC,
    PS2,
    PS3,
    PSP,
    Xbox,
    Xbox360,
    GameCube,
    Wii,
    Switch,
}

pub fn extension_to_platform(extension: &OsStr) -> Option<Platform> {
    match extension.to_ascii_uppercase().to_str() {
        Some("DPC") => Some(Platform::PC),
        Some("DUA") => Some(Platform::UWP),
        Some("DMC") => Some(Platform::Maci386),
        Some("DBM") => Some(Platform::MacPPC),
        Some("DPS") => Some(Platform::PS2),
        Some("DP3") => Some(Platform::PS3),
        Some("DPP") => Some(Platform::PSP),
        Some("DXB") => Some(Platform::Xbox),
        Some("D36") => Some(Platform::Xbox360),
        Some("DGC") => Some(Platform::GameCube),
        Some("DRV") => Some(Platform::Wii),
        Some("DNX") => Some(Platform::Switch),
        _ => None,
    }
}

pub fn platform_to_extension(platform: Platform) -> &'static OsStr {
    match platform {
        Platform::PC => OsStr::new("DPC"),
        Platform::UWP => OsStr::new("DUA"),
        Platform::Maci386 => OsStr::new("DMC"),
        Platform::MacPPC => OsStr::new("DBM"),
        Platform::PS2 => OsStr::new("DPS"),
        Platform::PS3 => OsStr::new("DP3"),
        Platform::PSP => OsStr::new("DPP"),
        Platform::Xbox => OsStr::new("DXB"),
        Platform::Xbox360 => OsStr::new("D36"),
        Platform::GameCube => OsStr::new("DGC"),
        Platform::Wii => OsStr::new("DRV"),
        Platform::Switch => OsStr::new("DNX"),
    }
}

pub fn platform_to_endian(platform: Platform) -> Endian {
    match platform {
        Platform::PC => Endian::Little,
        Platform::UWP => Endian::Little,
        Platform::Maci386 => Endian::Little,
        Platform::MacPPC => Endian::Big,
        Platform::PS2 => Endian::Little,
        Platform::PS3 => Endian::Big,
        Platform::PSP => Endian::Little,
        Platform::Xbox => Endian::Big,
        Platform::Xbox360 => Endian::Big,
        Platform::GameCube => Endian::Big,
        Platform::Wii => Endian::Big,
        Platform::Switch => Endian::Little,
    }
}

pub fn extension_to_endian(extension: &OsStr) -> Option<Endian> {
    match extension_to_platform(extension) {
        Some(platform) => Some(platform_to_endian(platform)),
        None => None,
    }
}
