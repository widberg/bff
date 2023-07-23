use std::ffi::OsString;

use derive_more::{Constructor, Display, Error, From};

use crate::name::Name;
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
    object_name: Name,
    class_name: Name,
    version: Version,
    platform: Platform,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(fmt = "Invalid BigFile extension {:#?}", extension)]
pub struct InvalidExtensionError {
    extension: OsString,
}

#[derive(Debug, Display, Error, From)]
pub enum Error {
    UnimplementedClass(UnimplementedClassError),
    InvalidExtension(InvalidExtensionError),
    BinRWError(binrw::Error),
}
