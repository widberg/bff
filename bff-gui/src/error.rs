use derive_more::From;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, From)]
pub enum BffGuiError {
    Io(std::io::Error),
    EFrame(eframe::Error),
    Hound(hound::Error),
    Other(String),
}

#[cfg(not(target_arch = "wasm32"))]
pub type BffGuiResult<T> = Result<T, BffGuiError>;
