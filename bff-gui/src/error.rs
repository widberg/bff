#![cfg(not(target_arch = "wasm32"))]

use derive_more::From;

#[derive(Debug, From)]
pub enum BffGuiError {
    Io(std::io::Error),
    EFrame(eframe::Error),
    Hound(hound::Error),
    Other(String),
}

pub type BffGuiResult<T> = Result<T, BffGuiError>;
