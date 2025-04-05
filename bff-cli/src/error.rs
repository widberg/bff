use bff::BffError;
use bff::names::Name;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum BffCliError {
    Bff(BffError),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    StripPrefix(std::path::StripPrefixError),
    #[display(
        "No filler found in length range [{}, {}], consider expanding the range",
        min_filler_length,
        max_filler_length
    )]
    NoFillerFound {
        min_filler_length: usize,
        max_filler_length: usize,
    },
    #[display("Found duplicate resource with name {}", name)]
    DuplicateResource {
        name: Name,
    },
}

pub type BffCliResult<T> = Result<T, BffCliError>;
