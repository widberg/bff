#[derive(Debug)]
pub struct SimpleError(pub String);

impl std::error::Error for SimpleError {}

impl std::fmt::Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GuiError {
    #[error(transparent)]
    Dds(#[from] ddsfile::Error),
    #[error(transparent)]
    CreateImage(#[from] image_dds::error::CreateImageError),
    #[error(transparent)]
    Image(#[from] image::error::ImageError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Hound(#[from] hound::Error),
    #[error(transparent)]
    Simple(#[from] SimpleError),
    #[error(transparent)]
    Bff(#[from] bff::error::Error),
    #[error(transparent)]
    De(#[from] quick_xml::DeError),
    #[error(transparent)]
    InvalidVersion(#[from] bff::error::InvalidVersionError),
    #[error(transparent)]
    AnsiToHtml(#[from] ansi_to_html::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl serde::Serialize for GuiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&ansi_to_html::convert_escaped(self.to_string().as_ref()).unwrap())
    }
}
