use std::path::Path;

use bff::names::Name;

use crate::{error::BffGuiResult, PreviewData};

pub trait Export {
    fn export(&self, export_path: &Path, name: Name) -> BffGuiResult<PreviewData>;
}
