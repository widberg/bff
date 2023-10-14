use bff::names::Name;
use bff::platforms::Platform;
use bff::versions::Version;
use bff::BffError;
use derive_more::{Constructor, Display, Error, From};

#[derive(Debug, Constructor, Display, Error)]
#[display(
    fmt = "unimplemented exporter for class {} (version: {}, platform: {}) for resource {}",
    class_name,
    version,
    platform,
    object_name
)]
pub struct UnimplementedExporterError {
    pub object_name: Name,
    pub class_name: Name,
    pub version: Version,
    pub platform: Platform,
}

#[derive(Debug, Display, Error, From)]
pub enum BffGuiError {
    Bff(BffError),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    UnimplementedExporter(UnimplementedExporterError),
}

pub type BffGuiResult<T> = Result<T, BffGuiError>;
