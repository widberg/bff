use bff::BffError;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum BffCliError {
    Bff(BffError),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
}

pub type BffCliResult<T> = Result<T, BffCliError>;
