use std::ffi::OsString;

use derive_more::{Constructor, Display, Error, From};

use crate::bigfile::platforms::{Platform, Style};
use crate::bigfile::versions::Version;
use crate::names::Name;

#[derive(Debug, Constructor, Display, Error)]
#[display(
    "unimplemented class {} (version: {}, platform: {}) for object {}",
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
    "Unsupported BigFile version, platform combination: {}, {}",
    version,
    platform
)]
pub struct UnimplementedVersionPlatformError {
    pub version: Version,
    pub platform: Platform,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Unsupported BigFile version: {}", version)]
pub struct UnimplementedVersionError {
    pub version: Version,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Invalid BigFile extension {:#?}", extension)]
pub struct InvalidExtensionError {
    pub extension: OsString,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Invalid Platform/Style combination: {} {}", platform, style)]
pub struct InvalidPlatformStyleError {
    pub platform: Platform,
    pub style: Style,
}

#[derive(Debug, Display, Error, From)]
pub enum Error {
    BinRW(binrw::Error),
    Fmt(std::fmt::Error),
    InvalidExtension(InvalidExtensionError),
    InvalidPlatformStyle(InvalidPlatformStyleError),
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    UnimplementedClass(UnimplementedClassError),
    UnimplementedVersion(UnimplementedVersionError),
    UnimplementedVersionPlatform(UnimplementedVersionPlatformError),
    Utf8(std::string::FromUtf8Error),
}
