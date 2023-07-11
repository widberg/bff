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

#[derive(Debug, Display, Error, From)]
pub enum Error {
    UnimplementedClass(UnimplementedClassError),
    BinRWError(binrw::Error),
}
