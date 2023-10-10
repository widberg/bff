use bff::names::Name;
use bff::BffError;
use derive_more::{Constructor, Display, Error, From};

#[derive(Debug, Constructor, Display, Error)]
#[display(fmt = "failed to find resource {}", resource_name)]
pub struct InvalidResourceError {
    pub resource_name: Name,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(fmt = "failed to find preview for resource {}", resource_name)]
pub struct InvalidPreviewError {
    pub resource_name: Name,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(
    fmt = "unimplemented exporter for resource {} of class {}",
    resource_name,
    class_name
)]
pub struct UnimplementedExporterError {
    pub resource_name: Name,
    pub class_name: Name,
}

#[derive(Debug, Display, Error, From)]
pub enum Error {
    Bff(BffError),
    InvalidResource(InvalidResourceError),
    InvalidPreview(InvalidPreviewError),
    UnimplementedExporter(UnimplementedExporterError),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    AnsiToHtml(ansi_to_html::Error),
    Dds(ddsfile::Error),
    CreateImage(image_dds::error::CreateImageError),
    Image(image::error::ImageError),
    Hound(hound::Error),
    De(quick_xml::DeError),
    Decode(base64::DecodeError),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&ansi_to_html::convert_escaped(self.to_string().as_ref()).unwrap())
    }
}

pub type BffGuiResult<T> = Result<T, Error>;
