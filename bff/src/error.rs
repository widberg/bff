use std::ffi::OsString;

use derive_more::{Constructor, Display, Error, From};

use crate::names::Name;
use crate::platforms::Platform;
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
#[display(fmt = "Invalid BigFile extension {:#?}", extension)]
pub struct InvalidExtensionError {
    pub extension: OsString,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(fmt = "Unknown BigFile version {}", version)]
pub struct InvalidVersionError {
    pub version: String,
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
    InvalidVersion(InvalidVersionError),
    Io(std::io::Error),
    MismatchCrc32(MismatchCrc32Error),
    ParseInt(std::num::ParseIntError),
    UnimplementedClass(UnimplementedClassError),
    Utf8(std::string::FromUtf8Error),
}
