use std::path::Path;

use bff::names::Name;

use crate::error::BffGuiResult;

pub trait Export {
    fn export(&self, export_path: &Path, name: Name) -> BffGuiResult<String>;
}
