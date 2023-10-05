use std::ffi::OsString;

use derive_more::{Constructor, Display, Error, From};

use crate::names::Name;
use crate::platforms::{Platform, Style};
use crate::versions::Version;

#[derive(Debug, Constructor, Display, Error)]
#[display(
    fmt = "unimplemented class {} (version: {}, platform: {}) for object {}",
    class_name,
    version,
    platform,
    object_name
)]
pub struct UnimplementedClassError {
    pub object_name: Name,
    pub class_name: Name,
    pub version: Version,
    pub platform: Platform,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(
    fmt = "Unsupported BigFile version, platform combination: {}, {}",
    version,
    platform
)]
pub struct UnimplementedVersionPlatformError {
    pub version: Version,
    pub platform: Platform,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(fmt = "Invalid BigFile extension {:#?}", extension)]
pub struct InvalidExtensionError {
    pub extension: OsString,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(fmt = "Invalid Platform/Style combination: {} {}", platform, style)]
pub struct InvalidPlatformStyleError {
    pub platform: Platform,
    pub style: Style,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(
    fmt = "CRC-32 mismatch for {}: expected {}, actual {}",
    string,
    expected,
    actual
)]
pub struct MismatchCrc32Error {
    pub string: String,
    pub expected: Name,
    pub actual: Name,
}

#[derive(Debug, Display, Error, From)]
pub enum Error {
    BinRW(binrw::Error),
    InvalidExtension(InvalidExtensionError),
    InvalidPlatformStyle(InvalidPlatformStyleError),
    Io(std::io::Error),
    MismatchCrc32(MismatchCrc32Error),
    ParseInt(std::num::ParseIntError),
    UnimplementedClass(UnimplementedClassError),
    UnimplementedVersionPlatform(UnimplementedVersionPlatformError),
    Utf8(std::string::FromUtf8Error),
}
