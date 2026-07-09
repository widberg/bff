use bff::BffError;
use bff::bigfile::platforms::Platform;
use bff::names::Name;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum BffCliError {
    Bff(BffError),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    StripPrefix(std::path::StripPrefixError),
    #[display("Found duplicate resource with name {}", name)]
    DuplicateResource {
        name: Name,
    },
    #[display("BigFile has no extension: {}", path.display())]
    MissingBigFileExtension {
        path: std::path::PathBuf,
    },
    #[display(
        "Manifest platform {} does not match the platform {} implied by the BigFile extension: {}",
        manifest_platform,
        path_platform,
        path.display()
    )]
    PlatformMismatch {
        manifest_platform: Platform,
        path_platform: Platform,
        path: std::path::PathBuf,
    },
}

pub type BffCliResult<T> = Result<T, BffCliError>;
